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

use crate::result::HandlerResult;
use crate::routes::MyRoute;
use crate::state::AppState;

#[derive(FromRow)]
struct Item {
    id: i32,
    text: String,
    time: DateTime<Utc>,
}

#[derive(Template)]
#[template(path = "item.html")]
struct TemplateParams<'a> {
    my_route: &'a MyRoute,
    item: &'a Item,
}

#[cfg_attr(not(coverage), tracing::instrument(skip_all))]
pub async fn item(
    my_route: MyRoute,
    Path(id): Path<u64>,
    State(state): State<Arc<AppState>>,
) -> HandlerResult {
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

    Ok(Html(
        TemplateParams {
            my_route: &my_route,
            item: &item,
        }
        .render()?,
    )
    .into_response())
}
