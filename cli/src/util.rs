use std::fmt;
use std::error::Error;


pub struct StoreError(pub rkv::StoreError);

impl Error for StoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.0 {
            rkv::StoreError::IoError(err) => Some(err),
            rkv::StoreError::DataError(err) => match err {
                rkv::DataError::DecodingError { err, .. } => Some(&*err),
                rkv::DataError::EncodingError(err) => Some(&*err),
                _ => None
            },
            rkv::StoreError::LmdbError(err) => Some(err),
            _ => None
        }
    }
}

impl From<rkv::StoreError> for StoreError {
    fn from(err: rkv::StoreError) -> StoreError {
        StoreError(err)
    }
}

impl fmt::Debug for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
