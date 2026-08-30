// SPDX-License-Identifier: GPL-3.0-only

//! Debug-only driver: `JJB_SCRIPT="new;type:Hello #tag;wait:1500;exit"` feeds
//! the app the same messages real input would, so end-to-end behaviour can be
//! exercised (and captured with `tools/xshot.py`) without a human at the
//! keyboard. Steps are separated by `;`; `\n` and `\;` are unescaped in text.

use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    /// Create a new note (Ctrl+N).
    New,
    /// Insert text into the editor at the cursor.
    Type(String),
    /// Type into the search box.
    Search(String),
    /// Select the n-th note in the current list (0-based).
    Select(usize),
    /// Toggle pin on the current note.
    Pin,
    /// Move the current note to the trash.
    Trash,
    /// Create a folder (tag) and switch to it.
    Folder(String),
    /// Apply a dock format action by key (bold, italic, code, h1, h2, bullet, todo, link, tag, rule).
    Format(String),
    /// Select all text in the editor.
    SelectAll,
    /// Toggle the dock's `+` section.
    Dock,
    /// Undo / redo one step.
    Undo,
    Redo,
    /// Toggle the shortcuts overlay.
    Shortcuts,
    /// Click the save tick.
    SavedInfo,
    /// Open the theme picker drawer.
    Themes,
    /// Toggle editor-only layout.
    Solo,
    /// Import an image file into the note.
    Image(String),
    /// Open the image picker (portal dialog).
    Pick,
    /// Set the frame style of the n-th image: `imgframe:n:key`.
    ImgFrame(usize, String),
    /// Set alignment of the n-th image: `imgalign:n:left|center|right`.
    ImgAlign(usize, String),
    /// Set width in px of the n-th image (0 = full): `imgwidth:n:px`.
    ImgWidth(usize, u32),
    /// Open the ⋯ menu of the n-th image.
    ImgMenu(usize),
    /// Show the n-th image mid-drag with the drop line before body line `line`: `imgdrag:n:line`.
    ImgDrag(usize, usize),
    /// Move the n-th image to sit before body line `line`: `imgmove:n:line`.
    ImgMove(usize, usize),
    /// Set the caption of the n-th image: `imgcaption:n:text`.
    ImgCaption(usize, String),
    /// Switch theme by key.
    Theme(String),
    /// Fold / unfold the sub-tags of a tag: `fold:travels`.
    Fold(String),
    /// Editor font by key: `font:plex`.
    Font(String),
    /// Designer font pairing by key: `pairing:editorial`.
    Pairing(String),
    /// Step a pane's text size: `size:editor:+3`, `size:sidebar:-1`, `size:list:+2`.
    Size(String, i16),
    /// Dock size by key: `docksize:wow`.
    DockSize(String),
    /// Fold / unfold an Appearance section: `section:colour|font|size`.
    Section(String),
    /// Open the right-click menu of a tag: `tagmenu:travels`.
    TagMenu(String),
    /// Set the finished-task mark: `marker:🦆`.
    Marker(String),
    /// Set the text column width: `measure:narrow|medium|wide|full`.
    Measure(String),
    /// Follow a wiki link by title, as Ctrl+click would: `follow:Kyoto!`.
    Follow(String),
    /// Launcher icon: a theme key or `follow`: `icon:amber`.
    Icon(String),
    /// Toggle the neon coffee sign.
    Coffee,
    /// Walk the notes list like ↑/↓: `nav:+1` / `nav:-1`.
    Nav(i32),
    /// Toggle the task box at line:column of the focused block: `togglebox:2:3`.
    ToggleBox(usize, usize),
    /// Rename a tag everywhere: `renametag:old:new`.
    RenameTag(String, String),
    /// Pause for the given milliseconds (lets autosave run).
    Wait(u64),
    /// Flush and quit.
    Exit,
}

