// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use axum::http::Request;
use http::StatusCode;
use sqlx::PgPool;
use tower_service::Service;

use foobar_web::create_app;

#[sqlx::test(migrator = "foobar_common::MIGRATOR")]
async fn test_nonexistent(pool: PgPool) {
    let mut router = create_app(pool).await.unwrap();
    let request = Request::builder()
        .uri("/static/nonexistent")
        .body("".to_owned())
        .unwrap();
    let response = router.call(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test(migrator = "foobar_common::MIGRATOR")]
async fn test_css(pool: PgPool) {
    let mut router = create_app(pool).await.unwrap();
    let request = Request::builder()
        .uri("/static/amdmi3.min.css")
        .body("".to_owned())
        .unwrap();
    let response = router.call(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap(),
        "text/css"
    );
    let body = axum::body::to_bytes(response.into_body(), 1000000)
        .await
        .unwrap();
    let body = std::str::from_utf8(&body).unwrap();
    assert!(body.contains("light-dark"));
    assert!(body.len() > 1000);
}
