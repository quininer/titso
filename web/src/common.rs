use std::cell::Cell;
use crate::error::JsResult;


pub struct Password {
    bytes: Box<[u8]>,
    len: usize
}

impl Password {
    pub fn new() -> Password {
        Password {
            bytes: vec![0; 256].into_boxed_slice(),
            len: 0
        }
    }
}

pub struct Lock {
    inner: Cell<bool>
}

pub struct Guard<'a>(&'a Cell<bool>);

impl Lock {
    pub const fn new() -> Lock {
        Lock {
            inner: Cell::new(true)
        }
    }

    pub fn acquire(&self) -> JsResult<Guard<'_>> {
        if self.inner.replace(false) {
            Ok(Guard(&self.inner))
        } else {
            Err("status is locked".into())
        }
    }
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        self.0.set(true);
    }
}
