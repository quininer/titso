use byteorder::{ ByteOrder, LittleEndian };
use rand_core::{ RngCore, CryptoRng };
use gimli_permutation::S;
use crate::packet::Rule;


#[inline]
pub fn with<F>(state: &mut [u32; S], f: F)
    where F: FnOnce(&mut [u8; S * 4])
{
    #[inline]
    fn transmute(arr: &mut [u32; S]) -> &mut [u8; S * 4] {
        unsafe { &mut *(arr as *mut [u32; S] as *mut [u8; S * 4]) }
    }

    LittleEndian::from_slice_u32(state);
    f(transmute(state));
    LittleEndian::from_slice_u32(state);
}

pub fn generate<R: RngCore + CryptoRng>(rng: &mut R, rule: &Rule) -> String {
    let chars = &rule.chars;
    (0..rule.length)
        .map(|_| chars[rng.next_u32() as usize % chars.len()])
        .collect()
}

pub fn suggest(chars_len: usize) -> usize {
    const ENTROPY: f64 = 96.0;

    (ENTROPY / (chars_len as f64).log2()).ceil() as usize
}
