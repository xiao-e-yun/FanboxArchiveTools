use std::collections::HashSet;

use crate::utils::DefinedFile;
use post_archiver::{ArchiveAuthor, ArchiveFrom, ArchivePost};

pub fn archive_author(author: &DefinedFile, posts: &Vec<ArchivePost>) -> ArchiveAuthor {
    let name = author.name();
    let thumb = posts
        .first()
        .map(|post: &ArchivePost| post.thumb.clone())
        .unwrap_or_default();
    ArchiveAuthor {
        id: name.clone(),
        name,
        thumb,
        from: HashSet::from([ArchiveFrom::Fanbox]),
        posts: posts.iter().map(|post| post.clone().into()).collect(),
    }
}
