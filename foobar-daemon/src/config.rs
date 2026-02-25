// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use url::Url;

const DEFAULT_DSN: &str = "postgresql://foobar@localhost/foobar";

// Note: do not use default values for args which are also present in
// FileConfig, otherwise config settings will always be overwritten
// by default clap value. Also, since clap does not allow to provide
// default values but not use them, we have to fill default values
// in options docs manually,
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Path to configuration file with default and/or additional settings
    #[arg(short = 'c', long, value_name = "PATH")]
    config: Option<PathBuf>,

    /// PostgreSQL database DSN
    ///
    /// Default: postgresql://foobar@localhost/foobar
    #[arg(short = 'd', long = "dsn", value_name = "DSN")]
    dsn: Option<String>,

    /// Path to log directory
    ///
    /// When specified, output is redirected to a log file in the
    /// given directory with daily rotation and 14 kept rotated files.
    #[arg(long, value_name = "PATH")]
    log_directory: Option<PathBuf>,

    /// Loki log collector URL
    #[arg(long, value_name = "URL")]
    loki_url: Option<Url>,

    /// Socket address for serving Prometheus metrics
    #[arg(long, value_name = "ADDR:PORT")]
    prometheus_export: Option<SocketAddr>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
struct FileConfig {
    dsn: Option<String>,
    log_directory: Option<PathBuf>,
    loki_url: Option<Url>,
    prometheus_export: Option<SocketAddr>,
}

#[derive(Debug)]
pub struct Config {
    pub dsn: String,
    pub log_directory: Option<PathBuf>,
    pub loki_url: Option<Url>,
    pub prometheus_export: Option<SocketAddr>,
}

impl Config {
    pub fn parse() -> Result<Self> {
        let args = CliArgs::parse();

        let config: FileConfig = if let Some(path) = args.config {
            // XXX: a good case for try block to avoid with_context repetition, but heterogeneous
            // try blocks are currently broken, see https://github.com/rust-lang/rust/issues/149025
            let toml = std::fs::read(&path)
                .with_context(|| format!("cannot read config file {}", path.display()))?;
            let toml = std::str::from_utf8(&toml)
                .with_context(|| format!("cannot parse config file {}", path.display()))?;
            toml::from_str(toml)
                .with_context(|| format!("cannot parse config file {}", path.display()))?
        } else {
            Default::default()
        };

        let dsn = args
            .dsn
            .as_deref()
            .or(config.dsn.as_deref())
            .unwrap_or(DEFAULT_DSN)
            .to_string();

        Ok(Config {
            dsn,
            log_directory: args.log_directory.or(config.log_directory),
            loki_url: args.loki_url.or(config.loki_url),
            prometheus_export: args.prometheus_export.or(config.prometheus_export),
        })
    }
}
