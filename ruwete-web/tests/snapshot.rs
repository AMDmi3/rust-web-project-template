// SPDX-FileCopyrightText: Copyright 2026 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

macro_rules! assert_snapshot {
    ($what:expr) => {
        let response = $what;
        insta::with_settings!({
            filters => vec![
                (r#"("/static/.*\.)[0-9a-f]{16}(\..*")"#, "$1[     hash     ]$2"),
            ],
            description => format!("{:?} {}", response.request_method(), response.request_url()),
            omit_expression => true,
        }, {
            insta::assert_snapshot!(response);
        });
    }
}

mod snapshot_tests {
    automod::dir!("tests/snapshot_tests");
}
