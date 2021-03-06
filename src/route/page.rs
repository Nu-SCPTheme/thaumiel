/*
 * route/page.rs
 *
 * thaumiel - Wikidot-like web server to provide pages, forums, and other services
 * Copyright (C) 2019-2020 Ammon Smith
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
use std::collections::HashMap;
use wikidot_path::Request as PageRequest;

// Public route methods

/// Route handling for pages, with arguments or not.
pub async fn page_get(req: HttpRequest) -> HttpResult {
    let host = get_host(&req);
    let path = req.uri().path();

    info!("GET page {} [{}]", path, host.unwrap_or("none"));

    let _page_req = PageRequest::parse(path);

    // TODO retrieve page from client
    Ok(HttpResponse::NotImplemented().finish())
}

/// Route for root, which is the same as whatever the `main` page is.
pub async fn page_main(req: HttpRequest) -> HttpResult {
    let host = get_host(&req);

    info!("GET / [{}]", host.unwrap_or("none"));

    let _page_req = PageRequest {
        slug: "",
        categories: Vec::new(),
        arguments: HashMap::new(),
    };

    // TODO get page request
    Ok(HttpResponse::NotImplemented().finish())
}
