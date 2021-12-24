use thiserror::Error;


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("decrypt failed")]
    DecryptFailed,

    #[error("cbor encode error: {0}")]
    EncodeFailed(#[from] cbor4ii::EncodeError<std::collections::TryReserveError>),

    #[error("cbor decode error: {0}")]
    DecodeFailed(#[from] cbor4ii::DecodeError<std::convert::Infallible>)
}
