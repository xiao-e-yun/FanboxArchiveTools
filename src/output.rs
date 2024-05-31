use std::{
    fs,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{
    post::Post,
    utils::cyrb53,
};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct OutputBuilder {
    pub target: PathBuf,
    pub files: Vec<OutputType>,
}

impl OutputBuilder {
    pub fn new(target: PathBuf) -> Self {
        Self {
            target,
            files: vec![],
        }
    }
    pub fn write(&mut self, path: PathBuf, data: String) {
        self.files.push(OutputType::Json(path, data));
    }
    pub fn copy(&mut self, from: PathBuf, to: PathBuf) {
        self.files.push(OutputType::File(from, to));
    }
    pub fn folder(&mut self, path: PathBuf) {
        self.files.push(OutputType::Folder(path));
    }

    pub fn finish(&self) {
        let root = &self.target;
        for file in &self.files {
            match file {
                OutputType::Folder(path) => {
                    let full_path = root.join(path);
                    if !full_path.exists() {
                        std::fs::create_dir_all(&full_path).unwrap();
                    }
                }
                OutputType::Json(path, data) => {
                    let full_path = root.join(path);
                    fs::write(full_path, data).unwrap();
                }
                OutputType::File(from, to) => {
                    let full_from = from.canonicalize().unwrap();
                    let full_to = root.join(to);
                    if fs::read_link(&full_to).is_ok() {
                        continue;
                    }

                    #[cfg(unix)]
                    std::os::unix::fs::symlink(&full_from, &full_to).expect(&format!("Link Error: {:?}\n",&full_to.exists()));
                    #[cfg(windows)]
                    std::os::windows::fs::symlink_file(&full_from, &full_to).expect(&format!("Link Error: {:?}\n",&full_to));
                }
            }
        }
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum OutputType {
    Folder(PathBuf),
    Json(PathBuf, String),
    File(PathBuf, PathBuf),
}

pub type AuthorsJson = Vec<OutputAuthors>;
pub type AuthorJson = Vec<OutputPosts>;
pub type PostJson = OutputPostMetadata;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct OutputAuthors(String, Option<PathBuf>);

impl OutputAuthors {
    pub fn new(name: String, thumb: Option<PathBuf>) -> Self {
        Self(name, thumb)
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct OutputPosts {
    id: String,
    name: String,
    date: String,
    preview: Option<PathBuf>,
    length: u32,
}

impl OutputPosts {
    pub fn new(object: crate::post::Post) -> (PathBuf, Self, Vec<(PathBuf, PathBuf)>) {
        let id: String = cyrb53(&object.filename);
        let path = PathBuf::from(id.clone());
        let thumb = object
            .files
            .images
            .first()
            .map(|file| path.join(file.0.name()));
        let output = Self {
            id,
            name: object.name(),
            date: object.date(),
            preview: thumb,
            length: object.files.len() as u32,
        };
        let cllect = object
            .files
            .collect_all()
            .iter()
            .map(|file| (file.path.clone(), PathBuf::from(file.name())))
            .collect();
        (path, output, cllect)
    }
    pub fn thumb(&self) -> Option<PathBuf> {
        self.preview.clone()
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct OutputPostMetadata {
    name: String,
    date: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    images: Vec<OutputImageFile>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    videos: Vec<OutputNormalFile>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    files: Vec<OutputNormalFile>,
}

impl OutputPostMetadata {
    pub fn new(object: Post) -> Self {
        let mut images: Vec<OutputImageFile> = object
            .files
            .images
            .iter()
            .map(|(file, size)| OutputImageFile(file.name(), size.clone()))
            .collect();

        let mut videos: Vec<OutputNormalFile> = object
            .files
            .videos
            .iter()
            .map(|file| OutputNormalFile(file.name()))
            .collect();

        let mut files: Vec<OutputNormalFile> = object
            .files
            .others
            .iter()
            .map(|file| OutputNormalFile(file.name()))
            .collect();

        // 1-<filename> 2-<filename> file-3-<filename> ...
        // sort by index
        fn cmp(a: &str, b: &str) -> std::cmp::Ordering {
            fn get_date(a: &str) -> Result<u32,&str> {
              a.split("-").find_map(|x|x.parse::<u32>().ok()).ok_or(a)
            }
            get_date(a).cmp(&get_date(b))
        }

        images.sort_by(|a, b| cmp(&a.0, &b.0));
        videos.sort_by(|a, b| cmp(&a.0, &b.0));
        files.sort_by(|a, b| cmp(&a.0, &b.0));

        Self {
            name: object.name(),
            date: object.date(),
            images,
            videos,
            files,
        }
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialOrd, Eq, Ord, PartialEq)]
pub struct OutputImageFile(String, (u32, u32));

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialOrd, Eq, Ord, PartialEq)]
pub struct OutputNormalFile(String);
