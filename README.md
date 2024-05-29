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

| Source: `src/output.rs`

`authors.json`

```ts
type AuthorsJson = [AuthorName: String, PreviewPath: String][];
let authorsJson = ["author","fb136d4fbf86d0a5/0-image.png"]
```

utils example
```ts
function getPreviewPath(authors: AuthorsJson) {
  return path.join(ARCHIVE_PATH,AuthorsJson[0],AuthorsJson[1]);
}
```

`author.json`

```ts
type AuthorJson = {
  id: string;
  name: string;
  date: string;
  preview?: string;
  length: u32;
}[];
let authorJson = [{
  id: "15617624",
  name: "PostName",
  date: "2022-01-01",
  preview: "fb136d4fbf86d0a5/0-image.png",
  length: 10
}];
```

utils example
```ts
const authorPath  = `author/`
function getPreviewPath(authorPath: string, author: AuthorJson) {
  return path.join(ARCHIVE_PATH,authorPath,AuthorJson.preview);
}
```

`post.json`
```ts
type PostJson = {
  name: string;
  date: string;
  images?: Image[];
  videos?: Video[];
  files?: File[];
};

const postJson = {
  name: "Sample Post",
  date: "2022-01-01",
  images: [
    ["image1.png", [800, 600]],
    ["image2.png", [1024, 768]]
  ],
  files: ["file.pdf"]
}

type Image = [
  string, //filename
  [
    number, //width
    number //height
  ]
];
type Video = string; //filename
type File = string; //filename
```

utils example
```ts
const postPath = `author/fb136d4fbf86d0a5/`
function getFilePath(postPath: string, filename: string) {
  return path.join(ARCHIVE_PATH,postPath,filename);
}
```
