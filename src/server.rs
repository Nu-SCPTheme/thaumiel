/*
 * server.rs
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

use crate::config::RuntimeSettings;
use crate::middleware as crate_middleware;
use crate::remote::{DeepwellPool, FtmlPool};
use crate::route::*;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::client::Client;
use actix_web::cookie::SameSite;
use actix_web::middleware as actix_middleware;
use actix_web::{http, web, App, HttpResponse, HttpServer};
use std::io;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Server {
    pub hostname: String,
    pub http_address: SocketAddr,
    pub keep_alive: usize,
    pub cookie_secure: bool,
    pub cookie_max_age: i64,
    pub cookie_same_site: SameSite,
    pub cookie_key: Box<[u8]>,
    pub deepwell: DeepwellPool,
    pub ftml: FtmlPool,
}

impl Server {
    #[cold]
    pub async fn run(self, settings: RuntimeSettings) -> io::Result<()> {
        let Self {
            hostname,
            http_address,
            keep_alive,
            cookie_secure,
            cookie_max_age,
            cookie_same_site,
            cookie_key,
            deepwell,
            ftml,
        } = self;

        HttpServer::new(move || {
            App::new()
                // Shared data and clients
                .data(Client::default())
                .data(deepwell.clone())
                .data(ftml.clone())
                .data(settings.clone())
                // Middleware
                .wrap(actix_middleware::Compress::default())
                .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(&cookie_key)
                        .name("thaumiel-auth")
                        .secure(cookie_secure)
                        .max_age(cookie_max_age)
                        .same_site(cookie_same_site),
                ))
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
                // User information
                .service(web::resource("user:info/{name}").to(user_info))
                // Other special routes
                .service(web::resource("verify-email/{token}").to(verify_email))
                // API handling
                .service(
                    web::scope("api")
                        .route("", web::get().to(api_route))
                        .service(
                            web::scope("v0")
                                .route("", web::get().to(api_route))
                                .route("ping", web::get().to(api_ping))
                                .route("ping", web::post().to(api_ping))
                                .route("ping", web::put().to(api_ping))
                                .route("services", web::get().to(api_services))
                                .route("services", web::post().to(api_services))
                                .route("services", web::put().to(api_services))
                                .route("time", web::get().to(api_time))
                                .route("version", web::get().to(api_version))
                                .route("build", web::get().to(api_build))
                                .route("debug", web::to(api_debug))
                                .service(
                                    web::scope("auth")
                                        .route("", web::get().to(api_route))
                                        .route("login", web::post().to(api_login))
                                        .route("logout", web::post().to(api_logout))
                                        .route("logout", web::delete().to(api_logout))
                                        .route("register", web::post().to(temp_api))
                                        .route("confirm-register", web::post().to(temp_api))
                                        .route("confirm-reset-password", web::post().to(temp_api))
                                        .route("reset-password", web::post().to(temp_api))
                                        .route("reset-password", web::put().to(temp_api)),
                                )
                                .service(
                                    web::scope("page")
                                        .route("", web::get().to(api_route))
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
                        ),
                )
                // Pages
                .service(web::resource("{name}").to(page_get))
                .service(web::resource("{name}/{options:.*}").to(page_get))
                .service(web::resource("/").to(page_main))
        })
        .server_hostname(&hostname)
        .keep_alive(keep_alive)
        .bind(http_address)
        .expect("Unable to bind to HTTP socket")
        .run()
        .await
    }
}

async fn redirect(url: &str) -> HttpResponse {
    info!("REDIRECT {}", url);

    HttpResponse::Found()
        .header(http::header::LOCATION, url)
        .finish()
}
