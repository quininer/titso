use std::fmt;
use std::pin::Pin;
use std::ops::Deref;
use std::cell::{ Cell, RefCell };
use std::future::Future;
use std::task::{ Waker, Poll, Context };
use std::collections::VecDeque;
use getrandom::getrandom;
use seckey::zero;
use crate::Titso;


pub trait AlertExt {
    type Ok;
    type Err: fmt::Debug;

    fn unwrap_alert(self, titso: &Titso) -> Self::Ok;
}

impl<T, E: fmt::Debug> AlertExt for Result<T, E> {
    type Ok = T;
    type Err = E;

    #[inline]
    fn unwrap_alert(self, titso: &Titso) -> T {
        #[cold]
        fn alert_panic(err: &dyn fmt::Debug, titso: &Titso) -> ! {
            if let Ok(mut core) = titso.core.try_borrow_mut() {
                core.take();
            }

            if let Ok(mut password) = titso.password.try_borrow_mut() {
                password.take();
            }

            let msg = format!("{:?}", err);

            let _ = titso.window.alert_with_message(&msg);

            panic!("{}", msg);
        }

        match self {
            Ok(t) => t,
            Err(err) => alert_panic(&err, titso)
        }
    }
}

pub struct Password {
    bytes: Box<[u8]>,
    len: usize
}

pub struct PassGuard<'a>(&'a mut Password);

impl Password {
    pub fn new() -> Password {
        Password {
            bytes: vec![0; 256].into_boxed_slice(),
            len: 0
        }
    }

    pub fn push(&mut self, c: u8) {
        if self.len + 1 < self.bytes.len() {
            self.bytes[self.len] = c;
            self.len += 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.len > 0 {
            self.len -= 1;
            self.bytes[self.len] = 0;
        }
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
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
    flag: Cell<bool>,
    queue: RefCell<VecDeque<Waker>>
}

pub struct LockGuard<'a>(&'a Lock);

impl Lock {
    pub fn new() -> Lock {
        Lock {
            flag: Cell::new(false),
            queue: RefCell::new(VecDeque::new())
        }
    }

    pub async fn acquire(&self) -> LockGuard<'_> {
        struct Acquire<'a>(&'a Lock);

        impl<'a> Future for Acquire<'a> {
            type Output = LockGuard<'a>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let Acquire(this) = self.get_mut();
                let mut queue = this.queue.borrow_mut();

                if queue.is_empty() && !this.flag.replace(true) {
                    Poll::Ready(LockGuard(this))
                } else {
                    queue.push_back(cx.waker().clone());
                    Poll::Pending
                }
            }
        }

        Acquire(self).await
    }
}

impl Drop for LockGuard<'_> {
    fn drop(&mut self) {
        self.0.flag.set(false);

        let mut queue = self.0.queue.borrow_mut();

        if let Some(waker) = queue.pop_front() {
            waker.wake();
        }
    }
}

pub fn take_tags(tags: &str) -> Vec<&str> {
    let mut tags = tags
        .split_whitespace()
        .collect::<Vec<_>>();
    tags.sort_unstable();
    tags.dedup();
    tags
}

pub fn padding() -> Vec<u8> {
    let mut len = [0; 1];
    getrandom(&mut len).unwrap();
    vec![0; len[0] as usize % 32]
}
