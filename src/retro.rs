// SPDX-License-Identifier: GPL-3.0-only

//! The retro look: btop-style frames, phosphor/amber/WordPerfect palettes,
//! and the style helpers the views use. The COSMIC header bar is left alone;
//! everything below it is drawn with these.

use cosmic::iced::font::{Family, Weight};
use cosmic::iced::{Alignment, Background, Border, Color, Font, Length};
use cosmic::widget::{self, text_input};
use cosmic::{Element, theme};

/// Text typed against the COSMIC theme (the bare `widget::Text<'a>` alias defaults to iced's).
pub type Text<'a> = cosmic::iced::widget::Text<'a, cosmic::Theme, cosmic::Renderer>;

/// Bundled title face (SIL OFL, see resources/fonts/VT323-OFL.txt).
pub const TITLE_FONT_BYTES: &[u8] = include_bytes!("../resources/fonts/VT323-Regular.ttf");
pub const TITLE_FONT: Font = Font {
    family: Family::Name("VT323"),
    weight: Weight::Normal,
    ..Font::DEFAULT
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    /// P1 green phosphor, the classic terminal.
    #[default]
    Phosphor,
    /// P3 amber monochrome monitor.
    Amber,
    /// White-on-blue word processor.
    WordPerfect,
    /// P4 white phosphor — paper-white on black.
    Paper,
    /// Orange gas-plasma display (GRiD Compass, early Toshiba laptops).
    Plasma,
    /// Commodore 64 light-blue on blue.
    C64,
    /// Original Game Boy LCD: four greens, the only light theme.
    GameBoy,
    /// 80s synthwave: pink and cyan on deep purple.
    Synthwave,
    /// Plain white with #0c0c0c ink — the light one for daylight.
    White,
    /// After "Giant Goldfish" on COLOURlovers.
    Goldfish,
    /// Giant Goldfish with the tangerine turned down.
    GoldfishMuted,
    /// After "Thought Provoking" on COLOURlovers.
    Provoking,
    /// After "Cheer Up Emo Kid" on COLOURlovers.
    EmoKid,
    /// After "Ocean Five" on COLOURlovers.
    OceanFive,
    /// After "Adrift in Dreams" on COLOURlovers.
    Adrift,
    /// Follows the COSMIC system theme; frames only.
    Cosmic,
}

impl Theme {
    pub const ALL: [Theme; 16] = [
        Theme::Phosphor,
        Theme::Amber,
        Theme::WordPerfect,
        Theme::Paper,
        Theme::White,
        Theme::Plasma,
        Theme::C64,
        Theme::GameBoy,
        Theme::Synthwave,
        Theme::Goldfish,
        Theme::GoldfishMuted,
        Theme::Provoking,
        Theme::EmoKid,
        Theme::OceanFive,
        Theme::Adrift,
        Theme::Cosmic,
    ];

