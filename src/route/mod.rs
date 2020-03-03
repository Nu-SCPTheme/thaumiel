/*
 * route/mod.rs
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

mod prelude {
    pub use super::permissions::*;
    pub use crate::StdResult;
    pub use crate::utils::*;
    pub use actix_web::Error as ActixError;
    pub use actix_web::{http, web, HttpRequest, HttpResponse, HttpServer};
    pub use futures::{future, Future};

    pub type HttpResult = StdResult<HttpResponse, ActixError>;
}

mod account;
mod api;
mod files;
mod forum;
mod page;
mod permissions;
mod temp;
mod user;

pub use self::account::*;
pub use self::api::*;
pub use self::files::*;
pub use self::forum::*;
pub use self::page::*;
pub use self::permissions::*;
pub use self::temp::*;
pub use self::user::*;
