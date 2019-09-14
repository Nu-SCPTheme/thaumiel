/*
 * test/request.rs
 *
 * kant-router - Wikidot-compatible router for web applications
 * Copyright (C) 2019 Ammon Smith
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use crate::request::{PageRequest, PageArgumentValue as Value};

#[test]
fn test_page_request() {
    macro_rules! check {
        ($path:expr, $expected:expr) => {{
            let page_req = PageRequest::parse(None, $path);
            assert_eq!(
                page_req, $expected,
                "Parsed PageRequest doesn't match expected"
            );
        }};
    }

    check!(
        "scp-1000",
        PageRequest {
            host: None,
            slug: "scp-1000",
            categories: vec![],
            arguments: hashmap! {},
        }
    );
    check!(
        "scp-1000/edit",
        PageRequest {
            host: None,
            slug: "scp-1000",
            categories: vec![],
            arguments: hashmap! { "edit" => Value::Empty },
        }
    );
    check!(
        "scp-1000/edit/1",
        PageRequest {
            host: None,
            slug: "scp-1000",
            categories: vec![],
            arguments: hashmap! { "edit" => Some(1) },
        }
    );
    check!(
        "scp-1000/edit/true",
        PageRequest {
            host: None,
            slug: "scp-1000",
            categories: vec![],
            arguments: hashmap! { "edit" => Some(1) },
        }
    );
    check!(
        "component:image-block",
        PageRequest {
            host: None,
            slug: "image-block",
            categories: vec!["component"],
            arguments: hashmap! {},
        }
    );
    check!(
        "deleted:component:image-block",
        PageRequest {
            host: None,
            slug: "image-block",
            categories: vec!["deleted", "component"],
            arguments: hashmap! {},
        }
    );
    check!(
        "fragment:scp-4447-1",
        PageRequest {
            host: None,
            slug: "scp-4447-1",
            categories: vec!["fragment"],
            arguments: hashmap! {},
        }
    );
    check!(
        "fragment:scp-4447-1/discuss",
        PageRequest {
            host: None,
            slug: "scp-4447-1",
            categories: vec!["fragment"],
            arguments: hashmap! { "discuss" => None },
        }
    );
    check!(
        "fragment:scp-4447-1/discuss/true",
        PageRequest {
            host: None,
            slug: "scp-4447-1",
            categories: vec!["fragment"],
            arguments: hashmap! { "discuss" => Some(1) },
        }
    );
    check!(
        "scp-series-5",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! {},
        }
    );
    check!(
        "scp-series-5/norender",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => None },
        }
    );
    check!(
        "scp-series-5/norender/1",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1) },
        }
    );
    check!(
        "scp-series-5/norender/true",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1) },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1), "noredirect" => None },
        }
    );
    check!(
        "scp-series-5/norender/1/noredirect",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1), "noredirect" => None },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1), "noredirect" => None },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect/true",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1), "noredirect" => Some(1) },
        }
    );
    check!(
        "scp-series-5/norender/1/noredirect/1",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1), "noredirect" => Some(1) },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect/true",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1), "noredirect" => Some(1) },
        }
    );
    check!(
        "aaa:page/true/false/true/false/edit",
        PageRequest {
            host: None,
            slug: "page",
            categories: vec!["aaa"],
            arguments: hashmap! { "edit" => None },
        }
    );
    check!(
        "aaa:page/true/false/true/false/edit/1",
        PageRequest {
            host: None,
            slug: "page",
            categories: vec!["aaa"],
            arguments: hashmap! { "edit" => Some(1) },
        }
    );
    check!(
        "aaa:page/true/false/true/false/edit/true",
        PageRequest {
            host: None,
            slug: "page",
            categories: vec!["aaa"],
            arguments: hashmap! { "edit" => Some(1) },
        }
    );
    check!(
        "aaa:bbb:page/noredirect/false/norender/0/true/false",
        PageRequest {
            host: None,
            slug: "page",
            categories: vec!["aaa", "bbb"],
            arguments: hashmap! { "noredirect" => Some(0), "norender" => Some(0) },
        }
    );
}
