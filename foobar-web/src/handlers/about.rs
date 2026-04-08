// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::result::EndpointResult;
use crate::routes::SelfRoute;

#[derive(Template)]
#[template(path = "about.html")]
struct TemplateParams<'a> {
    route: &'a SelfRoute,
}

#[cfg_attr(not(coverage), tracing::instrument(skip_all))]
pub async fn about(route: SelfRoute) -> EndpointResult {
    Ok(Html(TemplateParams { route: &route }.render()?).into_response())
}
