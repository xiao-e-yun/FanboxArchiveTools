use std::{collections::HashSet, fs, path::PathBuf};

use log::info;
use post_archiver::{
    ArchiveAuthor, ArchiveAuthorsList, ArchiveFile, ArchivePost, ArchivePostShort,
};

use crate::config::Config;

macro_rules! write_json {
    ($path:expr, $data:expr, extend) => {{
        let mut data = $data;
        if let Some(old) = serde_json::from_slice(&fs::read(&$path).unwrap_or_default()).ok() {
            data.extend(old);
        };
        write_json!($path, data);
    }};
    ($path:expr, $data:expr) => {{
        let json = serde_json::to_string(&$data).unwrap();
        fs::write($path, json).unwrap();
    }};
}

pub fn build(
    config: Config,
    authors_list: ArchiveAuthorsList,
    authors: Vec<ArchiveAuthor>,
    posts: Vec<ArchivePost>,
    files: Vec<(PathBuf, ArchiveFile)>,
) {
    let output = config.output;

    info!("Writing authors list");
    if !output.exists() {
        fs::create_dir(&output).unwrap();
    }

    write_json!(output.join("authors.json"), authors_list, extend);

    info!("Writing authors");
    let mut skips = HashSet::new();
    for mut author in authors.into_iter() {
        let output = join_and_mkdir(&output, &author.id);
        if let Some(old) =
            serde_json::from_slice::<ArchiveAuthor>(&fs::read(&output.join("author.json")).unwrap_or_default()).ok()
        {
            old.posts.iter().for_each(|p| {
                skips.insert((p.author.clone(), p.title.clone()));
            });

            let post: Vec<ArchivePostShort> = author
                .posts
                .iter()
                .filter(|p| !skips.contains(&(p.author.clone(), p.title.clone())))
                .cloned()
                .collect();

            author.extend(old);
            author.posts = post;
        };

        write_json!(output.join("author.json"), author, extend);
    }

    info!("Writing posts");
    for post in posts.into_iter() {
        if skips.contains(&(post.author.clone(), post.title.clone())) {
            continue;
        }
        let output = join_and_mkdir(&output.join(&post.author), &post.id);
        write_json!(output.join("post.json"), post);
    }

    info!("Copying files");
    for (source, file) in files {
        let output = output.join(&file.path());

        if !output.parent().unwrap().exists() {
            continue;
        }

        if fs::read_link(&output).is_ok() {
            continue;
        }

        #[cfg(unix)]
        std::os::unix::fs::symlink(&source, &output)
            .expect(&format!("Link Error: {:?} {:?}", &source,&output));
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(&source, &output)
            .expect(&format!("Link Error: {:?}", &output));
    }
}

fn join_and_mkdir(path: &PathBuf, rhs: &str) -> PathBuf {
    let path = path.join(rhs);
    if !path.exists() {
        fs::create_dir(&path).unwrap();
    }
    path
}
