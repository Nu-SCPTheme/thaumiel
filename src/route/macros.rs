/*
 * route/macros.rs
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

macro_rules! try_io {
    ($result:expr) => {
        match $result {
            Ok(object) => object,
            Err(error) => {
                use deepwell_core::error::Error;
                let error = Error::ServiceTransport(error).to_sendable();

                return HttpResponse::BadGateway().json(error);
            }
        }
    };
}

macro_rules! try_io_option {
    ($result:expr) => {
        match $result {
            Ok(object) => object,
            Err(error) => {
                use deepwell_core::error::Error;
                let error = Error::ServiceTransport(error).to_sendable();

                return Some(HttpResponse::BadGateway().json(error));
            }
        }
    };
}

macro_rules! try_resp {
    ($result:expr) => {
        match $result {
            Ok(object) => object,
            Err(resp) => return resp,
        }
    };
}
