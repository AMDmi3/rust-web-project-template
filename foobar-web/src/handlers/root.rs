// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::Arc;

use askama::Template;
use axum::extract::State;
use axum::response::{Html, IntoResponse};

use foobar_common::db::items;
use foobar_common::models::items::Item;

use crate::result::HandlerResult;
use crate::routes::MyRoute;
use crate::state::AppState;

#[derive(Template)]
#[template(path = "root.html")]
struct TemplateParams<'a> {
    my_route: &'a MyRoute,
    items: &'a [Item],
}

#[cfg_attr(not(coverage), tracing::instrument(skip_all))]
pub async fn root(my_route: MyRoute, State(state): State<Arc<AppState>>) -> HandlerResult {
    let items = items::get_all(&state.pool).await?;

    Ok(Html(
        TemplateParams {
            my_route: &my_route,
            items: &items,
        }
        .render()?,
    )
    .into_response())
}
