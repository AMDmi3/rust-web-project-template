// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::Arc;

use askama::Template;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};

use ruwete_common::db::items;
use ruwete_common::models::items::Item;

use crate::result::HandlerResult;
use crate::routes::MyRoute;
use crate::state::AppState;

#[derive(Template)]
#[template(path = "item.html")]
struct TemplateParams<'a> {
    my_route: &'a MyRoute,
    item: &'a Item,
}

#[cfg_attr(not(coverage), tracing::instrument(skip_all))]
pub async fn item(
    my_route: MyRoute,
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> HandlerResult {
    let Some(item) = items::get_by_id(&state.pool, id.try_into()?).await? else {
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
