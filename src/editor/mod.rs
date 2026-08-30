// SPDX-License-Identifier: GPL-3.0-only

//! The rich editor (build step 3). See `RICH-EDITOR-PLAN.md`.
//!
//! `Content` wraps either iced's stock editor content or [`RichContent`];
//! which one new blocks get is decided by the `rich_editor` flag.

pub mod content;
pub mod spike;
pub mod style;
pub mod widget;

pub use content::Content;
pub use widget::RichEditor;

use std::sync::atomic::{AtomicBool, Ordering};

static RICH: AtomicBool = AtomicBool::new(false);

/// Whether new text blocks use the rich editor.
pub fn rich_enabled() -> bool {
    RICH.load(Ordering::Relaxed)
}

pub fn set_rich(on: bool) {
    RICH.store(on, Ordering::Relaxed);
}
