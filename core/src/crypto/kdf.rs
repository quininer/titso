use gimli_permutation::{ SIZE, gimli, state_with as with };
use gimli_hash::GimliHash;
use seckey::zero;

pub const HASH_ALG: u8 = 0x01;
pub const RATE: usize = 16;

pub struct Kdf {
    opslimit: u64,
    memlimit: usize,
    threads: u8,
    hasher: GimliHash
}

impl Default for Kdf {
    fn default() -> Kdf {
        Kdf {
            opslimit: 500000,
            memlimit: 1,
            threads: 1,
            hasher: GimliHash::default()
        }
    }
}

impl Kdf {
    pub fn with_hasher(&mut self, hasher: GimliHash) -> &mut Kdf {
        self.hasher = hasher;
        self
    }

    pub fn derive(&self, passwd: &[u8], salt: &[u8; 32], output: &mut [u8; 32]) {
        let mut state = [0; SIZE];

        // init state
        let mut hasher = self.hasher.clone();
        hasher.update(b"titso kdf");
        hasher.update(&passwd.len().to_le_bytes());
        hasher.update(passwd);
        hasher.update(salt);
        hasher.update(&HASH_ALG.to_le_bytes());
        hasher.update(&self.threads.to_le_bytes());
        hasher.update(&self.memlimit.to_le_bytes());
        with(&mut state, |state| hasher.finalize(state));

        // kdf iter
        with(&mut state, |state| state[RATE - 1] ^= 1);
        gimli(&mut state);
        for i in 0..self.opslimit {
            with(&mut state, |state| {
                state[..8].copy_from_slice(&i.to_le_bytes());
                state[8..][..RATE - 8].copy_from_slice(&[0; RATE - 8]);
            });
            gimli(&mut state);
        }

        with(&mut state, |state| {
            output.copy_from_slice(&state[RATE..][..32]);
            zero(&mut state[..]);
        });
    }
}
