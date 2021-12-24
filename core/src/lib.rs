mod util;
pub mod error;
pub mod primitive;
pub mod shield;
pub mod packet;

pub use util::suggest;

use cbor4ii::serde as cbor;
use rand_core::{ RngCore, CryptoRng };
use arrayref::{ array_ref, array_mut_ref };
use seckey::zero;
use shield::{ SecShield, SecShieldGuard };
use util::generate;
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
    mkey: SecShield
}

pub struct TitsoRef<'a> {
    mkey: SecShieldGuard<'a>
}

impl Titso {
    pub fn open(password: &[u8], buf: &[u8]) -> error::Result<Titso> {
        let MasterSecret { salt, secret } = cbor::from_slice(buf)?;
        let salt = array_ref!(salt, 0, 32);
        let secret = array_ref!(secret, 0, 32);

        let mkey = SecShield::with(32, |mkey| {
            let mkey = array_mut_ref!(mkey, 0, 32);
            Kdf::default().derive(password, salt, mkey);

            for i in 0..32 {
                mkey[i] ^= secret[i];
            }
        });

        Ok(Titso { mkey })
    }

    pub fn init<R: RngCore + CryptoRng>(mut rng: R, password: &[u8])
        -> error::Result<(Titso, Vec<u8>)>
    {
        let mut salt = [0; 32];
        let mut secret = [0; 32];
        rng.fill_bytes(&mut salt);

        Kdf::default()
            .derive(password, &salt, &mut secret);

        let mkey = SecShield::with(32, |mkey| {
            rng.fill_bytes(mkey);

            for i in 0..32 {
                secret[i] ^= mkey[i];
            }
        });

        let buf = Vec::with_capacity(96);
        let buf = cbor::to_vec(buf, &MasterSecret { salt: &salt, secret: &secret })?;

        zero(&mut salt[..]);
        zero(&mut secret[..]);

        Ok((Titso { mkey }, buf))
    }

    pub fn ready(&mut self) -> TitsoRef<'_> {
        TitsoRef {
            mkey: self.mkey.get_mut()
        }
    }
}

impl TitsoRef<'_> {
    #[inline]
    pub fn store_tag(&self, tags: &[impl AsRef<str>]) -> Tag {
        self.tag("store", tags)
    }

    fn tag(&self, lable: &str, tags: &[impl AsRef<str>]) -> Tag {
        let mkey = array_ref!(&self.mkey, 0, 32);

        let mut itag = [0; 16];
        for tag in tags {
            let mut tmp = [0; 16];
            let mut hasher = KeyedHash::new(mkey, b"tag");
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
        let mkey = array_ref!(&self.mkey, 0, 32);

        let mut hasher = KeyedHash::new(mkey, b"derive");
        let Tag(kdf_tag) = self.tag("kdf", tags);
        hasher.update(&kdf_tag);
        hasher.update(&rule.count.to_le_bytes());
        hasher.update(&rule.length.to_le_bytes());
        hasher.update(&rule.chars.iter()
            .map(|c| c.len_utf8() as u32)
            .sum::<u32>()
            .to_le_bytes()
        );
        for c in &rule.chars {
            let mut buf = [0; 4];
            c.encode_utf8(&mut buf);
            hasher.update(&buf);
        }
        let mut rng = HashRng::from(hasher.xof());
        generate(&mut rng, rule)
    }

    pub fn get(&self, tags: &[impl AsRef<str>], value: &[u8]) -> error::Result<Item> {
        let mkey = array_ref!(&self.mkey, 0, 32);
        let Tag(aead_tag) = self.tag("aead", tags);

        let mut value = value.to_vec();
        let (atag, buf) = value.split_at_mut(primitive::aead::TAG_LENGTH);
        let atag = array_ref!(atag, 0, primitive::aead::TAG_LENGTH);

        if Aead::new(mkey, &aead_tag)
            .decrypt(b"item", buf, &atag)
        {
            cbor::from_slice(&buf).map_err(Into::into)
        } else {
            Err(error::Error::DecryptFailed)
        }
    }

    pub fn put(&self, tags: &[impl AsRef<str>], item: &Item) -> error::Result<Vec<u8>> {
        let mkey = array_ref!(&self.mkey, 0, 32);
        let Tag(aead_tag) = self.tag("aead", tags);

        let value = vec![0; primitive::aead::TAG_LENGTH];
        let mut value = cbor::to_vec(value, item)?;

        let (atag, buf) = value.split_at_mut(primitive::aead::TAG_LENGTH);
        let atag = array_mut_ref!(atag, 0, primitive::aead::TAG_LENGTH);

        Aead::new(mkey, &aead_tag)
            .encrypt(b"item", buf, atag);

        Ok(value)
    }
}
