// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::Arc;

use askama::Template;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use chrono::{DateTime, Utc};
use indoc::indoc;
use sqlx::FromRow;

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
#[template(path = "item.html")]
struct TemplateParams<'a> {
    ctx: TemplateContext,
    item: &'a Item,
}

#[cfg_attr(not(coverage), tracing::instrument(skip_all))]
pub async fn item(
    Path(gen_path): Path<Vec<(String, String)>>,
    Path(id): Path<u64>,
    State(state): State<Arc<AppState>>,
) -> EndpointResult {
    let ctx = TemplateContext::new(Endpoint::Item).with_params(&gen_path);

    let item: Option<Item> = sqlx::query_as(indoc! {r#"
        SELECT
            id,
            text,
            time
        FROM items
        WHERE id = $1
    "#})
    .bind(id as i64)
    .fetch_optional(&state.pool)
    .await?;

    let Some(item) = item else {
        return Ok((StatusCode::NOT_FOUND, "Item not found").into_response());
    };

    Ok(Html(TemplateParams { ctx, item: &item }.render()?).into_response())
}