    pub fn key(self) -> &'static str {
        match self {
            Theme::Phosphor => "phosphor",
            Theme::Amber => "amber",
            Theme::WordPerfect => "wordperfect",
            Theme::Paper => "paper",
            Theme::Plasma => "plasma",
            Theme::C64 => "c64",
            Theme::GameBoy => "gameboy",
            Theme::Synthwave => "synthwave",
            Theme::White => "white",
            Theme::Goldfish => "goldfish",
            Theme::GoldfishMuted => "goldfish-muted",
            Theme::Provoking => "provoking",
            Theme::EmoKid => "emokid",
            Theme::OceanFive => "oceanfive",
            Theme::Adrift => "adrift",
            Theme::Cosmic => "cosmic",
        }
    }

    pub fn from_key(key: &str) -> Theme {
        Theme::ALL
            .into_iter()
            .find(|t| t.key() == key)
            .unwrap_or_default()
    }

    pub fn label(self) -> &'static str {
        match self {
            Theme::Phosphor => "The Terminalator",
            Theme::Amber => "Amber Dawn",
            Theme::WordPerfect => "Big Trouble in Little Blue",
            Theme::Paper => "Monochromando",
            Theme::Plasma => "Lethal Plasma",
            Theme::C64 => "Escape from C64",
            Theme::GameBoy => "Handhelder",
            Theme::Synthwave => "Streets of Neon",
            Theme::White => "Ghostwriters",
            Theme::Goldfish => "Big Fish in Little China",
            Theme::GoldfishMuted => "Little Fish in Big China",
            Theme::Provoking => "Blade Thinker",
            Theme::EmoKid => "Cheer Up, Karate Kid",
            Theme::OceanFive => "The Abyss Five",
            Theme::Adrift => "Adrift in Dreamscape",
            Theme::Cosmic => "Desktop Cop",
        }
    }

    /// One-line flavour text for the picker.
    pub fn blurb(self) -> &'static str {
        match self {
            Theme::Phosphor => "P1 green phosphor. It'll be back.",
            Theme::Amber => "P3 amber monochrome. Wolverines!",
            Theme::WordPerfect => "white on WordPerfect blue, F-keys optional",
            Theme::Paper => "P4 white phosphor. Let off some steam.",
            Theme::Plasma => "orange gas-plasma. Too old for this bit depth.",
            Theme::C64 => "load \"*\",8,1 — call me Snake",
            Theme::GameBoy => "four LCD greens. There can be only one screen.",
            Theme::Synthwave => "pink and cyan, 1984 forever",
            Theme::White => "plain white, #0c0c0c ink. Who you gonna call?",
            Theme::Goldfish => "COLOURlovers 'Giant Goldfish' — teal, tangerine, cream",
            Theme::GoldfishMuted => "Giant Goldfish with the tangerine turned down",
            Theme::Provoking => "COLOURlovers 'Thought Provoking' — plum, brick, mustard",
            Theme::EmoKid => "COLOURlovers 'Cheer Up Emo Kid' — slate, mint, lime",
            Theme::OceanFive => "COLOURlovers 'Ocean Five' — deep sea, coral, sand",
            Theme::Adrift => "COLOURlovers 'Adrift in Dreams' — sea greens on ink blue",
            Theme::Cosmic => "follows your desktop theme. Serve the public trust.",
        }
    }

    pub fn palette(self, cosmic: &cosmic::Theme) -> Palette {
        match self {
            Theme::Phosphor => Palette {
                bg: hex(0x050806),
                panel: hex(0x070b08),
                fg: hex(0xb9f2bf),
                dim: hex(0x57a36a),
                mute: hex(0x264a2e),
                accent: hex(0x4dff8f),
                accent2: hex(0x9ad3ff),
                border: hex(0x1f5a2f),
                sel: hex(0x0f3a1c),
                selfg: hex(0xd7ffe0),
            },
            Theme::Amber => Palette {
                bg: hex(0x0b0704),
                panel: hex(0x0e0905),
                fg: hex(0xffc978),
                dim: hex(0xb3772c),
                mute: hex(0x4a3010),
                accent: hex(0xffb02e),
                accent2: hex(0xffe9a8),
                border: hex(0x5c3c12),
                sel: hex(0x3a2408),
                selfg: hex(0xfff1d6),
            },
            Theme::WordPerfect => Palette {
                bg: hex(0x1c2874),
                panel: hex(0x1c2874),
                fg: hex(0xe6e8ee),
                dim: hex(0x9ea7d8),
                mute: hex(0x4f5ca8),
                accent: hex(0xf0e07c),
                accent2: hex(0x85d4da),
                border: hex(0x7c86c6),
                sel: hex(0x2f3d94),
                selfg: hex(0xffffff),
            },
            Theme::Paper => Palette {
                bg: hex(0x0a0a0b),
                panel: hex(0x0f0f11),
                fg: hex(0xe8e8e8),
                dim: hex(0x8c8c8c),
                mute: hex(0x4c4c4c),
                accent: hex(0xffffff),
                accent2: hex(0xc9d3ff),
                border: hex(0x3c3c3c),
                sel: hex(0x2c2c30),
                selfg: hex(0xffffff),
            },
            Theme::Plasma => Palette {
                bg: hex(0x180500),
                panel: hex(0x1f0700),
                fg: hex(0xff8f3c),
                dim: hex(0xcb682a),
                mute: hex(0x5e2c0e),
                accent: hex(0xffb66b),
                accent2: hex(0xffdcbb),
                border: hex(0x6e3414),
                sel: hex(0x4c2009),
                selfg: hex(0xffe6cf),
            },
            Theme::C64 => Palette {
                bg: hex(0x443c86),
                panel: hex(0x443c86),
                fg: hex(0xbdb8ee),
                dim: hex(0x9c96d8),
                mute: hex(0x655cb0),
                accent: hex(0xd0d68e),
                accent2: hex(0x9cd29a),
                border: hex(0x7c75c0),
                sel: hex(0x5a52a8),
                selfg: hex(0xffffff),
            },
            Theme::GameBoy => Palette {
                // The real DMG screen: muted olive, not emulator lime.
                bg: hex(0xa9b58a),
                panel: hex(0xa1ad82),
                fg: hex(0x1f261a),
                dim: hex(0x4d5a3c),
                mute: hex(0x7d8a63),
                accent: hex(0x2c3a24),
                accent2: hex(0x445236),
                border: hex(0x6e7a58),
                sel: hex(0x6e7a58),
                selfg: hex(0xdbe3c4),
            },
            Theme::Synthwave => Palette {
                bg: hex(0x120823),
                panel: hex(0x190c2f),
                fg: hex(0xf2dcff),
                dim: hex(0x8f72b8),
                mute: hex(0x4d3778),
                accent: hex(0xff6ec7),
                accent2: hex(0x00e5ff),
                border: hex(0x5e3d94),
                sel: hex(0x3b2063),
                selfg: hex(0xffffff),
            },
            Theme::White => Palette {
                bg: hex(0xffffff),
                panel: hex(0xfbfbfb),
                fg: hex(0x0c0c0c),
                dim: hex(0x5c5c5c),
                mute: hex(0xb4b4b4),
                accent: hex(0x0c0c0c),
                accent2: hex(0x3b5b8c),
                border: hex(0xd2d2d2),
                sel: hex(0xe8e8e8),
                selfg: hex(0x0c0c0c),
            },
            Theme::Goldfish => Palette {
                bg: hex(0x0f1a1e),
                panel: hex(0x13222a),
                fg: hex(0xe0e4cc),
                dim: hex(0x8fbcbc),
                mute: hex(0x2f4d56),
                accent: hex(0xfa6900),
                accent2: hex(0x69d2e7),
                border: hex(0x2f5260),
                sel: hex(0x1f3d48),
                selfg: hex(0xffffff),
            },
            Theme::GoldfishMuted => Palette {
                bg: hex(0x0f1a1e),
                panel: hex(0x13222a),
                fg: hex(0xe0e4cc),
                dim: hex(0x8fbcbc),
                mute: hex(0x2f4d56),
                accent: hex(0xc9773f),
                accent2: hex(0x69d2e7),
                border: hex(0x2f5260),
                sel: hex(0x1f3d48),
                selfg: hex(0xffffff),
            },
            Theme::Provoking => Palette {
                bg: hex(0x2a1520),
                panel: hex(0x311a27),
                fg: hex(0xf0dcb0),
                dim: hex(0xb08a6a),
                mute: hex(0x5e3a4a),
                accent: hex(0xd95b43),
                accent2: hex(0x8ab4b8),
                border: hex(0x6d3f52),
                sel: hex(0x542437),
                selfg: hex(0xffffff),
            },
            Theme::EmoKid => Palette {
                bg: hex(0x1e252b),
                panel: hex(0x242c33),
                fg: hex(0xe6edf0),
                dim: hex(0x8f9ea9),
                mute: hex(0x3d4a54),
                accent: hex(0x4ecdc4),
                accent2: hex(0xc7f464),
                border: hex(0x455360),
                sel: hex(0x323e49),
                selfg: hex(0xffffff),
            },
            Theme::OceanFive => Palette {
                bg: hex(0x0d1b1e),
                panel: hex(0x122226),
                fg: hex(0xf2e8d0),
                dim: hex(0xa39a7c),
                mute: hex(0x31494e),
                accent: hex(0xeb6841),
                accent2: hex(0x00a0b0),
                border: hex(0x2f4f55),
                sel: hex(0x1e3d43),
                selfg: hex(0xffffff),
            },
            Theme::Adrift => Palette {
                bg: hex(0x081f2d),
                panel: hex(0x0c2737),
                fg: hex(0xdfe8d8),
                dim: hex(0x79a99a),
                mute: hex(0x244d5c),
                accent: hex(0xcff09e),
                accent2: hex(0x79bd9a),
                border: hex(0x2a5d6d),
                sel: hex(0x1a4a5c),
                selfg: hex(0xffffff),
            },
            Theme::Cosmic => {
                let c = cosmic.cosmic();
                let container = c.background(false);
                let bg: Color = container.base.into();
                let fg: Color = container.on.into();
                let accent: Color = c.accent_color().into();
                Palette {
                    bg,
                    panel: container.component.base.into(),
                    fg,
                    dim: fg.scale_alpha(0.55),
                    mute: fg.scale_alpha(0.3),
                    accent,
                    accent2: accent,
                    border: container.component.divider.into(),
                    sel: accent,
                    selfg: c.on_accent_color().into(),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    pub bg: Color,
    pub panel: Color,
    pub fg: Color,
    pub dim: Color,
    pub mute: Color,
    pub accent: Color,
    pub accent2: Color,
    pub border: Color,
    pub sel: Color,
    pub selfg: Color,
}

fn hex(rgb: u32) -> Color {
    Color::from_rgb8(
        ((rgb >> 16) & 0xff) as u8,
        ((rgb >> 8) & 0xff) as u8,
        (rgb & 0xff) as u8,
    )
}

pub fn mono() -> Font {
    cosmic::font::mono()
}

/// Bundled editor fonts (all OFL/UFL, files under `resources/fonts`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditorFont {
    #[default]
    System,
    Plex,
    Fira,
    Ubuntu,
    Anonymous,
    Space,
    Courier,
    B612,
    Vt323,
    PlexSerif,
    Lato,
    PtSans,
    PtSerif,
    Atkinson,
    UbuntuSans,
    Spectral,
    DmSerif,
    SpecialElite,
}

macro_rules! font_files {
    ($($path:literal),* $(,)?) => { &[$(include_bytes!(concat!("../resources/fonts/", $path))),*] };
}

impl EditorFont {
    pub const ALL: [EditorFont; 18] = [
        EditorFont::System,
        EditorFont::Plex,
        EditorFont::Fira,
        EditorFont::Ubuntu,
        EditorFont::Anonymous,
        EditorFont::Space,
        EditorFont::Courier,
        EditorFont::B612,
        EditorFont::Vt323,
        EditorFont::PlexSerif,
        EditorFont::Lato,
        EditorFont::PtSans,
        EditorFont::PtSerif,
        EditorFont::Atkinson,
        EditorFont::UbuntuSans,
        EditorFont::Spectral,
        EditorFont::DmSerif,
        EditorFont::SpecialElite,
    ];

    pub fn key(self) -> &'static str {
        match self {
            EditorFont::System => "system",
            EditorFont::Plex => "plex",
            EditorFont::Fira => "fira",
            EditorFont::Ubuntu => "ubuntu",
            EditorFont::Anonymous => "anonymous",
            EditorFont::Space => "space",
            EditorFont::Courier => "courier",
            EditorFont::B612 => "b612",
            EditorFont::Vt323 => "vt323",
            EditorFont::PlexSerif => "plexserif",
            EditorFont::Lato => "lato",
            EditorFont::PtSans => "ptsans",
            EditorFont::PtSerif => "ptserif",
            EditorFont::Atkinson => "atkinson",
            EditorFont::UbuntuSans => "ubuntusans",
            EditorFont::Spectral => "spectral",
            EditorFont::DmSerif => "dmserif",
            EditorFont::SpecialElite => "specialelite",
        }
    }

    pub fn from_key(key: &str) -> EditorFont {
        EditorFont::ALL
            .into_iter()
            .find(|f| f.key() == key)
            .unwrap_or_default()
    }

    pub fn label(self) -> &'static str {
        match self {
            EditorFont::System => "System monospace",
            EditorFont::Plex => "IBM Plex Mono",
            EditorFont::Fira => "Fira Mono",
            EditorFont::Ubuntu => "Ubuntu Mono",
            EditorFont::Anonymous => "Anonymous Pro",
            EditorFont::Space => "Space Mono",
            EditorFont::Courier => "Courier Prime",
            EditorFont::B612 => "B612 Mono",
            EditorFont::Vt323 => "VT323",
            EditorFont::PlexSerif => "IBM Plex Serif",
            EditorFont::Lato => "Lato",
            EditorFont::PtSans => "PT Sans",
            EditorFont::PtSerif => "PT Serif",
            EditorFont::Atkinson => "Atkinson Hyperlegible",
            EditorFont::UbuntuSans => "Ubuntu",
            EditorFont::Spectral => "Spectral",
            EditorFont::DmSerif => "DM Serif Display",
            EditorFont::SpecialElite => "Special Elite",
        }
    }

    pub fn blurb(self) -> &'static str {
        match self {
            EditorFont::System => "whatever your desktop uses",
            EditorFont::Plex => "IBM's clean workhorse",
            EditorFont::Fira => "Mozilla's, wide and friendly",
            EditorFont::Ubuntu => "narrow, humanist, a bit cheeky",
            EditorFont::Anonymous => "classic terminal, sharp serifs",
            EditorFont::Space => "Colophon's geometric oddball",
            EditorFont::Courier => "the typewriter, done properly",
            EditorFont::B612 => "Airbus cockpit font, built to be read",
            EditorFont::Vt323 => "the title font, pixel terminal",
            EditorFont::PlexSerif => "IBM's serif, made for screens",
            EditorFont::Lato => "warm, friendly humanist sans",
            EditorFont::PtSans => "ParaType's public sans",
            EditorFont::PtSerif => "ParaType's public serif",
            EditorFont::Atkinson => "Braille Institute's low-vision face",
            EditorFont::UbuntuSans => "the Ubuntu sans",
            EditorFont::Spectral => "a serif drawn for reading on screens",
            EditorFont::DmSerif => "high-contrast display serif",
            EditorFont::SpecialElite => "a well-used typewriter",
        }
    }

    /// Faces that only ship a Regular; bold is synthesised by the renderer's
    /// nearest match, so titles in them are set at their normal weight.
    pub fn has_bold(self) -> bool {
        !matches!(
            self,
            EditorFont::Vt323 | EditorFont::DmSerif | EditorFont::SpecialElite
        )
    }

    /// The font to hand to widgets. Bundled faces are loaded at start-up
    /// (see [`EDITOR_FONT_FILES`]); until then cosmic-text falls back.
    pub fn font(self) -> Font {
        match self {
            EditorFont::System => mono(),
            EditorFont::Vt323 => TITLE_FONT,
            _ => Font {
                family: Family::Name(self.label()),
                ..Font::DEFAULT
            },
        }
    }
}

