// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::endpoints::Endpoint;
use crate::result::EndpointResult;
use crate::template_context::TemplateContext;

#[derive(Template)]
#[template(path = "about.html")]
struct TemplateParams {
    ctx: TemplateContext,
}

#[cfg_attr(not(coverage), tracing::instrument(skip_all))]
pub async fn about() -> EndpointResult {
    let ctx = TemplateContext::new_without_params(Endpoint::About);

    Ok(Html(TemplateParams { ctx }.render()?).into_response())
}
