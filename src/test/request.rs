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

use crate::request::{PageArgumentValue as Value, PageRequest};

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
            arguments: hashmap! { "edit" => Value::from(1) },
        }
    );
    check!(
        "scp-1000/edit/true",
        PageRequest {
            host: None,
            slug: "scp-1000",
            categories: vec![],
            arguments: hashmap! { "edit" => Value::from(true) },
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
            arguments: hashmap! { "discuss" => Value::Empty },
        }
    );
    check!(
        "fragment:scp-4447-1/discuss/true",
        PageRequest {
            host: None,
            slug: "scp-4447-1",
            categories: vec!["fragment"],
            arguments: hashmap! { "discuss" => Value::from(true) },
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
            arguments: hashmap! { "norender" => Value::Empty },
        }
    );
    check!(
        "scp-series-5/norender/1",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Value::from(1) },
        }
    );
    check!(
        "scp-series-5/norender/true",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Value::from(true) },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Value::from(true), "noredirect" => Value::Empty },
        }
    );
    check!(
        "scp-series-5/norender/1/noredirect",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Value::from(1), "noredirect" => Value::Empty },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Value::from(true), "noredirect" => Value::Empty },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect/true",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Value::from(true), "noredirect" => Value::from(true) },
        }
    );
    check!(
        "scp-series-5/norender/1/noredirect/1",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Value::from(1), "noredirect" => Value::from(1) },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect/true",
        PageRequest {
            host: None,
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Value::from(true), "noredirect" => Value::from(true) },
        }
    );
    check!(
        "aaa:page/true/false/true/false/edit",
        PageRequest {
            host: None,
            slug: "page",
            categories: vec!["aaa"],
            arguments: hashmap! { "edit" => Value::Empty },
        }
    );
    check!(
        "aaa:page/true/false/true/false/edit/1",
        PageRequest {
            host: None,
            slug: "page",
            categories: vec!["aaa"],
            arguments: hashmap! { "edit" => Value::from(1) },
        }
    );
    check!(
        "aaa:page/true/false/true/false/edit/true",
        PageRequest {
            host: None,
            slug: "page",
            categories: vec!["aaa"],
            arguments: hashmap! { "edit" => Value::from(true) },
        }
    );
    check!(
        "aaa:bbb:page/noredirect/false/norender/0/true/false",
        PageRequest {
            host: None,
            slug: "page",
            categories: vec!["aaa", "bbb"],
            arguments: hashmap! { "noredirect" => Value::from(false), "norender" => Value::from(0) },
        }
    );
    check!(
        "aaa:bbb:page/tags/tale/title/A Story/edit",
        PageRequest {
            host: None,
            slug: "page",
            categories: vec!["aaa", "bbb"],
            arguments: hashmap! {
                "tags" => Value::from("tale"),
                "title" => Value::from("A Story"),
                "edit" => Value::Empty,
            },
        }
    );
}
