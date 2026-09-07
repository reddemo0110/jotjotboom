// SPDX-License-Identifier: GPL-3.0-only

//! Which desktop we are running on, and what to borrow from it.
//!
//! libcosmic assumes COSMIC: it reads fonts, icon theme and header-button
//! preferences from cosmic-config, which on GNOME does not exist, so it
//! falls back to "Open Sans", "Noto Sans Mono" and the "Cosmic" icon theme —
//! none of which a GNOME machine has installed. On GNOME (and anything else
//! that is not COSMIC) we ask the XDG desktop portal for the user's actual
//! settings instead: interface and monospace font, icon theme, and which
//! window buttons the shell shows. Dark/light and the accent colour already
//! arrive through the portal via libcosmic itself.
//!
//! The values are pushed into libcosmic's toolkit config (`COSMIC_TK`).
//! libcosmic replaces that struct wholesale when its own config subscription
//! reports (which it does once at start-up, with defaults, when the file is
//! missing), so [`reassert`] is cheap and idempotent and the app calls it
//! on every view.

use std::collections::HashMap;
use std::sync::OnceLock;

/// The desktop environment the app was launched from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Desktop {
    Cosmic,
    Gnome,
    Other,
}

impl Desktop {
    /// Classify `XDG_CURRENT_DESKTOP` (a colon-separated list, e.g.
    /// `ubuntu:GNOME`, `pop:GNOME`, `COSMIC`, `KDE`).
    pub fn from_xdg(value: &str) -> Desktop {
        let mut names = value.split(':').map(|n| n.trim().to_ascii_lowercase());
        if names.clone().any(|n| n == "cosmic") {
            Desktop::Cosmic
        } else if names.any(|n| n == "gnome" || n.ends_with("-gnome") || n.starts_with("gnome")) {
            Desktop::Gnome
        } else {
            Desktop::Other
        }
    }

    pub fn current() -> Desktop {
        static CURRENT: OnceLock<Desktop> = OnceLock::new();
        *CURRENT.get_or_init(|| {
            std::env::var("XDG_CURRENT_DESKTOP").map_or(Desktop::Other, |v| Desktop::from_xdg(&v))
        })
    }
}

/// What the desktop told us. Every field is optional: a key the portal
/// does not know leaves libcosmic's default in place.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Settings {
    /// Family name of the interface font (Pango description with the size
    /// and style stripped), e.g. `Cantarell`.
    pub interface_font: Option<String>,
    /// Family name of the monospace font, e.g. `Source Code Pro`.
    pub monospace_font: Option<String>,
    /// Icon theme name, e.g. `Adwaita`.
    pub icon_theme: Option<String>,
    /// Whether the shell shows minimize / maximize buttons in title bars
    /// (GNOME's `button-layout`). `None` = unknown, keep the default.
    pub show_minimize: Option<bool>,
    pub show_maximize: Option<bool>,
}

static SETTINGS: OnceLock<Settings> = OnceLock::new();

/// The settings read at start-up (empty on COSMIC, where libcosmic reads
/// its own config).
pub fn settings() -> &'static Settings {
    SETTINGS.get_or_init(|| {
        if Desktop::current() == Desktop::Cosmic {
            return Settings::default();
        }
        let s = read_portal().unwrap_or_else(read_gsettings);
        tracing::info!(desktop = ?Desktop::current(), ?s, "desktop settings");
        s
    })
}

/// Push the desktop's fonts and button layout into libcosmic's toolkit
/// config if they are not there already. Safe to call often.
pub fn reassert() {
    let s = settings();
    if *s == Settings::default() {
        return;
    }
    let tk = cosmic::config::COSMIC_TK.read().unwrap();
    let stale = s
        .interface_font
        .as_ref()
        .is_some_and(|f| tk.interface_font.family != *f)
        || s.monospace_font
            .as_ref()
            .is_some_and(|f| tk.monospace_font.family != *f)
        || s.show_minimize.is_some_and(|b| tk.show_minimize != b)
        || s.show_maximize.is_some_and(|b| tk.show_maximize != b);
    drop(tk);
    if !stale {
        return;
    }
    let mut tk = cosmic::config::COSMIC_TK.write().unwrap();
    if let Some(f) = &s.interface_font {
        tk.interface_font.family.clone_from(f);
    }
    if let Some(f) = &s.monospace_font {
        tk.monospace_font.family.clone_from(f);
    }
    if let Some(b) = s.show_minimize {
        tk.show_minimize = b;
    }
    if let Some(b) = s.show_maximize {
        tk.show_maximize = b;
    }
}

