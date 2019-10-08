use rand::rngs::OsRng;
use anyhow::anyhow;
use rkv::{ Rkv, StoreOptions };
use titso_core::packet::*;
use crate::util::StoreError;


pub struct Titso<'a> {
    core: titso_core::Titso,
    db: &'a Rkv
}

impl<'a> Titso<'a> {
    pub fn init(db: &'a Rkv, password: &[u8]) -> anyhow::Result<Titso<'a>> {
        let (core, buf) = titso_core::Titso::init(&mut OsRng, password)?;
        let store = db.open_single("master", StoreOptions::create())
            .map_err(StoreError)?;
        let mut writer = db.write().map_err(StoreError)?;
        store.put(&mut writer, "secret", &rkv::Value::Blob(&buf))
            .map_err(StoreError)?;
        writer.commit().map_err(StoreError)?;
        Ok(Titso { core, db })
    }

    pub fn open(db: &'a Rkv, password: &[u8]) -> anyhow::Result<Titso<'a>> {
        let store = db.open_single("master", StoreOptions::default())
            .map_err(StoreError)?;
        let reader = db.read().map_err(StoreError)?;
        if let Some(rkv::Value::Blob(buf)) = store.get(&reader, "secret")
            .map_err(StoreError)?
        {
            let core = titso_core::Titso::open(password, buf)?;
            Ok(Titso { core, db })
        } else {
            Err(anyhow!("Database not initialized"))
        }
    }

    pub fn derive(&self, tags: &[impl AsRef<str>], rule: &Rule) -> String {
        self.core.derive(tags, rule)
    }

    pub fn get(&self, tags: &[impl AsRef<str>]) -> anyhow::Result<Item> {
        let Tag(tag) = self.core.store_tag(tags);

        let store = self.db.open_single("item", StoreOptions::default())
            .map_err(StoreError)?;
        let reader = self.db.read().map_err(StoreError)?;
        if let Some(rkv::Value::Blob(buf)) = store.get(&reader, &tag)
            .map_err(StoreError)?
        {
            self.core.get(&tags, buf).map_err(Into::into)
        } else {
            Err(anyhow!("Tag not found or Password wrong"))
        }
    }

    pub fn put(&self, tags: &[impl AsRef<str>], item: &Item) -> anyhow::Result<()> {
        let buf = self.core.put(tags, item)?;
        let Tag(tag) = self.core.store_tag(&tags);

        let store = self.db.open_single("item", StoreOptions::create())
            .map_err(StoreError)?;
        let mut writer = self.db.write().map_err(StoreError)?;
        store.put(&mut writer, &tag, &rkv::Value::Blob(&buf))
            .map_err(StoreError)?;
        writer.commit().map_err(StoreError)?;

        Ok(())
    }

    pub fn del(&self, tags: &[impl AsRef<str>]) -> anyhow::Result<()> {
        let Tag(tag) = self.core.store_tag(tags);

        let store = self.db.open_single("item", StoreOptions::create())
            .map_err(StoreError)?;
        let mut writer = self.db.write().map_err(StoreError)?;
        store.delete(&mut writer, &tag).map_err(StoreError)?;
        writer.commit().map_err(StoreError)?;

        Ok(())
    }
}
