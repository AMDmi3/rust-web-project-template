// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use axum_test::TestServer;
use sqlx::PgPool;

use ruwete_web::create_app;

#[sqlx::test(migrator = "ruwete_common::MIGRATOR")]
async fn test_nonexistent(pool: PgPool) {
    let server = TestServer::new(create_app(pool).await.unwrap());
    let response = server.get("/static/nonexistent").await;
    response.assert_status_not_found();
}

#[sqlx::test(migrator = "ruwete_common::MIGRATOR")]
async fn test_css(pool: PgPool) {
    let server = TestServer::new(create_app(pool).await.unwrap());
    let response = server.get("/static/amdmi3.min.css").await;
    response.assert_status_ok();
    response.assert_header("content-type", "text/css");
    response.assert_text_contains("light-dark");
    assert!(response.text().len() > 1000);
}
