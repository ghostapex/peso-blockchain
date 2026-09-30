use rocksdb::{DB, Options};
use serde::Serialize;
use std::path::Path;

pub struct PersistentStore {
    db: DB,
}

impl PersistentStore {
    pub fn new(path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = DB::open(&opts, path).unwrap();

        Self { db }
    }

    pub fn put(&self, key: &str, value: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.db.put(key.as_bytes(), value)?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
        Ok(self.db.get(key.as_bytes())?)
    }

    pub fn delete(&self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.db.delete(key.as_bytes())?;
        Ok(())
    }
}
