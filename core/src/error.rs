use snafu::Snafu;
use crate::db::DataBase;
use crate::packet::Tag;


pub type Result<T, DB> = std::result::Result<T, Error<DB>>;

#[derive(Debug, Snafu)]
pub enum Error<DB: DataBase> {
    #[snafu(display("DataBase Error: {}", source))]
    Db { source: DB::Error },

    #[snafu(display("Decryption Failed"))]
    Decrypt {},

    #[snafu(display("Cbor ser/de Failed: {}", source))]
    Cbor { source: serde_cbor::Error }
}
