// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use serde_derive::{Deserialize, Serialize};
use std::{path::PathBuf, time::Duration};

const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(2);
const DEFAULT_GRPC_KEEPALIVE_TIME: Duration = Duration::from_secs(10);
const DEFAULT_GRPC_KEEPALIVE_TIMEOUT: Duration = Duration::from_secs(3);
const DEFAULT_GRPC_COMPLETION_QUEUE_SIZE: usize = 1;
const DEFAULT_MAX_BATCH_WAIT_TIME: Duration = Duration::from_millis(0);
const DEFAULT_MAX_BATCH_SIZE: usize = 8;
const DEFAULT_OVERLOAD_THRESHOLD: usize = 200;

/// The configuration for either a [`RawClient`](crate::RawClient) or a
/// [`TransactionClient`](crate::TransactionClient).
///
/// See also [`TransactionOptions`](crate::TransactionOptions) which provides more ways to configure
/// requests.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub ca_path: Option<PathBuf>,
    pub cert_path: Option<PathBuf>,
    pub key_path: Option<PathBuf>,
    pub timeout: Duration,
    pub kv_client_config: KVClientConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct KVClientConfig {
    pub completion_queue_size: usize,
    pub grpc_keepalive_time: Duration,
    pub grpc_keepalive_timeout: Duration,
    pub allow_batch: bool,
    pub overload_threshold: usize,
    pub max_batch_wait_time: Duration,
    pub max_batch_size: usize,
}

impl Default for KVClientConfig {
    fn default() -> Self {
        Self {
            completion_queue_size: DEFAULT_GRPC_COMPLETION_QUEUE_SIZE,
            grpc_keepalive_time: DEFAULT_GRPC_KEEPALIVE_TIME,
            grpc_keepalive_timeout: DEFAULT_GRPC_KEEPALIVE_TIMEOUT,
            allow_batch: true,
            overload_threshold: DEFAULT_OVERLOAD_THRESHOLD,
            max_batch_wait_time: DEFAULT_MAX_BATCH_WAIT_TIME,
            max_batch_size: DEFAULT_MAX_BATCH_SIZE,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ca_path: None,
            cert_path: None,
            key_path: None,
            timeout: DEFAULT_REQUEST_TIMEOUT,
            kv_client_config: KVClientConfig::default(),
        }
    }
}

impl Config {
    /// Set the certificate authority, certificate, and key locations for clients.
    ///
    /// By default, this client will use an insecure connection over instead of one protected by
    /// Transport Layer Security (TLS). Your deployment may have chosen to rely on security measures
    /// such as a private network, or a VPN layer to provide secure transmission.
    ///
    /// To use a TLS secured connection, use the `with_security` function to set the required
    /// parameters.
    ///
    /// TiKV does not currently offer encrypted storage (or encryption-at-rest).
    ///
    /// # Examples
    /// ```rust
    /// # use tikv_client::Config;
    /// let config = Config::default().with_security("root.ca", "internal.cert", "internal.key");
    /// ```
    #[must_use]
    pub fn with_security(
        mut self,
        ca_path: impl Into<PathBuf>,
        cert_path: impl Into<PathBuf>,
        key_path: impl Into<PathBuf>,
    ) -> Self {
        self.ca_path = Some(ca_path.into());
        self.cert_path = Some(cert_path.into());
        self.key_path = Some(key_path.into());
        self
    }

    /// Set the timeout for clients.
    ///
    /// The timeout is used for all requests when using or connecting to a TiKV cluster (including
    /// PD nodes). If the request does not complete within timeout, the request is cancelled and
    /// an error returned to the user.
    ///
    /// The default timeout is two seconds.
    ///
    /// # Examples
    /// ```rust
    /// # use tikv_client::Config;
    /// # use std::time::Duration;
    /// let config = Config::default().with_timeout(Duration::from_secs(10));
    /// ```
    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    // TODO: add more config options for tivk client config
}
