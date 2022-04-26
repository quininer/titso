mod util;
mod crypto;
mod shield;
mod error;
pub mod packet;

use cbor4ii::serde as cbor;
use gimli_aead::GimliAead;
use crypto::kdf::Kdf;
use crypto::keyedhash::KeyedHash;
use crypto::rng::HashRng;
use shield::Shield;
use util::ScopeZeroed;
pub use error::Error;


pub struct Core {
    mkey: Shield
}

pub struct Ready<'a> {
    mkey: shield::Ready<'a>
}

pub struct Functions {
    pub rng: fn(&mut [u8]),
    pub zero: fn(&mut [u8]),
    pub malloc: fn() -> Box<dyn SecBytes>
}

pub trait SecBytes: Send + 'static {
    fn get_and_unlock(&self) -> &[u8; 32];
    fn get_mut_and_unlock(&mut self) -> &mut [u8; 32];
    fn lock(&self);
}

impl Core {
    pub fn create(fns: &Functions, password: &[u8]) -> Result<(Core, Vec<u8>), Error> {
        let mut salt = ScopeZeroed([0; 32], fns.zero);
        let mut secret = ScopeZeroed([0; 32], fns.zero);
        let salt: &mut [u8; 32] = salt.get_mut();
        let secret = secret.get_mut();

        (fns.rng)(salt);

        Kdf::default().derive(password, salt, secret);

        let mkey = {
            let mut mkey = (fns.malloc)();
            let mkey_ref = mkey.get_mut_and_unlock();
            (fns.rng)(mkey_ref);

            for i in 0..32 {
                secret[i] ^= mkey_ref[i];
            }

            Shield::new(fns, mkey)
        };

        let secret_buf = {
            let secret_buf = Vec::with_capacity(128);
            let master_secret = packet::MasterSecret { salt, secret };
            cbor::to_vec(secret_buf, &master_secret)
                .map_err(|_| Error::encode_error("master secret encode failed"))?
        };

        Ok((Core { mkey }, secret_buf))
    }

    pub fn open(fns: &Functions, buf: &[u8], password: &[u8]) -> Result<Core, Error> {
        let packet::MasterSecret { salt, secret } = cbor::from_slice(buf)
            .map_err(|_| Error::decode_error("master secret decode failed"))?;
        let salt: &[u8; 32] = salt.try_into()
            .map_err(|_| Error::decode_error("bad salt length"))?;
        let secret: &[u8; 32] = secret.try_into()
            .map_err(|_| Error::decode_error("bad secret length"))?;

        let mkey = {
            let mut mkey = (fns.malloc)();
            let mkey_ref = mkey.get_mut_and_unlock();

            Kdf::default().derive(password, salt, mkey_ref);

            for i in 0..32 {
                mkey_ref[i] ^= secret[i];
            }

            Shield::new(fns, mkey)
        };

        Ok(Core { mkey })
    }

    pub fn ready(&mut self) -> Result<Ready<'_>, Error> {
        let mkey = self.mkey.ready()
            .ok_or_else(|| Error::aead_failed("memory shield check failed"))?;
        Ok(Ready { mkey })
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

impl Ready<'_> {
    #[inline]
    pub fn store_tag(&self, tags: &[impl AsRef<str>]) -> packet::Tag {
        let mkey = self.mkey.get();
        let mut iter = tags.iter().map(|tag| tag.as_ref());
        tag(&mkey, "store", &mut iter)
    }

    pub fn derive(&self, tags: &[impl AsRef<str>], rule: &packet::Rule) -> String {
        let mkey = self.mkey.get();
        let mut iter = tags.iter().map(|tag| tag.as_ref());
        let packet::Tag(kdf_tag) = tag(&mkey, "kdf", &mut iter);

        let mut hasher = KeyedHash::new(&mkey, b"derive");
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

        (0..rule.length)
            .map(|_| rule.chars[rng.next_u32() as usize % rule.chars.len()])
            .collect()
    }

    pub fn put(&self, tags: &[impl AsRef<str>], item: &packet::Item) -> Result<Vec<u8>, Error> {
        let mkey = self.mkey.get();
        let mut iter = tags.iter().map(|tag| tag.as_ref());
        let packet::Tag(aead_tag) = tag(&mkey, "aead", &mut iter);

        let value = vec![0; 16];
        let mut value = cbor::to_vec(value, item)
            .map_err(|_| Error::encode_error("put item encode failed"))?;
        let (atag, buf) = value.split_at_mut(16);

        let atag2 = GimliAead::new(&mkey, &aead_tag).encrypt(b"item", buf);
        atag.copy_from_slice(&atag2);

        Ok(value)
    }

    pub fn get(&self, tags: &[impl AsRef<str>], value: &mut [u8]) -> Result<packet::Item, Error> {
        let mkey = self.mkey.get();
        let mut iter = tags.iter().map(|tag| tag.as_ref());
        let packet::Tag(aead_tag) = tag(&mkey, "aead", &mut iter);

        if value.len() <= 16 {
            return Err(Error::aead_failed("get item ciphertext too short"));
        }

        let (atag, buf) = value.split_at_mut(16);
        let atag: &[u8] = atag;
        let atag: &[u8; 16] = atag.try_into().unwrap();
        let result = GimliAead::new(&mkey, &aead_tag).decrypt(b"item", buf, atag);

        if result {
            cbor::from_slice(buf)
                .map_err(|_| Error::decode_error("get item decode failed"))
        } else {
            Err(Error::aead_failed("get item aead decrypt failed"))
        }
    }
}
