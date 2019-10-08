use snafu::Snafu;


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum Error {
    #[snafu(display("Decryption Failed"))]
    Decrypt {},

    #[snafu(display("Cbor ser/de Failed: {}", source))]
    Cbor { source: serde_cbor::Error }
}
