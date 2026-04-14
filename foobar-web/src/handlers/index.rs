// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::Arc;

use askama::Template;
use axum::extract::State;
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
#[template(path = "index.html")]
struct TemplateParams<'a> {
    my_route: &'a MyRoute,
    items: &'a [Item],
}

#[cfg_attr(not(coverage), tracing::instrument(skip_all))]
pub async fn index(my_route: MyRoute, State(state): State<Arc<AppState>>) -> HandlerResult {
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
            my_route: &my_route,
            items: &items,
        }
        .render()?,
    )
    .into_response())
}
