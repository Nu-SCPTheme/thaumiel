/*
 * route/api/auth/session.rs
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
use actix_identity::Identity;

#[derive(Deserialize, Debug)]
pub struct LoginInput {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginOutput<'a> {
    logged_in: &'a str,
    success: bool,
}

pub async fn api_login(req: HttpRequest, id: Identity, arg: web::Json<LoginInput>) -> HttpResponse {
    info!("API /auth/login");

    let LoginInput { username, password } = &*arg;

    // TODO authenticate with deepwell
    let valid = true;

    if valid {
        debug!("Logging in user '{}'", username);

        id.remember(username.clone());

        let result = LoginOutput {
            logged_in: username,
            success: true,
        };

        HttpResponse::Ok().json(Success::from(result))
    } else {
        debug!("Failed login attempt for user '{}'", username);

        let error = Error::AuthenticationFailed;

        HttpResponse::Unauthorized().json(Failure::from(&error))
    }
}

#[derive(Serialize, Debug)]
pub struct LogoutOutput<'a> {
    logged_out: &'a str,
    success: bool,
}

pub async fn api_logout(req: HttpRequest, id: Identity) -> HttpResponse {
    info!("API /auth/logout");

    match id.identity() {
        Some(username) => {
            debug!("Logging out user '{}'", username);

            id.forget();

            let result = LogoutOutput {
                logged_out: &username,
                success: true,
            };

            HttpResponse::Ok().json(Success::from(result))
        }
        None => {
            debug!("Cannot logout, no session cookie");

            let error = Error::NotLoggedIn;

            HttpResponse::Unauthorized().json(Failure::from(&error))
        }
    }
}
