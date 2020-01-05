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
use actix_web::{http, web, App, HttpResponse, HttpServer};
use std::io;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Server {
    pub hostname: String,
    pub http_address: SocketAddr,
    pub keep_alive: usize,
}

impl Server {
    #[cold]
    pub async fn run(&self, settings: RuntimeSettings) -> io::Result<()> {
        HttpServer::new(move || {
            App::new()
                .data(Client::default())
                .data(settings.clone())
                .wrap(actix_middleware::Compress::default())
                .wrap(crate_middleware::WikidotNormalizePath::default())
                .wrap(actix_middleware::Logger::default())
                // Static files (e.g. favicon, robots.txt)
                .service(web::resource("{filename}.{ext}").to(static_file))
                // Forum redirects
                .service(web::resource("forum:start").to(|| redirect("/forum")))
                .service(web::resource("forum:recent-posts").to(|| redirect("/forum/recent-posts")))
                .service(
                    web::resource("forum:recent-threads").to(|| redirect("/forum/recent-threads")),
                )
                .service(
                    web::resource("forum:new-thread/c/{category}").to(forum_redirect_new_thread),
                )
                // Forum links
                .service(web::resource("forum").to(forum_main))
                .service(web::resource("forum/c-{category}").to(forum_category))
                .service(web::resource("forum/c-{category}/{name:.*}").to(forum_category_name))
                .service(web::resource("forum/t-{thread}").to(forum_thread))
                .service(web::resource("forum/t-{thread}/{name:.*}").to(forum_thread_name))
                .service(web::resource("forum/new-thread/{category}").to(forum_new_thread))
                .service(web::resource("forum/recent-posts").to(forum_recent_posts))
                .service(web::resource("forum/recent-threads").to(forum_recent_threads))
                // API handling
                .service(
                    web::scope("api")
                        .route("/", web::get().to(api_info))
                        .route("ping", web::get().to(api_ping))
                        .route("ping", web::post().to(api_ping))
                        .route("ping", web::put().to(api_ping))
                        .service(
                            web::scope("auth")
                                .route("/", web::get().to(api_info))
                                .route("login", web::post().to(temp_api))
                                .route("logout", web::delete().to(temp_api))
                                .route("register", web::post().to(temp_api))
                                .route("confirm-register", web::post().to(temp_api))
                                .route("confirm-reset-password", web::post().to(temp_api))
                                .route("reset-password", web::post().to(temp_api))
                                .route("reset-password", web::put().to(temp_api)),
                        )
                        .service(
                            web::scope("page")
                                .route("/", web::get().to(api_info))
                                .route("edit-lock", web::post().to(temp_api))
                                .route("history", web::get().to(temp_api))
                                .route("parent", web::get().to(temp_api))
                                .route("parent", web::post().to(temp_api))
                                .route("rename", web::post().to(temp_api))
                                .route("revision", web::get().to(temp_api))
                                .route("source", web::get().to(temp_api))
                                .route("tags", web::get().to(temp_api))
                                .route("tags", web::post().to(temp_api))
                                .route("vote", web::get().to(temp_api))
                                .route("vote", web::post().to(temp_api))
                                .route("vote", web::delete().to(temp_api)),
                        )
                        .service(web::scope("user").route("info", web::get().to(temp_api))),
                )
                // Pages
                .service(web::resource("{name}").to(temp_debug))
                .service(web::resource("{name}/").to(temp_debug))
                .service(web::resource("/{name}/{options:.*}").to(temp_debug))
                .service(web::resource("/").to(temp_debug))
        })
        .server_hostname(&self.hostname)
        .keep_alive(self.keep_alive)
        .bind(self.http_address)
        .expect("Unable to bind to HTTP socket")
        .run()
        .await

        /*
            original old httpserver that doesn't compile

        HttpServer::new(move || {
            App::new()
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
        })
        */
    }
}

async fn redirect(url: &str) -> HttpResponse {
    info!("REDIRECT {}", url);

    HttpResponse::Found()
        .header(http::header::LOCATION, url)
        .finish()
}
