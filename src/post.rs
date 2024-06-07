use std::{mem, path::PathBuf};

use chrono::{DateTime, Local, NaiveDate, TimeZone};
use post_archiver::{ArchiveContent, ArchiveFile, ArchiveFrom, ArchivePost};

use crate::{
    cache::ImageCache, files::{collect_files, get_thumb}, utils::{cyrb53, parse_dir, DefinedFile, FileType}
};

pub fn archive_posts_files(author: &DefinedFile, cache:&ImageCache ) -> (Vec<ArchivePost>,Vec<(PathBuf,ArchiveFile)>) {
    let posts = parse_dir(&author.path, FileType::Folder);
    let mut result_posts = Vec::with_capacity(posts.len());
    let mut result_files = Vec::with_capacity(posts.len());

    let output = PathBuf::from(&author.name());
    for post in posts {
        let (date, name) = date_and_name(post.name());

        let id = cyrb53(&name);
        let output = output.join(&id);
        let files_with_path = collect_files(post.clone(),cache).into_iter().map(|mut file|{
            let mut output = output.join(file.filename().to_string_lossy().to_string());
            mem::swap(&mut output,match &mut file {
                ArchiveFile::Image { path, .. } => path,
                ArchiveFile::Video { path, .. } => path,
                ArchiveFile::File { path, .. } => path,
            });
            (output.canonicalize().unwrap(),file)
        }).collect::<Vec<_>>();

        let files: Vec<ArchiveFile> = files_with_path.iter().map(|(_,file)|file.clone()).collect();
        let thumb = get_thumb(&files);

        let mut content = vec![ArchiveContent::Text("Archive from `fanbox-dl`".to_string())];
        content.extend(files.iter().map(|file| match file {
            ArchiveFile::Image { path, .. } => {
                ArchiveContent::Image(path.to_string_lossy().to_string())
            }
            ArchiveFile::Video { path, .. } => {
                ArchiveContent::Video(path.to_string_lossy().to_string())
            }
            ArchiveFile::File { path, .. } => {
                ArchiveContent::File(path.to_string_lossy().to_string())
            }
        }));

        result_files.extend(files_with_path);

        result_posts.push(ArchivePost {
            id,
            title: name,
            author: author.name(),
            from: ArchiveFrom::Fanbox,
            thumb,
            files,
            updated: date,
            published: date,
            content,
            comments: vec![],
        });
    };

    result_posts.sort_by(|a, b| a.updated.cmp(&b.updated));

    (result_posts,result_files)
}

pub fn date_and_name(filename: String) -> (DateTime<Local>, String) {
    let date = filename[0..10].to_string();
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").expect(&format!("ERROR ({})",date)).and_hms_opt(0, 0, 0).unwrap();
    let date = Local.from_local_datetime(&date).single().unwrap();
    let name = filename[11..].to_string();
    (date, name)
}
