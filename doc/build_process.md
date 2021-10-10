# Build Process

## Build Metadata

`rusty` searches all directories described in `config.toml` recursively. While
walking the directories, `rusty` builds a database that contains:

- file extension:
    - doc(`.md`)
    - img(`.jpg`, `.png`, `.svg`)
    - archive(`.zip`, `.gz`, `.7z`, `.tgz`, `.tar`)
- slug (html link)
    - `.md`
        - filename without `YYYY-MM-DD-` format
        - If `slug` is in `.md` metadata, use the `slug`
        - `index.md` will have its directory as its `slug` by default
        - if `noslug` is true in the `.md` metadata, skip
    - img
        - filename as `slug` with `img:` prefix.
            e.g) `block-diagram.png` --> `img:block-diagram.png`
        - OR: `path/to/img/filename.png` as `slug` to remove dup.
        - file location is stored in another field `loc` or `path`
        - TODO: How to handle the duplicated image names?
        - imgs will be copied into other directory and upload separately (maybe
          to GCS or S3?)
    - archive
        - similar to img but `file:` prefix
- path
    - Each file path (source). The target path may differ while processing the
      files. For example, image can be stored into different domain and into
      single directory with hashed filename rather than original name (or hashed
      directory/filename.ext)

### Intra link `.md`

Then, build a big intra link markdown file. It is basically:

```md
[slug-1]: /path/to/the-doc.thml
[slug-2]: /path/to/another-doc.html
[img:/path/to/image-1.jpg]: https://maybe.another.domain/path/to/image-1.jpg
[file:/path/to/archive-1.zip]: https://maybe.another.domain/path/to/archive-1.zip
```