/// Every bundled face that must be registered before the editor uses it.
pub const EDITOR_FONT_FILES: &[&[u8]] = font_files![
    "ofl/ibmplexmono/IBMPlexMono-Regular.ttf",
    "ofl/ibmplexmono/IBMPlexMono-Bold.ttf",
    "ofl/ibmplexmono/IBMPlexMono-Italic.ttf",
    "ofl/ibmplexmono/IBMPlexMono-BoldItalic.ttf",
    "ofl/firamono/FiraMono-Regular.ttf",
    "ofl/firamono/FiraMono-Bold.ttf",
    "ufl/ubuntumono/UbuntuMono-Regular.ttf",
    "ufl/ubuntumono/UbuntuMono-Bold.ttf",
    "ufl/ubuntumono/UbuntuMono-Italic.ttf",
    "ufl/ubuntumono/UbuntuMono-BoldItalic.ttf",
    "ofl/anonymouspro/AnonymousPro-Regular.ttf",
    "ofl/anonymouspro/AnonymousPro-Bold.ttf",
    "ofl/anonymouspro/AnonymousPro-Italic.ttf",
    "ofl/anonymouspro/AnonymousPro-BoldItalic.ttf",
    "ofl/spacemono/SpaceMono-Regular.ttf",
    "ofl/spacemono/SpaceMono-Bold.ttf",
    "ofl/spacemono/SpaceMono-Italic.ttf",
    "ofl/spacemono/SpaceMono-BoldItalic.ttf",
    "ofl/courierprime/CourierPrime-Regular.ttf",
    "ofl/courierprime/CourierPrime-Bold.ttf",
    "ofl/courierprime/CourierPrime-Italic.ttf",
    "ofl/courierprime/CourierPrime-BoldItalic.ttf",
    "ofl/b612mono/B612Mono-Regular.ttf",
    "ofl/b612mono/B612Mono-Bold.ttf",
    "ofl/b612mono/B612Mono-Italic.ttf",
    "ofl/b612mono/B612Mono-BoldItalic.ttf",
    "ofl/ibmplexserif/IBMPlexSerif-Regular.ttf",
    "ofl/ibmplexserif/IBMPlexSerif-Bold.ttf",
    "ofl/ibmplexserif/IBMPlexSerif-Italic.ttf",
    "ofl/ibmplexserif/IBMPlexSerif-BoldItalic.ttf",
    "ofl/lato/Lato-Regular.ttf",
    "ofl/lato/Lato-Bold.ttf",
    "ofl/lato/Lato-Italic.ttf",
    "ofl/lato/Lato-BoldItalic.ttf",
    "ofl/ptsans/PT_Sans-Web-Regular.ttf",
    "ofl/ptsans/PT_Sans-Web-Bold.ttf",
    "ofl/ptsans/PT_Sans-Web-Italic.ttf",
    "ofl/ptsans/PT_Sans-Web-BoldItalic.ttf",
    "ofl/ptserif/PT_Serif-Web-Regular.ttf",
    "ofl/ptserif/PT_Serif-Web-Bold.ttf",
    "ofl/ptserif/PT_Serif-Web-Italic.ttf",
    "ofl/ptserif/PT_Serif-Web-BoldItalic.ttf",
    "ofl/atkinsonhyperlegible/AtkinsonHyperlegible-Regular.ttf",
    "ofl/atkinsonhyperlegible/AtkinsonHyperlegible-Bold.ttf",
    "ofl/atkinsonhyperlegible/AtkinsonHyperlegible-Italic.ttf",
    "ofl/atkinsonhyperlegible/AtkinsonHyperlegible-BoldItalic.ttf",
    "ufl/ubuntu/Ubuntu-Regular.ttf",
    "ufl/ubuntu/Ubuntu-Bold.ttf",
    "ufl/ubuntu/Ubuntu-Italic.ttf",
    "ufl/ubuntu/Ubuntu-BoldItalic.ttf",
    "ofl/spectral/Spectral-Regular.ttf",
    "ofl/spectral/Spectral-Bold.ttf",
    "ofl/spectral/Spectral-Italic.ttf",
    "ofl/spectral/Spectral-BoldItalic.ttf",
    "ofl/dmserifdisplay/DMSerifDisplay-Regular.ttf",
    "ofl/dmserifdisplay/DMSerifDisplay-Italic.ttf",
    "apache/specialelite/SpecialElite-Regular.ttf",
];

