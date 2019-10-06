use snafu::{ OptionExt, ResultExt };
use serde_cbor as cbor;
use rand_core::{ RngCore, CryptoRng };
use crate::primitive::kdf::Kdf;
use crate::primitive::keyedhash::KeyedHash;
use crate::primitive::rng::HashRng;
use crate::primitive::aead::Aead;
use crate::common::generate;
use crate::packet::*;
use crate::kv::{ KvStore, Table };
use crate::error;


pub struct Titso<Kv> {
    kv: Kv,
    mkey: [u8; 32],
}

impl<Kv: KvStore> Titso<Kv> {
    pub async fn open(mut kv: Kv, password: &[u8])
        -> error::Result<Titso<Kv>, Kv::Error>
    {
        let metadata = kv.open("metadata")
            .await.context(error::Db)?;
        let secret = metadata.get(b"secret")
            .await.context(error::Db)?
            .context(error::Uninitialized)?;

        let MasterSecret { salt, secret } = cbor::from_slice(&secret)
            .context(error::Cbor)?;

        let mut mkey = [0; 32];
        Kdf::default().derive(password, &salt, &mut mkey);

        for i in 0..32 {
            mkey[i] ^= secret[i];
        }

        Ok(Titso { kv, mkey })
    }

    pub async fn init<R: RngCore + CryptoRng>(mut kv: Kv, mut rng: R, password: &[u8])
        -> error::Result<Titso<Kv>, Kv::Error>
    {
        let mut salt = [0; 32];
        let mut mkey = [0; 32];
        let mut secret = [0; 32];
        rng.fill_bytes(&mut salt);
        rng.fill_bytes(&mut mkey);

        Kdf::default().derive(password, &salt, &mut secret);

        for i in 0..32 {
            secret[i] ^= mkey[i];
        }

        let secret = cbor::to_vec(&MasterSecret { salt, secret })
            .context(error::Cbor)?;

        let metadata = kv.open("metadata").await.context(error::Db)?;
        metadata.put(b"secret", &secret).await.context(error::Db)?;

        Ok(Titso { kv, mkey })
    }

    pub fn tag(&self, tags: &[impl AsRef<str>]) -> Tag {
        let mut itag = [0; 16];
        for tag in tags {
            let mut tmp = [0; 16];
            let mut hasher = KeyedHash::new(&self.mkey, b"tag");
            hasher.update(tag.as_ref().as_bytes());
            hasher.finalize(&mut tmp);
            for i in 0..16 {
                itag[i] ^= tmp[i];
            }
        }
        Tag(itag)
    }

    pub fn derive(&self, Tag(itag): Tag, rule: &Rule) -> String {
        let mut hasher = KeyedHash::new(&self.mkey, b"derive");
        hasher.update(&itag);
        hasher.update(&rule.count.to_le_bytes());
        hasher.update(&rule.length.to_le_bytes());
        hasher.update(&rule.chars.len().to_le_bytes());
        hasher.update(rule.chars.as_bytes());
        let mut rng = HashRng::from(hasher.xof());
        generate(&mut rng, rule)
    }

    pub async fn hint(&mut self, Tag(itag): Tag) -> error::Result<Option<String>, Kv::Error> {
        let hint = self.kv.open("hint").await.context(error::Db)?;
        let packet = match hint.get(&itag).await.context(error::Db)? {
            Some(packet) => packet,
            None => return Ok(None)
        };
        let Packet { mut data, tag: atag } = cbor::from_slice(&packet)
            .context(error::Cbor)?;

        if Aead::new(&self.mkey, &itag)
            .decrypt(b"hint", &mut data, &atag)
        {
            cbor::from_slice(&data)
                .map(Some)
                .context(error::Cbor)
        } else {
            Err(error::Error::Decrypt {})
        }
    }

    pub async fn get(&mut self, Tag(itag): Tag) -> error::Result<Option<Item>, Kv::Error> {
        let data = self.kv.open("data").await.context(error::Db)?;
        let packet = match data.get(&itag).await.context(error::Db)? {
            Some(packet) => packet,
            None => return Ok(None)
        };
        let Packet { mut data, tag: atag } = cbor::from_slice(&packet)
            .context(error::Cbor)?;

        if Aead::new(&self.mkey, &itag)
            .decrypt(b"data", &mut data, &atag)
        {
            cbor::from_slice(&data)
                .map(Some)
                .context(error::Cbor)
        } else {
            Err(error::Error::Decrypt {})
        }
    }

    pub async fn put(&mut self, Tag(itag): Tag, item: &Item) -> error::Result<(), Kv::Error> {
        let mut buf = cbor::to_vec(item).context(error::Cbor)?;
        let mut atag = [0; 16];

        Aead::new(&self.mkey, &itag)
            .encrypt(b"data", &mut buf, &mut atag);

        let packet = Packet { data: buf, tag: atag };
        let packet = cbor::to_vec(&packet).context(error::Cbor)?;

        let data = self.kv.open("data").await.context(error::Db)?;
        data.put(&itag, &packet).await.context(error::Db)?;

        Ok(())
    }

    pub async fn del(&mut self, Tag(itag): Tag) -> error::Result<bool, Kv::Error> {
        let data = self.kv.open("data").await.context(error::Db)?;
        data.del(&itag).await.context(error::Db)
    }

    pub async fn export(&mut self) {
        unimplemented!()
    }

    pub async fn import(&mut self) {
        unimplemented!()
    }
}
