// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[routes(state_type = Arc<AppState>, props_type = RouteProps)]
pub enum Route {
    #[get("/static/{file_name}", handler = handlers::static_file)]
    StaticFile,
    #[get("/", handler = handlers::index, props = RouteProps { section: Section::Items, ..Default::default() })]
    Index,
    #[get("/item/{id}", handler = handlers::item, props = RouteProps { section: Section::Items, ..Default::default() })]
    Item,
    #[get("/about", handler = handlers::about, props = RouteProps { section: Section::Docs, ..Default::default() })]
    About,
}