/// A designer pairing: one face for pane titles, one for the sidebar and
/// list, one for the note. Sourced from the usual pairing guides — a
/// display or serif over a calm sans, or one superfamily throughout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pairing {
    pub key: &'static str,
    pub name: &'static str,
    pub blurb: &'static str,
    pub title: EditorFont,
    pub ui: EditorFont,
    pub body: EditorFont,
}

pub const PAIRINGS: [Pairing; 8] = [
    Pairing {
        key: "jotjotboom",
        name: "JotJotBoom",
        blurb: "the default — VT323 titles, your system monospace everywhere else",
        title: EditorFont::Vt323,
        ui: EditorFont::System,
        body: EditorFont::System,
    },
    Pairing {
        key: "plex",
        name: "Plex",
        blurb: "IBM's superfamily: Plex Serif titles and notes, Plex Mono for the chrome",
        title: EditorFont::PlexSerif,
        ui: EditorFont::Plex,
        body: EditorFont::PlexSerif,
    },
    Pairing {
        key: "editorial",
        name: "Editorial",
        blurb: "Spectral, a screen serif, over Lato — the classic serif-body / sans-UI split",
        title: EditorFont::Spectral,
        ui: EditorFont::Lato,
        body: EditorFont::Spectral,
    },
    Pairing {
        key: "magazine",
        name: "Magazine",
        blurb: "DM Serif Display headlines, Lato for everything you read and click",
        title: EditorFont::DmSerif,
        ui: EditorFont::Lato,
        body: EditorFont::Lato,
    },
    Pairing {
        key: "paratype",
        name: "ParaType",
        blurb: "PT Serif and PT Sans, drawn together to be used together",
        title: EditorFont::PtSerif,
        ui: EditorFont::PtSans,
        body: EditorFont::PtSerif,
    },
    Pairing {
        key: "hyperlegible",
        name: "Hyperlegible",
        blurb: "Atkinson Hyperlegible throughout — built for low vision, kind to everyone",
        title: EditorFont::Atkinson,
        ui: EditorFont::Atkinson,
        body: EditorFont::Atkinson,
    },
    Pairing {
        key: "ubuntu",
        name: "Ubuntu",
        blurb: "the Ubuntu family: humanist sans for the chrome, Ubuntu Mono for the note",
        title: EditorFont::UbuntuSans,
        ui: EditorFont::UbuntuSans,
        body: EditorFont::Ubuntu,
    },
    Pairing {
        key: "typewriter",
        name: "Typewriter",
        blurb: "Special Elite titles over Courier Prime — a manuscript in progress",
        title: EditorFont::SpecialElite,
        ui: EditorFont::Courier,
        body: EditorFont::Courier,
    },
];

