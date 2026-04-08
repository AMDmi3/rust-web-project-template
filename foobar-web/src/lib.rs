// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

#![feature(coverage_attribute)]

pub mod config;
mod handlers;
mod middleware;
mod result;
mod routes;
mod state;
mod static_files;

use std::sync::Arc;

use axum::Router;

use sqlx::PgPool;
use tracing::info;

//use crate::config::AppConfig;
use crate::routes::Route;
use crate::state::AppState;
use crate::static_files::STATIC_FILES;

#[cfg_attr(not(coverage), tracing::instrument(name = "app init", skip_all))]
pub async fn create_app(pool: PgPool) -> anyhow::Result<Router> {
    let state = Arc::new(AppState::new(pool.clone()));

    info!("initializing static files");
    let _ = &*STATIC_FILES;

    info!("initializing routes");
    Ok(Route::to_router_with(|router| {
        router.layer(axum::middleware::from_fn(middleware::metrics_middleware))
    })
    .with_state(state))
}
