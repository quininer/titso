use gimli_hash::XofReader;


pub struct HashRng {
    state: XofReader
}

impl HashRng {
    pub fn next_u32(&mut self) -> u32 {
        let mut buf = [0; 4];
        self.state.squeeze(&mut buf);
        u32::from_le_bytes(buf)
    }
}

impl From<XofReader> for HashRng {
    fn from(xof: XofReader) -> HashRng {
        HashRng { state: xof }
    }
}
