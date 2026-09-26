// SPDX-License-Identifier: GPL-3.0-only

//! The core's image handling plus what only the iced shell needs: the
//! drag-and-drop payload type and the palette → inks conversion.

pub use jjb_core::images::*;

use crate::retro::Palette;
use cosmic::iced::clipboard::mime::AllowedMimeTypes;
use std::borrow::Cow;
use std::path::PathBuf;

/// A `text/uri-list` drop (files dragged from a file manager).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UriList(pub Vec<PathBuf>);

impl AllowedMimeTypes for UriList {
    fn allowed() -> Cow<'static, [String]> {
        Cow::Owned(vec!["text/uri-list".to_owned()])
    }
}

impl TryFrom<(Vec<u8>, String)> for UriList {
    type Error = anyhow::Error;

    fn try_from((data, mime): (Vec<u8>, String)) -> anyhow::Result<Self> {
        anyhow::ensure!(
            mime.starts_with("text/uri-list"),
            "unsupported mime type {mime}"
        );
        Ok(UriList(uri_list_paths(&String::from_utf8_lossy(&data))))
    }
}

fn c8(c: cosmic::iced::Color) -> [f32; 3] {
    [c.r * 255.0, c.g * 255.0, c.b * 255.0]
}

/// The colours the pixel treatments paint with, from a theme palette.
pub fn inks(p: &Palette) -> Inks {
    Inks {
        bg: c8(p.bg),
        mute: c8(p.mute),
        dim: c8(p.dim),
        fg: c8(p.fg),
        accent: c8(p.accent),
    }
}
