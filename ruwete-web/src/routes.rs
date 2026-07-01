// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::Arc;

use axum_myroutes::routes;

use crate::handlers;
use crate::state::AppState;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    #[default]
    Undefined,
    Items,
    Docs,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RouteProps {
    pub section: Section,
    // XXX: Set on routes which produce embeddable resources, such as badge images
    // Affects headers middleware
    pub allow_embedding: bool,
}

impl RouteProps {
    fn section(mut self, section: Section) -> Self {
        self.section = section;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[routes(state_type = Arc<AppState>, props_type = RouteProps)]
pub enum Route {
    #[get("/static/{file_name}", handler = handlers::static_file)]
    StaticFile,
    #[get("/", handler = handlers::root, props = RouteProps::default().section(Section::Items) )]
    Root,
    #[get("/item/{id}", handler = handlers::item, props = RouteProps::default().section(Section::Items) )]
    Item,
    #[get("/about", handler = handlers::about, props = RouteProps::default().section(Section::Docs) )]
    About,
}