impl Pairing {
    pub fn from_key(key: &str) -> Option<&'static Pairing> {
        PAIRINGS.iter().find(|p| p.key == key)
    }

    pub fn default_pairing() -> &'static Pairing {
        &PAIRINGS[0]
    }
}

/// How wide the note text may get before the column centres itself with
/// margins (Craft style). Narrower windows always wrap to fit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Measure {
    Narrow,
    #[default]
    Medium,
    Wide,
    Full,
}

impl Measure {
    pub const ALL: [Measure; 4] = [
        Measure::Narrow,
        Measure::Medium,
        Measure::Wide,
        Measure::Full,
    ];

    pub fn key(self) -> &'static str {
        match self {
            Measure::Narrow => "narrow",
            Measure::Medium => "medium",
            Measure::Wide => "wide",
            Measure::Full => "full",
        }
    }

    pub fn from_key(key: &str) -> Measure {
        Measure::ALL
            .into_iter()
            .find(|m| m.key() == key)
            .unwrap_or_default()
    }

    /// One fish per notch: the wider the water, the more fish.
    pub fn label(self) -> &'static str {
        match self {
            Measure::Narrow => "🐟",
            Measure::Medium => "🐟🐟",
            Measure::Wide => "🐟🐟🐟",
            Measure::Full => "🐟🐟🐟🐟",
        }
    }

    pub fn blurb(self) -> &'static str {
        match self {
            Measure::Narrow => "narrow — a paperback",
            Measure::Medium => "medium — a letter",
            Measure::Wide => "wide — a broadsheet",
            Measure::Full => "full — the whole pane",
        }
    }

    /// Maximum text column width in px (`None` = no limit).
    pub fn max_width(self) -> Option<f32> {
        match self {
            Measure::Narrow => Some(560.0),
            Measure::Medium => Some(720.0),
            Measure::Wide => Some(920.0),
            Measure::Full => None,
        }
    }
}

/// Marks offered for a finished task (`- [x]`, `- [✓]`, `- [🦆]`, …).
pub const TASK_MARKERS: [(&str, &str); 14] = [
    ("x", "the classic"),
    ("✓", "tick"),
    ("✔", "heavy tick"),
    ("–", "dash"),
    ("•", "dot"),
    ("★", "star"),
    ("🦆", "duck"),
    ("🔥", "on fire"),
    ("💀", "dead to me"),
    ("🍕", "pizza'd"),
    ("🐈", "cat approved"),
    ("🚀", "shipped"),
    ("👍", "thumbs up"),
    ("🍺", "beer o'clock"),
];

/// Editor text size limits (px).
pub const FONT_SIZE_DEFAULT: u16 = 15;
pub const FONT_SIZE_MIN: u16 = 10;
pub const FONT_SIZE_MAX: u16 = 48;
/// Sidebar / notes-list text size limits (px).
pub const PANE_SIZE_DEFAULT: u16 = 13;
pub const PANE_SIZE_MIN: u16 = 9;
pub const PANE_SIZE_MAX: u16 = 30;

/// How big the dock pill and its buttons are drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DockSize {
    Small,
    #[default]
    Medium,
    Large,
    Wow,
}

impl DockSize {
    pub const ALL: [DockSize; 4] = [
        DockSize::Small,
        DockSize::Medium,
        DockSize::Large,
        DockSize::Wow,
    ];

    pub fn key(self) -> &'static str {
        match self {
            DockSize::Small => "small",
            DockSize::Medium => "medium",
            DockSize::Large => "large",
            DockSize::Wow => "wow",
        }
    }

    pub fn from_key(key: &str) -> DockSize {
        DockSize::ALL
            .into_iter()
            .find(|d| d.key() == key)
            .unwrap_or_default()
    }

    pub fn label(self) -> &'static str {
        match self {
            DockSize::Small => "Small",
            DockSize::Medium => "Medium",
            DockSize::Large => "Large",
            DockSize::Wow => "WOW!",
        }
    }

    /// Glyph size of a format button.
    pub fn glyph(self) -> f32 {
        match self {
            DockSize::Small => 12.0,
            DockSize::Medium => 14.0,
            DockSize::Large => 18.0,
            DockSize::Wow => 26.0,
        }
    }

    /// Button padding [vertical, horizontal].
    pub fn pad(self) -> [u16; 2] {
        match self {
            DockSize::Small => [2, 5],
            DockSize::Medium => [3, 7],
            DockSize::Large => [5, 10],
            DockSize::Wow => [8, 15],
        }
    }

    /// Pill padding [vertical, horizontal].
    pub fn pill(self) -> [u16; 2] {
        match self {
            DockSize::Small => [2, 4],
            DockSize::Medium => [3, 6],
            DockSize::Large => [5, 9],
            DockSize::Wow => [8, 14],
        }
    }
}

/// Body text in the panel colour scheme.
pub fn text<'a>(p: &Palette, s: impl Into<std::borrow::Cow<'a, str>> + 'a) -> Text<'a> {
    widget::text(s)
        .font(mono())
        .size(13)
        .class(theme::Text::Color(p.fg))
}

