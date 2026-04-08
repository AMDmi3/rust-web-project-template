// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::Arc;

use axum_enumroutes::routes;

use crate::state::AppState;
use crate::views;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    #[default]
    Undefined,
    Items,
    Docs,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct EndpointProps {
    pub section: Section,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[routes(state_type = Arc<AppState>, props_type = EndpointProps)]
pub enum Endpoint {
    #[get("/static/{file_name}", handler = views::static_file)]
    StaticFile,
    #[get("/", handler = views::index, props = EndpointProps { section: Section::Items })]
    Index,
    #[get("/item/{id}", handler = views::item, props = EndpointProps { section: Section::Items })]
    Item,
    #[get("/about", handler = views::about, props = EndpointProps { section: Section::Docs })]
    About,
}
