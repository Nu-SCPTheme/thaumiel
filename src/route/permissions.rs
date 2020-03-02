/*
 * route/permissions.rs
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

use crate::remote::DeepwellPool;
use crate::session::CookieSession;
use crate::StdResult;
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use deepwell_core::error::Error;
use deepwell_core::roles::Role;

pub async fn get_role(
    id: Identity,
    host: Option<&str>,
    deepwell: web::Data<DeepwellPool>,
) -> StdResult<Role, HttpResponse> {
    debug!("Checking role information from host {:?}", host);

    match id.identity() {
        None => Ok(Role::Guest),
        Some(ref data) => {
            let session = CookieSession::read(data)?;
            let mut deepwell = deepwell.claim().await;
            session.verify(&mut deepwell).await?;

            // TODO fetch role based on wiki_membership

            Ok(Role::Member)
        }
    }
}

pub async fn check_role(
    expected_role: Role,
    id: Identity,
    host: Option<&str>,
    deepwell: web::Data<DeepwellPool>,
) -> StdResult<(), HttpResponse> {
    let actual_role = get_role(id, host, deepwell).await?;

    if actual_role >= expected_role {
        Ok(())
    } else {
        let error = Error::InsufficientPermissions(actual_role, expected_role).to_sendable();

        Err(HttpResponse::Forbidden().json(error))
    }
}
