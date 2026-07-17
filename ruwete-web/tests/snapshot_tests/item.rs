// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use axum_test::TestServer;
use sqlx::PgPool;

use ruwete_web::create_app;

#[sqlx::test(migrator = "ruwete_common::MIGRATOR", fixtures("sample_items"))]
async fn test_item(pool: PgPool) {
    let server = TestServer::new(create_app(pool).await.unwrap());
    assert_snapshot!(server.get("/item/1").await);
}

#[sqlx::test(migrator = "ruwete_common::MIGRATOR", fixtures("sample_items"))]
async fn test_item_not_found(pool: PgPool) {
    let server = TestServer::new(create_app(pool).await.unwrap());
    assert_snapshot!(server.get("/item/999").await);
}
