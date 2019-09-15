/*
 * ssl.rs
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

use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct NetworkOptions {
    pub hostname: String,
    pub http_address: SocketAddr,
    pub https_address: SocketAddr,
    pub redirect_http: bool,
    pub private_key_file: PathBuf,
    pub certificate_file: PathBuf,
}

impl Into<(String, SocketAddr, SocketAddr, bool, ServerConfig)> for NetworkOptions {
    fn into(self) -> (String, SocketAddr, SocketAddr, bool, ServerConfig) {
        let mut config = ServerConfig::new(NoClientAuth::new());

        let mut key_file = {
            let file = File::open(&self.private_key_file).expect("Unable to open private key file");
            BufReader::new(file)
        };

        let mut cert_file = {
            let file = File::open(&self.certificate_file).expect("Unable to open certificate file");
            BufReader::new(file)
        };

        let mut keys = pkcs8_private_keys(&mut key_file).expect("Unable to create PKCS8 private keys");
        let cert_chain = certs(&mut cert_file).expect("Unable to create certificate chain");
        config.set_single_cert(cert_chain, keys.remove(0)).expect("Unable to set certificate in configuration");

        (self.hostname, self.http_address, self.https_address, self.redirect_http, config)
    }
}
