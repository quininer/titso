use gimli_permutation::S;
use seckey::zero;
use crate::common::with;
use crate::primitive::aead::{ KEY_LENGTH, NONCE_LENGTH, init, encrypt_data };


pub fn stream_xor(key: &[u8; KEY_LENGTH], nonce: &[u8; NONCE_LENGTH], m: &mut [u8]) {
    let mut state = [0; S];
    let state = &mut state;

    init(state, key, nonce);
    encrypt_data(state, m);
    with(state, |state| zero(&mut state[..]));
}
