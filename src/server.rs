/*
 * server.rs
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

use crate::route::*;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::io;
use std::net::SocketAddr;

pub fn run(hostname: String, addr: SocketAddr) -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .hostname(&hostname)
            .route("favicon.ico", web::get().to(|| HttpResponse::NotFound()))
            .route("user/{id}", web::get().to(user_get))
            .route("user/{id}", web::post().to(user_set))
            .route("{slug}/{options:.*}", web::get().to(page_args))
            .route("{slug}", web::get().to(page_get))
            .route("/", web::get().to(page_main))
            .route("/", web::route().to(|| HttpResponse::MethodNotAllowed()))
    })
    .bind(addr)
    .expect("Unable to bind to address")
    .run()
}
