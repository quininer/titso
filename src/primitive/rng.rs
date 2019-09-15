use rand::{ SeedableRng, CryptoRng, RngCore };
use gimli_hash::{ GimliHash, XofReader };


pub struct HashRng {
    state: XofReader
}

impl SeedableRng for HashRng {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut hasher = GimliHash::default();
        hasher.update(b"titso rng");
        hasher.update(&seed);
        HashRng { state: hasher.xof() }
    }
}

impl RngCore for HashRng {
    fn next_u32(&mut self) -> u32 {
        let mut buf = [0; 4];
        self.state.squeeze(&mut buf);
        u32::from_le_bytes(buf)
    }

    fn next_u64(&mut self) -> u64 {
        let mut buf = [0; 8];
        self.state.squeeze(&mut buf);
        u64::from_le_bytes(buf)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.state.squeeze(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.state.squeeze(dest);
        Ok(())
    }
}

impl CryptoRng for HashRng {}

impl From<XofReader> for HashRng {
    fn from(xof: XofReader) -> HashRng {
        HashRng { state: xof }
    }
}
