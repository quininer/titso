mod common;
pub mod error;
pub mod primitive;
pub mod packet;

pub use common::suggest;
use snafu::ResultExt;
use serde_cbor as cbor;
use rand_core::{ RngCore, CryptoRng };
use arrayref::{ array_ref, array_mut_ref };
use common::generate;
use primitive::{
    kdf::Kdf,
    keyedhash::KeyedHash,
    rng::HashRng,
    aead::Aead
};
use packet::*;

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


pub struct Titso {
    mkey: [u8; 32]
}

impl Titso {
    pub fn open(password: &[u8], buf: &[u8]) -> error::Result<Titso> {
        let MasterSecret { salt, secret } = cbor::from_slice(buf)
            .context(error::Cbor)?;

        let mut mkey = [0; 32];
        Kdf::default().derive(password, &salt, &mut mkey);

        for i in 0..32 {
            mkey[i] ^= secret[i];
        }

        Ok(Titso { mkey })
    }

    pub fn init<R: RngCore + CryptoRng>(mut rng: R, password: &[u8])
        -> error::Result<(Titso, Vec<u8>)>
    {
        let mut salt = [0; 32];
        let mut mkey = [0; 32];
        let mut secret = [0; 32];
        rng.fill_bytes(&mut salt);
        rng.fill_bytes(&mut mkey);

        Kdf::default()
            .derive(password, &mut salt, &mut secret);

        for i in 0..32 {
            secret[i] ^= mkey[i];
        }

        let buf = cbor::to_vec(&MasterSecret { salt, secret })
            .context(error::Cbor)?;

        Ok((Titso { mkey }, buf))
    }

    pub fn store_tag(&self, tags: &[impl AsRef<str>]) -> Tag {
        self.tag("store", tags)
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

    pub fn get(&self, tags: &[impl AsRef<str>], value: &[u8]) -> error::Result<Item> {
        let Tag(aead_tag) = self.tag("aead", tags);

        let mut value = value.to_vec();
        let (atag, buf) = value.split_at_mut(primitive::aead::TAG_LENGTH);
        let atag = array_ref!(atag, 0, primitive::aead::TAG_LENGTH);

        if Aead::new(&self.mkey, &aead_tag)
            .decrypt(b"item", buf, &atag)
        {
            cbor::from_slice(&buf).context(error::Cbor)
        } else {
            Err(error::Error::Decrypt {})
        }
    }

    pub fn put(&self, tags: &[impl AsRef<str>], item: &Item) -> error::Result<Vec<u8>> {
        let Tag(aead_tag) = self.tag("aead", tags);

        let mut value = vec![0; primitive::aead::TAG_LENGTH];
        cbor::to_writer(&mut value, item).context(error::Cbor)?;

        let (atag, buf) = value.split_at_mut(primitive::aead::TAG_LENGTH);
        let atag = array_mut_ref!(atag, 0, primitive::aead::TAG_LENGTH);

        Aead::new(&self.mkey, &aead_tag)
            .encrypt(b"item", buf, atag);

        Ok(value)
    }
}
