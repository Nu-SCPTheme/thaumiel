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

use crate::forwarder::Forwarder;
use crate::route::*;
use actix_web::client::Client;
use actix_web::{http, middleware, web, App, HttpResponse, HttpServer, Responder};
use std::io;
use std::net::SocketAddr;

#[inline]
fn redirect<S: AsRef<str>>(url: S) -> impl Responder {
    let url = url.as_ref();
    info!("REDIRECT {}", url);

    HttpResponse::Found()
        .header(http::header::LOCATION, url)
        .finish()
}

#[cold]
pub fn run(hostname: String, addr: SocketAddr, forwarder: Forwarder) -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(forwarder.clone())
            .data(Client::new())
            .hostname(&hostname)
            .wrap(middleware::Logger::default())
            // Miscellaneous
            .route("favicon.ico", web::get().to(|| HttpResponse::NotFound()))
            // Forum redirects
            .route("forum:start", web::get().to(|| redirect("/forum")))
            .route("forum:start/", web::get().to(|| redirect("/forum")))
            .route(
                "forum:new-thread/c/{category}",
                web::get().to(forum_redirect_new_thread),
            )
            .route(
                "forum:new-thread/c/{category}/",
                web::get().to(forum_redirect_new_thread),
            )
            .route(
                "forum:recent-posts",
                web::get().to(|| redirect("/forum/recent-posts")),
            )
            .route(
                "forum:recent-posts/",
                web::get().to(|| redirect("/forum/recent-posts")),
            )
            .route(
                "forum:recent-threads",
                web::get().to(|| redirect("/forum/recent-threads")),
            )
            .route(
                "forum:recent-threads/",
                web::get().to(|| redirect("/forum/recent-threads")),
            )
            // Forum links
            .route("forum", web::get().to(forum_main))
            .route("forum/c-{category}", web::get().to(forum_category))
            .route(
                "forum/c-{category}/{name:.*}",
                web::get().to(forum_category_name),
            )
            .route("forum/t-{thread}", web::get().to(forum_thread))
            .route(
                "forum/t-{thread}/{name:.*}",
                web::get().to(forum_thread_name),
            )
            .route(
                "forum/new-thread/{category}",
                web::get().to(forum_new_thread),
            )
            .route(
                "forum/new-thread/{category}/",
                web::get().to(forum_new_thread),
            )
            .route("forum/recent-posts", web::get().to(forum_recent_posts))
            .route("forum/recent-posts/", web::get().to(forum_recent_posts))
            .route("forum/recent-threads", web::get().to(forum_recent_threads))
            .route("forum/recent-threads/", web::get().to(forum_recent_threads))
            // User
            .route("user/{id}", web::get().to(user_get))
            .route("user/{id}", web::post().to(user_set))
            .route("user/avatars/{id}", web::get().to(user_avatar_get))
            // Regular pages
            .route("{name}", web::get().to_async(page_get))
            .route("{name}/", web::get().to_async(page_get))
            .route("{name}/{options:.*}", web::get().to_async(page_get))
            // Main page
            .route("/", web::get().to_async(page_main))
            .route("/", web::route().to(|| HttpResponse::MethodNotAllowed()))
    })
    .bind(addr)
    .expect("Unable to bind to address")
    .run()
}
