// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use axum_test::TestServer;
use sqlx::PgPool;

use foobar_web::create_app;

#[sqlx::test(migrator = "foobar_common::MIGRATOR")]
async fn test_index(pool: PgPool) {
    let server = TestServer::new(create_app(pool).await.unwrap());
    insta::assert_snapshot!(server.get("/").await);
}
