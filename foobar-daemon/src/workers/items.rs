// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::Duration;

use indoc::indoc;
use sqlx::PgPool;
use tracing::error;

const RETRY_INTERVAL: Duration = Duration::from_mins(1);
const ITERATION_INTERVAL: Duration = Duration::from_mins(1);

pub struct ItemsWorker {
    pool: PgPool,
}

impl ItemsWorker {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn iteration(&self) -> anyhow::Result<()> {
        let (num_items, random): (i64, f64) = sqlx::query_as(indoc! {"
                        SELECT
                            count(*), random()
                        FROM items
                    "})
        .fetch_one(&self.pool)
        .await?;

        if num_items < 10 || (num_items < 20 && random < 0.5) {
            let text = format!("{:x}", {
                use std::hash::{Hash, Hasher};
                let mut hasher = std::hash::DefaultHasher::new();
                random.to_bits().hash(&mut hasher);
                hasher.finish()
            });
            sqlx::query(indoc! {"
                            INSERT INTO items(text)
                            VALUES($1)
                        "})
            .bind(&text)
            .execute(&self.pool)
            .await?;
        } else {
            sqlx::query(indoc! {"
                            DELETE FROM items
                            WHERE
                                id = (SELECT min(id) FROM items)
                        "})
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn run(&self) {
        loop {
            match self.iteration().await {
                Err(error) => {
                    error!(%error, "failure in worker iteration");
                    tokio::time::sleep(RETRY_INTERVAL).await;
                }
                Ok(()) => {
                    tokio::time::sleep(ITERATION_INTERVAL).await;
                }
            }
        }
    }
}
