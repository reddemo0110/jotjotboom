// SPDX-License-Identifier: GPL-3.0-only

//! Linux: XDG directories and the Secret Service keyring over D-Bus (the
//! same keyring GNOME's and COSMIC's password managers show).

use super::Platform;
use anyhow::{Context, Result};
use secret_service::EncryptionType;
use secret_service::blocking::SecretService;
use std::collections::HashMap;
use std::path::PathBuf;

const APP_ATTR: (&str, &str) = ("application", "attic");

fn attrs(key: &str) -> HashMap<&str, &str> {
    HashMap::from([APP_ATTR, ("key", key)])
}

pub struct Linux;

impl Platform for Linux {
    fn name(&self) -> &'static str {
        "linux"
    }

    fn default_notes_dir(&self) -> PathBuf {
        super::documents_or_home().join(super::NOTES_FOLDER)
    }

    fn data_dir(&self, app_id: &str) -> PathBuf {
        super::data_root().join(app_id)
    }

    fn store_secret(&self, key: &str, label: &str, secret: &[u8]) -> Result<()> {
        let ss =
            SecretService::connect(EncryptionType::Dh).context("connecting to secret service")?;
        let collection = ss
            .get_default_collection()
            .context("opening default keyring collection")?;
        collection
            .create_item(label, attrs(key), secret, true, "text/plain")
            .context("storing secret")?;
        Ok(())
    }

    fn get_secret(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let ss =
            SecretService::connect(EncryptionType::Dh).context("connecting to secret service")?;
        let items = ss.search_items(attrs(key)).context("searching keyring")?;
        let Some(item) = items.unlocked.into_iter().next() else {
            return Ok(None);
        };
        Ok(Some(item.get_secret().context("reading secret")?))
    }

    fn delete_secret(&self, key: &str) -> Result<()> {
        let ss =
            SecretService::connect(EncryptionType::Dh).context("connecting to secret service")?;
        let items = ss.search_items(attrs(key)).context("searching keyring")?;
        for item in items.unlocked.into_iter().chain(items.locked) {
            item.delete().context("deleting secret")?;
        }
        Ok(())
    }

    fn open(&self, target: &str) -> Result<()> {
        super::open_detached(target)
    }
}
