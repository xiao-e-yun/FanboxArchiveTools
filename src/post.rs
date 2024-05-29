use std::{fs, io, path::PathBuf};

use mime_guess::{mime, MimeGuess};
use serde::{Deserialize, Serialize};

use crate::utils::{cyrb53, parse_dir, DefinedFile, FileType};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Post {
    pub filename: String,
    pub path: PathBuf,
    pub files: PostFiles,
}

impl Post {
    pub fn name(&self) -> String {
        self.filename[11..].to_string()
    }
    pub fn date(&self) -> String {
        self.filename[0..10].to_string()
    }
}

impl From<DefinedFile> for Post {
    fn from(folder: DefinedFile) -> Self {
        assert!(folder.ty == FileType::Folder);
        let mut files = PostFiles::new();
        files.add(folder.clone());
        Self {
            filename: folder.name(),
            path: folder.path,
            files,
        }
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct PostFiles {
    pub videos: Vec<DefinedFile>,
    pub others: Vec<DefinedFile>,
    pub images: Vec<(DefinedFile, (u32, u32))>,
}

impl PostFiles {
    pub fn new() -> Self {
        Self {
            videos: vec![],
            others: vec![],
            images: vec![],
        }
    }
    pub fn add(&mut self, file: DefinedFile) {
        match file.ty {
            FileType::File => {
                let mime = MimeGuess::from_path(&file.path);

                let Some(mime) = mime.first() else {
                    return self.others.push(file);
                };

                let is_zip = ["zip","7z","tar"].contains(&file.path.extension().unwrap_or_default().to_str().unwrap());
                if is_zip {
                    
                    //we assume it's a zip file
                    let parent = file.path.parent().unwrap();
                    let prefix = file.path.file_prefix().unwrap().to_string_lossy();
                    
                    //check is it already extracted
                    if !parent.join(prefix.to_string()).exists() {
                    
                        //extract zip file
                        println!("Extracting: {:?}", file.path);
                        let file = fs::File::open(file.path.clone()).unwrap();
                        let Ok(mut archive) = zip::ZipArchive::new(file) else {
                            fs::create_dir(parent.join(prefix.to_string())).unwrap();
                            return;
                        };
                        //extract all files to post folder and rename to `<ZIP_NAME>.<FILE_NAME>`
                        for i in 0..archive.len() {
                            let mut file = archive.by_index(i).unwrap();

                            //ignore directories
                            if file.is_dir() {
                                continue;
                            }

                            let outpath = file.mangled_name();
                            let outpath = format!("{}.{}",cyrb53(outpath.to_str().unwrap()), outpath.extension().unwrap_or_default().to_str().unwrap());

                            let outpath =
                                parent.join(format!("{}.{}", prefix, outpath.replace("/", ".")));
                            let mut outfile = fs::File::create(&outpath).unwrap();
                            io::copy(&mut file, &mut outfile).unwrap();
                            self.add(DefinedFile {
                                path: outpath,
                                ty: FileType::File,
                            });
                        }

                        //create a folder to store extracted files
                        fs::create_dir(parent.join(prefix.to_string())).unwrap();

                    }
                }

                match mime.type_() {
                    mime::IMAGE => {
                        self.images.push((file, (0, 0)));
                    }
                    mime::VIDEO => {
                        self.videos.push(file);
                    }
                    _ => {
                        self.others.push(file);
                    }
                }
            }
            FileType::Folder => {
                let files = parse_dir(&file.path, FileType::File);
                for file in files {
                    self.add(file);
                }
            }
            _ => unreachable!(),
        }
    }
    pub fn len(&self) -> usize {
        self.videos.len() + self.others.len() + self.images.len()
    }
    pub fn collect_all(&self) -> Vec<&DefinedFile> {
        let mut output = vec![];
        output.extend(self.videos.iter());
        output.extend(self.others.iter());
        output.extend(self.images.iter().map(|(file, _)| file));
        output
    }
}
