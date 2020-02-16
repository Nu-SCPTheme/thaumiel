/*
 * remote.rs
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

//! Wrappers for RPC client pools.

use deadpool::unmanaged::{Object, Pool};
use deepwell_rpc::Client as DeepwellClient;
use ftml_rpc::Client as FtmlClient;
use std::fmt::{self, Debug};
use std::net::SocketAddr;
use std::time::Duration;

pub struct RemotePool<T> {
    pool: Pool<T>,
}

impl<T> RemotePool<T> {
    #[inline]
    pub async fn get(&self) -> Object<T> {
        self.pool.get().await
    }
}

impl<T> Clone for RemotePool<T> {
    #[inline]
    fn clone(&self) -> Self {
        let pool = self.pool.clone();

        Self { pool }
    }
}

pub type DeepwellPool = RemotePool<DeepwellClient>;

impl RemotePool<DeepwellClient> {
    pub async fn connect(address: SocketAddr, timeout: Duration, size: usize) -> Self {
        info!("Initializing DEEPWELL client");

        let pool = Pool::new(size);

        for _ in 0..size {
            let worker = DeepwellClient::new(address, timeout)
                .await
                .expect("Unable to create new DEEPWELL client");

            pool.add(worker).await;
        }

        Self { pool }
    }
}

pub type FtmlPool = RemotePool<FtmlClient>;

impl RemotePool<FtmlClient> {
    pub async fn connect(address: SocketAddr, timeout: Duration, size: usize) -> Self {
        info!("Initializing ftml client");

        let pool = Pool::new(size);

        for _ in 0..size {
            let worker = FtmlClient::new(address, timeout)
                .await
                .expect("Unable to create new ftml client");

            pool.add(worker).await;
        }

        Self { pool }
    }
}

impl<T> Debug for RemotePool<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RemotePool")
            .field("pool", &"deadpool::unmanaged::Pool { .. }")
            .finish()
    }
}
