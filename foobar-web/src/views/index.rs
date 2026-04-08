// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::Arc;

use askama::Template;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use chrono::{DateTime, Utc};
use indoc::indoc;
use sqlx::FromRow;

use crate::endpoints::SelfEndpoint;
use crate::result::EndpointResult;
use crate::state::AppState;

#[derive(FromRow)]
struct Item {
    id: i32,
    text: String,
    time: DateTime<Utc>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct TemplateParams<'a> {
    endpoint: &'a SelfEndpoint,
    items: &'a [Item],
}

#[cfg_attr(not(coverage), tracing::instrument(skip_all))]
pub async fn index(endpoint: SelfEndpoint, State(state): State<Arc<AppState>>) -> EndpointResult {
    let items: Vec<Item> = sqlx::query_as(indoc! {r#"
        SELECT
            id,
            text,
            time
        FROM items
        ORDER BY time
    "#})
    .fetch_all(&state.pool)
    .await?;

    Ok(Html(
        TemplateParams {
            endpoint: &endpoint,
            items: &items,
        }
        .render()?,
    )
    .into_response())
}
