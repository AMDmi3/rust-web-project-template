// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(FromRow)]
pub struct Item {
    pub id: i32,
    pub text: String,
    pub time: OffsetDateTime,
}
