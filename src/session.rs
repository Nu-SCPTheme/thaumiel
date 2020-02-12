/*
 * session.rs
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

use crate::StdResult;
use actix_web::HttpResponse;
use deepwell_core::{Error, SessionId, UserId};
use serde_json as json;

/// Value kept in encrypted secret cookies.
///
/// The data here is not modifiable or viewable by
/// any clients, including the person logged in below.
///
/// Shorter field names are used to minimize network traffic
/// as cookies are sent with each request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CookieSession {
    #[serde(rename = "s")]
    pub session_id: SessionId,

    #[serde(rename = "u")]
    pub user_id: UserId,
}

impl CookieSession {
    pub fn parse(value: &str) -> Option<Self> {
        match json::from_str(value) {
            Ok(data) => Some(data),
            Err(error) => {
                warn!("Invalid JSON session data in cookie: {}", error);

                None
            }
        }
    }

    pub fn serialize(&self) -> StdResult<String, HttpResponse> {
        match json::to_string(self) {
            Ok(data) => Ok(data),
            Err(serde_err) => {
                error!("Unable to serialize session cookie data: {}", serde_err);

                let error = Error::StaticMsg("Unable to serialize session cookie data").to_sendable();

                Err(HttpResponse::InternalServerError().json(error))
            }
        }
    }
}
