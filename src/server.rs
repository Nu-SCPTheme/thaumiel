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

use crate::config::RuntimeSettings;
use crate::middleware as crate_middleware;
use crate::route::*;
use actix_web::client::Client;
use actix_web::middleware as actix_middleware;
use actix_web::{http, web, App, HttpResponse, HttpServer, Responder};
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
pub async fn run(
    hostname: String,
    address: SocketAddr,
    keep_alive: usize,
    settings: RuntimeSettings,
) -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(Client::default())
            .data(settings.clone())
            .wrap(actix_middleware::Compress::default())
            .wrap(actix_middleware::Logger::default())
            .service(web::resource("{filename}.{ext}").to(static_file))
            .service(
                // TODO
                web::scope("forum:{page}")
                    .route("/", web::get().to(forum_page))
                    .route("/c/{category}", web::get().to(forum_category)),
            )
            .service(
                web::resource("/{page:.*}")
                    .wrap(crate_middleware::WikidotNormalizePath::default())
                    .to(temp_debug)
            )
    })
    .server_hostname(&hostname)
    .keep_alive(keep_alive)
    .bind(address)
    .expect("Unable to bind to HTTP socket")
    .run()
    .await

    /*
        original old httpserver that doesn't compile

    HttpServer::new(move || {
        App::new()
            .server_hostname(&hostname)
            .keep_alive(60)
            .data(Client::new())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            // Miscellaneous
            .route("favicon.ico", web::get().to(file_get))
            .route("robots.txt", web::get().to(file_get))
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
            .service(
                web::scope("forum")
                    .route("/", web::get().to(forum_main))
                    .route("c-{category}", web::get().to(forum_category))
                    .route(
                        "c-{category}/{name:.*}",
                        web::get().to(forum_category_name),
                    )
                    .route("t-{thread}", web::get().to(forum_thread))
                    .route(
                        "t-{thread}/{name:.*}",
                        web::get().to(forum_thread_name),
                    )
                    .route(
                        "new-thread/{category}",
                        web::get().to(forum_new_thread),
                    )
                    .route(
                        "new-thread/{category}/",
                        web::get().to(forum_new_thread),
                    )
                    .route("recent-posts", web::get().to(forum_recent_posts))
                    .route("recent-posts/", web::get().to(forum_recent_posts))
                    .route("recent-threads", web::get().to(forum_recent_threads))
                    .route("recent-threads/", web::get().to(forum_recent_threads))
            )
            .service(
                web::scope("/")
                    .route("{name}", web::get().to(page_get))
                    .route("{name}/", web::get().to(page_get))
                    .route("{name}/{options:.*}", web::get().to(page_get))
                    .route("/", web::get().to(page_main))
                    .route("/", web::route().to(HttpResponse::MethodNotAllowed))
            )
    })
    .bind(address)
    .expect("Unable to bind to HTTP socket")
    .run()
    */
}
