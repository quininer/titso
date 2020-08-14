use std::ops::Deref;
use std::cell::Cell;
use getrandom::getrandom;
use seckey::{ TempKey, zero };
use crate::error::JsResult;


pub struct Password {
    bytes: TempKey<Box<[u8]>>,
    len: usize
}

pub struct PassGuard<'a>(&'a mut Password);

impl Password {
    pub fn new() -> Password {
        Password {
            bytes: TempKey::new(vec![0; 256].into_boxed_slice()),
            len: 0
        }
    }

    pub fn push(&mut self, c: u8) -> Result<(), u8> {
        if self.len + 1 < self.bytes.len() {
            self.bytes[self.len] = c;
            self.len += 1;
            Ok(())
        } else {
            Err(c)
        }
    }

    pub fn backspace(&mut self) {
        if self.len > 0 {
            self.len -= 1;
            self.bytes[self.len] = 0;
        }
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    #[inline]
    pub fn take(&mut self) -> PassGuard<'_> {
        PassGuard(self)
    }
}

impl Deref for PassGuard<'_> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_bytes()
    }
}

impl Drop for PassGuard<'_> {
    fn drop(&mut self) {
        zero(&mut self.0.bytes[..self.0.len]);
        self.0.len = 0;
    }
}

pub struct Lock {
    inner: Cell<bool>
}

pub struct LockGuard<'a>(&'a Cell<bool>);

impl Lock {
    pub const fn new() -> Lock {
        Lock {
            inner: Cell::new(true)
        }
    }

    pub fn acquire(&self) -> JsResult<LockGuard<'_>> {
        if self.inner.replace(false) {
            Ok(LockGuard(&self.inner))
        } else {
            Err("status is locked".into())
        }
    }
}

impl Drop for LockGuard<'_> {
    fn drop(&mut self) {
        self.0.set(true);
    }
}

pub fn take_tags(tags: &str) -> Vec<&str> {
    let mut tags = tags
        .split_whitespace()
        .collect::<Vec<_>>();
    tags.sort();
    tags.dedup();
    tags
}

pub fn padding() -> Vec<u8> {
    let mut len = [0; 1];
    getrandom(&mut len).unwrap();
    vec![0; len[0] as usize % 32]
}
