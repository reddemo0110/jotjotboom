// SPDX-License-Identifier: GPL-3.0-only

//! The app icon: "dot dot hash" — two jots and a `#` — on a macOS-grid
//! squircle tile, generated from a theme palette so the launcher icon can
//! match the colour option in use. The geometry (tile at 80 % of the canvas,
//! marks on the golden ratio) is baked into `TEMPLATE`; see DECISIONS.md.

use crate::retro::Palette;
use anyhow::{Context, Result};
use cosmic::iced::Color;
use std::path::PathBuf;

const TEMPLATE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="64" height="64"><defs><linearGradient id="g" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="TOP"/><stop offset="1" stop-color="BOTTOM"/></linearGradient></defs><path d="M57.75,32.00L57.70,42.17L57.55,45.39L57.30,47.70L56.95,49.54L56.49,51.06L55.92,52.36L55.23,53.46L54.42,54.42L53.46,55.23L52.36,55.92L51.06,56.49L49.54,56.95L47.70,57.30L45.39,57.55L42.17,57.70L32.00,57.75L21.83,57.70L18.61,57.55L16.30,57.30L14.46,56.95L12.94,56.49L11.64,55.92L10.54,55.23L9.58,54.42L8.77,53.46L8.08,52.36L7.51,51.06L7.05,49.54L6.70,47.70L6.45,45.39L6.30,42.17L6.25,32.00L6.30,21.83L6.45,18.61L6.70,16.30L7.05,14.46L7.51,12.94L8.08,11.64L8.77,10.54L9.58,9.58L10.54,8.77L11.64,8.08L12.94,7.51L14.46,7.05L16.30,6.70L18.61,6.45L21.83,6.30L32.00,6.25L42.17,6.30L45.39,6.45L47.70,6.70L49.54,7.05L51.06,7.51L52.36,8.08L53.46,8.77L54.42,9.58L55.23,10.54L55.92,11.64L56.49,12.94L56.95,14.46L57.30,16.30L57.55,18.61L57.70,21.83Z" fill="url(#g)"/><circle cx="16.22" cy="38.21" r="3.9" fill="MARK"/><circle cx="26.43" cy="38.21" r="3.9" fill="MARK"/><g transform="translate(42.96 34.33) skewX(-9) translate(-42.96 -34.33)"><rect x="36.83" y="24.12" width="4.48" height="20.42" rx="2.24" fill="MARK"/><rect x="44.59" y="24.12" width="4.48" height="20.42" rx="2.24" fill="MARK"/><rect x="32.95" y="28.21" width="20.01" height="4.48" rx="2.24" fill="MARK"/><rect x="32.95" y="35.97" width="20.01" height="4.48" rx="2.24" fill="MARK"/></g></svg>"#;

fn hex(c: Color) -> String {
    format!(
        "#{:02x}{:02x}{:02x}",
        (c.r * 255.0).round() as u8,
        (c.g * 255.0).round() as u8,
        (c.b * 255.0).round() as u8
    )
}

fn mix(a: Color, b: Color, t: f32) -> Color {
    Color::from_rgb(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
    )
}

/// The icon for a palette: the tile is the theme's background (a touch
/// lighter at the top so it reads as a tile, not a hole) and the marks are
/// the note-writing colour — the icon looks like the note pane it opens.
pub fn svg(p: &Palette) -> String {
    let top = mix(p.bg, p.fg, 0.10);
    TEMPLATE
        .replace("TOP", &hex(top))
        .replace("BOTTOM", &hex(p.bg))
        .replace("MARK", &hex(p.fg))
}

/// Where the launcher looks for our icon (per-user hicolor theme).
pub fn installed_path(app_id: &str) -> Option<PathBuf> {
    Some(
        dirs::data_dir()?
            .join("icons/hicolor/scalable/apps")
            .join(format!("{app_id}.svg")),
    )
}

/// Write `svg` as the launcher icon if it differs from what is there, and
/// nudge the icon cache so the dock and app library pick it up.
pub fn install(app_id: &str, svg: &str) -> Result<bool> {
    let Some(path) = installed_path(app_id) else {
        anyhow::bail!("no data dir");
    };
    if std::fs::read_to_string(&path).is_ok_and(|cur| cur == svg) {
        return Ok(false);
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    }
    std::fs::write(&path, svg).with_context(|| format!("writing {}", path.display()))?;
    if let Some(hicolor) = path.ancestors().nth(3) {
        let _ = std::process::Command::new("gtk-update-icon-cache")
            .args(["-q", "-t", "-f"])
            .arg(hicolor)
            .status();
    }
    // Touching the applications dir makes COSMIC re-read launcher entries.
    if let Some(apps) = dirs::data_dir().map(|d| d.join("applications")) {
        let _ = filetime_touch(&apps);
    }
    Ok(true)
}

fn filetime_touch(dir: &std::path::Path) -> std::io::Result<()> {
    let marker = dir.join(".jotjotboom-icon");
    std::fs::write(&marker, b"")?;
    std::fs::remove_file(&marker)
}
