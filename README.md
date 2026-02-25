# rust-web-project-template

[![CI](https://github.com/AMDmi3/rust-web-project-template/actions/workflows/ci.yml/badge.svg)](https://github.com/AMDmi3/rust-web-project-template/actions/workflows/ci.yml)

A template for rust project consisting of a website backend and update
daemon, connected through a PostgreSQL database. Suitable for building
services like [Repology](https://github.com/repology/repology-rs).

## Features

### General

- Configuration.
  - `clap` based CLI argument parser.
  - `toml` based config file parser.
  - Result from both is merged, CLI overriding config file.
- Logging support.
  - `tracing` based logging.
  - Support for logging to files.
    - Local timezone for timestamps.
    - Daily rotation (though it's not affected by the timezone yet).
  - Support for shipping logs to Grafana Loki.
- Metrics support.
  - `metrics` support with Prometheus export.
  - Out of box `metrics-process` collection.
  - Out of box tokio metrics collection, including unstable metrics.
- PostgreSQL database support through `sqlx`.
  - `indoc` is used for nicely formatted multiline queries in the code.
  - `sqlx::query!` macros are not used (yet?), so no database connection
    is required for compilation.
  - Migrations support. Migration are available from the common module,
    and can be used from both the daemon and webapp (and tests also).

### Daemon

- No much more that what's listed in the section above.
- A place to add the update logic right away.

### Webapp

- `axum` based web service.
- `askama` based templates.
  - Template inheritance is used, so all common HTML code resides in the
    single `_base.html` file .
  - Minimal custom CSS framework is included (responsive, supporting,
    automatic light/dark themes, basic page elements and styling).
- Advanced static files handling.
  - Files from the `static` directory are automatically compiled into binary.
  - Endpoint for serving these is included.
  - A file may be accessed by a hashed name (e.g. `<filename>.<hash>.<ext>`),
    which allows infinite caching without invalidation issues. Corresponding
	headers are set out of box.
  - Files are compressed on startup, and compressed content is served for
    clients which accept it.
- Statically enumerated endpoint registry with template helpers.
  - Each endpoint is assigned an unique `enum` value.
  - When constructing internal links, endpoints are referred by such enum value,
    which makes broken internal links impossible, and simplifies moving endpoints
	around.
  - Endpoints can also be grouped into sections for use in website navigation.
- Includes middleware for tracking response codes, sizes, and latency for each
  endpoint.
- Extensive integration tests support.
  - Migrations and fixtures support from `sqlx`.
  - Webapp may be constructed as an object and used in the tests.
  - Additionally to regular tests which check given invariants, `insta` based
    snapshot tests are supported, which compare complete webapp output
    to previously stored state. 

## Requirements

This code requires latest Rust-nightly.

## Running

0. You can rename the project (fixin all occurrances of placeholder name
   in paths and the code) by calling

   ```
   ./rename.sh <target name>
   ```

1. Prepare a database

   ```
   psql --username postgres -c "CREATE DATABASE foobar"
   psql --username postgres -c "CREATE USER foobar WITH PASSWORD 'foobar'"
   psql --username postgres -c "GRANT ALL ON DATABASE foobar TO foobar"
   ```

2. Run the daemon

   ```
   cargo run --bin foobar-daemon -- --dsn postgresql://foobar:foobar@localhost/foobar
   ```

3. Run the webapp

   ```
   cargo run --bin foobar-web -- --dsn postgresql://foobar:foobar@localhost/foobar --listen 127.0.0.1:3000
   ```

## Author

- [Dmitry Marakasov](https://github.com/AMDmi3) <amdmi3@amdmi3.ru>

## License

- [GPLv3 or later](LICENSE).
