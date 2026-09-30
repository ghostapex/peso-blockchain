use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Store {
    pub path: PathBuf,
}

impl Store {
    pub fn new(dir: &str) -> Self {
        let path = PathBuf::from(dir);
        if !path.exists() {
            fs::create_dir_all(&path).unwrap();
        }

        Self { path }
    }

    pub fn save_json<T: Serialize>(&self, filename: &str, value: &T) {
        let path = self.path.join(filename);
        let data = serde_json::to_string_pretty(value).unwrap();
        fs::write(path, data).unwrap();
    }

    pub fn load_json<T: for<'de> Deserialize<'de>>(&self, filename: &str) -> Option<T> {
        let path = self.path.join(filename);
        let raw = fs::read(path).ok()?;
        serde_json::from_str(&raw).ok()
    }
}
