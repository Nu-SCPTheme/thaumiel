/*
 * route/api/misc.rs
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

use super::prelude::*;
use crate::build;
use std::time::SystemTime;

lazy_static! {
    static ref CRATE_VERSION: &'static str =
        build::GIT_VERSION.unwrap_or(env!("CARGO_PKG_VERSION"));
    //
    static ref CRATE_BUILD: String = {
        format!(
            "{} {}\n{} on {}\nBuilt {}",
            env!("CARGO_PKG_NAME"),
            *CRATE_VERSION,
            build::RUSTC_VERSION,
            build::TARGET,
            build::BUILT_TIME_UTC,
        )
    };
}

/// Redirects invalid `/api` routes to prevent users from seeing invalid pages.
pub async fn api_route() -> HttpResponse {
    info!("REDIRECT / [api]");

    HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish()
}

/// No-op call to determine if a connection to the server can be established.
pub async fn api_ping() -> HttpResponse {
    info!("API /ping");

    HttpResponse::Ok().json(Success::from("pong!"))
}

/// Returns the current, local time on the web server.
/// This is neither monotonic nor in any particular timezone.
pub async fn api_time() -> HttpResponse {
    info!("API /time");

    let now = SystemTime::now();
    let unix_time = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("System time before epoch")
        .as_secs_f64();

    HttpResponse::Ok().json(Success::from(unix_time))
}

/// Returns what crate version the current web server has.
pub async fn api_version() -> HttpResponse {
    info!("API /version");

    HttpResponse::Ok().json(Success::from(*CRATE_VERSION))
}

/// Returns build information about the current web server.
pub async fn api_build() -> HttpResponse {
    info!("API /build");

    HttpResponse::Ok().json(Success::from(&*CRATE_BUILD))
}

/// Echoes the user's request back to them, to help with API debugging.
pub async fn api_debug(req: HttpRequest) -> HttpResponse {
    info!("API /debug");

    let output = format!("{:#?}", &req);

    HttpResponse::Ok().body(output)
}

/// Determines server health by pinging remote services and getting their statuses.
pub async fn api_health(
    deepwell: web::Data<DeepwellPool>,
    ftml: web::Data<FtmlPool>,
) -> HttpResponse {
    info!("API /services");

    #[derive(Serialize, Debug)]
    struct Status {
        deepwell: bool,
        ftml: bool,
    }

    // Get remote handles
    let (mut deepwell, mut ftml) = join!(
        deepwell.claim(), //
        ftml.claim(),
    );

    // Ping the services
    let (deepwell, ftml) = join!(
        deepwell.ping(), //
        ftml.ping(),
    );

    // Build status
    let status = Status {
        deepwell: deepwell.is_ok(),
        ftml: ftml.is_ok(),
    };

    HttpResponse::Ok().json(Success::from(status))
}
