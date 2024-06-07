use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn parse_dir(path: &Path, filter: FileType) -> Vec<DefinedFile> {
    let files = fs::read_dir(path).expect(&format!("`{}` folder not found", path.display()));
    let mut output: Vec<DefinedFile> = vec![];
    for file in files {
        let file: DefinedFile = file.unwrap().into();

        //skip dot files
        if file.name().starts_with(".") {
            continue;
        }

        //filter by type
        if filter == FileType::Both || file.ty == filter {
            output.push(file)
        }
    }
    output
}

pub fn cyrb53(str: &str) -> String {
    let seed: u64 = 1;
    let mut h1 = 0xdeadbeef ^ seed;
    let mut h2 = 0x41c6ce57 ^ seed;
    for ch in str.chars() {
        let code = ch as u64;
        h1 = (h1 ^ code).wrapping_mul(2654435761);
        h2 = (h2 ^ code).wrapping_mul(1597334677);
    }
    h1 = (h1 ^ (h1 >> 16)).wrapping_mul(2246822507);
    h1 ^= (h2 ^ (h2 >> 13)).wrapping_mul(3266489909);
    h2 = (h2 ^ (h2 >> 16)).wrapping_mul(2246822507);
    h2 ^= (h1 ^ (h1 >> 13)).wrapping_mul(3266489909);

    format!("{:x}", (4294967296 * (2097151 & h2) + h1))
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileType {
    Folder,
    File,
    Both,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct DefinedFile {
    pub path: PathBuf,
    pub ty: FileType,
}

impl DefinedFile {
    pub fn new(path: PathBuf, ty: FileType) -> Self {
        Self { path, ty }
    }
    pub fn name(&self) -> String {
        let name = self.path.file_name().unwrap_or_default();
        name.to_str().unwrap().to_string()
    }
}

impl From<fs::DirEntry> for DefinedFile {
    fn from(entry: fs::DirEntry) -> Self {
        let path = entry.path();
        let ty = if path.is_dir() {
            FileType::Folder
        } else {
            FileType::File
        };
        Self::new(path, ty)
    }
}