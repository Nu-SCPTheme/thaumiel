/*
 * route/temp.rs
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

pub async fn temp_a(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Page A!")
}

pub async fn temp_b(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("The second page, B!")
}

pub async fn temp_debug(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body(format!("{:#?}", &req))
}
