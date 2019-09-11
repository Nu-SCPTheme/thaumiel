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

pub fn page_args(parts: web::Path<(String, String)>) -> impl Responder {
    // TODO proxy request to web server
    format!("page:{}:{}", parts.0, parts.1)
}

pub fn page_get(slug: web::Path<String>) -> impl Responder {
    format!("page:{}", slug)
}

pub fn page_main() -> impl Responder {
    "page:main"
}