pub fn dim<'a>(p: &Palette, s: impl Into<std::borrow::Cow<'a, str>> + 'a) -> Text<'a> {
    widget::text(s)
        .font(mono())
        .size(12)
        .class(theme::Text::Color(p.dim))
}

pub fn accent<'a>(p: &Palette, s: impl Into<std::borrow::Cow<'a, str>> + 'a) -> Text<'a> {
    widget::text(s)
        .font(mono())
        .size(13)
        .class(theme::Text::Color(p.accent))
}

pub fn accent2<'a>(p: &Palette, s: impl Into<std::borrow::Cow<'a, str>> + 'a) -> Text<'a> {
    widget::text(s)
        .font(mono())
        .size(13)
        .class(theme::Text::Color(p.accent2))
}

/// Frame title face.
pub fn title<'a>(p: &Palette, s: impl Into<std::borrow::Cow<'a, str>> + 'a) -> Text<'a> {
    widget::text(s)
        .font(TITLE_FONT)
        .size(21)
        .class(theme::Text::Color(p.accent))
}

/// A flat pane in the flat layout: the title (and optional badge) as a
/// header row, the content below, no border — panes sit against each other
/// separated by [`vrule`] / [`hrule`] hairlines.
pub fn pane<'a, M: 'static>(
    p: &Palette,
    title_font: Font,
    title_text: impl Into<std::borrow::Cow<'a, str>> + 'a,
    badge: Option<String>,
    content: impl Into<Element<'a, M>>,
    bg: Color,
) -> Element<'a, M> {
    let badge = badge.map(|b| dim(p, b).into());
    pane_el(p, title_font, title_text, badge, content, bg)
}

/// [`pane`] with an arbitrary element as the right-hand badge.
pub fn pane_el<'a, M: 'static>(
    p: &Palette,
    title_font: Font,
    title_text: impl Into<std::borrow::Cow<'a, str>> + 'a,
    badge: Option<Element<'a, M>>,
    content: impl Into<Element<'a, M>>,
    bg: Color,
) -> Element<'a, M> {
    let mut bar = widget::row::with_capacity(3)
        .push(
            title(p, title_text)
                .font(title_font)
                .size(21)
                .wrapping(cosmic::iced::widget::text::Wrapping::None),
        )
        .push(widget::Space::new().width(Length::Fill))
        .spacing(8)
        .align_y(Alignment::Center);
    if let Some(badge) = badge {
        bar = bar.push(badge);
    }
    let col = widget::column::with_capacity(2)
        .push(
            widget::container(bar)
                .padding([0, 0, 6, 0])
                .width(Length::Fill),
        )
        .push(content)
        .width(Length::Fill)
        .height(Length::Fill);
    widget::container(col)
        .padding([10, 12, 10, 12])
        .width(Length::Fill)
        .height(Length::Fill)
        .class(theme::Container::custom(move |_| {
            widget::container::Style {
                background: Some(Background::Color(bg)),
                ..Default::default()
            }
        }))
        .into()
}

/// A horizontal rule in the note: a full-width line in the muted colour.
pub fn rule_block<'a, M: 'static>(p: &Palette) -> Element<'a, M> {
    let mute = p.mute;
    widget::container(
        widget::container(widget::Space::new().width(Length::Fill).height(2))
            .width(Length::Fill)
            .class(theme::Container::custom(move |_| {
                widget::container::Style {
                    background: Some(Background::Color(mute)),
                    border: Border {
                        radius: 1.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })),
    )
    .padding([10, 10])
    .width(Length::Fill)
    .into()
}

/// One-pixel vertical hairline between panes.
pub fn vrule<'a, M: 'static>(p: &Palette) -> Element<'a, M> {
    let border = p.border;
    widget::container(widget::Space::new().width(1).height(Length::Fill))
        .width(Length::Fixed(1.0))
        .height(Length::Fill)
        .class(theme::Container::custom(move |_| {
            widget::container::Style {
                background: Some(Background::Color(border)),
                ..Default::default()
            }
        }))
        .into()
}

/// One-pixel horizontal hairline between stacked panes.
pub fn hrule<'a, M: 'static>(p: &Palette) -> Element<'a, M> {
    let border = p.border;
    widget::container(widget::Space::new().width(Length::Fill).height(1))
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .class(theme::Container::custom(move |_| {
            widget::container::Style {
                background: Some(Background::Color(border)),
                ..Default::default()
            }
        }))
        .into()
}

/// [`frame`] with an explicit height (`Length::Shrink` for inline cards).
pub fn frame_sized<'a, M: 'static>(
    p: &Palette,
    title_text: impl Into<std::borrow::Cow<'a, str>> + 'a,
    badge: Option<String>,
    content: impl Into<Element<'a, M>>,
    height: Length,
    title_size: f32,
) -> Element<'a, M> {
    let badge = badge.map(|b| dim(p, b).into());
    frame_el(p, title_text, badge, content, height, title_size)
}

/// [`frame_sized`] with an arbitrary element as the right-hand badge
/// (e.g. a clickable save indicator).
pub fn frame_el<'a, M: 'static>(
    p: &Palette,
    title_text: impl Into<std::borrow::Cow<'a, str>> + 'a,
    badge: Option<Element<'a, M>>,
    content: impl Into<Element<'a, M>>,
    height: Length,
    title_size: f32,
) -> Element<'a, M> {
    let panel = p.panel;
    let border = p.border;

    let boxed = widget::container(content)
        .padding([14, 12, 10, 12])
        .width(Length::Fill)
        .height(height)
        .class(theme::Container::custom(move |_| {
            widget::container::Style {
                background: Some(Background::Color(panel)),
                border: Border {
                    color: border,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            }
        }));
    // Push the box down so the title can sit centred on its top edge.
    let outer = widget::container(boxed)
        .padding([9, 0, 0, 0])
        .width(Length::Fill)
        .height(height);

    let chip = move |el: Element<'a, M>| {
        widget::container(el)
            .padding([0, 6])
            .class(theme::Container::custom(move |_| {
                widget::container::Style {
                    background: Some(Background::Color(panel)),
                    ..Default::default()
                }
            }))
    };
    let mut bar = widget::row::with_capacity(3)
        .push(chip(title(p, title_text).size(title_size).into()))
        .push(widget::Space::new().width(Length::Fill))
        .align_y(Alignment::Center);
    if let Some(badge) = badge {
        bar = bar.push(chip(badge));
    }
    let title_bar = widget::container(bar)
        .padding([0, 12])
        .width(Length::Fill)
        .align_y(Alignment::Start);

    cosmic::iced::widget::stack([outer.into(), title_bar.into()])
        .width(Length::Fill)
        .height(height)
        .into()
}

