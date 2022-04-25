use std::fmt;
use std::error::Error as StdError;


pub struct Error {
    kind: ErrorKind,
    msg: &'static str
}

#[derive(Debug)]
enum ErrorKind {
    Encode,
    Decode,
    AeadFailed
}

impl Error {
    pub(crate) fn encode_error(msg: &'static str) -> Error {
        Error {
            kind: ErrorKind::Encode,
            msg
        }
    }

    pub(crate) fn decode_error(msg: &'static str) -> Error {
        Error {
            kind: ErrorKind::Decode,
            msg
        }
    }

    pub(crate) fn aead_failed(msg: &'static str) -> Error {
        Error {
            kind: ErrorKind::AeadFailed,
            msg
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Error")
            .field("kind", &self.kind)
            .field("msg", &self.msg)
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl StdError for Error {}
