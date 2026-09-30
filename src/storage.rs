use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
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
        let file = self.path.join(filename);
        let bytes = serde_json::to_vec_pretty(value).unwrap();
        fs::write(file, bytes).unwrap();
    }

    pub fn load_json<T: for<'de> Deserialize<'de>>(&self, filename: &str) -> Option<T> {
        let file = self.path.join(filename);
        let bytes = fs::read(file).ok()?;
        serde_json::from_slice(&bytes).ok()
    }
}
