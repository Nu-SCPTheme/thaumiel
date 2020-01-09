/*
 * route/files.rs
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
use crate::config::RuntimeSettings;
use actix_files::NamedFile;

pub async fn static_file(req: HttpRequest, settings: web::Data<RuntimeSettings>) -> HttpResult {
    let info = req.match_info();

    let mut path = settings.static_dir.clone();
    path.push(info.query("filename"));
    path.set_extension(info.query("ext"));

    let file = NamedFile::open(&path)?;
    let resp = file.into_response(&req)?;
    Ok(resp)
}
