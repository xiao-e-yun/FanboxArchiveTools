use std::{fs, io, path::PathBuf};

use log::info;
use mime_guess::{mime, Mime, MimeGuess};
use post_archiver::ArchiveFile;

use crate::{
    cache::ImageCache,
    utils::{parse_dir, DefinedFile, FileType},
};

pub fn collect_files(post: DefinedFile, cache: &ImageCache) -> Vec<ArchiveFile> {
    let files = parse_dir(&post.path, FileType::File);
    let mut result = Vec::with_capacity(files.len());

    for file in files {
        match_file(file, &mut result, cache);
    }

    result.sort_by(|a, b| a.filename().cmp(&b.filename()));

    result
}

pub fn get_thumb(files: &[ArchiveFile]) -> Option<PathBuf> {
    let mut thumb = None;
    let mut newest = String::new();
    for file in files {
        let ArchiveFile::Image { path, filename, .. } = file else {
            continue;
        };
        let filename = filename.to_string_lossy().to_string();
        if newest.is_empty() || filename < newest {
            newest = filename;
            thumb = Some(path.clone());
        }
    }
    thumb
}

fn match_file(file: DefinedFile, result: &mut Vec<ArchiveFile>, cache: &ImageCache) -> Option<()> {
    match file.ty {
        FileType::File => {
            let mime = MimeGuess::from_path(&file.path);
            let mime = try_get_mime(&file, mime, result)?;

            //extract zip files if it is
            try_extract(&file, result, cache);

            //handle file
            result.push(handle_file_mime(mime, &file, cache));
        }
        _ => {}
    }
    Some(())
}

fn handle_file_mime(mime: Mime, file: &DefinedFile, cache: &ImageCache) -> ArchiveFile {
    let filename = PathBuf::from(file.name());
    let path = file.path.clone();
    match mime.type_() {
        mime::IMAGE => {
            let (width, height) = cache.get(&file.path).unwrap_or_else(|| {
                let size = imagesize::size(&file.path)
                    .map(|s| (s.width as u32, s.height as u32))
                    .unwrap_or_default();
                cache.insert(file.path.clone(), size);
                size
            });

            ArchiveFile::Image {
                width,
                height,
                filename,
                path,
            }
        }
        mime::VIDEO => ArchiveFile::Video { filename, path },
        _ => ArchiveFile::File { filename, path },
    }
}

fn try_get_mime(
    file: &DefinedFile,
    mime: MimeGuess,
    result: &mut Vec<ArchiveFile>,
) -> Option<Mime> {
    let mime = mime.first();
    if mime.is_some() {
        return mime;
    };

    result.push(ArchiveFile::File {
        filename: PathBuf::from(file.name()),
        path: file.path.clone(),
    });
    None
}

fn try_extract(file: &DefinedFile, result: &mut Vec<ArchiveFile>, cache: &ImageCache) {
    let ext = file.path.extension().unwrap_or_default().to_str().unwrap();
    if "zip" == ext {
        //we assume it's a zip file
        let parent = file.path.parent().unwrap();
        let prefix = file.path.file_prefix().unwrap().to_string_lossy();

        //check is it already extracted
        let extract_path = parent.join(prefix.to_string());
        if extract_path.exists() {
            return;
        }

        //extract zip file
        info!("Extracting: {:?}", file.path);
        let file = fs::File::open(file.path.clone()).unwrap();
        let Ok(mut archive) = zip::ZipArchive::new(file) else {
            fs::create_dir(parent.join(prefix.to_string())).unwrap();
            return; // skip if failed to open
        };

        //extract all files to post folder and rename to `<ZIP_NAME>.<FILE_NAME>`
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();

            //ignore directories
            if file.is_dir() {
                continue;
            }

            //ignore psd files
            let outpath = file.mangled_name();
            let ext = outpath.extension().unwrap_or_default().to_str().unwrap();

            if ["psd"].contains(&ext) {
                info!("Ignoring: {:?}", outpath);
                continue;
            }

            //extract file
            let outpath = parent.join(format!(
                "{}.{}",
                prefix,
                outpath.to_string_lossy().to_string().replace("/", ".")
            ));
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
            result.extend(collect_files(
                DefinedFile {
                    path: outpath,
                    ty: FileType::File,
                },
                cache,
            ));
        }

        //create a folder to store extracted files
        fs::create_dir(parent.join(prefix.to_string())).unwrap();
    }
}
