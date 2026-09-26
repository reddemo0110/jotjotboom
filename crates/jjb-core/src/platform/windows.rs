// SPDX-License-Identifier: GPL-3.0-only

//! Windows (and, until it gets its own file, macOS): known folders through
//! `dirs`, opening through the shell. The keyring is a stub — Phase C wires
//! the Credential Manager through the `keyring` crate.

use super::Platform;
use anyhow::{Result, bail};
use std::path::PathBuf;

pub struct Windows;

impl Platform for Windows {
    fn name(&self) -> &'static str {
        if cfg!(windows) { "windows" } else { "other" }
    }

    fn default_notes_dir(&self) -> PathBuf {
        super::documents_or_home().join(super::NOTES_FOLDER)
    }

    fn data_dir(&self, app_id: &str) -> PathBuf {
        super::data_root().join(app_id)
    }

    fn store_secret(&self, _key: &str, _label: &str, _secret: &[u8]) -> Result<()> {
        bail!("the keyring is not wired up on this platform yet")
    }

    fn get_secret(&self, _key: &str) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }

    fn delete_secret(&self, _key: &str) -> Result<()> {
        Ok(())
    }

    fn open(&self, target: &str) -> Result<()> {
        super::open_detached(target)
    }
}
