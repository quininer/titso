use crate::primitive::kdf::Kdf;
use crate::primitive::keyedhash::KeyedHash;
use crate::primitive::aead::Aead;
use crate::packet::*;


pub struct Titso<DB> {
    db: DB,
    mkey: [u8; 32],
}

impl<DB> Titso<DB> {
    pub async fn new(db: DB, password: &[u8]) -> Result<Titso<DB>, ()> {
        let master_store = db.transaction(|store| {
            let table = store.open("metadata")?;
            let value = table.get("master-store")?;
            let master_store = cbor::from_slice(value)?;
            Ok(master_store)
        })?;

        let mut mkey = [0; 32];
        Kdf::default().derive(password, &master_store.salt, &mut mkey);
        for i in 0..32 {
            mkey[i] ^= master_store.store[i];
        }

        Ok(Titso { db, mkey })
    }

    pub fn tag(&self, tag: Tag, new_tag: &str) -> Tag {
        let Tag(mut tag) = tag;
        let mut tmp = [0; 16];
        let mut hasher = KeyedHash::new(&self.mkey);
        hasher.update(new_tag);
        hasher.finalize(&mut tmp);
        for i in 0..16 {
            tag[i] ^= tmp[i];
        }
        Tag(tag)
    }

    pub async fn hint(&self, Tag(itag): Tag) -> Result<Vec<String>, ()> {
        let Packet { mut data, tag } = self.db.transaction(|store| {
            let table = store.open(b"hint")?;
            let value = table.get(&itag)?;
            let packet = cbor::from_slice(value)?;
            Ok(packet)
        })?;

        if Aead::new(&self.mkey, &itag)
            .decrypt(b"hint", &mut data, &tag)
        {
            let hints: Vec<Hint> = serde_cbor::from_slice(&data)?;

            //
        } else {
            //
        }

        unimplemented!()
    }

    pub async fn view(&self, Tag(itag): Tag) {
        let Packet { mut data, tag } = self.db.transaction(|store| {
            let table = store.open(b"data")?;
            let value = table.get(&itag)?;
            let packet = serde_cbor::from_slice(value)?;
            Ok(packet)
        })?;

        if Aead::new(&self.mkey, &itag)
            .decrypt(b"data", &mut data, &tag)
        {
            let Item { password, data } = serde_cbor::from_slice(&data)?;

            //
        } else {
            //
        }

        unimplemented!()
    }
}
