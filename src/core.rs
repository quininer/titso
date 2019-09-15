use std::error::Error;
use serde::{ Deserialize, Serialize };


pub trait Database {
    type Store: Store<Self::Error>;
    type Error: Error + Send + Sync;

    fn transaction<F, R>(&self, f: F)
        -> Result<R, Self::Error>
    where F: FnOnce(&Self::Store) -> Result<R, Self::Error>;
}

pub trait Store<E> {
    type Table: Table<E>;

    fn open(&self, name: &[u8]) -> Result<Self::Table, E>;
}

pub trait Table<E> {
    fn get(&self, key: &[u8]) -> Result<Option<&[u8]>, E>;
    fn set(&self, key: &[u8], val: &[u8]) -> Result<(), E>;
    fn del(&self, key: &[u8]) -> Result<Option<&[u8]>, E>;
}


#[derive(Deserialize, Serialize)]
pub struct MasterStore {
    salt: [u8; 32],
    store: [u8; 32]
}

#[derive(Deserialize, Serialize)]
pub struct Tag([u8; 16]);

#[derive(Deserialize, Serialize)]
pub struct Item {
    /// encrypted rule
    #[serde(with = "serde_bytes")]
    rule: Vec<u8>,

    #[serde(with = "serde_bytes")]
    data: Vec<u8>
}

#[derive(Deserialize, Serialize)]
pub struct Rule {
    count: u64,
    entropy: u16,
}

#[derive(Deserialize, Serialize)]
pub struct Hint(#[serde(with = "serde_bytes")] Vec<u8>);
