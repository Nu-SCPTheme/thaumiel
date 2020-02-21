/*
 * route/api/auth/account.rs
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
#[serde(rename_all = "kebab-case")]
pub struct RegisterInput {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct RegisterOutput {
    user_id: UserId,
    success: bool,
}

pub async fn api_register(
    req: HttpRequest,
    id: Identity,
    arg: web::Json<RegisterInput>,
    deepwell: web::Data<DeepwellPool>,
) -> HttpResponse {
    info!("API v0 /auth/register");

    let RegisterInput {
        username,
        email,
        password,
    } = &*arg;

    // Create user
    let result = deepwell
        .get()
        .await
        .create_user(username.clone(), email.clone(), password.clone())
        .await;

    // Send verification email
    // TODO

    match try_io!(result) {
        Ok(user_id) => {
            info!(
                "Created new user '{}' (email '{}'), has ID {}",
                username, email, user_id,
            );

            let result = RegisterOutput {
                user_id,
                success: true,
            };

            HttpResponse::Ok().json(Success::from(result))
        }
        Err(error) => {
            warn!("Failed to create new user: {}", error);

            HttpResponse::InternalServerError().json(error)
        }
    }
}
