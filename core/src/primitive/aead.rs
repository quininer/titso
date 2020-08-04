//! MEM-AEAD MRS mode

use std::mem;
use arrayref::{ array_ref, array_mut_ref };
use gimli_permutation::{ gimli, S };
use crate::common::with;

pub const ALL: usize = 48;
pub const RATE: usize = 16;
pub const KEY_LENGTH: usize = 32;
pub const NONCE_LENGTH: usize = 16;
pub const TAG_LENGTH: usize = 16;


#[derive(Default)]
pub struct Aead {
    state: [u32; S]
}

impl Aead {
    pub fn new(key: &[u8; KEY_LENGTH], nonce: &[u8; NONCE_LENGTH]) -> Aead {
        let mut state = [0; S];
        init(&mut state, key, nonce);
        Aead { state }
    }

    pub fn encrypt(&self, aad: &[u8], m: &mut [u8], tag: &mut [u8; TAG_LENGTH]) {
        // absorption phase
        let mut state = self.state;
        absorb(&mut state, aad);
        absorb(&mut state, m);
        finalise(&mut state, aad.len(), m.len(), tag);

        // encryption phase
        let mut state = self.state;
        with(&mut state, |state| state[..NONCE_LENGTH].copy_from_slice(tag));
        state[S - 1] ^= 1;
        encrypt_data(&mut state, m);

        // TODO zero state
    }

    pub fn decrypt(&self, aad: &[u8], c: &mut [u8], tag: &[u8; TAG_LENGTH]) -> bool {
        let mut tag2 = [0; TAG_LENGTH];

        // decryption phase
        let mut state = self.state;
        with(&mut state, |state| state[..NONCE_LENGTH].copy_from_slice(tag));
        state[S - 1] ^= 1;
        decrypt_data(&mut state, c);

        // absorption phase
        let mut state = self.state;
        absorb(&mut state, aad);
        absorb(&mut state, c);
        finalise(&mut state, aad.len(), c.len(), &mut tag2);

        // TODO zero state
        // TODO ct eq

        // verification phase
        tag == &tag2
    }
}

fn init(state: &mut [u32; S], key: &[u8; KEY_LENGTH], nonce: &[u8; NONCE_LENGTH]) {
    with(state, |state| {
        state[..NONCE_LENGTH].copy_from_slice(nonce);
        state[NONCE_LENGTH..].copy_from_slice(key);
    });
}

fn absorb(state: &mut [u32; S], aad: &[u8]) {
    #[inline]
    fn absorb_block(state: &mut [u32; S], chunk: &[u8; ALL]) {
        gimli(state);

        with(state, |state| {
            for i in 0..ALL {
                state[i] ^= chunk[i];
            }
        });
    }

    let mut iter = aad.chunks_exact(ALL);
    for chunk in &mut iter {
        let chunk = array_ref!(chunk, 0, ALL);
        absorb_block(state, chunk);
    }

    let chunk = iter.remainder();
    if !chunk.is_empty() {
        gimli(state);

        with(state, |state| {
            for i in 0..chunk.len() {
                state[i] ^= chunk[i];
            }
        });
    }
}

fn finalise(state: &mut [u32; S], hlen: usize, mlen: usize, tag: &mut [u8; TAG_LENGTH]) {
    gimli(state);

    state[0] ^= hlen as u32;
    state[1] ^= mlen as u32;

    gimli(state);
    with(state, |state| tag.copy_from_slice(&state[..TAG_LENGTH]));
}

fn encrypt_data(state: &mut [u32; S], m: &mut [u8]) {
    #[inline]
    fn encrypt_block(state: &mut [u32; S], chunk: &mut [u8; RATE]) {
        gimli(state);

        with(state, |state| {
            for i in 0..RATE {
                state[i] ^= chunk[i];
                chunk[i] = state[i];
            }
        });
    }

    let mut iter = m.chunks_exact_mut(RATE);
    for chunk in &mut iter {
        let chunk = array_mut_ref!(chunk, 0, RATE);
        encrypt_block(state, chunk);
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
}

fn decrypt_data(state: &mut [u32; S], c: &mut [u8]) {
    #[inline]
    fn decrypt_block(state: &mut [u32; S], chunk: &mut [u8; RATE]) {
        gimli(state);

        with(state, |state| {
            for i in 0..RATE {
                let s = mem::replace(&mut state[i], chunk[i]);
                chunk[i] ^= s;
            }
        });
    }

    let mut iter = c.chunks_exact_mut(RATE);
    for chunk in &mut iter {
        let chunk = array_mut_ref!(chunk, 0, RATE);
        decrypt_block(state, chunk);
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
}
