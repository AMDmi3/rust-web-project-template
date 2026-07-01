// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use indoc::indoc;
use sqlx::PgPool;

use crate::models::items::Item;

pub async fn get_count(pool: &PgPool) -> sqlx::Result<i64> {
    let count = sqlx::query_scalar("SELECT count(*) FROM items")
        .fetch_one(pool)
        .await?;

    Ok(count)
}

pub async fn remove_oldest(pool: &PgPool) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM items WHERE id = (SELECT min(id) FROM items)")
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn insert_with_text(pool: &PgPool, text: &str) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO items(text) VALUES ($1)")
        .bind(text)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_all(pool: &PgPool) -> sqlx::Result<Vec<Item>> {
    let items = sqlx::query_as(indoc! {r#"
        SELECT
            id,
            text,
            time
        FROM items
        ORDER BY time, id
    "#})
    .fetch_all(pool)
    .await?;

    Ok(items)
}

pub async fn get_by_id(pool: &PgPool, id: i32) -> sqlx::Result<Option<Item>> {
    let item = sqlx::query_as(indoc! {r#"
        SELECT
            id,
            text,
            time
        FROM items
        WHERE id = $1
    "#})
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(item)
}
