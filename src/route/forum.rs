/*
 * route/forum.rs
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

// TODO

pub fn forum_thread(thread: web::Path<String>) -> impl Responder {
    let thread = thread.into_inner();
    format!("thread:{}", thread)
}

pub fn forum_thread_name(thread: web::Path<(String, String)>) -> impl Responder {
    let thread = thread.into_inner().0;
    format!("thread:{}", thread)
}

pub fn forum_category(category: web::Path<String>) -> impl Responder {
    let category = category.into_inner();
    format!("category:{}", category)
}

pub fn forum_category_name(category: web::Path<(String, String)>) -> impl Responder {
    let category = category.into_inner().0;
    format!("category:{}", category)
}

pub fn forum_main() -> impl Responder {
    "forum"
}
