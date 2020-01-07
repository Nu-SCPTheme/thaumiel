/*
 * route/api/misc.rs
 *
 * thaumiel - Wikidot-like web server to provide pages, forums, and other services
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
use std::time::SystemTime;

pub async fn api_route() -> HttpResponse {
    info!("REDIRECT / [from api]");

    HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish()
}

pub async fn api_ping() -> HttpResponse {
    info!("API /ping");

    HttpResponse::Ok().json(Success::from("pong!"))
}

pub async fn api_time() -> HttpResponse {
    info!("API /time");

    let now = SystemTime::now();
    let unix_time = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("System time before epoch")
        .as_secs_f64();

    HttpResponse::Ok().json(Success::from(unix_time))
}