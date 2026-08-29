// SPDX-License-Identifier: GPL-3.0-only

//! The retro look: btop-style frames, phosphor/amber/WordPerfect palettes,
//! and the style helpers the views use. The COSMIC header bar is left alone;
//! everything below it is drawn with these.

use cosmic::iced::font::{Family, Weight};
use cosmic::iced::{Alignment, Background, Border, Color, Font, Length};
use cosmic::widget::{self, text_editor, text_input};
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
    /// Follows the COSMIC system theme; frames only.
    Cosmic,
}

impl Theme {
    pub const ALL: [Theme; 9] = [
        Theme::Phosphor,
        Theme::Amber,
        Theme::WordPerfect,
        Theme::Paper,
        Theme::Plasma,
        Theme::C64,
        Theme::GameBoy,
        Theme::Synthwave,
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
                bg: hex(0x0000aa),
                panel: hex(0x0000aa),
                fg: hex(0xe8e8e8),
                dim: hex(0xa4a4ff),
                mute: hex(0x4a4aff),
                accent: hex(0xffff55),
                accent2: hex(0x55ffff),
                border: hex(0x9a9aff),
                sel: hex(0x0000ff),
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
                bg: hex(0x40318d),
                panel: hex(0x40318d),
                fg: hex(0xb8b2ff),
                dim: hex(0x9e96ea),
                mute: hex(0x5f52b8),
                accent: hex(0xd5df7c),
                accent2: hex(0x94e089),
                border: hex(0x7869c4),
                sel: hex(0x5b4ab5),
                selfg: hex(0xffffff),
            },
            Theme::GameBoy => Palette {
                bg: hex(0x9bbc0f),
                panel: hex(0x8bac0f),
                fg: hex(0x0f380f),
                dim: hex(0x306230),
                mute: hex(0x6a9a2f),
                accent: hex(0x1a4d1a),
                accent2: hex(0x306230),
                border: hex(0x306230),
                sel: hex(0x306230),
                selfg: hex(0x9bbc0f),
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

/// A btop-style frame: 1px rounded border with the title cut into the top edge
/// and an optional right-hand badge.
pub fn frame<'a, M: 'static>(
    p: &Palette,
    title_text: impl Into<std::borrow::Cow<'a, str>> + 'a,
    badge: Option<String>,
    content: impl Into<Element<'a, M>>,
) -> Element<'a, M> {
    let panel = p.panel;
    let border = p.border;

    let boxed = widget::container(content)
        .padding([14, 12, 10, 12])
        .width(Length::Fill)
        .height(Length::Fill)
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
        .height(Length::Fill);

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
        .push(chip(title(p, title_text).into()))
        .push(widget::Space::new().width(Length::Fill))
        .align_y(Alignment::Center);
    if let Some(badge) = badge {
        bar = bar.push(chip(dim(p, badge).into()));
    }
    let title_bar = widget::container(bar)
        .padding([0, 12])
        .width(Length::Fill)
        .align_y(Alignment::Start);

    cosmic::iced::widget::stack([outer.into(), title_bar.into()])
        .width(Length::Fill)
        .height(Length::Fill)
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

pub fn editor_style(
    p: Palette,
) -> impl Fn(&cosmic::Theme, text_editor::Status) -> text_editor::Style + use<> {
    move |_, _| text_editor::Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        placeholder: p.dim,
        value: p.fg,
        selection: p.sel,
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
