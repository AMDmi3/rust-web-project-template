// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod config;
mod init;
mod workers;

use anyhow::Context as _;
use tracing::info;

use crate::config::Config;
use crate::init::{init_database, init_logging, init_metrics};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::parse().with_context(|| "failed to process configuration")?;

    init_logging(&config).with_context(|| "failed to init logging")?;
    init_metrics(&config).with_context(|| "failed to init metrics")?;
    let pool = init_database(&config)
        .await
        .with_context(|| "failed to init database")?;

    if !config.skip_migrations {
        info!("running migrations");

        sqlx::query("CREATE SCHEMA IF NOT EXISTS foobar")
            .execute(&pool)
            .await
            .context("failed to create schema")?;

        foobar_common::MIGRATOR
            .run(&pool)
            .await
            .context("failed to run migrations")?;
    }

    info!("running workers");
    let items = workers::ItemsWorker::new(pool.clone());
    tokio::try_join!(
        items.run(),
        // TODO: add more workers
    )
    .context("worker finished with error")?;

    Ok(())
}