/// Style words a Pango font description may end with, after the family.
const STYLE: &[&str] = &[
    "regular",
    "normal",
    "italic",
    "oblique",
    "bold",
    "semi-bold",
    "semibold",
    "demi-bold",
    "demibold",
    "extra-bold",
    "ultra-bold",
    "light",
    "extra-light",
    "ultra-light",
    "thin",
    "medium",
    "black",
    "heavy",
    "book",
    "condensed",
    "semi-condensed",
    "extra-condensed",
    "ultra-condensed",
    "expanded",
    "semi-expanded",
    "extra-expanded",
    "ultra-expanded",
    "small-caps",
];

/// The family part of a Pango font description: `"Cantarell 11"` →
/// `"Cantarell"`, `"Source Code Pro Semi-Bold Italic 10.5"` →
/// `"Source Code Pro"`. GTK/GNOME settings store fonts this way.
pub fn pango_family(desc: &str) -> Option<String> {
    let desc = desc.trim().trim_matches('\'').trim();
    let mut words: Vec<&str> = desc.split_whitespace().collect();
    // Trailing size, possibly with a `px` suffix.
    if let Some(last) = words.last()
        && last
            .trim_end_matches("px")
            .parse::<f32>()
            .is_ok_and(|n| n > 0.0)
    {
        words.pop();
    }
    // Trailing style words. Pango allows any number of them, in any order.
    while words.len() > 1
        && words
            .last()
            .is_some_and(|w| STYLE.contains(&w.to_ascii_lowercase().as_str()))
    {
        words.pop();
    }
    let family = words.join(" ").trim_end_matches(',').trim().to_owned();
    (!family.is_empty()).then_some(family)
}

/// Which buttons GNOME's `button-layout` shows, as (minimize, maximize).
/// The value looks like `appmenu:minimize,maximize,close` or `:close`.
pub fn button_layout(layout: &str) -> (bool, bool) {
    let has = |name: &str| {
        layout
            .split([':', ','])
            .any(|b| b.trim().eq_ignore_ascii_case(name))
    };
    (has("minimize"), has("maximize"))
}

const INTERFACE_NS: &str = "org.gnome.desktop.interface";
const WM_NS: &str = "org.gnome.desktop.wm.preferences";

fn settings_from(interface: &HashMap<String, String>, wm: &HashMap<String, String>) -> Settings {
    let (show_minimize, show_maximize) = match wm.get("button-layout") {
        Some(layout) => {
            let (min, max) = button_layout(layout);
            (Some(min), Some(max))
        }
        None => (None, None),
    };
    Settings {
        interface_font: interface.get("font-name").and_then(|f| pango_family(f)),
        monospace_font: interface
            .get("monospace-font-name")
            .and_then(|f| pango_family(f)),
        icon_theme: interface
            .get("icon-theme")
            .map(|t| t.trim().trim_matches('\'').to_owned())
            .filter(|t| !t.is_empty()),
        show_minimize,
        show_maximize,
    }
}

/// `org.freedesktop.portal.Settings.ReadAll` for the GNOME namespaces.
/// xdg-desktop-portal-gtk / -gnome expose the gsettings keys one-to-one.
fn read_portal() -> Option<Settings> {
    use zbus::zvariant::OwnedValue;
    let conn = zbus::blocking::Connection::session().ok()?;
    let reply = conn
        .call_method(
            Some("org.freedesktop.portal.Desktop"),
            "/org/freedesktop/portal/desktop",
            Some("org.freedesktop.portal.Settings"),
            "ReadAll",
            &(&[INTERFACE_NS, WM_NS][..],),
        )
        .ok()?;
    let all: HashMap<String, HashMap<String, OwnedValue>> = reply.body().deserialize().ok()?;
    let strings = |ns: &str| -> HashMap<String, String> {
        all.get(ns)
            .map(|m| {
                m.iter()
                    .filter_map(|(k, v)| {
                        <&str>::try_from(&**v)
                            .ok()
                            .map(|s| (k.clone(), s.to_owned()))
                    })
                    .collect()
            })
            .unwrap_or_default()
    };
    let interface = strings(INTERFACE_NS);
    if interface.is_empty() {
        // The portal answered but does not serve GNOME's namespaces (KDE,
        // a bare compositor): nothing to borrow.
        return None;
    }
    Some(settings_from(&interface, &strings(WM_NS)))
}