/// Class for a selectable row (nav entry, note in the list).
pub fn row_class(p: &Palette, selected: bool) -> theme::Button {
    let sel = p.sel;
    let selfg = p.selfg;
    let fg = p.fg;
    let hover = p.mute;
    let base = move |bg: Option<Color>, text: Color| {
        let mut s = widget::button::Style::new();
        s.background = bg.map(Background::Color);
        s.text_color = Some(text);
        s.icon_color = Some(text);
        s.border_radius = 3.0.into();
        s
    };
    theme::Button::Custom {
        active: Box::new(move |_, _| {
            if selected {
                base(Some(sel), selfg)
            } else {
                base(None, fg)
            }
        }),
        disabled: Box::new(move |_| base(None, fg)),
        hovered: Box::new(move |_, _| {
            if selected {
                base(Some(sel), selfg)
            } else {
                base(Some(hover), fg)
            }
        }),
        pressed: Box::new(move |_, _| base(Some(sel), selfg)),
    }
}

/// Terminal-prompt style search box.
pub fn search_class(p: &Palette) -> theme::TextInput {
    let p = *p;
    let make = move |border: Color| text_input::Appearance {
        background: Background::Color(Color::TRANSPARENT),
        border_radius: 3.0.into(),
        border_offset: None,
        border_width: 1.0,
        border_color: border,
        label_color: p.fg,
        placeholder_color: p.dim,
        selected_text_color: p.selfg,
        icon_color: Some(p.accent),
        text_color: Some(p.fg),
        selected_fill: p.sel,
    };
    theme::TextInput::Custom {
        active: Box::new(move |_| make(p.mute)),
        error: Box::new(move |_| make(p.accent)),
        hovered: Box::new(move |_| make(p.border)),
        focused: Box::new(move |_| make(p.accent)),
        disabled: Box::new(move |_| make(p.mute)),
    }
}

/// A clickable theme swatch for the picker: a mini frame in that theme's
/// own colours, with the name underneath.
pub fn swatch<'a, M: Clone + 'static>(
    theme: Theme,
    p: &Palette,
    selected: bool,
    label_color: Color,
    on_press: M,
) -> Element<'a, M> {
    let p = *p;
    let border = if selected { p.accent } else { p.border };
    let sample = widget::column::with_capacity(3)
        .push(
            widget::text("Aa")
                .font(TITLE_FONT)
                .size(24)
                .class(theme::Text::Color(p.accent)),
        )
        .push(
            widget::row::with_capacity(3)
                .push(
                    widget::text("bold")
                        .font(Font {
                            weight: Weight::Bold,
                            ..mono()
                        })
                        .size(12)
                        .class(theme::Text::Color(p.fg)),
                )
                .push(
                    widget::text("#tag")
                        .font(mono())
                        .size(12)
                        .class(theme::Text::Color(p.accent2)),
                )
                .push(
                    widget::text("dim")
                        .font(mono())
                        .size(12)
                        .class(theme::Text::Color(p.dim)),
                )
                .spacing(8),
        )
        .push(
            widget::container(widget::Space::new().width(Length::Fill).height(3))
                .width(Length::Fill)
                .class(theme::Container::custom(move |_| {
                    widget::container::Style {
                        background: Some(Background::Color(p.sel)),
                        ..Default::default()
                    }
                })),
        )
        .spacing(6);
    let card = widget::container(sample)
        .padding([10, 12])
        .width(Length::Fill)
        .class(theme::Container::custom(move |_| {
            widget::container::Style {
                background: Some(Background::Color(p.bg)),
                border: Border {
                    color: border,
                    width: if selected { 2.0 } else { 1.0 },
                    radius: 6.0.into(),
                },
                ..Default::default()
            }
        }));
    let label = widget::column::with_capacity(2)
        .push(widget::text::body(theme.label()).class(theme::Text::Color(label_color)))
        .push(
            widget::text::caption(theme.blurb())
                .class(theme::Text::Color(label_color.scale_alpha(0.7))),
        )
        .spacing(2);
    widget::button::custom(
        widget::column::with_capacity(2)
            .push(card)
            .push(label)
            .spacing(8)
            .width(Length::Fill),
    )
    .padding(6)
    .width(Length::Fill)
    .class(theme::Button::Transparent)
    .on_press(on_press)
    .into()
}

// ---------- image cards ----------

fn plain_box<'a, M: 'static>(
    p: &Palette,
    content: Element<'a, M>,
    bg: Color,
    border: Color,
    radius: f32,
    padding: [u16; 4],
) -> Element<'a, M> {
    let _ = p;
    widget::container(content)
        .padding(padding)
        .width(Length::Fill)
        .class(theme::Container::custom(move |_| {
            widget::container::Style {
                background: Some(Background::Color(bg)),
                border: Border {
                    color: border,
                    width: 1.0,
                    radius: radius.into(),
                },
                ..Default::default()
            }
        }))
        .into()
}

