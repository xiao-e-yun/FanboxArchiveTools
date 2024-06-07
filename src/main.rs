#![feature(path_file_prefix)]

use author::archive_author;
use cache::ImageCache;
use clap::Parser;
use log::info;
use output::build;
use post::archive_posts_files;
use post_archiver::{ArchiveAuthor, ArchiveAuthorsList};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use utils::{parse_dir, FileType};

mod config;
mod author;
mod cache;
pub mod files;
mod output;
mod post;
mod utils;

fn main() {
    let config = config::Config::parse();
    let time = std::time::Instant::now();
    env_logger::builder().filter_level(config.log_level()).init();

    info!("");
    info!("=Init=====================================");
    info!("");
    info!("Input Folder: {}", config.input.display());
    info!("Output Folder: {}", config.output.display());

    if !config.output.exists() {
        std::fs::create_dir_all(&config.output).unwrap();
        info!("`{}` output folder created", config.output.display());
    }

    info!("");
    info!("=Reading==================================");
    info!("");
    
    
    

    let cache = ImageCache::load(&config.input, config.force);

    let author_dirs = parse_dir(&config.input, FileType::Folder);
    let (authors, posts, files) = author_dirs.par_iter().map(|author_dir| {
            //collect
            let (posts,files) = archive_posts_files(&author_dir, &cache);
            let author: ArchiveAuthor = archive_author(&author_dir, &posts);

            //log
            info!("+ Author: {}", author.name);
            info!("|- Total Posts: {}", posts.len());
            info!("|- Total Files: {}", files.len());

            (vec![author],posts,files)
        }
    ).reduce(||(vec![],vec![],vec![]), |mut a,b|{
        a.0.extend(b.0);
        a.1.extend(b.1);
        a.2.extend(b.2);
        a
    });

    let author_list = ArchiveAuthorsList::from_vector(authors.clone());

    cache.save(&config.input);

    info!("");
    info!("=Writing & Linking========================");
    info!("");

    // expected output
    //
    //   authors.json
    //   [author]
    //   - author.json
    //   - [cyrb53(post.name)]
    //     - post.json
    //     - [files]
    //
    build(config, author_list, authors, posts, files);

    info!("");
    info!("=All done================================");
    info!("");
    info!("Time: {} ms", time.elapsed().as_millis());
}
