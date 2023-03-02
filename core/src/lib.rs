mod util;
mod crypto;
mod shield;
mod error;
pub mod packet;

use std::marker::PhantomData;
use cbor4ii::serde as cbor;
use gimli_aead::GimliAead;
use crypto::kdf::Kdf;
use crypto::keyedhash::KeyedHash;
use crypto::rng::HashRng;
use shield::Shield;
use util::ScopeZeroed;
pub use error::Error;


pub trait SafeFeatures {
    type SafeBytes: SafeBytes;

    fn rng_fill(buf: &mut [u8]);
    fn zero_bytes(buf: &mut [u8]);
    fn safe_heap_alloc() -> Self::SafeBytes;
}

pub trait SafeBytes {
    type Ref<'a>: AsRef<[u8; 32]>
        where Self: 'a;
    type RefMut<'a>: AsMut<[u8; 32]>
        where Self: 'a;

    fn get<'a>(&'a self) -> Self::Ref<'a>;
    fn get_mut<'a>(&'a mut self) -> Self::RefMut<'a>;
}

pub struct Core<F: SafeFeatures> {
    mkey: Shield<F>,
    _phantom: PhantomData<F>
}

impl<F: SafeFeatures> Core<F> {
    pub fn create(password: &[u8]) -> Result<(Core<F>, Vec<u8>), Error> {
        let mut salt = ScopeZeroed([0; 32], F::zero_bytes);
        let mut secret = ScopeZeroed([0; 32], F::zero_bytes);
        let salt: &mut [u8; 32] = salt.get_mut();
        let secret = secret.get_mut();

        F::rng_fill(salt);

        Kdf::default().derive(password, salt, secret);

        let mkey = {
            let mut mkey = F::safe_heap_alloc();

            {
                let mut mkey_ref = mkey.get_mut();
                let mkey_ref = mkey_ref.as_mut();
                F::rng_fill(mkey_ref);

                for i in 0..32 {
                    secret[i] ^= mkey_ref[i];
                }
            }

            Shield::new(mkey)
        };

        let secret_buf = {
            let secret_buf = Vec::with_capacity(128);
            let master_secret = packet::MasterSecret { salt, secret };
            cbor::to_vec(secret_buf, &master_secret)
                .map_err(|_| Error::encode_error("master secret encode failed"))?
        };

        Ok((Core { mkey, _phantom: PhantomData }, secret_buf))
    }

    pub fn open(buf: &[u8], password: &[u8]) -> Result<Core<F>, Error> {
        let packet::MasterSecret { salt, secret } = cbor::from_slice(buf)
            .map_err(|_| Error::decode_error("master secret decode failed"))?;
        let salt: &[u8; 32] = salt.try_into()
            .map_err(|_| Error::decode_error("bad salt length"))?;
        let secret: &[u8; 32] = secret.try_into()
            .map_err(|_| Error::decode_error("bad secret length"))?;

        let mkey = {
            let mut mkey = F::safe_heap_alloc();

            {
                let mut mkey_ref = mkey.get_mut();
                let mkey_ref = mkey_ref.as_mut();

                Kdf::default().derive(password, salt, mkey_ref);

                for i in 0..32 {
                    mkey_ref[i] ^= secret[i];
                }
            }

            Shield::new(mkey)
        };

        Ok(Core { mkey, _phantom: PhantomData })
    }
}


fn tag(mkey: &[u8; 32], lable: &str, tags: &mut dyn Iterator<Item = &str>)
    -> packet::Tag
{
    let mut itag = [0; 16];
    for tag in tags {
        let mut tmp = [0; 16];
        let mut hasher = KeyedHash::new(mkey, b"tag");
        hasher.update(&lable.len().to_le_bytes());
        hasher.update(lable.as_bytes());
        hasher.update(tag.as_bytes());
        hasher.finalize(&mut tmp);
        for i in 0..16 {
            itag[i] ^= tmp[i];
        }
    }

    packet::Tag(itag)
}

impl<F: SafeFeatures> Core<F> {
    #[inline]
    pub fn store_tag(&mut self, tags: &[impl AsRef<str>]) -> packet::Tag {
        let mkey = self.mkey.ready();
        let mkey = mkey.get();
        let mut iter = tags.iter().map(|tag| tag.as_ref());
        tag(mkey.as_ref(), "store", &mut iter)
    }

    pub fn derive(&mut self, tags: &[impl AsRef<str>], rule: &packet::Rule) -> String {
        fn derive_inner(mkey: &[u8; 32], tag: &packet::Tag, rule: &packet::Rule) -> String {
            let mut hasher = KeyedHash::new(&mkey, b"derive");
            hasher.update(&tag.0);
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

            (0..rule.length)
                .map(|_| rule.chars[rng.next_u32() as usize % rule.chars.len()])
                .collect()
        }

        let mkey = self.mkey.ready();
        let mkey = mkey.get();
        let mut iter = tags.iter().map(|tag| tag.as_ref());
        let kdf_tag = tag(mkey.as_ref(), "kdf", &mut iter);

        derive_inner(mkey.as_ref(), &kdf_tag, rule)
    }

    pub fn put(&mut self, tags: &[impl AsRef<str>], item: &packet::Item) -> Result<Vec<u8>, Error> {
        fn put_inner(mkey: &[u8; 32], tag: &packet::Tag, item: &packet::Item)
            -> Result<Vec<u8>, Error>
        {
            let value = vec![0; 16];
            let mut value = cbor::to_vec(value, item)
                .map_err(|_| Error::encode_error("put item encode failed"))?;
            let (atag, buf) = value.split_at_mut(16);

            let atag2 = GimliAead::new(&mkey, &tag.0).encrypt(b"item", buf);
            atag.copy_from_slice(&atag2);

            Ok(value)
        }

        let mkey = self.mkey.ready();
        let mkey = mkey.get();
        let mut iter = tags.iter().map(|tag| tag.as_ref());
        let aead_tag = tag(mkey.as_ref(), "aead", &mut iter);

        put_inner(mkey.as_ref(), &aead_tag, item)
    }

    pub fn get(&mut self, tags: &[impl AsRef<str>], value: &mut [u8]) -> Result<packet::Item, Error> {
        fn get_inner<F: SafeFeatures>(mkey: &[u8; 32], tag: &packet::Tag, value: &mut [u8])
            -> Result<packet::Item, Error>
        {
            if value.len() <= 16 {
                return Err(Error::aead_failed("get item ciphertext too short"));
            }

            let mut value = ScopeZeroed(value, F::zero_bytes);
            let value = value.get_mut();
            let (atag, buf) = value.split_at_mut(16);
            let atag: &[u8] = atag;
            let atag: &[u8; 16] = atag.try_into().unwrap();
            let result = GimliAead::new(&mkey, &tag.0).decrypt(b"item", buf, atag);

            if result {
                cbor::from_slice(buf)
                    .map_err(|_| Error::decode_error("get item decode failed"))
            } else {
                Err(Error::aead_failed("get item aead decrypt failed"))
            }
        }

        let mkey = self.mkey.ready();
        let mkey = mkey.get();
        let mut iter = tags.iter().map(|tag| tag.as_ref());
        let aead_tag = tag(mkey.as_ref(), "aead", &mut iter);

        get_inner::<F>(mkey.as_ref(), &aead_tag, value)
    }
}
