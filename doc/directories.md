# Rusty Directory Structure

Usual Rusty website directories are shown below:

```
/
  _site/            -- Generated sites (preview, deploy)
  _tpl/             -- Site templates
  _static/          -- Files being used in the template (included together with md)

  config.toml       -- Site configuration file

  posts/            -- Blog posts. Handled differently than other markdown files

  **/*.md           -- Generated html follows same hierarchy
```

Other than the markdown files in `posts/`, markdown files, images, and archives
maintain the hierarchy in the generated site. For example,
`review/digital/gta-v.md` is generated into `review/digital/gta-v.html` or
`review/digital/gta-v/index.html` based on config.

## `_site/`

`_site/` is a directory that holds all the generated files. `_site/` may have
multiple subfolders. One is for a preview. If you run `build` with `--preview`,
`rusty` creates a site under `_site/preview`. In preview mode, regardless of the
config, all images, archive files are stored into the same bucket. All draft
documents will be rendered too. It is basically configued as
`config.fileserver.is_separate_fileserver = false`.

In deploy mode (or build without `--preview` option), `rusty` generates the site
under `_site/deploy` and optionally `_site/storage` if
`config.fileserver.is_separate_fileserver = true`.

## `_tpl/`

`_tpl/` contains [tera](https://github.com/Keats/tera) template files. Details
are explained in the [Template](template.md) document.

## `_static/`

`_static/`  holds the images, css styles, favicons, etc that are used to build
the site. These static files are copied over into the document bucket not into
the storage bucket. If any files are referred inside the template files, `rusty`
does not parse the template but uses the referred URI directly. In this case, it
is wise to put those files under `_static/`.

For example, `_static/css/website.css` can be referred in the html template file
with `src='/_static/css/website.css'`.

## `posts/`

You can put any markdown files into `posts/` directory. The hierarchy inside
`posts/` directory is ignored. `rusty` uses [Metadata](metadata.md) and reorders
the posts in canonical order. Details are explained in [Blog](blog.md).

## `pages/`

`pages/` contains markdown documents, images, and files. All the assets (md,
img, file) under `pages/` moves up one ladder when being processed. For
instance, the tool processes `pages/article/how-to-design-ssg.md` document and
generates `/article/how-to-design-ssg.html` OR
`/article/how-to-design-ssg/index.html` (May not be supported).