pub fn parse(script: &str) -> Vec<Step> {
    split_steps(script)
        .into_iter()
        .filter_map(|raw| {
            let raw = raw.trim();
            if raw.is_empty() {
                return None;
            }
            let (cmd, arg) = raw
                .split_once(':')
                .map_or((raw, ""), |(c, a)| (c.trim(), a));
            let step = match cmd {
                "new" => Step::New,
                "type" => Step::Type(unescape(arg)),
                "search" => Step::Search(unescape(arg)),
                "select" => Step::Select(arg.trim().parse().ok()?),
                "pin" => Step::Pin,
                "trash" => Step::Trash,
                "folder" => Step::Folder(unescape(arg)),
                "format" => Step::Format(arg.trim().to_owned()),
                "selectall" => Step::SelectAll,
                "dock" => Step::Dock,
                "undo" => Step::Undo,
                "shortcuts" => Step::Shortcuts,
                "savedinfo" => Step::SavedInfo,
                "redo" => Step::Redo,
                "themes" => Step::Themes,
                "solo" => Step::Solo,
                "image" => Step::Image(unescape(arg)),
                "pick" => Step::Pick,
                "imgframe" => {
                    let (n, key) = arg.trim().split_once(':')?;
                    Step::ImgFrame(n.parse().ok()?, key.to_owned())
                }
                "imgalign" => {
                    let (n, key) = arg.trim().split_once(':')?;
                    Step::ImgAlign(n.parse().ok()?, key.to_owned())
                }
                "imgwidth" => {
                    let (n, w) = arg.trim().split_once(':')?;
                    Step::ImgWidth(n.parse().ok()?, w.parse().ok()?)
                }
                "imgmenu" => Step::ImgMenu(arg.trim().parse().ok()?),
                "imgdrag" => {
                    let (n, line) = arg.trim().split_once(':')?;
                    Step::ImgDrag(n.parse().ok()?, line.parse().ok()?)
                }
                "imgmove" => {
                    let (n, line) = arg.trim().split_once(':')?;
                    Step::ImgMove(n.parse().ok()?, line.parse().ok()?)
                }
                "imgcaption" => {
                    let (n, text) = arg.split_once(':')?;
                    Step::ImgCaption(n.trim().parse().ok()?, unescape(text))
                }
                "theme" => Step::Theme(arg.trim().to_owned()),
                "fold" => Step::Fold(arg.trim().to_owned()),
                "font" => Step::Font(arg.trim().to_owned()),
                "pairing" => Step::Pairing(arg.trim().to_owned()),
                "size" => {
                    let (pane, delta) = arg.trim().split_once(':')?;
                    Step::Size(
                        pane.to_owned(),
                        delta.trim().trim_start_matches('+').parse().ok()?,
                    )
                }
                "docksize" => Step::DockSize(arg.trim().to_owned()),
                "section" => Step::Section(arg.trim().to_owned()),
                "tagmenu" => Step::TagMenu(arg.trim().to_owned()),
                "nav" => Step::Nav(arg.trim().trim_start_matches('+').parse().ok()?),
                "marker" => Step::Marker(arg.trim().to_owned()),
                "measure" => Step::Measure(arg.trim().to_owned()),
                "follow" => Step::Follow(unescape(arg)),
                "icon" => Step::Icon(arg.trim().to_owned()),
                "coffee" => Step::Coffee,
                "togglebox" => {
                    let (l, c) = arg.trim().split_once(':')?;
                    Step::ToggleBox(l.parse().ok()?, c.parse().ok()?)
                }
                "renametag" => {
                    let (old, new) = arg.trim().split_once(':')?;
                    Step::RenameTag(old.to_owned(), new.to_owned())
                }
                "wait" => Step::Wait(arg.trim().parse().ok()?),
                "exit" => Step::Exit,
                other => {
                    tracing::warn!(step = other, "unknown JJB_SCRIPT step");
                    return None;
                }
            };
            Some(step)
        })
        .collect()
}

/// Split on `;` but keep `\;` intact.
fn split_steps(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' && chars.peek() == Some(&';') {
            cur.push_str("\\;");
            chars.next();
        } else if c == ';' {
            out.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
        }
    }
    out.push(cur);
    out
}

fn unescape(s: &str) -> String {
    s.replace("\\n", "\n").replace("\\;", ";")
}

/// Runner state kept in the app model.
#[derive(Debug)]
pub struct Runner {
    steps: std::collections::VecDeque<Step>,
    resume_at: Instant,
}

impl Runner {
    pub fn from_env() -> Option<Self> {
        let script = std::env::var("JJB_SCRIPT").ok()?;
        let steps = parse(&script);
        tracing::info!(count = steps.len(), "JJB_SCRIPT loaded");
        Some(Self {
            steps: steps.into(),
            // Give the window a moment to appear before driving it.
            resume_at: Instant::now() + Duration::from_millis(1200),
        })
    }

    /// Re-queue a step to run on the next tick (used to split multi-line typing).
    pub fn push_front(&mut self, step: Step) {
        self.steps.push_front(step);
    }

    pub fn is_active(&self) -> bool {
        !self.steps.is_empty()
    }

    /// The next step, if its time has come.
    pub fn next(&mut self) -> Option<Step> {
        if Instant::now() < self.resume_at {
            return None;
        }
        let step = self.steps.pop_front()?;
        if let Step::Wait(ms) = step {
            self.resume_at = Instant::now() + Duration::from_millis(ms);
        }
        Some(step)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_steps() {
        let steps = parse(
            "new; type:Hello\\nworld\\; ok ;wait:500;select:2;search:milk;pin;trash;bogus;exit",
        );
        assert_eq!(
            steps,
            vec![
                Step::New,
                Step::Type("Hello\nworld; ok".into()),
                Step::Wait(500),
                Step::Select(2),
                Step::Search("milk".into()),
                Step::Pin,
                Step::Trash,
                Step::Exit,
            ]
        );
    }
}
