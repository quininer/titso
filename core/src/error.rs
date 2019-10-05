use snafu::Snafu;


pub type Result<T, Kv> = std::result::Result<T, Error<Kv>>;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum Error<DE: std::error::Error + Send + Sync + 'static> {
    #[snafu(display("Database Error: {}", source))]
    Db { source: DE },

    #[snafu(display("Database not initialized"))]
    Uninitialized {},

    #[snafu(display("Decryption Failed"))]
    Decrypt {},

    #[snafu(display("Cbor ser/de Failed: {}", source))]
    Cbor { source: serde_cbor::Error }
}
