/*
 * route/forum.rs
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

// TODO

pub async fn forum_main(req: HttpRequest) -> HttpResult {
    Ok(HttpResponse::NotImplemented().body("forum main page"))
}

pub async fn forum_page(req: HttpRequest) -> HttpResult {
    Ok(HttpResponse::NotImplemented().body(format!("forum page:\n{:#?}", req)))
}

pub async fn forum_category(req: HttpRequest, category: web::Path<String>) -> HttpResult {
    Ok(HttpResponse::NotImplemented().body(format!("forum category: {}", &category)))
}

pub async fn forum_redirect_new_thread(category: web::Path<String>) -> HttpResult {
    info!("REDIRECT new-thread {}", category);

    let url = format!("/forum/new-thread/{}", category);
    let resp = HttpResponse::Found()
        .header(http::header::LOCATION, url)
        .finish();

    Ok(resp)
}

// old handlers, here for future reference
#[allow(dead_code)]
mod old {
    use super::super::prelude::*;
    use actix_web::Responder;

    fn get_thread(thread: String) -> impl Responder {
        info!("GET forum thread {}", thread);

        // TODO
        format!("forum:thread={}", thread)
    }

    #[inline]
    pub fn forum_thread(thread: web::Path<String>) -> impl Responder {
        get_thread(thread.into_inner())
    }

    #[inline]
    pub fn forum_thread_name(thread: web::Path<(String, String)>) -> impl Responder {
        get_thread(thread.into_inner().0)
    }

    fn get_category(category: String) -> impl Responder {
        info!("GET forum category {}", category);

        // TODO
        format!("forum:category={}", category)
    }

    #[inline]
    pub fn forum_category(category: web::Path<String>) -> impl Responder {
        get_category(category.into_inner())
    }

    #[inline]
    pub fn forum_category_name(category: web::Path<(String, String)>) -> impl Responder {
        get_category(category.into_inner().0)
    }

    pub fn forum_new_thread(category: web::Path<String>) -> impl Responder {
        info!("GET new-thread {}", category);

        // TODO
        format!("forum:new-thread:{}", category)
    }

    pub fn forum_recent_posts() -> impl Responder {
        info!("GET recent-posts");

        // TODO
        "forum:recent-posts"
    }

    pub fn forum_recent_threads() -> impl Responder {
        info!("GET recent-threads");

        // TODO
        "forum:recent-threads"
    }

    pub fn forum_main() -> impl Responder {
        info!("GET forum main");

        "forum"
    }
}
