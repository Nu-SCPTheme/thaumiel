/*
 * middleware/normalize.rs
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

//! Middleware to normalize URLs in accordance to Wikidot's redirection rules.
//!
//! Uses the `wikidot_normalize` crate.

use crate::StdResult;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{ok, Ready};
use std::task::{Context, Poll};
use void::Void;
use wikidot_normalize::normalize;

#[derive(Debug, Copy, Clone)]
pub struct WikidotNormalizePath;

impl<S, B> Transform<S> for WikidotNormalizePath
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = Void;
    type Transform = WikidotPathNormalization<S>;
    type Future = Ready<StdResult<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> <WikidotNormalizePath as Transform<S>>::Future {
        ok(WikidotPathNormalization { service })
    }
}

pub struct WikidotPathNormalization<S> {
    service: S,
}

impl<S, B> Service for WikidotPathNormalization<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<StdResult<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let head = req.head_mut();
        let path = head.uri.path();

        // TODO

        self.service.call(req)
    }
}
