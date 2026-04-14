// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::result::HandlerResult;
use crate::routes::MyRoute;

#[derive(Template)]
#[template(path = "about.html")]
struct TemplateParams<'a> {
    my_route: &'a MyRoute,
}

#[cfg_attr(not(coverage), tracing::instrument(skip_all))]
pub async fn about(my_route: MyRoute) -> HandlerResult {
    Ok(Html(
        TemplateParams {
            my_route: &my_route,
        }
        .render()?,
    )
    .into_response())
}
