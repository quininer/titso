use std::ops::{ Deref, DerefMut };
use once_cell::sync::OnceCell;
use getrandom::getrandom;
use gimli_hash::GimliHash;
use seckey::{ zero, SecBytes, SecWriteGuard };
use crate::primitive::stream::stream_xor;


pub struct SecShield(SecBytes);

impl SecShield {
    #[inline]
    pub fn new(len: usize) -> SecShield {
        fn id(_: &mut [u8]) {}

        SecShield::with(len, id)
    }

    pub fn with<F>(len: usize, f: F) -> SecShield
        where F: FnOnce(&mut [u8])
    {
        SecShield(SecBytes::with(len, |buf| {
            let prekey = pre_shield_key();
            f(buf);
            let mut key = shield(buf, &prekey[..]);
            zero(&mut key[..]);
        }))
    }

    pub fn get_mut(&mut self) -> SecShieldGuard<'_> {
        let prekey = pre_shield_key();
        let mut buf = self.0.write();
        let key = shield(&mut buf[..], &prekey[..]);

        SecShieldGuard { buf, key }
    }
}



/// Shield Guard
pub struct SecShieldGuard<'a> {
    buf: SecWriteGuard<'a>,
    key: Box<[u8; 32]>
}

impl<'a> Deref for SecShieldGuard<'a> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.buf.deref()
    }
}

impl<'a> DerefMut for SecShieldGuard<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        self.buf.deref_mut()
    }
}

impl<'a> Drop for SecShieldGuard<'a> {
    fn drop(&mut self) {
        unshield(&mut self.buf[..], &self.key);
        zero(&mut self.key[..]);
    }
}


fn pre_shield_key() -> &'static [u8] {
    const SHIELD_LENGTH: usize = 16 * 1024;
    static PRE_SHIELD_KEY: OnceCell<Box<[u8]>> = OnceCell::new();

    let buf = PRE_SHIELD_KEY.get_or_init(|| {
        let mut key = vec![0; SHIELD_LENGTH].into_boxed_slice();
        let mut hasher = GimliHash::default();
        getrandom(&mut key[..32]).unwrap();
        hasher.update(b"memsec shield");
        hasher.update(&key[..32]);
        hasher.finalize(&mut key[..]);
        key
    });
    &*buf
}

fn shield(buf: &mut [u8], prekey: &[u8]) -> Box<[u8; 32]> {
    let mut nonce = [0; 16];
    let ptr = buf.as_ptr() as usize as u64;
    let len = buf.len() as u64;
    nonce[..8].copy_from_slice(&ptr.to_le_bytes());
    nonce[8..].copy_from_slice(&len.to_le_bytes());

    let mut key = Box::new([0; 32]);
    let mut hasher = GimliHash::default();
    hasher.update(&nonce[..]);
    hasher.update(prekey);
    hasher.finalize(&mut key[..]);

    stream_xor(&key, &nonce, buf);

    key
}

fn unshield(buf: &mut [u8], key: &[u8; 32]) {
    let mut nonce = [0; 16];
    let ptr = buf.as_ptr() as usize as u64;
    let len = buf.len() as u64;
    nonce[..8].copy_from_slice(&ptr.to_le_bytes());
    nonce[8..].copy_from_slice(&len.to_le_bytes());

    stream_xor(&key, &nonce, buf);
}

#[test]
fn test_shield() {
    let mut key = SecShield::with(8, |buf| buf.copy_from_slice(&[0x42; 8]));

    {
        let mut buf = key.get_mut();
        assert_eq!(&*buf, &[0x42; 8]);
        buf[1] = 0;
    }

    {
        let buf = key.get_mut();
        assert_eq!(&*buf, &[0x42, 0, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42][..]);
    }
}
