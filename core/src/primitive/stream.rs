use arrayref::array_mut_ref;
use gimli_permutation::{ gimli, S };
use seckey::zero;
use crate::common::with;
use crate::primitive::aead::{ RATE, KEY_LENGTH, NONCE_LENGTH, init };


pub fn stream_xor(key: &[u8; KEY_LENGTH], nonce: &[u8; NONCE_LENGTH], m: &mut [u8]) {
    let mut state = [0; S];
    let state = &mut state;

    init(state, key, nonce);

    let mut iter = m.chunks_exact_mut(RATE);
    for chunk in &mut iter {
        let chunk = array_mut_ref!(chunk, 0, RATE);

        gimli(state);

        with(state, |state| {
            for i in 0..RATE {
                chunk[i] ^= state[i];
            }
        })
    }

    let chunk = iter.into_remainder();
    if !chunk.is_empty() {
        gimli(state);

        with(state, |state| {
            for i in 0..chunk.len() {
                chunk[i] ^= state[i];
            }
        });
    }

    with(state, |state| zero(&mut state[..]));
}
