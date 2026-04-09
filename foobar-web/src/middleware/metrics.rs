// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::Instant;

use axum::body::HttpBody;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use metrics::{counter, histogram};

use crate::routes::MyRoute;

pub async fn metrics_middleware(
    route: Option<MyRoute>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let start = Instant::now();
    let response = next.run(request).await;
    let latency = start.elapsed().as_secs_f64();

    let route_name = route.map(|route| route.path()).unwrap_or("???");
    let status = response.status().as_u16().to_string();

    counter!("foobar_web_http_requests_total", "route" => route_name, "status" => status)
        .increment(1);
    histogram!("foobar_web_http_requests_duration_seconds", "route" => route_name).record(latency);

    if let Some(body_size) = response.body().size_hint().exact() {
        histogram!("foobar_web_http_response_size_bytes", "route" => route_name)
            .record(body_size as f64);
    }

    response
}
