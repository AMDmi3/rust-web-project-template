// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use axum_test::TestServer;
use sqlx::PgPool;

use foobar_web::create_app;

#[sqlx::test(migrator = "foobar_common::MIGRATOR", fixtures("sample_items"))]
async fn test_index(pool: PgPool) {
    let server = TestServer::new(create_app(pool).await.unwrap());
    let response = server.get("/").await;
    response.assert_status_ok();
    response.assert_header("content-type", "text/html");
    response.assert_text_contains("Sample item foo");
    response.assert_text_contains("Sample item bar");
    assert!(
        !tidier::Doc::new(response.text(), false)
            .unwrap()
            .has_issues()
    );
}
