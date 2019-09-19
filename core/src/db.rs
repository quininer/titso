use std::error::Error;
use std::future::Future;
use async_trait::async_trait;


#[async_trait]
pub trait DataBase {
    type Store: Store<Self::Error>;
    type Error: Error + Send + Sync + 'static;

    async fn transaction<F, F2, R>(&self, f: F)
        -> Result<R, Self::Error>
    where
        F: FnOnce(&Self::Store) -> F2,
        F2: Future<Output = Result<R, Self::Error>>;
}

#[async_trait]
pub trait Store<E> {
    type Table: Table<E>;

    async fn open(&self, name: &[u8]) -> Result<Self::Table, E>;
}

#[async_trait]
pub trait Table<E> {
    async fn get(&self, key: &[u8]) -> Result<Option<&[u8]>, E>;
    async fn set(&self, key: &[u8], val: &[u8]) -> Result<(), E>;
    async fn del(&self, key: &[u8]) -> Result<Option<&[u8]>, E>;
}
