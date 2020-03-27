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

use crate::{remote::DeepwellPool, route::prelude::*};
use deepwell_core::prelude::*;
use ftml::{
    data::User as FtmlUser,
    handle::{NullHandle, RemoteHandle},
    prefilter,
    prelude::*,
    RemoteError, RemoteResult,
};
use std::collections::HashMap;
use wikidot_path::Request as PageRequest;

// Public route methods

/* TODO: Temporary Measure
 *
 * Until we can figure out a good way of divining the WikiId from the wiki request,
 * just assume that these functions are retrieving the wiki at WikiId 1.
 */
lazy_static! {
    static ref TEMP_WIKI_ID: WikiId = WikiId::from_raw(1);
}

/// Return an HttpResult for getting a page by its slug, using deepwell.
pub async fn get_deepwell_page(
    wiki_id: WikiId,
    slug: &str,
    deepwell: &web::Data<DeepwellPool>,
) -> Option<HttpResponse> {
    debug!("Retrieving page by WikiId {} and slug {}", wiki_id, slug);
    let result = deepwell
        .claim()
        .await
        .get_page_contents(wiki_id.clone(), slug.to_string())
        .await;

    match try_io_option!(result) {
        Ok(Some(page)) => {
            // now run FTML on it
            let mut contents = match String::from_utf8(page.to_vec()) {
                Ok(v) => v,
                Err(e) => return Some(HttpResponse::InternalServerError().json(format!("{:?}", e))),
            };

            // TODO: something that uses Deepwell
            let remote_handle = NullHandle;
            let renderer = HtmlRender::new(&remote_handle);

            // TODO: also get page metadata
            let page_info = PageInfo {
                title: "SCP-XXXX",
                alt_title: Some("Template Page"),
                header: None,
                subheader: None,
                rating: 1000,
                tags: vec![
                    "scp",
                ],
            };

            let output = match renderer.transform(&mut contents, page_info, &remote_handle) {
                Ok(o) => o,
                Err(e) => return Some(HttpResponse::InternalServerError().json(format!("{:?}", e))),
            };

            // TODO: use jinja/other templater to put this text into a better template
            let mut buffer = String::from("<html><head>");

            for meta in &output.meta {
                meta.render(&mut buffer).unwrap(); // TODO: let's not unwrap this
            }

            buffer.push_str("<style>");
            buffer.push_str(&output.style);
            buffer.push_str("</style></head><body>");
            buffer.push_str(&output.html);
            buffer.push_str("</body></html>\n");

            Some(HttpResponse::Ok().body(buffer))
        }
        Ok(None) => None,
        Err(e) => {
            warn!("Failed to retrieve page contents: {}", e);

            Some(HttpResponse::InternalServerError().json(e))
        }
    }
}

pub async fn get_deepwell_page_wrapped(
    wiki_id: WikiId,
    slug: &str,
    deepwell: web::Data<DeepwellPool>,
) -> HttpResponse {
    match get_deepwell_page(wiki_id, slug, &deepwell).await { 
        Some(o) => o,
        None => get_deepwell_page(wiki_id, "_404", &deepwell).await.unwrap(), 
    }
}

/// Route handling for pages, with arguments or not.
pub async fn page_get(req: HttpRequest, deepwell: web::Data<DeepwellPool>) -> HttpResult {
    let host = get_host(&req);
    let path = req.uri().path();

    info!("GET page {} [{}]", path, host.unwrap_or("none"));

    let page_req = PageRequest::parse(path);

    Ok(get_deepwell_page_wrapped(*TEMP_WIKI_ID, page_req.slug, deepwell).await)
}

/// Route for root, which is the same as whatever the `main` page is.
pub async fn page_main(req: HttpRequest, deepwell: web::Data<DeepwellPool>) -> HttpResult {
    let host = get_host(&req);

    info!("GET / [{}]", host.unwrap_or("none"));

    let page_req = PageRequest {
        slug: "main",
        categories: Vec::new(),
        arguments: HashMap::new(),
    };

    Ok(get_deepwell_page_wrapped(*TEMP_WIKI_ID, page_req.slug, deepwell).await)
}
