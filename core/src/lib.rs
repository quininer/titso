mod common;
pub mod error;
pub mod kv;
pub mod primitive;
pub mod packet;

pub use common::suggest;
use snafu::{ OptionExt, ResultExt };
use serde_cbor as cbor;
use rand_core::{ RngCore, CryptoRng };
use common::generate;
use primitive::{ Kdf, KeyedHash, HashRng, Aead };
use packet::*;
use kv::{ KvStore, Table };

#[macro_export]
macro_rules! chars {
    ( numeric ) => { "0123456789" };
    ( alphabet_lowercase ) => { "abcdefghijklmnopqrstuvwxyz" };
    ( alphabet_uppercase ) => { "ABCDEFGHIJKLMNOPQRSTUVWXYZ" };
    ( punctuation_simple ) => { ",.;-=_+?~!@#" };
    ( punctuation_one ) => { ",./;'[]=-\\`" };
    ( punctuation_more ) => { "~!@#$%^&*()_+{}|:\"<>?" };

    ( $( $name:tt ),* ) => {
        concat!(
            $(
                $crate::chars!($name)
            ),*
        )
    }
}


pub struct Titso<Kv> {
    kv: Kv,
    mkey: [u8; 32],
}

impl<Kv: KvStore> Titso<Kv> {
    pub async fn open(kv: Kv, password: &[u8])
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

    pub async fn init<R: RngCore + CryptoRng>(kv: Kv, mut rng: R, password: &[u8])
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

    fn tag(&self, lable: &str, tags: &[impl AsRef<str>]) -> Tag {
        let mut itag = [0; 16];
        for tag in tags {
            let mut tmp = [0; 16];
            let mut hasher = KeyedHash::new(&self.mkey, b"tag");
            hasher.update(&lable.len().to_le_bytes());
            hasher.update(lable.as_bytes());
            hasher.update(tag.as_ref().as_bytes());
            hasher.finalize(&mut tmp);
            for i in 0..16 {
                itag[i] ^= tmp[i];
            }
        }
        Tag(itag)
    }

    pub fn derive(&self, tags: &[impl AsRef<str>], rule: &Rule) -> String {
        let mut hasher = KeyedHash::new(&self.mkey, b"derive");
        let Tag(kdf_tag) = self.tag("kdf", tags);
        hasher.update(&kdf_tag);
        hasher.update(&rule.count.to_le_bytes());
        hasher.update(&rule.length.to_le_bytes());
        hasher.update(&rule.chars.len().to_le_bytes());
        hasher.update(rule.chars.as_bytes());
        let mut rng = HashRng::from(hasher.xof());
        generate(&mut rng, rule)
    }

    pub async fn hint(&self, tags: &[impl AsRef<str>]) -> error::Result<Option<String>, Kv::Error> {
        let Tag(store_tag) = self.tag("store", tags);

        let hint = self.kv.open("hint").await.context(error::Db)?;
        let packet = match hint.get(&store_tag).await.context(error::Db)? {
            Some(packet) => packet,
            None => return Ok(None)
        };
        let Packet { mut data, tag: atag } = cbor::from_slice(&packet)
            .context(error::Cbor)?;

        let Tag(aead_tag) = self.tag("aead", tags);
        if Aead::new(&self.mkey, &aead_tag)
            .decrypt(b"hint", &mut data, &atag)
        {
            cbor::from_slice(&data)
                .map(Some)
                .context(error::Cbor)
        } else {
            Err(error::Error::Decrypt {})
        }
    }

    pub async fn get(&self, tags: &[impl AsRef<str>]) -> error::Result<Option<Item>, Kv::Error> {
        let Tag(store_tag) = self.tag("store", tags);

        let data = self.kv.open("data").await.context(error::Db)?;
        let packet = match data.get(&store_tag).await.context(error::Db)? {
            Some(packet) => packet,
            None => return Ok(None)
        };
        let Packet { mut data, tag: atag } = cbor::from_slice(&packet)
            .context(error::Cbor)?;

        let Tag(aead_tag) = self.tag("aead", tags);
        if Aead::new(&self.mkey, &aead_tag)
            .decrypt(b"data", &mut data, &atag)
        {
            cbor::from_slice(&data)
                .map(Some)
                .context(error::Cbor)
        } else {
            Err(error::Error::Decrypt {})
        }
    }

    pub async fn put(&self, tags: &[impl AsRef<str>], item: &Item) -> error::Result<(), Kv::Error> {
        let Tag(aead_tag) = self.tag("aead", tags);
        let mut buf = cbor::to_vec(item).context(error::Cbor)?;
        let mut atag = [0; 16];

        Aead::new(&self.mkey, &aead_tag)
            .encrypt(b"data", &mut buf, &mut atag);

        let packet = Packet { data: buf, tag: atag };
        let packet = cbor::to_vec(&packet).context(error::Cbor)?;

        let Tag(store_tag) = self.tag("store", tags);
        let data = self.kv.open("data").await.context(error::Db)?;
        data.put(&store_tag, &packet).await.context(error::Db)?;

        Ok(())
    }

    pub async fn del(&self, tags: &[impl AsRef<str>]) -> error::Result<bool, Kv::Error> {
        let Tag(store_tag) = self.tag("store", tags);
        let data = self.kv.open("data").await.context(error::Db)?;
        data.del(&store_tag).await.context(error::Db)
    }

    pub async fn export(&self) {
        unimplemented!()
    }

    pub async fn import(&self) {
        unimplemented!()
    }
}
