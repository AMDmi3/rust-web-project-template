// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use strum::EnumProperty;
use strum_macros::{EnumString, IntoStaticStr};

#[derive(EnumProperty, IntoStaticStr, EnumString, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endpoint {
    // Static
    #[strum(props(path = "/static/{file_name}"))]
    StaticFile,

    #[strum(props(path = "/"))]
    Index,

    #[strum(props(path = "/about"))]
    About,
}

#[derive(EnumString, Clone, Copy, PartialEq, Eq)]
pub enum Section {}

impl Endpoint {
    pub fn path(&self) -> &'static str {
        self.get_str("path")
            .expect("path should exist for the endpoint")
    }

    pub fn name(&self) -> &'static str {
        self.into()
    }

    pub fn is_section(&self, section: Section) -> bool {
        use std::str::FromStr as _;
        self.get_str("section")
            .is_some_and(|endpoint_section| Section::from_str(endpoint_section).unwrap() == section)
    }
}
