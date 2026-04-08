// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

#![feature(coverage_attribute)]

pub mod config;
mod handlers;
mod result;
mod routes;
mod state;
mod static_files;

use std::sync::Arc;
use std::time::Instant;

use axum::{
    Router,
    body::HttpBody,
    extract::Request,
    middleware::{self, Next},
    response::IntoResponse,
};

use metrics::{counter, histogram};
use sqlx::PgPool;
use tracing::info;

//use crate::config::AppConfig;
use crate::routes::{Route, SelfRoute};
use crate::state::AppState;
use crate::static_files::STATIC_FILES;

async fn track_metrics(route: SelfRoute, request: Request, next: Next) -> impl IntoResponse {
    let start = Instant::now();
    let response = next.run(request).await;
    let latency = start.elapsed().as_secs_f64();

    let route_name = route.path();
    let status = response.status().as_u16().to_string();

    counter!("foobar_web_http_requests_total", "route" => route_name, "status" => status)
        .increment(1);
    histogram!("foobar_web_http_requests_duration_seconds", "route" => route_name).record(latency);

    if let Some(body_size) = response.body().size_hint().exact() {
        histogram!("foobar_web_http_response_size_bytes", "route" => route_name)
            .record(body_size as f64);
    }

    response
}

#[cfg_attr(not(coverage), tracing::instrument(name = "app init", skip_all))]
pub async fn create_app(pool: PgPool) -> anyhow::Result<Router> {
    let state = Arc::new(AppState::new(pool.clone()));

    info!("initializing static files");
    let _ = &*STATIC_FILES;

    info!("initializing routes");
    Ok(
        Route::to_router_with(|router| router.layer(middleware::from_fn(track_metrics)))
            .with_state(state),
    )
}
