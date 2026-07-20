// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use indoc::indoc;
use sqlx::Postgres;

use crate::models::items::Item;

pub async fn get_count(conn: impl sqlx::Acquire<'_, Database = Postgres>) -> sqlx::Result<i64> {
    let mut tx = conn.begin().await?;

    let count = sqlx::query_scalar("SELECT count(*) FROM items")
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(count)
}

pub async fn remove_oldest(conn: impl sqlx::Acquire<'_, Database = Postgres>) -> sqlx::Result<()> {
    let mut tx = conn.begin().await?;

    sqlx::query("DELETE FROM items WHERE id = (SELECT min(id) FROM items)")
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn insert_with_text(
    conn: impl sqlx::Acquire<'_, Database = Postgres>,
    text: &str,
) -> sqlx::Result<()> {
    let mut tx = conn.begin().await?;

    sqlx::query("INSERT INTO items(text) VALUES ($1)")
        .bind(text)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn get_all(conn: impl sqlx::Acquire<'_, Database = Postgres>) -> sqlx::Result<Vec<Item>> {
    let mut tx = conn.begin().await?;

    let items = sqlx::query_as(indoc! {r#"
        SELECT
            id,
            text,
            time
        FROM items
        ORDER BY time, id
    "#})
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(items)
}

pub async fn get_by_id(
    conn: impl sqlx::Acquire<'_, Database = Postgres>,
    id: i32,
) -> sqlx::Result<Option<Item>> {
    let mut tx = conn.begin().await?;

    let item = sqlx::query_as(indoc! {r#"
        SELECT
            id,
            text,
            time
        FROM items
        WHERE id = $1
    "#})
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(item)
}
