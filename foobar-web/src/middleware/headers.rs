// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use axum::extract::Request;
use axum::http::header::HeaderValue;
use axum::middleware::Next;
use axum::response::IntoResponse;

use crate::routes::SelfRoute;

pub async fn headers_middleware(
    route: SelfRoute,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let mut response = next.run(request).await;

    response.headers_mut().insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff"),
    );
    if !route.props().allow_embedding {
        response.headers_mut().insert("Content-Security-Policy", HeaderValue::from_static("default-src 'none'; style-src 'self'; script-src 'self'; img-src 'self'; font-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'self'"));
        response
            .headers_mut()
            .insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    } else {
        // relaxed headers to allow some embedding cases, see https://github.com/repology/repology-webapp/issues/175
        response.headers_mut().insert("Content-Security-Policy", HeaderValue::from_static("default-src 'none'; style-src 'self'; script-src 'self'; img-src 'self'; font-src 'self'; frame-ancestors *; base-uri 'none'; form-action 'self'"));
    }
    // NOTE: Strict-Transport-Security must be set where HTTPS is terminated, e.g. nginx

    // XXX: Uncomment if desired, e.g. the site does not contain private
    // info and referrer announcing does make sence (e.g. Repology)
    //response.headers_mut().insert("Referrer-Policy", HeaderValue::from_static("unsafe-url"));

    response
}
