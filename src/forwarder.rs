/*
 * forwarder.rs
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

use crate::request::PageRequest;
use actix_web::client::Client;
use actix_web::{http, Error, HttpResponse};
use futures::Future;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Forwarder {
    pub file_dir: PathBuf,
    pub page_host: String,
}

impl Forwarder {
    pub fn get_file(&self, path: &str) -> HttpResponse {
        assert!(path.starts_with("/"));
        let full_path = self.file_dir.join(&path[1..]);

        debug!("Getting file at {}", full_path.display());

        fn read_file(path: &Path) -> io::Result<String> {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        }

        match read_file(&full_path) {
            Ok(contents) => HttpResponse::Ok().body(contents),
            Err(error) => {
                use io::ErrorKind::*;

                let mut resp = match error.kind() {
                    NotFound => HttpResponse::NotFound(),
                    PermissionDenied => HttpResponse::Forbidden(),
                    TimedOut => HttpResponse::RequestTimeout(),
                    _ => HttpResponse::InternalServerError(),
                };

                resp.body(str!(error))
            }
        }
    }

    pub fn get_page(
        &self,
        client: &Client,
        request: &PageRequest,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        debug!("Sending page request: {:?}", request);

        let body = serde_json::to_string(request).expect("Unable to serialize PageRequest to JSON");

        client
            .get(&self.page_host)
            .header(http::header::USER_AGENT, "kant-router")
            .header(http::header::CONTENT_TYPE, "application/json")
            .send_body(body)
            .map_err(Error::from)
            .map(|resp| {
                let mut client_resp = HttpResponse::build(resp.status());
                let headers = resp
                    .headers()
                    .iter()
                    .filter(|(h, _)| *h != "connection" && *h != "content-length");

                for (name, value) in headers {
                    client_resp.header(name.clone(), value.clone());
                }

                client_resp.streaming(resp)
            })
    }

    pub fn send_login(&self, _client: ()) {
        debug!("Login stub")
    }

    pub fn send_logout(&self, _client: ()) {
        debug!("Logout stub")
    }
}
