// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(FromRow)]
pub struct Item {
    pub id: i32,
    pub text: String,
    pub time: OffsetDateTime,
}
