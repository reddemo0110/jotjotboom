// SPDX-License-Identifier: GPL-3.0-only

//! The rich editor (build step 3): a cosmic-text buffer drawn straight
//! through the renderer, with markdown rendered as attributes and overlays.
//! See `RICH-EDITOR-PLAN.md`.

pub mod content;
pub mod style;
pub mod widget;

pub use content::Content;
pub use widget::RichEditor;
