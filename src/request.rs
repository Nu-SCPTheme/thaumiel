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

use self::PageArgumentValue as Value;
use crate::StdResult;
use serde::{Serialize, Serializer};
use std::collections::HashMap;

// Request struct

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PageRequest<'a> {
    pub host: Option<&'a str>,
    pub slug: &'a str,
    pub categories: Vec<&'a str>,
    pub arguments: HashMap<&'a str, PageArgumentValue<'a>>,
}

impl<'a> PageRequest<'a> {
    pub fn parse(host: Option<&'a str>, mut path: &'a str) -> Self {
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

                let value = Value::from(parts.next());
                arguments.insert(key, value);
            }

            arguments
        };

        PageRequest {
            host,
            slug,
            categories,
            arguments,
        }
    }
}

// Request argument value

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageArgumentValue<'a> {
    String(&'a str),
    Boolean(bool),
    Integer(u32),
    Empty,
}

impl<'a> Serialize for PageArgumentValue<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> StdResult<S::Ok, S::Error> {
        use self::PageArgumentValue::*;

        match self {
            String(value) => serializer.serialize_str(value),
            Boolean(value) => serializer.serialize_bool(*value),
            Integer(value) => serializer.serialize_u32(*value),
            Empty => serializer.serialize_unit(),
        }
    }
}

impl<'a> From<Option<&'a str>> for PageArgumentValue<'a> {
    #[inline]
    fn from(value: Option<&'a str>) -> Self {
        match value {
            Some(value) => Value::from(value),
            None => Value::Empty,
        }
    }
}

impl<'a> From<&'a str> for PageArgumentValue<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "" => Value::Empty,
            "true" => Value::Boolean(true),
            "false" => Value::Boolean(false),
            _ => match value.parse::<u32>() {
                Ok(int) => Value::Integer(int),
                Err(_) => Value::String(value),
            },
        }
    }
}

impl<'a> From<bool> for PageArgumentValue<'a> {
    #[inline]
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl<'a> From<u32> for PageArgumentValue<'a> {
    #[inline]
    fn from(value: u32) -> Self {
        Value::Integer(value)
    }
}

impl<'a> From<()> for PageArgumentValue<'a> {
    #[inline]
    fn from(_: ()) -> Self {
        Value::Empty
    }
}
