// SPDX-License-Identifier: GPL-3.0-only

//! The keyring, as awaitable calls for the app's tasks. The work itself is
//! the core's `platform` (Secret Service on Linux), run on a blocking
//! thread so a slow or locked keyring never stalls the UI runtime.

use anyhow::Result;
use jjb_core::platform;

/// Store a secret under `key`, replacing any existing value.
pub async fn store(key: &str, label: &str, secret: &[u8]) -> Result<()> {
    let (key, label, secret) = (key.to_owned(), label.to_owned(), secret.to_vec());
    tokio::task::spawn_blocking(move || platform::current().store_secret(&key, &label, &secret))
        .await?
}

/// Fetch the secret stored under `key`, if any.
pub async fn get(key: &str) -> Result<Option<Vec<u8>>> {
    let key = key.to_owned();
    tokio::task::spawn_blocking(move || platform::current().get_secret(&key)).await?
}

/// Remove the secret stored under `key`. No-op if absent.
pub async fn delete(key: &str) -> Result<()> {
    let key = key.to_owned();
    tokio::task::spawn_blocking(move || platform::current().delete_secret(&key)).await?
}