/// Thin 1px border, for tinted / dithered / pixel images.
pub fn bordered<'a, M: 'static>(p: &Palette, content: Element<'a, M>) -> Element<'a, M> {
    plain_box(p, content, p.bg, p.border, 3.0, [3, 3, 3, 3])
}

/// Rounded dark bezel with an accent glow line.
pub fn bezel<'a, M: 'static>(p: &Palette, content: Element<'a, M>) -> Element<'a, M> {
    let inner = plain_box(p, content, Color::BLACK, p.mute, 8.0, [2, 2, 2, 2]);
    let shell = Color {
        a: 0.9,
        ..Color::BLACK
    };
    plain_box(
        p,
        inner,
        shell,
        p.accent.scale_alpha(0.35),
        14.0,
        [12, 12, 12, 12],
    )
}

/// Off-white instant print with the alt text as a handwritten caption.
pub fn print<'a, M: 'static>(
    p: &Palette,
    content: Element<'a, M>,
    caption: String,
) -> Element<'a, M> {
    let paper = hex(0xe9e6da);
    let ink = hex(0x3a3630);
    let col = widget::column::with_capacity(2)
        .push(content)
        .push(
            widget::container(
                widget::text(caption)
                    .font(TITLE_FONT)
                    .size(20)
                    .class(theme::Text::Color(ink)),
            )
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .padding([6, 0, 2, 0]),
        )
        .spacing(4);
    plain_box(p, col.into(), paper, hex(0xd6d2c4), 2.0, [10, 10, 14, 10])
}

/// Film strip: sprocket holes above and below, frame number in the corner.
pub fn film<'a, M: 'static>(p: &Palette, content: Element<'a, M>, number: usize) -> Element<'a, M> {
    let strip = hex(0x0b0f0c);
    let holes = || {
        let mut row = widget::row::with_capacity(12).spacing(9);
        for _ in 0..12 {
            row = row.push(
                widget::container(widget::Space::new().width(9).height(7)).class(
                    theme::Container::custom(|_| widget::container::Style {
                        background: Some(Background::Color(hex(0x1f1f22))),
                        border: Border {
                            radius: 2.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                ),
            );
        }
        widget::container(row)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .padding([3, 0])
    };
    let label = widget::container(
        widget::text(format!("▶ {number:02}A"))
            .font(TITLE_FONT)
            .size(15)
            .class(theme::Text::Color(p.accent)),
    )
    .width(Length::Fill)
    .align_x(Alignment::End)
    .padding([0, 4]);
    let col = widget::column::with_capacity(4)
        .push(holes())
        .push(widget::container(content).padding([0, 10]))
        .push(label)
        .push(holes())
        .spacing(2);
    plain_box(p, col.into(), strip, p.border, 2.0, [2, 0, 2, 0])
}

/// ASCII rendering in the accent colour.
/// A comic panel: newsprint gutter, thick ink border, and the caption in a
/// yellow narration box tucked into the top-left corner.
pub fn comic<'a, M: 'static>(
    p: &Palette,
    content: Element<'a, M>,
    caption: String,
) -> Element<'a, M> {
    let _ = p;
    let paper = hex(0xf2e8cd);
    let ink = hex(0x18121a);
    let yellow = hex(0xf6d85a);
    let panel = widget::container(content)
        .padding(0)
        .width(Length::Fill)
        .class(theme::Container::custom(move |_| {
            widget::container::Style {
                background: Some(Background::Color(ink)),
                border: Border {
                    color: ink,
                    width: 3.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        }));
    let inner: Element<'a, M> = if caption.trim().is_empty() {
        panel.into()
    } else {
        let tag = widget::container(
            widget::text(caption.to_uppercase())
                .font(TITLE_FONT)
                .size(18)
                .class(theme::Text::Color(ink)),
        )
        .padding([1, 8, 2, 8])
        .class(theme::Container::custom(move |_| {
            widget::container::Style {
                background: Some(Background::Color(yellow)),
                border: Border {
                    color: ink,
                    width: 2.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        }));
        cosmic::iced::widget::stack([
            panel.into(),
            widget::container(tag).padding([9, 0, 0, 9]).into(),
        ])
        .width(Length::Fill)
        .into()
    };
    widget::container(inner)
        .padding(8)
        .width(Length::Fill)
        .class(theme::Container::custom(move |_| {
            widget::container::Style {
                background: Some(Background::Color(paper)),
                border: Border {
                    color: hex(0xd9cfae),
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        }))
        .into()
}

/// The drop indicator while a picture is dragged: an accent rule with a tag,
/// drawn exactly where the picture will land.
pub fn drop_line<'a, M: 'static>(p: &Palette, label: String) -> Element<'a, M> {
    let accent = p.accent;
    let rule = || {
        widget::container(widget::Space::new().width(Length::Fill).height(2))
            .width(Length::Fill)
            .class(theme::Container::custom(move |_| {
                widget::container::Style {
                    background: Some(Background::Color(accent)),
                    ..Default::default()
                }
            }))
    };
    let tag = widget::container(
        widget::text(label)
            .font(TITLE_FONT)
            .size(16)
            .class(theme::Text::Color(p.bg)),
    )
    .padding([0, 8, 1, 8])
    .class(theme::Container::custom(move |_| {
        widget::container::Style {
            background: Some(Background::Color(accent)),
            border: Border {
                radius: 3.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }));
    widget::container(
        widget::row::with_capacity(3)
            .push(rule())
            .push(tag)
            .push(rule())
            .align_y(Alignment::Center)
            .width(Length::Fill),
    )
    .padding([2, 4])
    .width(Length::Fill)
    .into()
}

/// The picture being dragged: outlined in the accent so it reads as lifted.
pub fn lifted<'a, M: 'static>(p: &Palette, content: Element<'a, M>) -> Element<'a, M> {
    let accent = p.accent;
    widget::container(content)
        .width(Length::Fill)
        .class(theme::Container::custom(move |_| {
            widget::container::Style {
                border: Border {
                    color: accent,
                    width: 2.0,
                    radius: 3.0.into(),
                },
                ..Default::default()
            }
        }))
        .into()
}

pub fn ascii_card<'a, M: 'static>(p: &Palette, text: String, size: f32) -> Element<'a, M> {
    let txt = widget::text(text)
        .font(mono())
        .size(size)
        .line_height(cosmic::iced::widget::text::LineHeight::Relative(1.05))
        .class(theme::Text::Color(p.accent));
    plain_box(
        p,
        widget::container(txt).into(),
        p.bg,
        p.border,
        3.0,
        [6, 6, 6, 6],
    )
}

/// The floating dock tray at the bottom.
pub fn dock_class(p: &Palette) -> theme::Container<'static> {
    let panel = p.panel;
    let border = p.border;
    theme::Container::custom(move |_| widget::container::Style {
        background: Some(Background::Color(panel)),
        border: Border {
            color: border,
            width: 1.0,
            radius: 10.0.into(),
        },
        ..Default::default()
    })
}

/// Window background/text defaults for the content area.
pub fn app_style(p: &Palette) -> cosmic::iced::theme::Style {
    cosmic::iced::theme::Style {
        background_color: p.bg,
        text_color: p.fg,
        icon_color: p.fg,
    }
}
