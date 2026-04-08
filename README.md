# rust-web-project-template

[![CI](https://github.com/AMDmi3/rust-web-project-template/actions/workflows/ci.yml/badge.svg)](https://github.com/AMDmi3/rust-web-project-template/actions/workflows/ci.yml)

A template for rust project consisting of a website backend and update
daemon, connected through a PostgreSQL database. Suitable for building
services like [Repology](https://github.com/repology/repology-rs).

## Features

### General

- Async [tokio](https://crates.io/crates/tokio) runtime.
- Configuration.
  - [clap](https://crates.io/crates/clap) based CLI argument parser.
  - [toml](https://crates.io/crates/toml) based config file parser.
  - Result from both is merged, CLI overriding config file.
- Logging support.
  - [tracing](https://crates.io/crates/tracing) based logging.
  - Support for logging to files.
    - Local timezone for timestamps.
    - Daily rotation (though it's not affected by the timezone yet).
  - Support for shipping logs to Grafana Loki.
- Metrics collection.
  - [metrics](https://crates.io/crates/metrics) support with Prometheus export.
  - Out of box [metrics-process](https://crates.io/crates/metrics-process) collection.
  - Out of box tokio [metrics](https://docs.rs/tokio/latest/tokio/runtime/struct.RuntimeMetrics.html) collection, including unstable metrics.
- PostgreSQL database support through [sqlx](https://crates.io/crates/sqlx).
  - [indoc](https://crates.io/crates/indoc) is used for nicely formatted multiline queries in the code.
  - `sqlx::query!` macros are not used (yet?), so no database connection is required for compilation.
  - Migrations support. Migration are available from the common module, and can be used from both the daemon, the webapp, and tests.
  - All applications set PostgreSQL `application_name`.
- Testing.
  - Intended to use with [cargo llvm-cov](https://crates.io/crates/cargo-llvm-cov).
  - Unit tests are excluded from coverage for more correct coverage metrics (`#![feature(coverage_attribute)]`).
  - A hack is used to support coverage for functions instrumented with `tracing::instrument`.
- Continuous integration.
  - GitHub actions pipeline running build (both at workspace level, and for individual crates through [cargo hack](https://crates.io/crates/cargo-hack)) and all kinds of tests, for pushes, PRs and weekly (to avoid bitrot).
  - CI uses fresh version of PostgreSQL.
  - CI uses pinned nighly rust to avoid unwanted breakages and use cache more effectively.
  - [Swatinem/rust-cache](https://github.com/Swatinem/rust-cache) is used for more effective rust caching. Additionally, `clippy` is set up to use different target directory than normal build to avoid cache invalidation.
  - Dependabot config for keeping dependencies up to date, with 14 day cooldown for major and minor updates.

### Daemon

- No much more that what's listed in the section above.
- A place to add the update logic right away.

### Webapp

- [axum](https://crates.io/crates/axum) based web service.
- [askama](https://crates.io/crates/aslama) based templates.
  - Template inheritance is used, so all common HTML code resides in the single `_base.html` file.
  - Minimal [custom CSS framework](https://github.com/AMDmi3/amdmi3.css) is included (responsive, supporting automatic light/dark themes, basic page elements and styling).
- Advanced static files handling.
  - Files from the `static` directory are automatically compiled into binary.
  - Endpoint for serving these is included (`/static/<filename>`).
  - A file may be accessed by a hashed name (e.g. `<filename>.<hash>.<ext>`), which allows infinite caching without invalidation issues. Corresponding headers are set out of box.
  - Files are compressed on startup, and compressed content is served for clients which accept it.
- Statically enumerated endpoint registry with template helpers.
  - Each endpoint is assigned an unique `enum` value.
  - When constructing internal links, endpoints are referred by such enum value, which makes broken internal links impossible, and simplifies moving endpoints around.
  - Endpoints can also be grouped into sections for use in website navigation.
- Includes middleware for tracking response codes, sizes, and latency for each route.
- Includes middleware adding basic security HTTP headers, tunable from route properties.
- Extensive integration tests support.
  - Concise HTTP endpoint tests with [axum-test](https://crates.io/crates/axum-test).
  - Migrations and fixtures support from [sqlx](https://crates.io/crates/sqlx).
  - Webapp may be constructed as an object and used in the tests.
  - HTML validation in tests with [tidier](https://crates.io/crates/tidier).
  - [insta](https://crates.io/crates/insta) based snapshot tests in addition to integration tests (note: `profile.dev.package.insta.opt-level = 3` recommented setting is deliberately omitted, as it doesn't seem to provide any gain).

## Requirements

This code requires latest Rust-nightly.

## Running

0. You can rename the project (fixing all occurrances of placeholder name
   in paths and the code) by calling

   ```
   ./rename.sh <target name>
   ```

1. Prepare the database

   (note that you likely want stronger password for production usage)

   ```
   sudo -u postgres psql -c "CREATE DATABASE foobar"
   sudo -u postgres psql -c "CREATE USER foobar WITH PASSWORD 'foobar'"
   sudo -u postgres psql -c "GRANT ALL ON DATABASE foobar TO foobar"
   ```

2. Run the daemon

   (note that you DSN may vary depending on postgresql settings)

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
