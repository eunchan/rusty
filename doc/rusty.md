# Rusty

## Commands

### `build`

`rusty build` command let `rusty` builds a website based on `config.toml` into
the configured directory (`.site` by default)

### `clean`

`rusty clean` deletes cache files and the generated website files too.

### `deploy`

`rusty deploy` builds the website and uploads to remote servers. `rusty`
supports following services:

- Github repo
- Google Cloud Storage
- Amazon AWS S3

### `watch`

`rusty watch` builds website and run a local webserver with the port defined in
`config.toml` (`:8000` by default). If files are changed, `rusty` rebuilds the
site while running the local webserver.
