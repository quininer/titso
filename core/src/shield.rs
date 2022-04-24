use gimli_aead::GimliAead;
use gimli_hash::GimliHash;
use crate::{ Config, SecBytes };


const SHIELD_LENGTH: usize = 16 * 1024;

pub struct Shield {
    zero: fn(&mut [u8]),
    prekey: Box<[u8]>,
    cachekey: Box<[u8; 32]>,
    buf: Box<dyn SecBytes>,
    tag: [u8; 16]
}

pub struct Ready<'a>(&'a mut Shield);

impl Shield {
    pub fn new(config: &Config, mut buf: Box<dyn SecBytes>) -> Shield {
        let mut prekey = vec![0; SHIELD_LENGTH].into_boxed_slice();
        let mut hasher = GimliHash::default();
        (config.rng)(&mut prekey[..32]);
        hasher.update(b"memsec shield prekey");
        hasher.update(&prekey[..32]);
        hasher.finalize(&mut prekey[..]);

        let mut cachekey = Box::new([0; 32]);
        let buf_ref = buf.access_and_unlock();
        let nonce = derive_nonce(buf_ref);
        derive_key(&prekey, &nonce, &mut cachekey);
        let tag = GimliAead::new(&cachekey, &nonce).encrypt(&[], buf_ref);
        buf.lock();
        (config.zero)(&mut cachekey[..]);

        Shield {
            zero: config.zero,
            prekey, cachekey, buf, tag
        }
    }

    pub fn ready(&mut self) -> Option<Ready<'_>> {
        let buf_ref = self.buf.access_and_unlock();
        let nonce = derive_nonce(buf_ref);
        derive_key(&self.prekey, &nonce, &mut self.cachekey);

        let result = GimliAead::new(&self.cachekey, &nonce).decrypt(&[], buf_ref, &self.tag);

        if result {
            Some(Ready(self))
        } else {
            self.buf.lock();
            None
        }
    }
}

fn derive_key(prekey: &[u8], nonce: &[u8; 16], outkey: &mut [u8; 32]) {
    let mut hasher = GimliHash::default();
    hasher.update("memsec shield key");
    hasher.update(&nonce[..]);
    hasher.update(prekey);
    hasher.finalize(outkey);
}

fn derive_nonce(buf: &[u8]) -> [u8; 16] {
    let mut nonce = [0; 16];
    let ptr = buf.as_ptr() as usize as u64;
    let len = buf.len() as u64;
    nonce[..8].copy_from_slice(&ptr.to_le_bytes());
    nonce[8..].copy_from_slice(&len.to_le_bytes());
    nonce
}

impl Ready<'_> {
    pub fn get(&mut self) -> &mut [u8] {
        (self.0).buf.access_and_unlock()
    }
}

impl Drop for Ready<'_> {
    fn drop(&mut self) {
        let buf_ref = (self.0).buf.access_and_unlock();
        let nonce = derive_nonce(buf_ref);

        let tag = GimliAead::new(&(self.0).cachekey, &nonce).encrypt(&[], buf_ref);
        (self.0).tag.copy_from_slice(&tag);
        (self.0).buf.lock();
        ((self.0).zero)(&mut (self.0).cachekey[..]);
    }
}
