/*
 * route/api/types.rs
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

//! Helper structs and methods for the JSON API.
//!
//! All methods receive JSON and return JSON.
//!
//! If the operation succeeded, the result is returned:
//! ```text
//! {
//!     "result": { ... }
//! }
//! ```
//!
//! If the operation failed, an error is returned:
//! ```text
//! {
//!     "error": "not-logged-in",
//!     "message: "User is not logged in"
//! }
//! ```

use deepwell_core::Error as DeepwellError;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Success<T>
where
    T: Debug + Serialize,
{
    result: T,
}

impl<T> From<T> for Success<T>
where
    T: Debug + Serialize,
{
    #[inline]
    fn from(result: T) -> Self {
        Success { result }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Failure {
    error: &'static str,
    message: String,
}

impl From<&'_ DeepwellError> for Failure {
    #[inline]
    fn from(error: &DeepwellError) -> Self {
        Failure {
            error: error.fixed_name(),
            message: error.to_string(),
        }
    }
}
