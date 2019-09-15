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
use actix_web::client::Client;
use std::collections::HashMap;

// Public route methods

/// Route handling for pages, with arguments or not.
pub fn page_get(
    req: HttpRequest,
    forwarder: web::Data<Forwarder>,
    client: web::Data<Client>,
) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let uri = req.uri();
    let host = uri.host();
    let mut path = uri.path().to_string();

    info!("GET page {}", uri);

    if is_normal(&path) {
        let page_req = PageRequest::parse(host, &path);
        let future = forwarder.get_page(&*client, &page_req);
        Box::new(future)
    } else {
        let result = redirect_normal(&mut path, uri.query());
        Box::new(future::ok(result))
    }
}

/// Route for root, which is the same as whatever the `main` page is.
pub fn page_main(
    req: HttpRequest,
    forwarder: web::Data<Forwarder>,
    client: web::Data<Client>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    info!("GET /");

    let page_req = PageRequest {
        host: req.uri().host(),
        slug: "main",
        categories: Vec::new(),
        arguments: HashMap::new(),
    };

    forwarder.get_page(&*client, &page_req)
}

// Helper functions

/// Normalizes the path and redirects the user to that URL.
fn redirect_normal(path: &mut String, query: Option<&str>) -> HttpResponse {
    normalize(path);

    // Remove empty directories
    while let Some(idx) = path.find("//") {
        path.replace_range(idx..idx + 1, "");
    }

    // Add query at the end if relevant
    if let Some(query) = query {
        path.push('?');
        path.push_str(query);
    }

    info!("REDIRECT {}", path);

    HttpResponse::Found()
        .header(http::header::LOCATION, path.as_str())
        .finish()
}
