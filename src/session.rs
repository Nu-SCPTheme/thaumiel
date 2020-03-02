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
use deepwell_core::error::Error;
use deepwell_core::roles::Role;
use deepwell_core::types::{SessionId, UserId};
use deepwell_rpc::Client as DeepwellClient;
use serde_json as json;

macro_rules! http_err {
    ($message:expr) => {{
        let error = Error::StaticMsg($message).to_sendable();

        HttpResponse::InternalServerError().json(error)
    }};
}

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
    pub fn read(data: &str) -> StdResult<Self, HttpResponse> {
        json::from_str(data).map_err(|error| {
            error!("Invalid JSON session data in cookie: {}", error);

            http_err!("Unable to read session cookie")
        })
    }

    pub fn serialize(&self) -> StdResult<String, HttpResponse> {
        json::to_string(self).map_err(|error| {
            error!("Unable to serialize session cookie data: {}", error);

            http_err!("Unable to write session cookie")
        })
    }

    pub async fn verify(&self, deepwell: &mut DeepwellClient) -> StdResult<(), HttpResponse> {
        let result = deepwell.check_session(self.session_id, self.user_id).await;

        match result {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(error)) => Err(HttpResponse::Unauthorized().json(error)),
            Err(error) => {
                let error = Error::ServiceTransport(error).to_sendable();

                Err(HttpResponse::BadGateway().json(error))
            }
        }
    }
}
