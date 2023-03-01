use std::marker::PhantomData;
use gimli_aead::GimliAead;
use gimli_hash::GimliHash;
use crate::{ SafeFeatures, SafeBytes };
use crate::util::ScopeZeroed;


const SHIELD_LENGTH: usize = 16 * 1024;

pub struct Shield<F: SafeFeatures> {
    prekey: Box<[u8]>,
    buf: F::SafeBytes,
    tag: [u8; 16],
    _phantom: PhantomData<F>
}

pub struct Ready<'a, F: SafeFeatures> {
    shield: &'a mut Shield<F>,
    cachekey: Box<[u8; 32]>
}

impl<F: SafeFeatures> Shield<F> {
    pub fn new(mut buf: F::SafeBytes) -> Shield<F> {
        let mut prekey = vec![0; SHIELD_LENGTH].into_boxed_slice();
        let mut hasher = GimliHash::default();
        F::rng_fill(&mut prekey[..32]);
        hasher.update(b"memsec shield prekey");
        hasher.update(&prekey[..32]);
        hasher.finalize(&mut prekey[..]);

        let tag = {
            let mut cachekey = ScopeZeroed([0; 32], F::zero_bytes);
            let cachekey = cachekey.get_mut();
            let mut buf_ref = buf.get_mut();
            let buf_ref = buf_ref.as_mut();
            let nonce = derive_nonce(buf_ref);
            derive_key(&prekey, &nonce, cachekey);
            GimliAead::new(cachekey, &nonce).encrypt(b"memsec", buf_ref)
        };

        Shield {
            prekey, buf, tag,
            _phantom: PhantomData
        }
    }

    pub fn ready(&mut self) -> Ready<'_, F> {
        let mut cachekey = Box::new([0; 32]);

        let result = {
            let mut buf_ref = self.buf.get_mut();
            let buf_ref = buf_ref.as_mut();
            let nonce = derive_nonce(buf_ref);
            derive_key(&self.prekey, &nonce, &mut cachekey);

            GimliAead::new(&cachekey, &nonce).decrypt(b"memsec", buf_ref, &self.tag)
        };

        if result {
            Ready { shield: self, cachekey }
        } else {
            panic!("memory shield decrypt failed")
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

impl<F: SafeFeatures> Ready<'_, F> {
    pub fn get<'a>(&'a self) -> impl AsRef<[u8; 32]> + 'a {
        self.shield.buf.get()
    }
}

impl<F: SafeFeatures> Drop for Ready<'_, F> {
    fn drop(&mut self) {
        let mut buf_ref = self.shield.buf.get_mut();
        let buf_ref = buf_ref.as_mut();
        let nonce = derive_nonce(buf_ref);

        let tag = GimliAead::new(&self.cachekey, &nonce).encrypt(b"memsec", buf_ref);
        self.shield.tag.copy_from_slice(&tag);
        F::zero_bytes(&mut self.cachekey[..]);
    }
}
