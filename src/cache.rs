use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageCache {
    inner: HashMap<PathBuf, (u32, u32)>,
    pub ignore: bool,
}

impl ImageCache {
    pub fn new(ignore: bool) -> Self {
        Self {
            inner: HashMap::new(),
            ignore,
        }
    }
    pub fn insert(&mut self, key: PathBuf, value: (u32, u32)) {
        if self.ignore {
            return;
        }
        self.inner.insert(key, value);
    }
    pub fn get(&self, key: &Path) -> Option<&(u32, u32)> {
        self.inner.get(key)
    }
    pub fn load(path: &Path, ignore: bool) -> Option<Self> {
        let data: HashMap<PathBuf, (u32, u32)> = serde_json::from_reader(BufReader::new(
            File::open(path.join("image-cache.json")).ok()?,
        ))
        .ok()?;
        let mut cache = ImageCache::new(ignore);
        cache.inner = data;
        Some(cache)
    }
    pub fn save(&self, path: &Path) {
        if self.ignore {
            println!("Image cache is ignored");
            return;
        }

        let file = File::create(path.join("image-cache.json")).unwrap();
        serde_json::to_writer(file, &self.inner).unwrap();
        println!("Image cache saved");
    }
}
