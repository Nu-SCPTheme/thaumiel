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
use crate::request::PageRequest;
use actix_web::client::Client;
use std::collections::HashMap;
use wikidot_normalize::{is_normal, normalize_decode as normalize};

// Public route methods

/// Route handling for pages, with arguments or not.
pub async fn page_get(req: HttpRequest, client: web::Data<Client>) -> HttpResult {
    let host = get_host(&req);
    let uri = req.uri();
    let mut path = uri.path().to_string();

    info!("GET page {} [{}]", &path, host.unwrap_or("none"));

    if is_normal(&path, true) {
        let page_req = PageRequest::parse(host, &path);
        // TODO retrieve page from client
        Ok(HttpResponse::NotImplemented().finish())
    } else {
        Ok(redirect_normal(&mut path, uri.query()))
    }
}

/// Route for root, which is the same as whatever the `main` page is.
pub async fn page_main(req: HttpRequest, client: web::Data<Client>) -> HttpResult {
    let host = get_host(&req);

    info!("GET / [{}]", host.unwrap_or("none"));

    let page_req = PageRequest {
        host,
        slug: "main",
        categories: Vec::new(),
        arguments: HashMap::new(),
    };

    // TODO get page request
    Ok(HttpResponse::NotImplemented().finish())
}

// Helper functions

/// Gets the client hostname, from URI, then headers if present.
fn get_host(req: &HttpRequest) -> Option<&str> {
    if let Some(host) = req.uri().host() {
        return Some(host);
    }

    match req.headers().get(http::header::HOST) {
        Some(value) => value.to_str().ok(),
        None => None,
    }
}

/// Normalizes the path and redirects the user to that URL.
fn redirect_normal(path: &mut String, query: Option<&str>) -> HttpResponse {
    normalize(path);

    // Remove empty directories
    while let Some(idx) = path.find("//") {
        path.replace_range(idx..=idx, "");
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
