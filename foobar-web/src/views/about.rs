// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::endpoints::SelfEndpoint;
use crate::result::EndpointResult;

#[derive(Template)]
#[template(path = "about.html")]
struct TemplateParams<'a> {
    endpoint: &'a SelfEndpoint,
}

#[cfg_attr(not(coverage), tracing::instrument(skip_all))]
pub async fn about(endpoint: SelfEndpoint) -> EndpointResult {
    Ok(Html(
        TemplateParams {
            endpoint: &endpoint,
        }
        .render()?,
    )
    .into_response())
}
