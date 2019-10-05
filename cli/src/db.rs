use std::fmt;
use std::sync::Arc;
use std::path::Path;
use std::error::Error;
use std::collections::btree_map::{ BTreeMap, Entry };
use async_trait::async_trait;
use rkv::{ Rkv, SingleStore, StoreOptions };
use titso_core::kv;


pub struct RkvStore {
    db: Arc<Rkv>,
    cache: BTreeMap<Box<str>, SingleStore>
}

pub struct Store {
    db: Arc<Rkv>,
    store: SingleStore
}

pub struct StoreError(pub rkv::StoreError);

impl RkvStore {
    pub fn new(path: &Path) -> Result<RkvStore, StoreError> {
        let rkv = Rkv::new(path)?;
        Ok(RkvStore { db: Arc::new(rkv), cache: BTreeMap::new() })
    }
}

#[async_trait]
impl kv::KvStore for RkvStore {
    type Table = Store;
    type Error = StoreError;

    async fn open(&mut self, name: &str) -> Result<Self::Table, Self::Error> {
        let store = match self.cache.entry(Box::from(name)) {
            Entry::Vacant(entry) => {
                let store = self.db.open_single(name, StoreOptions::create())?;
                entry.insert(store).clone()
            },
            Entry::Occupied(entry) => entry.get().clone()
        };

        Ok(Store { db: self.db.clone(), store })
    }
}

#[async_trait]
impl kv::Table<StoreError> for Store {
    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, StoreError> {
        let reader = self.db.read()?;
        match self.store.get(&reader, key)? {
            Some(rkv::Value::Blob(val)) => Ok(Some(val.to_owned())),
            _ => Ok(None)
        }
    }

    async fn put(&self, key: &[u8], val: &[u8]) -> Result<(), StoreError> {
        let mut writer = self.db.write()?;
        self.store.put(&mut writer, key, &rkv::Value::Blob(val))?;
        writer.commit()?;
        Ok(())
    }

    async fn del(&self, key: &[u8]) -> Result<bool, StoreError> {
        let mut writer = self.db.write()?;
        match self.store.delete(&mut writer, key) {
            Ok(_) => {
                writer.commit()?;
                Ok(true)
            },
            Err(err) => match err {
                rkv::StoreError::LmdbError(lmdb::Error::NotFound) => Ok(false),
                err => Err(err.into())
            }
        }
    }

    async fn cas(&self, key: &[u8], old: &[u8], new: &[u8]) -> Result<Option<Vec<u8>>, StoreError> {
        let mut writer = self.db.write()?;
        match self.store.get(&writer, key)? {
            Some(rkv::Value::Blob(val)) if val == old => {
                let val = val.to_owned();
                self.store.put(&mut writer, key, &rkv::Value::Blob(new))?;
                writer.commit()?;
                Ok(Some(val))
            },
            _ => Ok(None)
        }
    }
}

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
