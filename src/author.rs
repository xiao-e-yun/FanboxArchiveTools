use std::path::PathBuf;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{post::Post, utils::{parse_dir, DefinedFile, FileType}};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub path: PathBuf,
    pub posts: Vec<Post>,
}

impl From<DefinedFile> for Author {
    fn from(folder: DefinedFile) -> Self {
      assert!(folder.ty == FileType::Folder);
      let posts = parse_dir(&folder.path, FileType::Folder);
      let posts = posts
          .into_par_iter()
          .map(|post| post.into())
          .collect::<Vec<Post>>();
        Self {
            name: folder.name(),
            path: folder.path,
            posts,
        }
    }
}