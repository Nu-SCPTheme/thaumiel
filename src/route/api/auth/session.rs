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

#[derive(Deserialize, Debug)]
pub struct LoginInput {
    username_or_email: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginOutput<'a> {
    logged_in: &'a str,
    success: bool,
}

pub async fn api_login(
    req: HttpRequest,
    id: Identity,
    arg: web::Json<LoginInput>,
    deepwell: web::Data<DeepwellPool>,
) -> HttpResponse {
    info!("API v0 /auth/login");

    let LoginInput {
        username_or_email,
        password,
    } = &*arg;

    let address = req.connection_info().remote().map(String::from);
    debug!("Trying to log in as '{}'", username_or_email);

    let result = deepwell
        .get()
        .await
        .login(username_or_email.clone(), password.clone(), address)
        .await;

    match try_io!(result) {
        Ok(_) => {
            debug!("Login succeeded, beginning session");

            // TODO store session json in cookie
            id.remember("_test".into());

            let result = LoginOutput {
                logged_in: "_test",
                success: true,
            };

            HttpResponse::Ok().json(Success::from(result))
        }
        Err(error) => {
            debug!("Failed login attempt");

            HttpResponse::Unauthorized().json(error)
        }
    }
}

#[derive(Serialize, Debug)]
pub struct LogoutOutput<'a> {
    logged_out: &'a str,
    success: bool,
}

pub async fn api_logout(req: HttpRequest, id: Identity) -> HttpResponse {
    info!("API v0 /auth/logout");

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

            HttpResponse::Unauthorized().json(Error::NotLoggedIn.to_sendable())
        }
    }
}
