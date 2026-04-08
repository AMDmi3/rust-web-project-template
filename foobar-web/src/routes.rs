// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::Arc;

use axum_enumroutes::routes;

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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[routes(state_type = Arc<AppState>, props_type = RouteProps)]
pub enum Route {
    #[get("/static/{file_name}", handler = handlers::static_file)]
    StaticFile,
    #[get("/", handler = handlers::index, props = RouteProps { section: Section::Items })]
    Index,
    #[get("/item/{id}", handler = handlers::item, props = RouteProps { section: Section::Items })]
    Item,
    #[get("/about", handler = handlers::about, props = RouteProps { section: Section::Docs })]
    About,
}
