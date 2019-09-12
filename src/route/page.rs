/*
 * route/page.rs
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

use super::prelude::*;
use crate::normalize::{is_normal, normalize};
use crate::request::PageRequest;
use std::collections::HashMap;

lazy_static! {
    static ref MAIN_PAGE: PageRequest<'static> = PageRequest {
        slug: "main",
        categories: Vec::new(),
        arguments: HashMap::new(),
    };
}

pub fn page_args(req: HttpRequest) -> impl Responder {
    debug!("page_args: req {:#?}", req);

    let uri = req.uri();
    let mut path = uri.path().to_string();

    debug!("path: {}", path);
    if is_normal(&path) {
        send_page(&parse_path(&path))
    } else {
        redirect_normal(&mut path, uri.query())
    }
}

pub fn page_get(path: web::Path<String>) -> impl Responder {
    info!("GET page {}", path);

    let mut path = path.into_inner();

    debug!("path: {}", path);
    if is_normal(&path) {
        send_page(&parse_path(&path))
    } else {
        redirect_normal(&mut path, None)
    }
}

pub fn page_main() -> impl Responder {
    info!("GET /");

    send_page(&*MAIN_PAGE)
}

fn send_page(page_req: &PageRequest) -> HttpResponse {
    debug!("page_req: {:#?}", page_req);

    // TODO

    HttpResponse::Ok().body(format!("page${:?}", page_req))
}

fn redirect_normal(path: &mut String, query: Option<&str>) -> HttpResponse {
    normalize(path);

    if let Some(query) = query {
        path.push('?');
        path.push_str(query);
    }

    HttpResponse::Found()
        .header(http::header::LOCATION, path.as_str())
        .finish()
}

fn parse_path(mut path: &str) -> PageRequest {
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

fn parse_value(value: &str) -> Option<u32> {
    match value {
        "" => None,
        "true" => Some(1),
        "false" => Some(0),
        _ => value.parse::<u32>().ok(),
    }
}

// TODO: add #[test] cases
