/*
 * utils.rs
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

use actix_web::dev::ServiceRequest;
use actix_web::{http, HttpRequest};
use std::net::{IpAddr, Ipv6Addr};

/// Gets the requested hostname, from URI, then headers if present.
pub fn get_host(req: &HttpRequest) -> Option<&str> {
    if let Some(host) = req.uri().host() {
        return Some(host);
    }

    match req.headers().get(http::header::HOST) {
        Some(value) => value.to_str().ok(),
        None => None,
    }
}

/// Gets the client's IP address.
/// Checks `X-Forwarded-Host` if it exists, then client address, then void.
pub fn get_client_ip(req: &ServiceRequest) -> IpAddr {
    // Check forwarded headers, if it's a valid IP address.
    if let Some(value) = req.headers().get("X-Forwarded-Host") {
        // Make sure it's UTF-8 and a valid IP address
        if let Ok(address) = value.to_str() {
            if let Ok(ip) = address.parse() {
                return ip;
            }
        }
    }

    // Check client address
    if let Some(address) = req.peer_addr() {
        return address.ip();
    }

    // Return [::] if all else fails
    IpAddr::V6(Ipv6Addr::UNSPECIFIED)
}
