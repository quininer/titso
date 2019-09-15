use gimli_hash::{ GimliHash, XofReader };

pub const KEY_LENGTH: usize = 32;

pub struct KeyedHash {
    hasher: GimliHash
}

impl KeyedHash {
    pub fn new(key: &[u8; KEY_LENGTH]) -> KeyedHash {
        let mut hasher = GimliHash::default();
        hasher.update(b"titso hash");
        hasher.update(key);
        hasher.fill_block();
        KeyedHash { hasher }
    }

    #[inline]
    pub fn update(&mut self, buf: &[u8]) {
        self.hasher.update(buf)
    }

    #[inline]
    pub fn finalize(self, buf: &mut [u8]) {
        self.hasher.finalize(buf);
    }

    #[inline]
    pub fn xof(self) -> XofReader {
        self.hasher.xof()
    }
}
