// SPDX-License-Identifier: GPL-3.0-only

//! System keyring access via the Secret Service D-Bus API.
//!
//! Nothing in the app needs a secret yet — this exists so the plumbing is in
//! place before sync lands (bearer tokens must never touch plaintext config).

#![allow(dead_code)]

use anyhow::{Context, Result};
use secret_service::{EncryptionType, SecretService};
use std::collections::HashMap;

const APP_ATTR: (&str, &str) = ("application", "jotjotboom");

fn attrs(key: &str) -> HashMap<&str, &str> {
    HashMap::from([APP_ATTR, ("key", key)])
}

/// Store a secret under `key`, replacing any existing value.
pub async fn store(key: &str, label: &str, secret: &[u8]) -> Result<()> {
    let ss = SecretService::connect(EncryptionType::Dh)
        .await
        .context("connecting to secret service")?;
    let collection = ss
        .get_default_collection()
        .await
        .context("opening default keyring collection")?;
    collection
        .create_item(label, attrs(key), secret, true, "text/plain")
        .await
        .context("storing secret")?;
    Ok(())
}

/// Fetch the secret stored under `key`, if any.
pub async fn get(key: &str) -> Result<Option<Vec<u8>>> {
    let ss = SecretService::connect(EncryptionType::Dh)
        .await
        .context("connecting to secret service")?;
    let items = ss
        .search_items(attrs(key))
        .await
        .context("searching keyring")?;
    let Some(item) = items.unlocked.into_iter().next() else {
        return Ok(None);
    };
    Ok(Some(item.get_secret().await.context("reading secret")?))
}

/// Remove the secret stored under `key`. No-op if absent.
pub async fn delete(key: &str) -> Result<()> {
    let ss = SecretService::connect(EncryptionType::Dh)
        .await
        .context("connecting to secret service")?;
    let items = ss
        .search_items(attrs(key))
        .await
        .context("searching keyring")?;
    for item in items.unlocked.into_iter().chain(items.locked) {
        item.delete().await.context("deleting secret")?;
    }
    Ok(())
}
