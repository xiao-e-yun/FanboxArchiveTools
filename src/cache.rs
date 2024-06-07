use std::{
    collections::HashMap,
    fs::{self, File},
    path::{Path, PathBuf}, sync::{Arc, Mutex},
};

use log::info;

#[derive(Debug, Clone)]
pub struct ImageCache {
    inner: Arc<Mutex<HashMap<PathBuf, (u32, u32)>>>,
    pub ignore: bool,
}

impl ImageCache {
    pub fn new(ignore: bool) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            ignore,
        }
    }
    pub fn insert(&self, key: PathBuf, value: (u32, u32)) {
        if self.ignore {
            return;
        }
        self.inner.lock().unwrap().insert(key, value);
    }
    pub fn get(&self, key: &Path) -> Option<(u32, u32)> {
        self.inner.lock().unwrap().get(key).cloned()
    }
    pub fn load(path: &Path, ignore: bool) -> Self {
        if ignore {
            return Self::new(ignore);
        };

        let json = fs::read(path.join("image-cache.json")).unwrap_or_default();
        let data: HashMap<PathBuf, (u32, u32)> =
            serde_json::from_slice(json.as_slice()).unwrap_or_default();

        ImageCache {
            inner: Arc::new(Mutex::new(data)),
            ignore,
        }
    }
    pub fn save(&self, path: &Path) {
        if self.ignore {
            info!("Image cache is ignored");
            return;
        }

        let file = File::create(path.join("image-cache.json")).unwrap();
        serde_json::to_writer(file, &*self.inner.lock().unwrap()).unwrap();
        info!("Image cache saved");
    }
}
