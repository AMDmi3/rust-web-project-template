// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::{Result, anyhow};

use crate::endpoints::{Endpoint, Section};
use crate::static_files::STATIC_FILES;
use crate::url_for::UrlConstructor;

pub struct TemplateContext {
    pub endpoint: Endpoint,
    params: Vec<(String, String)>,
}

impl TemplateContext {
    pub fn new(endpoint: Endpoint) -> Self {
        Self {
            endpoint,
            params: vec![],
        }
    }

    pub fn with_params(mut self, params: &[(String, String)]) -> Self {
        params.iter().cloned().collect_into(&mut self.params);
        self
    }

    pub fn url_for_static(&self, file_name: &str) -> Result<String> {
        let file = STATIC_FILES
            .by_orig_name(file_name)
            .ok_or_else(|| anyhow!("unknown static file \"{}\"", file_name))?;

        UrlConstructor::new(Endpoint::StaticFile.path())
            .with_field("file_name", &file.hashed_name)
            .construct()
    }

    pub fn url_for_unversioned_static(&self, file_name: &str) -> Result<String> {
        UrlConstructor::new(Endpoint::StaticFile.path())
            .with_field("file_name", file_name)
            .construct()
    }

    pub fn url_for<'a>(&self, endpoint: Endpoint, fields: &[(&'a str, &'a str)]) -> Result<String> {
        UrlConstructor::new(endpoint.path())
            .with_fields(fields.iter().cloned())
            .construct()
    }

    pub fn url_for_self<'a>(&self, fields: &[(&'a str, &'a str)]) -> Result<String> {
        UrlConstructor::new(self.endpoint.path())
            .with_fields(self.params.iter().map(|(k, v)| (k.as_ref(), v.as_ref())))
            .with_fields(fields.iter().cloned())
            .construct()
    }

    pub fn is_section(&self, section: Section) -> bool {
        self.endpoint.is_section(section)
    }

    pub fn is_endpoint(&self, endpoint: Endpoint) -> bool {
        self.endpoint == endpoint
    }
}