/// Fallback when there is no portal on the bus: the `gsettings` tool.
fn read_gsettings() -> Settings {
    let get = |schema: &str, key: &str| -> Option<String> {
        let out = std::process::Command::new("gsettings")
            .args(["get", schema, key])
            .output()
            .ok()?;
        out.status
            .success()
            .then(|| String::from_utf8_lossy(&out.stdout).trim().to_owned())
    };
    let mut interface = HashMap::new();
    for key in ["font-name", "monospace-font-name", "icon-theme"] {
        if let Some(v) = get(INTERFACE_NS, key) {
            interface.insert(key.to_owned(), v);
        }
    }
    let mut wm = HashMap::new();
    if let Some(v) = get(WM_NS, "button-layout") {
        wm.insert("button-layout".to_owned(), v);
    }
    settings_from(&interface, &wm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_desktops() {
        assert_eq!(Desktop::from_xdg("COSMIC"), Desktop::Cosmic);
        assert_eq!(Desktop::from_xdg("GNOME"), Desktop::Gnome);
        assert_eq!(Desktop::from_xdg("ubuntu:GNOME"), Desktop::Gnome);
        assert_eq!(Desktop::from_xdg("pop:GNOME"), Desktop::Gnome);
        assert_eq!(Desktop::from_xdg("GNOME-Classic:GNOME"), Desktop::Gnome);
        assert_eq!(Desktop::from_xdg("KDE"), Desktop::Other);
        assert_eq!(Desktop::from_xdg(""), Desktop::Other);
    }

    #[test]
    fn strips_pango_size_and_style() {
        assert_eq!(pango_family("Cantarell 11").as_deref(), Some("Cantarell"));
        assert_eq!(pango_family("'Cantarell 11'").as_deref(), Some("Cantarell"));
        assert_eq!(
            pango_family("Source Code Pro 10").as_deref(),
            Some("Source Code Pro")
        );
        assert_eq!(
            pango_family("Adwaita Sans Semi-Bold Italic 10.5").as_deref(),
            Some("Adwaita Sans")
        );
        assert_eq!(pango_family("Noto Sans 12px").as_deref(), Some("Noto Sans"));
        assert_eq!(pango_family("Monospace").as_deref(), Some("Monospace"));
        // A family that is itself a style word survives.
        assert_eq!(pango_family("Light 12").as_deref(), Some("Light"));
        assert_eq!(pango_family("   "), None);
    }

    #[test]
    fn reads_button_layout() {
        assert_eq!(
            button_layout("appmenu:minimize,maximize,close"),
            (true, true)
        );
        assert_eq!(button_layout(":close"), (false, false));
        assert_eq!(button_layout("close,maximize:"), (false, true));
        assert_eq!(button_layout("icon:minimize,close"), (true, false));
    }

    #[test]
    fn builds_settings_from_keys() {
        let mut interface = HashMap::new();
        interface.insert("font-name".to_owned(), "'Cantarell 11'".to_owned());
        interface.insert("icon-theme".to_owned(), "'Adwaita'".to_owned());
        let mut wm = HashMap::new();
        wm.insert("button-layout".to_owned(), "'appmenu:close'".to_owned());
        let s = settings_from(&interface, &wm);
        assert_eq!(s.interface_font.as_deref(), Some("Cantarell"));
        assert_eq!(s.monospace_font, None);
        assert_eq!(s.icon_theme.as_deref(), Some("Adwaita"));
        assert_eq!(s.show_minimize, Some(false));
        assert_eq!(s.show_maximize, Some(false));
    }
}
