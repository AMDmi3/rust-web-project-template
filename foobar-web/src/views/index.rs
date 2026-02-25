// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::Arc;

use anyhow::Result;
use askama::Template;
use axum::extract::State;
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use indoc::indoc;
use sqlx::{FromRow, PgPool};

use crate::endpoints::Endpoint;
use crate::result::EndpointResult;
use crate::state::AppState;
use crate::template_context::TemplateContext;

#[derive(FromRow)]
struct Item {
    id: i32,
    text: String,
    time: DateTime<Utc>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct TemplateParams<'a> {
    ctx: TemplateContext,
    items: &'a [Item],
}

#[cfg_attr(not(feature = "coverage"), tracing::instrument(skip_all))]
pub async fn index(State(state): State<Arc<AppState>>) -> EndpointResult {
    let ctx = TemplateContext::new_without_params(Endpoint::Index);

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

    Ok((
        StatusCode::OK,
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::TEXT_HTML.as_ref()),
        )],
        TemplateParams { ctx, items: &items }.render()?,
    )
        .into_response())
}
