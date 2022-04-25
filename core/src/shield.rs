use gimli_aead::GimliAead;
use gimli_hash::GimliHash;
use crate::{ Functions, SecBytes };
use crate::util::ScopeZeroed;


const SHIELD_LENGTH: usize = 16 * 1024;

pub struct Shield {
    zero: fn(&mut [u8]),
    prekey: Box<[u8]>,
    buf: Box<dyn SecBytes>,
    tag: [u8; 16]
}

pub struct Ready<'a> {
    shield: &'a mut Shield,
    cachekey: Box<[u8; 32]>
}

impl Shield {
    pub fn new(fns: &Functions, mut buf: Box<dyn SecBytes>) -> Shield {
        let mut prekey = vec![0; SHIELD_LENGTH].into_boxed_slice();
        let mut hasher = GimliHash::default();
        (fns.rng)(&mut prekey[..32]);
        hasher.update(b"memsec shield prekey");
        hasher.update(&prekey[..32]);
        hasher.finalize(&mut prekey[..]);

        let mut cachekey = ScopeZeroed([0; 32], fns.zero);
        let cachekey = cachekey.get_mut();
        let buf_ref = buf.get_mut_and_unlock();
        let nonce = derive_nonce(buf_ref);
        derive_key(&prekey, &nonce, cachekey);
        let tag = GimliAead::new(cachekey, &nonce).encrypt(&[], buf_ref);
        buf.lock();

        Shield {
            zero: fns.zero,
            prekey, buf, tag
        }
    }

    pub fn ready(&mut self) -> Option<Ready<'_>> {
        let mut cachekey = Box::new([0; 32]);
        let buf_ref = self.buf.get_mut_and_unlock();
        let nonce = derive_nonce(buf_ref);
        derive_key(&self.prekey, &nonce, &mut cachekey);

        let result = GimliAead::new(&cachekey, &nonce).decrypt(&[], buf_ref, &self.tag);

        self.buf.lock();

        if result {
            Some(Ready { shield: self, cachekey })
        } else {
            None
        }
    }
}

fn derive_key(prekey: &[u8], nonce: &[u8; 16], outkey: &mut [u8; 32]) {
    let mut hasher = GimliHash::default();
    hasher.update(b"memsec shield key");
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
    pub fn get(&self) -> &[u8; 32] {
        self.shield.buf.get_and_unlock()
    }
}

impl Drop for Ready<'_> {
    fn drop(&mut self) {
        let buf_ref = self.shield.buf.get_mut_and_unlock();
        let nonce = derive_nonce(buf_ref);

        let tag = GimliAead::new(&self.cachekey, &nonce).encrypt(&[], buf_ref);
        self.shield.tag.copy_from_slice(&tag);
        self.shield.buf.lock();
        (self.shield.zero)(&mut self.cachekey[..]);
    }
}
