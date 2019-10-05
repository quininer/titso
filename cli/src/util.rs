use std::fmt;
use std::error::Error;


#[derive(Debug)]
pub struct MsgError(&'static str);

#[inline]
pub fn msg(s: &'static str) -> MsgError {
    MsgError(s)
}

impl fmt::Display for MsgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Error for MsgError {}
