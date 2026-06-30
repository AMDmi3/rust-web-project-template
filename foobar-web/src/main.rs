// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod init;

use anyhow::Context;
use tracing::info;

use foobar_web::config::Config;
use foobar_web::create_app;

use crate::init::{init_database, init_logging, init_metrics};

async fn async_main() -> anyhow::Result<()> {
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

    info!("initializing application");
    let app = create_app(pool).await?;

    info!("listening");
    let listener = tokio::net::TcpListener::bind(&config.listen).await.unwrap();
    axum::serve(listener, app)
        .await
        .context("error starting HTTP server")
}

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}
