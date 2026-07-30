// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::time::Duration;

use sqlx::PgPool;
use tracing::error;

use ruwete_common::db::items;

const RETRY_INTERVAL: Duration = Duration::from_mins(1);
const ITERATION_INTERVAL: Duration = Duration::from_secs(1);

pub struct ItemsWorker {
    pool: PgPool,
}

impl ItemsWorker {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn iteration(&self) -> anyhow::Result<()> {
        let num_items = items::get_count(&self.pool).await?;

        if num_items < 10 || (num_items < 20 && rand::random::<bool>()) {
            items::insert_with_text(&self.pool, &format!("{:x}", rand::random::<u64>())).await?;
        } else {
            items::remove_oldest(&self.pool).await?;
        }

        Ok(())
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        loop {
            match self.iteration().await {
                Err(error) => {
                    error!(
                        error = format_args!("{:#}", error),
                        "failure in worker iteration"
                    );
                    tokio::time::sleep(RETRY_INTERVAL).await;
                }
                Ok(()) => {
                    tokio::time::sleep(ITERATION_INTERVAL).await;
                }
            }
        }
    }
}
