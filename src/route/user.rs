/*
 * route/user.rs
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

pub fn user_get(id: web::Path<u64>) -> impl Responder {
    info!("GET user {}", id);

    // TODO
    format!("Getting user info for id {}", *id)
}

pub fn user_set(id: web::Path<u64>) -> impl Responder {
    info!("POST user {}", id);

    // TODO
    HttpResponse::Ok()
}
