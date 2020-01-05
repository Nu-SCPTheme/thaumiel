/*
 * route/api/misc.rs
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

pub async fn api_info(req: HttpRequest) -> HttpResponse {
    let path = req.uri().path();

    info!("GET api-info {}", path);

    HttpResponse::Ok().body("Some information about the API here idk")
}

pub async fn api_ping(req: HttpRequest) -> HttpResponse {
    info!("API /ping");

    // TODO setup proper JSON API response

    HttpResponse::Ok().body("pong!")
}
