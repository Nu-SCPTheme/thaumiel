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

#[derive(Serialize, Debug, Clone)]
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
                if key == "true" || key == "false" {
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

// TODO: add #[test] cases
