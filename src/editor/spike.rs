// SPDX-License-Identifier: GPL-3.0-only

//! Phase-0 spike: prove that a raw cosmic-text buffer drawn through
//! `fill_raw` gives us what iced's text editor cannot — per-span sizes,
//! collapsed markers, strike-through, and overlays (task box, code
//! background) placed from the layout. Throwaway; enabled with `JJB_SPIKE=1`.

use crate::retro::Palette;
use cosmic::iced::advanced::graphics::text as gtext;
use cosmic::iced::advanced::renderer::{self, Quad};
use cosmic::iced::advanced::text::Renderer as _;
use cosmic::iced::advanced::widget::Tree;
use cosmic::iced::advanced::{Layout, Renderer as _, Widget, layout};
use cosmic::iced::{Background, Border, Color, Element, Font, Length, Rectangle, Size, mouse};
use cosmic_text::{Attrs, Buffer, Metrics, Shaping, Weight};
use std::sync::Arc;

const CODE: usize = 1;
const TASK: usize = 2;

pub struct Spike {
    buffer: Arc<Buffer>,
    width: f32,
    height: f32,
    code_bgs: Vec<Rectangle>,
    boxes: Vec<Rectangle>,
    strikes: Vec<Rectangle>,
    fg: Color,
    accent: Color,
    accent2: Color,
    panel: Color,
}

impl Spike {
    pub fn new(p: &Palette, font: Font, size: f32, width: f32) -> Self {
        let mut guard = gtext::font_system().write().expect("font system");
        let fs = guard.raw();
        let base: Attrs<'static> = gtext::to_attributes(font).color(gtext::to_color(p.fg));
        let line_h = size * 1.5;
        let mut buffer = Buffer::new(fs, Metrics::new(size, line_h));
        buffer.set_size(Some(width), None);

        // A marker collapsed to (almost) nothing: transparent and tiny.
        let hidden = |a: Attrs<'static>| {
            a.color(cosmic_text::Color::rgba(0, 0, 0, 0))
                .metrics(Metrics::new(0.5, line_h))
        };
        let heading = base
            .clone()
            .weight(Weight::BOLD)
            .color(gtext::to_color(p.accent))
            .metrics(Metrics::new(size * 1.6, size * 1.6 * 1.35));
        let bold = base.clone().weight(Weight::BOLD);
        let code = base
            .clone()
            .family(cosmic_text::Family::Monospace)
            .color(gtext::to_color(p.accent2))
            .metadata(CODE);
        let mark = base
            .clone()
            .weight(Weight::BOLD)
            .color(gtext::to_color(p.accent))
            .metadata(TASK);
        let done = base
            .clone()
            .color(gtext::to_color(p.fg.scale_alpha(0.45)))
            .strikethrough()
            .strikethrough_color(gtext::to_color(p.fg.scale_alpha(0.55)));
        let struck = base.clone().strikethrough();

        let spans: Vec<(&str, Attrs<'static>)> = vec![
            ("# ", hidden(base.clone())),
            ("Rich editor spike", heading),
            ("\n", base.clone()),
            ("Body text with ", base.clone()),
            ("**", hidden(base.clone())),
            ("bold", bold),
            ("**", hidden(base.clone())),
            (" and ", base.clone()),
            ("`", hidden(base.clone())),
            ("code", code),
            ("`", hidden(base.clone())),
            (
                " spans; the markers are still in the text, just collapsed.\n",
                base.clone(),
            ),
            ("- ", hidden(base.clone())),
            ("[", hidden(base.clone()).metadata(TASK)),
            ("✓", mark),
            ("]", hidden(base.clone()).metadata(TASK)),
            (" walk the duck", done),
            ("\n", base.clone()),
            ("~~", hidden(base.clone())),
            ("struck out", struck),
            ("~~", hidden(base.clone())),
            (
                " and a normal tail that wraps when the column is narrow enough to need it.",
                base.clone(),
            ),
        ];
        buffer.set_rich_text(spans, &base, Shaping::Advanced, None);
        buffer.shape_until_scroll(fs, true);

        // Overlays from the layout: code backgrounds, task boxes, strikes.
        let mut code_bgs = Vec::new();
        let mut boxes = Vec::new();
        let mut strikes = Vec::new();
        let mut height = 0.0f32;
        for run in buffer.layout_runs() {
            height = height.max(run.line_top + run.line_height);
            for meta in [CODE, TASK] {
                let mut x0 = f32::MAX;
                let mut x1 = f32::MIN;
                for g in run.glyphs.iter().filter(|g| g.metadata == meta) {
                    x0 = x0.min(g.x);
                    x1 = x1.max(g.x + g.w);
                }
                if x0 < x1 {
                    let pad = if meta == CODE { 3.0 } else { 2.0 };
                    let r = Rectangle {
                        x: x0 - pad,
                        y: run.line_top + (run.line_height - size * 1.2) / 2.0,
                        width: (x1 - x0) + 2.0 * pad,
                        height: size * 1.2,
                    };
                    if meta == CODE {
                        code_bgs.push(r);
                    } else {
                        // Square box around the mark.
                        let side = size * 1.15;
                        boxes.push(Rectangle {
                            x: r.x + r.width / 2.0 - side / 2.0,
                            y: run.line_top + (run.line_height - side) / 2.0,
                            width: side,
                            height: side,
                        });
                    }
                }
            }
            for d in run.decorations {
                if !d.data.text_decoration.strikethrough || d.glyph_range.is_empty() {
                    continue;
                }
                let first = &run.glyphs[d.glyph_range.start];
                let last = &run.glyphs[d.glyph_range.end - 1];
                let m = d.data.strikethrough_metrics;
                strikes.push(Rectangle {
                    x: first.x,
                    y: run.line_y - m.offset * d.font_size,
                    width: last.x + last.w - first.x,
                    height: (m.thickness * d.font_size).max(1.0),
                });
            }
        }

        Self {
            buffer: Arc::new(buffer),
            width,
            height: height + 8.0,
            code_bgs,
            boxes,
            strikes,
            fg: p.fg,
            accent: p.accent,
            accent2: p.accent2,
            panel: p.panel,
        }
    }
}

impl<Message> Widget<Message, cosmic::Theme, cosmic::Renderer> for Spike {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.width), Length::Fixed(self.height))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &cosmic::Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.width, self.height))
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut cosmic::Renderer,
        _theme: &cosmic::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let at = |r: &Rectangle| Rectangle {
            x: bounds.x + r.x,
            y: bounds.y + r.y,
            ..*r
        };
        for r in &self.code_bgs {
            renderer.fill_quad(
                Quad {
                    bounds: at(r),
                    border: Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Background::Color(self.accent2.scale_alpha(0.15)),
            );
        }
        for r in &self.boxes {
            renderer.fill_quad(
                Quad {
                    bounds: at(r),
                    border: Border {
                        color: self.accent,
                        width: 1.5,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                },
                Background::Color(self.panel),
            );
        }
        renderer.fill_raw(gtext::Raw {
            buffer: Arc::downgrade(&self.buffer),
            position: bounds.position(),
            color: self.fg,
            clip_bounds: bounds,
        });
        for r in &self.strikes {
            renderer.fill_quad(
                Quad {
                    bounds: at(r),
                    ..Default::default()
                },
                Background::Color(self.fg.scale_alpha(0.6)),
            );
        }
    }
}

impl<'a, Message: 'a> From<Spike> for Element<'a, Message, cosmic::Theme, cosmic::Renderer> {
    fn from(s: Spike) -> Self {
        Element::new(s)
    }
}
