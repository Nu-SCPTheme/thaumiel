/*
 * config.rs
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

use crate::StdResult;
use actix_web::cookie::SameSite;
use dns_lookup::lookup_host;
use log::LevelFilter;
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, Read};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::time::Duration;
use structopt::StructOpt;

const DEFAULT_KEEP_ALIVE: usize = 20;
const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Info;

// Structopt argument parsing

#[derive(Debug, StructOpt)]
#[structopt(
    name = "thaumiel",
    about = "Wikidot-like web server to provide pages, forums, and other services"
)]
struct Options {
    /// Logging level to use.
    #[structopt(short, long)]
    level: Option<LevelFilter>,

    /// Override which path to read the cookie key from.
    #[structopt(short, long, name = "KEY_FILE", parse(from_os_str))]
    cookie_key_path: Option<PathBuf>,

    /// Configuration file.
    #[structopt(name = "CONFIG_FILE", parse(from_os_str))]
    config_file: PathBuf,
}

// Configuration objects

#[derive(Debug, Clone)]
pub struct Config {
    // Network
    pub hostname: String,
    pub http_address: SocketAddr,
    pub keep_alive: usize,
    // Server settings
    pub log_level: LevelFilter,
    pub cookie_secure: bool,
    pub cookie_max_age: i64,
    pub cookie_same_site: SameSite,
    pub cookie_key: Box<[u8]>,
    // Remote servers
    pub deepwell_address: SocketAddr,
    pub deepwell_timeout: Duration,
    pub deepwell_pool_size: usize,
    pub ftml_address: SocketAddr,
    pub ftml_timeout: Duration,
    pub ftml_pool_size: usize,
    // Runtime settings
    pub runtime: RuntimeSettings,
}

impl Config {
    #[cold]
    pub fn parse_args() -> Self {
        let Options {
            level,
            cookie_key_path,
            config_file,
        } = Options::from_args();

        // Build configuration from file
        let mut config = ConfigFile::read(&config_file);

        if let Some(cookie_key_path) = cookie_key_path {
            config.security.cookie_key_path = cookie_key_path;
        }

        // Convert into final config object
        let mut config: Self = config.into();

        if let Some(level) = level {
            config.log_level = level;
        }

        config
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeSettings {
    pub static_dir: PathBuf,
}

#[serde(rename_all = "kebab-case")]
#[derive(Deserialize, Debug)]
struct App {
    log_level: Option<String>,
}

#[serde(rename_all = "kebab-case")]
#[derive(Deserialize, Debug)]
struct Network {
    hostname: String,
    use_ipv6: bool,
    port: Option<u16>,
    keep_alive: Option<usize>,
}

#[serde(rename_all = "kebab-case")]
#[derive(Deserialize, Debug)]
struct Security {
    cookie_secure: bool,
    cookie_max_age: i64,
    cookie_same_site: String,
    cookie_key_path: PathBuf,
}

#[serde(rename_all = "kebab-case")]
#[derive(Deserialize, Debug)]
struct Files {
    static_dir: PathBuf,
}

#[serde(rename_all = "kebab-case")]
#[derive(Deserialize, Debug)]
struct Deepwell {
    host: String,
    port: u16,
    timeout: u32,
    pool_size: usize,
}

impl TryInto<(SocketAddr, Duration, usize)> for Deepwell {
    type Error = io::Error;

    fn try_into(self) -> StdResult<(SocketAddr, Duration, usize), io::Error> {
        let Self {
            host,
            port,
            timeout,
            pool_size,
        } = self;

        parse_service_config(&host, port, timeout, pool_size)
    }
}

#[serde(rename_all = "kebab-case")]
#[derive(Deserialize, Debug)]
struct Ftml {
    host: String,
    port: u16,
    timeout: u32,
    pool_size: usize,
}

impl TryInto<(SocketAddr, Duration, usize)> for Ftml {
    type Error = io::Error;

    fn try_into(self) -> StdResult<(SocketAddr, Duration, usize), io::Error> {
        let Self {
            host,
            port,
            timeout,
            pool_size,
        } = self;

        parse_service_config(&host, port, timeout, pool_size)
    }
}

#[serde(rename_all = "kebab-case")]
#[derive(Deserialize, Debug)]
struct ConfigFile {
    app: App,
    network: Network,
    security: Security,
    files: Files,
    deepwell: Deepwell,
    ftml: Ftml,
}

impl ConfigFile {
    #[cold]
    fn read(path: &Path) -> Self {
        let mut file = File::open(path).expect("Unable to open config file");
        let mut contents = String::new();
        let _ = file
            .read_to_string(&mut contents)
            .expect("Unable to read config file");

        let obj: Self = toml::from_str(&contents).expect("Unable to parse TOML in config file");

        obj
    }

    #[cold]
    fn parse_log_level(log_level: Option<&str>) -> LevelFilter {
        const LEVELS: [(&str, LevelFilter); 9] = [
            ("", DEFAULT_LOG_LEVEL),
            ("off", LevelFilter::Off),
            ("none", LevelFilter::Off),
            ("trace", LevelFilter::Trace),
            ("debug", LevelFilter::Debug),
            ("warn", LevelFilter::Warn),
            ("warning", LevelFilter::Warn),
            ("err", LevelFilter::Error),
            ("error", LevelFilter::Error),
        ];

        let log_level = match log_level {
            Some(ref log_level) => log_level,
            None => return DEFAULT_LOG_LEVEL,
        };

        for (text, level) in &LEVELS {
            if log_level.eq_ignore_ascii_case(text) {
                return *level;
            }
        }

        panic!("No log level for '{}'", log_level);
    }

    #[cold]
    fn parse_same_site(same_site: &str) -> SameSite {
        const POLICIES: [(&str, SameSite); 6] = [
            ("", SameSite::None),
            ("strict", SameSite::Strict),
            ("always", SameSite::Strict),
            ("lax", SameSite::Lax),
            ("none", SameSite::None),
            ("disabled", SameSite::None),
        ];

        for (text, policy) in &POLICIES {
            if same_site.eq_ignore_ascii_case(text) {
                return *policy;
            }
        }

        panic!("No same-site cookie policy for '{}'", same_site);
    }

    #[cold]
    fn read_cookie_key(path: &Path) -> Box<[u8]> {
        let mut file = File::open(path).expect("Unable to open cookie key file");
        let mut contents = Vec::new();
        let len = file
            .read_to_end(&mut contents)
            .expect("Unable to read bytes from cookie key file");

        assert!(
            len > 32,
            "Cookie key file did not contain enough bytes ({} < 32)",
            len,
        );

        contents.into_boxed_slice()
    }
}

impl Into<Config> for ConfigFile {
    #[cold]
    fn into(self) -> Config {
        let ConfigFile {
            app,
            network,
            security,
            files,
            deepwell,
            ftml,
        } = self;

        let Network {
            hostname,
            use_ipv6,
            port,
            keep_alive,
        } = network;

        let Security {
            cookie_secure,
            cookie_max_age,
            cookie_same_site,
            cookie_key_path,
        } = security;

        let App { log_level } = app;
        let Files { static_dir } = files;

        let (deepwell_address, deepwell_timeout, deepwell_pool_size) = deepwell
            .try_into()
            .expect("Unable to parse configuration for DEEPWELL connection");

        let (ftml_address, ftml_timeout, ftml_pool_size) = ftml
            .try_into()
            .expect("Unable to parse configuration for ftml connection");

        let ip_address = if use_ipv6 {
            IpAddr::V6(Ipv6Addr::UNSPECIFIED)
        } else {
            IpAddr::V4(Ipv4Addr::UNSPECIFIED)
        };

        let http_address = SocketAddr::new(ip_address, port.unwrap_or(80));
        let keep_alive = keep_alive.unwrap_or(DEFAULT_KEEP_ALIVE);
        let log_level = log_level.as_ref().map(|s| s.as_ref());

        let runtime = RuntimeSettings { static_dir };

        Config {
            hostname,
            http_address,
            keep_alive,
            log_level: Self::parse_log_level(log_level),
            cookie_secure,
            cookie_max_age,
            cookie_same_site: Self::parse_same_site(&cookie_same_site),
            cookie_key: Self::read_cookie_key(&cookie_key_path),
            deepwell_address,
            deepwell_timeout,
            deepwell_pool_size,
            ftml_address,
            ftml_timeout,
            ftml_pool_size,
            runtime,
        }
    }
}

fn parse_service_config(
    host: &str,
    port: u16,
    timeout_ms: u32,
    pool_size: usize,
) -> io::Result<(SocketAddr, Duration, usize)> {
    assert_ne!(pool_size, 0, "Connection pool size set to zero");

    let addresses = lookup_host(&host)?;

    assert!(!addresses.is_empty(), "No addresses returned");

    let address = addresses[0];
    let socket = SocketAddr::new(address, port);

    let timeout = Duration::from_millis(u64::from(timeout_ms));

    Ok((socket, timeout, pool_size))
}
