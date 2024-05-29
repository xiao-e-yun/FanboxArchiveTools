#![feature(path_file_prefix)]

use std::path::PathBuf;

use cache::ImageCache;
use clap::Parser;
use output::{
    AuthorJson, AuthorsJson, OutputAuthors, OutputBuilder, OutputPostMetadata, OutputPosts,
    PostJson,
};
use utils::{get_image_size, parse_dir, FileType};

mod args;
mod author;
mod cache;
mod output;
mod post;
mod utils;

fn main() {
    let args = args::Args::parse();
    let time = std::time::Instant::now();

    if !args.quiet {
        println!("\n=Init=====================================\n");
        println!("Input Folder: {}", args.input.display());
        println!("Output Folder: {}", args.output.display());
    }

    if !args.output.exists() {
        std::fs::create_dir_all(&args.output).unwrap();
        println!("`{}` output folder created", args.output.display());
    }

    let authors = parse_dir(&args.input, FileType::Folder);
    let mut authors = authors
        .into_iter()
        .map(|author| author.into())
        .collect::<Vec<author::Author>>();

    if !args.quiet {
        // display authors
        let authors_len = authors.len();
        println!("\n=Authors==================================\n");

        let mut total_posts = 0;
        let mut total_files = 0;
        for author in authors.clone() {
            println!("+ Author: {}", author.name);
            let post_len = author.posts.len();
            let files_len = author
                .posts
                .iter()
                .map(|post| post.files.len())
                .sum::<usize>();
            println!("|- Total Posts: {}", post_len);
            println!("|- Total Files: {}", files_len);
            println!("");
            total_posts += post_len;
            total_files += files_len;
        }

        println!("Total Authors: {}", authors_len);
        println!("Total Posts: {}", total_posts);
        println!("Total Files: {}", total_files);
    }

    if !args.quiet {
        println!("\n=Read Iamges Size=========================\n");
    }

    let force = args.force;
    let mut cache = ImageCache::load(&args.input, force).unwrap_or(ImageCache::new(force));
    get_image_size(&mut authors, &mut cache);
    cache.save(&args.input);

    if !args.quiet {
        println!("\n=Writing & Linking========================\n");
    }

    // expected output
    //
    //   authors.json
    //   [author]
    //   - author.json
    //   - [cyrb53(post.name)]
    //     - post.json
    //     - [files]
    //

    let mut builder = OutputBuilder::new(args.output);
    let mut authors_output: AuthorsJson = vec![];
    authors.sort_by(|a,b|a.name.cmp(&b.name));
    for author in authors.iter() {
        println!("Checking: {}", author.name);

        let author_path = PathBuf::from(author.name.clone());
        builder.folder(author_path.clone());

        let mut author_thumb = None;
        let mut posts_output: AuthorJson = vec![];
        let mut posts = author.posts.clone();
        posts.sort_by(|a,b|b.date().partial_cmp(&a.date()).unwrap());
        for post in posts {
            let (post_path, post_output, files) = OutputPosts::new(post.clone());
            let post_path = author_path.join(post_path);
            builder.folder(post_path.clone());

            for (from, to) in files {
                builder.copy(from, post_path.join(to));
            }

            if author_thumb.is_none() {
                author_thumb = post_output.thumb();
            }

            let postmeta: PostJson = OutputPostMetadata::new(post);
            builder.write(
                post_path.join("post.json"),
                serde_json::to_string(&postmeta).unwrap(),
            );
            posts_output.push(post_output);
        }

        builder.write(
            author_path.join("author.json"),
            serde_json::to_string(&posts_output).unwrap(),
        );
        authors_output.push(OutputAuthors::new(author.name.clone(), author_thumb));
    }
    builder.write(
        PathBuf::from("authors.json"),
        serde_json::to_string(&authors_output).unwrap(),
    );

    if !args.quiet {
        println!("Building");
    }
    builder.finish();

    if !args.quiet {
        println!("\n=All done================================\n");
        println!("Time: {} ms", time.elapsed().as_millis());
    }
}
