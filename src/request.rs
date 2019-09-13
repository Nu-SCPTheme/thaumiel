/*
 * request.rs
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

use std::collections::HashMap;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PageRequest<'a> {
    pub slug: &'a str,
    pub categories: Vec<&'a str>,
    pub arguments: HashMap<&'a str, Option<u32>>,
}

impl<'a> PageRequest<'a> {
    pub fn parse(mut path: &'a str) -> Self {
        // Remove leading slash to avoid empty slugs
        if path.starts_with("/") {
            path = &path[1..];
        }

        // Create part iterator and get slug
        let mut parts = path.split('/');
        let slug = parts.next().expect("Path split has no items");

        // Get all page categories
        let (slug, categories) = {
            let mut categories: Vec<_> = slug.split(':').collect();
            let slug = categories.pop().expect("Category split has no items");
            (slug, categories)
        };

        // Parse out Wikidot arguments
        //
        // This algorithm is compatible with the /KEY/true format,
        // but also allowing the more sensible /KEY for options
        // where a 'false' value doesn't make sense, like 'norender' or 'edit'.
        let arguments = {
            let mut arguments = HashMap::new();

            while let Some(key) = parts.next() {
                if key.is_empty() || key == "true" || key == "false" {
                    continue;
                }

                let value = match parts.next() {
                    Some(value) => parse_value(value),
                    None => None,
                };
                arguments.insert(key, value);
            }

            arguments
        };

        PageRequest {
            slug,
            categories,
            arguments,
        }
    }
}

fn parse_value(value: &str) -> Option<u32> {
    match value {
        "" => None,
        "true" => Some(1),
        "false" => Some(0),
        _ => value.parse::<u32>().ok(),
    }
}

#[test]
fn test_page_request() {
    macro_rules! check {
        ($path:expr, $expected:expr) => {{
            let page_req = PageRequest::parse($path);
            assert_eq!(
                page_req, $expected,
                "Parsed PageRequest doesn't match expected"
            );
        }};
    }

    check!(
        "scp-1000",
        PageRequest {
            slug: "scp-1000",
            categories: vec![],
            arguments: hashmap! {},
        }
    );
    check!(
        "scp-1000/edit",
        PageRequest {
            slug: "scp-1000",
            categories: vec![],
            arguments: hashmap! { "edit" => None },
        }
    );
    check!(
        "scp-1000/edit/1",
        PageRequest {
            slug: "scp-1000",
            categories: vec![],
            arguments: hashmap! { "edit" => Some(1) },
        }
    );
    check!(
        "scp-1000/edit/true",
        PageRequest {
            slug: "scp-1000",
            categories: vec![],
            arguments: hashmap! { "edit" => Some(1) },
        }
    );
    check!(
        "component:image-block",
        PageRequest {
            slug: "image-block",
            categories: vec!["component"],
            arguments: hashmap! {},
        }
    );
    check!(
        "deleted:component:image-block",
        PageRequest {
            slug: "image-block",
            categories: vec!["deleted", "component"],
            arguments: hashmap! {},
        }
    );
    check!(
        "fragment:scp-4447-1",
        PageRequest {
            slug: "scp-4447-1",
            categories: vec!["fragment"],
            arguments: hashmap! {},
        }
    );
    check!(
        "fragment:scp-4447-1/discuss",
        PageRequest {
            slug: "scp-4447-1",
            categories: vec!["fragment"],
            arguments: hashmap! { "discuss" => None },
        }
    );
    check!(
        "fragment:scp-4447-1/discuss/true",
        PageRequest {
            slug: "scp-4447-1",
            categories: vec!["fragment"],
            arguments: hashmap! { "discuss" => Some(1) },
        }
    );
    check!(
        "scp-series-5",
        PageRequest {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! {},
        }
    );
    check!(
        "scp-series-5/norender",
        PageRequest {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => None },
        }
    );
    check!(
        "scp-series-5/norender/1",
        PageRequest {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1) },
        }
    );
    check!(
        "scp-series-5/norender/true",
        PageRequest {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1) },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect",
        PageRequest {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1), "noredirect" => None },
        }
    );
    check!(
        "scp-series-5/norender/1/noredirect",
        PageRequest {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1), "noredirect" => None },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect",
        PageRequest {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1), "noredirect" => None },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect/true",
        PageRequest {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1), "noredirect" => Some(1) },
        }
    );
    check!(
        "scp-series-5/norender/1/noredirect/1",
        PageRequest {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1), "noredirect" => Some(1) },
        }
    );
    check!(
        "scp-series-5/norender/true/noredirect/true",
        PageRequest {
            slug: "scp-series-5",
            categories: vec![],
            arguments: hashmap! { "norender" => Some(1), "noredirect" => Some(1) },
        }
    );
    check!(
        "aaa:page/true/false/true/false/edit",
        PageRequest {
            slug: "page",
            categories: vec!["aaa"],
            arguments: hashmap! { "edit" => None },
        }
    );
    check!(
        "aaa:page/true/false/true/false/edit/1",
        PageRequest {
            slug: "page",
            categories: vec!["aaa"],
            arguments: hashmap! { "edit" => Some(1) },
        }
    );
    check!(
        "aaa:page/true/false/true/false/edit/true",
        PageRequest {
            slug: "page",
            categories: vec!["aaa"],
            arguments: hashmap! { "edit" => Some(1) },
        }
    );
    check!(
        "aaa:bbb:page/noredirect/false/norender/0/true/false",
        PageRequest {
            slug: "page",
            categories: vec!["aaa", "bbb"],
            arguments: hashmap! { "noredirect" => Some(0), "norender" => Some(0) },
        }
    );
}
