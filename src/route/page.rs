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

// Public route methods

/// Route handlers with arguments, like `/scp-1000/offset/2`
pub fn page_args(req: HttpRequest) -> impl Responder {
    debug!("page_args: req {:#?}", req);

    let uri = req.uri();
    let mut path = uri.path().to_string();

    debug!("path: {}", path);
    if is_normal(&path) {
        let page_req = PageRequest::parse(&path);
        send_page(&page_req)
    } else {
        redirect_normal(&mut path, uri.query())
    }
}

/// Route handler for those with only a slug, like `/scp-1000`
pub fn page_get(path: web::Path<String>) -> impl Responder {
    info!("GET page {}", path);

    let mut path = path.into_inner();

    debug!("path: {}", path);
    if is_normal(&path) {
        let page_req = PageRequest::parse(&path);
        send_page(&page_req)
    } else {
        redirect_normal(&mut path, None)
    }
}

/// Route for root, which is the same as whatever the `main` page is.
pub fn page_main() -> impl Responder {
    info!("GET /");

    send_page(&*MAIN_PAGE)
}

// Helper functions

/// Takes a page request and sends the appropriate HttpResponse for it.
fn send_page(page_req: &PageRequest) -> HttpResponse {
    debug!("page_req: {:#?}", page_req);

    // TODO

    HttpResponse::Ok().body(format!("page${:?}", page_req))
}

/// Normalizes the path and redirects the user to that URL.
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
