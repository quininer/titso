use std::error::Error;
use async_trait::async_trait;


#[async_trait]
pub trait KvStore {
    type Table: Table<Self::Error>;
    type Error: Error + Send + Sync + 'static;

    async fn open(&mut self, name: &str) -> Result<Self::Table, Self::Error>;
}

#[async_trait]
pub trait Table<E> {
    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, E>;
    async fn put(&self, key: &[u8], val: &[u8]) -> Result<(), E>;
    async fn del(&self, key: &[u8]) -> Result<bool, E>;
    async fn cas(&self, key: &[u8], old: &[u8], new: &[u8]) -> Result<Option<Vec<u8>>, E>;

    // TODO iter
}
