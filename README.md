# Fanbox Archive Tools

Accept `hareku/fanbox-dl` files to archive.

| DON'T REMOVE YOUR SOURCE ARCHIVE NO MATTER.

1. Fast  
   Made by Rust
2. Safe  
   Parse all filename to url-safe
3. Extract Zip
    Auto Extract Zip File (ignore need password)

## Usage

```sh
#download archive from fanbox
fanbox-dl --sessid YOUR_SESSION --save-dir YOUR_INPUT_DIR --dir-by-post -all

#build archive to http server version
fanbox-archive-tools YOUR_INPUT_DIR YOUR_OUTPUT_DIR

#http-server
npx http-server YOUR_OUTPUT_DIR
```

in your `YOUR_OUTPUT_DIR`

```
  authors.json
  [author]
  | author.json
  | [hash(post.name)]
  | | post.json
  | | [files]
```

### Types
Check [PostArchiver
](https://github.com/xiao-e-yun/PostArchiver)