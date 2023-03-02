use std::fs;
use std::path::{ Path, PathBuf };
use serde_bytes::Bytes;
use titso_core::packet;


pub struct Storage {
    config_path: PathBuf
}

type StoreList<'a> = Vec<(&'a Bytes, &'a Bytes)>;

impl Storage {
    pub fn new(config_path: PathBuf) -> Storage {
        Storage { config_path }
    }

    pub fn get(&self, key: &packet::Tag) -> anyhow::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();
        let list = read_list(&self.config_path, &mut buf)?;

        let item = list.iter()
            .find(|(k, _)| &key.0 == k.as_ref())
            .map(|(_, v)| Vec::from(&***v));
        Ok(item)
    }

    pub fn set(&self, key: &packet::Tag, value: &[u8]) -> anyhow::Result<()> {
        let mut buf = Vec::new();
        let mut list = read_list(&self.config_path, &mut buf)?;

        list.retain(|(k, _)| &key.0 != k.as_ref());
        list.push((Bytes::new(&key.0), Bytes::new(value)));

        write_list(&self.config_path, &list)
    }

    pub fn remove(&self, key: &packet::Tag) -> anyhow::Result<()> {
        let mut buf = Vec::new();
        let mut list = read_list(&self.config_path, &mut buf)?;

        list.retain(|(k, _)| &key.0 != k.as_ref());

        write_list(&self.config_path, &list)
    }
}

fn read_list<'a>(path: &Path, buf: &'a mut Vec<u8>) -> anyhow::Result<StoreList<'a>> {
    use std::io::Read;

    let mut fd = fs::File::open(path.join("titso-storelist.bin"))?;
    buf.clear();
    fd.read_to_end(buf)?;
    cbor4ii::serde::from_slice(buf.as_slice()).map_err(Into::into)
}

fn write_list(path: &Path, list: &StoreList<'_>) -> anyhow::Result<()> {
    use std::io::Write;

    let tmppath = path.join("titso-storelist.bin.tmp");

    let mut fd = fs::File::options()
        .write(true)
        .create_new(true)
        .open(&tmppath)?;
    let buf = cbor4ii::serde::to_vec(Vec::new(), list)?;
    fd.write_all(&buf)?;
    fd.sync_all()?;
    drop(fd);

    fs::rename(tmppath, path.join("titso-storelist.bin"))?;

    Ok(())
}