// SPDX-License-Identifier: GPL-3.0-only

//! Editable note text. `Content` is what the rest of the app holds: either
//! iced's stock editor content or the rich editor's, chosen by the
//! `rich_editor` flag at construction. Both expose the same small API the
//! app was already using, so callers never care which one they have.

use crate::markdown;
use cosmic::iced::advanced::graphics::text as gtext;
use cosmic::iced::{Font, Point, Rectangle, Size};
use cosmic::widget::text_editor::{self, Action, Cursor, Edit, Line, LineEnding, Motion, Position};
use cosmic_text::{Attrs, AttrsList, Buffer, BufferRef, Edit as _, Metrics, Shaping};
use std::borrow::Cow;
use std::cell::RefCell;
use std::sync::{Arc, Weak};

/// The text of one block, behind whichever editor is enabled.
pub enum Content {
    Iced(text_editor::Content),
    Rich(RichContent),
}

impl Content {
    pub fn with_text(text: &str) -> Self {
        if super::rich_enabled() {
            Content::Rich(RichContent::with_text(text))
        } else {
            Content::Iced(text_editor::Content::with_text(text))
        }
    }

    pub fn perform(&mut self, action: Action) {
        match self {
            Content::Iced(c) => c.perform(action),
            Content::Rich(c) => c.perform(action),
        }
    }

    pub fn move_to(&mut self, cursor: Cursor) {
        match self {
            Content::Iced(c) => c.move_to(cursor),
            Content::Rich(c) => c.move_to(cursor),
        }
    }

    pub fn cursor(&self) -> Cursor {
        match self {
            Content::Iced(c) => c.cursor(),
            Content::Rich(c) => c.cursor(),
        }
    }

    pub fn line_count(&self) -> usize {
        match self {
            Content::Iced(c) => c.line_count(),
            Content::Rich(c) => c.line_count(),
        }
    }

    pub fn line(&self, index: usize) -> Option<Line<'_>> {
        match self {
            Content::Iced(c) => c.line(index),
            Content::Rich(c) => c.line(index),
        }
    }

    /// The whole text. Like iced's, ends with a newline.
    pub fn text(&self) -> String {
        match self {
            Content::Iced(c) => c.text(),
            Content::Rich(c) => c.text(),
        }
    }

    pub fn selection(&self) -> Option<String> {
        match self {
            Content::Iced(c) => c.selection(),
            Content::Rich(c) => c.selection(),
        }
    }
}

/// The rich editor's text: a cosmic-text editor over a shared buffer so the
/// renderer can draw the very same buffer (`fill_raw`) that is edited.
pub struct RichContent(RefCell<Inner>);

struct Inner {
    editor: cosmic_text::Editor<'static>,
    /// What the buffer was last shaped for (width, font, size, line height).
    shaped_for: Option<(f32, Font, f32, f32)>,
    style: Option<markdown::Settings>,
    /// Whether text changed since attributes were last applied.
    dirty: bool,
}

/// The caret or selection, in buffer coordinates.
pub enum Highlight {
    Caret(Point),
    Range(Vec<Rectangle>),
}

impl RichContent {
    pub fn with_text(text: &str) -> Self {
        let mut buffer = Buffer::new_empty(Metrics::new(15.0, 22.5));
        buffer.set_text(text, &Attrs::new(), Shaping::Advanced, None);
        let mut guard = gtext::font_system().write().expect("font system");
        buffer.shape_until_scroll(guard.raw(), false);
        let editor = cosmic_text::Editor::new(BufferRef::Arc(Arc::new(buffer)));
        Self(RefCell::new(Inner {
            editor,
            shaped_for: None,
            style: None,
            dirty: true,
        }))
    }

    fn with_buffer<T>(&self, f: impl FnOnce(&Buffer) -> T) -> T {
        self.0.borrow().editor.with_buffer(f)
    }

    /// A weak handle for the renderer; a new one every draw (mutation
    /// through `Arc::make_mut` invalidates old ones).
    pub fn weak_buffer(&self) -> Weak<Buffer> {
        match self.0.borrow().editor.buffer_ref() {
            BufferRef::Arc(a) => Arc::downgrade(a),
            _ => unreachable!("rich content always shares its buffer"),
        }
    }

    pub fn perform(&mut self, action: Action) {
        let inner = self.0.get_mut();
        let mut guard = gtext::font_system().write().expect("font system");
        let fs = guard.raw();
        let editor = &mut inner.editor;
        match action {
            Action::Move(motion) => {
                if let Some((start, end)) = editor.selection_bounds() {
                    editor.set_selection(cosmic_text::Selection::None);
                    match motion {
                        Motion::Home
                        | Motion::End
                        | Motion::DocumentStart
                        | Motion::DocumentEnd => {
                            editor.action(fs, cosmic_text::Action::Motion(to_motion(motion)));
                        }
                        _ => editor.set_cursor(match motion.direction() {
                            cosmic::iced::advanced::text::editor::Direction::Left => start,
                            cosmic::iced::advanced::text::editor::Direction::Right => end,
                        }),
                    }
                } else {
                    editor.action(fs, cosmic_text::Action::Motion(to_motion(motion)));
                }
            }
            Action::Select(motion) => {
                let cursor = editor.cursor();
                if editor.selection_bounds().is_none() {
                    editor.set_selection(cosmic_text::Selection::Normal(cursor));
                }
                editor.action(fs, cosmic_text::Action::Motion(to_motion(motion)));
                if let Some((s, e)) = editor.selection_bounds()
                    && s.line == e.line
                    && s.index == e.index
                {
                    editor.set_selection(cosmic_text::Selection::None);
                }
            }
            Action::SelectWord => {
                let c = editor.cursor();
                editor.set_selection(cosmic_text::Selection::Word(c));
            }
            Action::SelectLine => {
                let c = editor.cursor();
                editor.set_selection(cosmic_text::Selection::Line(c));
            }
            Action::SelectAll => {
                let non_empty = editor.with_buffer(|b| {
                    b.lines.len() > 1 || b.lines.first().is_some_and(|l| !l.text().is_empty())
                });
                if non_empty {
                    let c = editor.cursor();
                    editor.set_selection(cosmic_text::Selection::Normal(cosmic_text::Cursor {
                        line: 0,
                        index: 0,
                        ..c
                    }));
                    editor.action(
                        fs,
                        cosmic_text::Action::Motion(cosmic_text::Motion::BufferEnd),
                    );
                }
            }
            Action::ClearSelection => editor.set_selection(cosmic_text::Selection::None),
            Action::Edit(edit) => {
                match edit {
                    Edit::Insert(c) => editor.action(fs, cosmic_text::Action::Insert(c)),
                    Edit::Paste(text) => editor.insert_string(&text, None),
                    Edit::Indent => editor.action(fs, cosmic_text::Action::Indent),
                    Edit::Unindent => editor.action(fs, cosmic_text::Action::Unindent),
                    Edit::Enter => editor.action(fs, cosmic_text::Action::Enter),
                    Edit::Backspace => editor.action(fs, cosmic_text::Action::Backspace),
                    Edit::Delete => editor.action(fs, cosmic_text::Action::Delete),
                }
                inner.dirty = true;
            }
            Action::Click(p) => editor.action(
                fs,
                cosmic_text::Action::Click {
                    x: p.x as i32,
                    y: p.y as i32,
                },
            ),
            Action::Drag(p) => {
                editor.action(
                    fs,
                    cosmic_text::Action::Drag {
                        x: p.x as i32,
                        y: p.y as i32,
                    },
                );
                if let Some((s, e)) = editor.selection_bounds()
                    && s.line == e.line
                    && s.index == e.index
                {
                    editor.set_selection(cosmic_text::Selection::None);
                }
            }
            // The note's scrollable scrolls; the buffer never does.
            Action::Scroll { .. } => {}
        }
        editor.shape_as_needed(fs, false);
    }

    pub fn move_to(&mut self, cursor: Cursor) {
        let inner = self.0.get_mut();
        inner.editor.set_cursor(cosmic_text::Cursor {
            line: cursor.position.line,
            index: cursor.position.column,
            affinity: cosmic_text::Affinity::Before,
        });
        inner.editor.set_selection(match cursor.selection {
            Some(s) => cosmic_text::Selection::Normal(cosmic_text::Cursor {
                line: s.line,
                index: s.column,
                affinity: cosmic_text::Affinity::Before,
            }),
            None => cosmic_text::Selection::None,
        });
    }

    pub fn cursor(&self) -> Cursor {
        let inner = self.0.borrow();
        let c = inner.editor.cursor();
        let selection = match inner.editor.selection() {
            cosmic_text::Selection::None => None,
            cosmic_text::Selection::Normal(s)
            | cosmic_text::Selection::Line(s)
            | cosmic_text::Selection::Word(s) => Some(Position {
                line: s.line,
                column: s.index,
            }),
        };
        Cursor {
            position: Position {
                line: c.line,
                column: c.index,
            },
            selection,
        }
    }

    pub fn line_count(&self) -> usize {
        self.with_buffer(|b| b.lines.len())
    }

    pub fn line(&self, index: usize) -> Option<Line<'_>> {
        self.with_buffer(|b| {
            b.lines.get(index).map(|l| Line {
                text: Cow::Owned(l.text().to_owned()),
                ending: match l.ending() {
                    cosmic_text::LineEnding::Lf => LineEnding::Lf,
                    cosmic_text::LineEnding::CrLf => LineEnding::CrLf,
                    cosmic_text::LineEnding::Cr => LineEnding::Cr,
                    cosmic_text::LineEnding::LfCr => LineEnding::LfCr,
                    cosmic_text::LineEnding::None => LineEnding::None,
                },
            })
        })
    }

    pub fn text(&self) -> String {
        // Match iced: lines joined by their endings, plus a final newline.
        self.with_buffer(|b| {
            let mut out = String::new();
            for (i, l) in b.lines.iter().enumerate() {
                out.push_str(l.text());
                if i + 1 < b.lines.len() {
                    let e = l.ending();
                    out.push_str(if e == cosmic_text::LineEnding::None {
                        "\n"
                    } else {
                        e.as_str()
                    });
                }
            }
            out.push('\n');
            out
        })
    }

    pub fn selection(&self) -> Option<String> {
        self.0.borrow().editor.copy_selection()
    }

    pub fn is_empty(&self) -> bool {
        self.with_buffer(|b| {
            b.lines.is_empty() || (b.lines.len() == 1 && b.lines[0].text().is_empty())
        })
    }

    /// Bring the buffer in line with the widget: width, metrics, and the
    /// per-span attributes from the markdown scanner. Called from layout.
    pub fn update(
        &self,
        width: f32,
        font: Font,
        size: f32,
        line_height: f32,
        style: &markdown::Settings,
    ) {
        let mut inner = self.0.borrow_mut();
        let key = (width, font, size, line_height);
        let restyle = inner.style.as_ref() != Some(style);
        if inner.shaped_for == Some(key) && !inner.dirty && !restyle {
            return;
        }
        let mut guard = gtext::font_system().write().expect("font system");
        let fs = guard.raw();
        let base = gtext::to_attributes(font).color(gtext::to_color(style.palette.fg));
        inner.editor.with_buffer_mut(|b| {
            if b.metrics().font_size != size || b.metrics().line_height != line_height {
                b.set_metrics(Metrics::new(size, line_height));
            }
            if b.size().0 != Some(width) {
                b.set_size(Some(width), None);
            }
            // Attributes: today's highlighter look, per line.
            let mut in_fence = false;
            for line in &mut b.lines {
                let (spans, after) = markdown::scan_line(line.text(), in_fence);
                in_fence = after;
                let mut list = AttrsList::new(&base);
                for span in spans {
                    let h = markdown::style_for(span.kind, style);
                    let mut attrs = match h.font {
                        Some(f) => gtext::to_attributes(f),
                        None => base.clone(),
                    };
                    if let Some(c) = h.color {
                        attrs = attrs.color(gtext::to_color(c));
                    }
                    list.add_span(span.range, &attrs);
                }
                let _ = line.set_attrs_list(list);
            }
        });
        inner.editor.shape_as_needed(fs, false);
        inner.shaped_for = Some(key);
        inner.style = Some(style.clone());
        inner.dirty = false;
    }

    /// Height of all laid-out lines.
    pub fn height(&self) -> f32 {
        self.with_buffer(|b| {
            b.layout_runs()
                .last()
                .map_or(b.metrics().line_height, |r| r.line_top + r.line_height)
        })
    }

    /// Where the caret is, or the rectangles of the selection.
    pub fn highlight(&self) -> Highlight {
        let inner = self.0.borrow();
        let cursor = inner.editor.cursor();
        let bounds = inner.editor.selection_bounds();
        inner.editor.with_buffer(|b| {
            let line_height = b.metrics().line_height;
            match bounds {
                Some((start, end)) => Highlight::Range(
                    b.layout_runs()
                        .filter(|r| r.line_i >= start.line && r.line_i <= end.line)
                        .flat_map(|r| {
                            let top = r.line_top;
                            let h = r.line_height;
                            r.highlight(start, end)
                                .filter(|(_, w)| *w > 0.0)
                                .map(move |(x, w)| Rectangle {
                                    x,
                                    y: top,
                                    width: w,
                                    height: h.max(line_height),
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect(),
                ),
                None => {
                    let point = b
                        .layout_runs()
                        .filter(|r| r.line_i == cursor.line)
                        .find_map(|r| {
                            r.cursor_position(&cursor)
                                .map(|x| Point::new(x, r.line_top))
                        })
                        .unwrap_or_else(|| {
                            // Past the last laid-out run (empty buffer or trailing empty line).
                            let y = b
                                .layout_runs()
                                .filter(|r| r.line_i < cursor.line)
                                .last()
                                .map_or(0.0, |r| r.line_top + r.line_height);
                            Point::new(0.0, y)
                        });
                    Highlight::Caret(point)
                }
            }
        })
    }

    pub fn line_height_at_cursor(&self) -> f32 {
        let inner = self.0.borrow();
        let cursor = inner.editor.cursor();
        inner.editor.with_buffer(|b| {
            b.layout_runs()
                .find(|r| r.line_i == cursor.line)
                .map_or(b.metrics().line_height, |r| r.line_height)
        })
    }

    pub fn size(&self) -> Size {
        self.with_buffer(|b| Size::new(b.size().0.unwrap_or(0.0), self.height()))
    }
}

fn to_motion(motion: Motion) -> cosmic_text::Motion {
    match motion {
        Motion::Left => cosmic_text::Motion::Left,
        Motion::Right => cosmic_text::Motion::Right,
        Motion::Up => cosmic_text::Motion::Up,
        Motion::Down => cosmic_text::Motion::Down,
        Motion::WordLeft => cosmic_text::Motion::LeftWord,
        Motion::WordRight => cosmic_text::Motion::RightWord,
        Motion::Home => cosmic_text::Motion::Home,
        Motion::End => cosmic_text::Motion::End,
        Motion::PageUp => cosmic_text::Motion::PageUp,
        Motion::PageDown => cosmic_text::Motion::PageDown,
        Motion::DocumentStart => cosmic_text::Motion::BufferStart,
        Motion::DocumentEnd => cosmic_text::Motion::BufferEnd,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(line: usize, column: usize) -> Cursor {
        Cursor {
            position: Position { line, column },
            selection: None,
        }
    }

    #[test]
    fn rich_content_edits_like_iced() {
        let mut c = RichContent::with_text("hello\nworld");
        assert_eq!(c.line_count(), 2);
        assert_eq!(c.text(), "hello\nworld\n");
        c.perform(Action::Move(Motion::DocumentEnd));
        for ch in "!\nnew".chars() {
            c.perform(Action::Edit(if ch == '\n' {
                Edit::Enter
            } else {
                Edit::Insert(ch)
            }));
        }
        assert_eq!(c.text(), "hello\nworld!\nnew\n");
        assert_eq!(c.cursor().position, Position { line: 2, column: 3 });
        c.move_to(at(0, 0));
        for _ in 0..5 {
            c.perform(Action::Select(Motion::Right));
        }
        assert_eq!(c.selection().as_deref(), Some("hello"));
        c.perform(Action::Edit(Edit::Paste(Arc::new("bye".into()))));
        assert_eq!(c.text(), "bye\nworld!\nnew\n");
        assert_eq!(c.line(1).unwrap().text, "world!");
        c.perform(Action::Edit(Edit::Backspace));
        assert_eq!(c.text(), "by\nworld!\nnew\n");
        c.perform(Action::SelectAll);
        assert_eq!(c.selection().as_deref(), Some("by\nworld!\nnew"));
        c.perform(Action::Move(Motion::Left));
        assert_eq!(c.cursor().selection, None);
        assert_eq!(c.cursor().position, Position { line: 0, column: 0 });
    }

    #[test]
    fn rich_content_layout_and_caret() {
        let mut c = RichContent::with_text("one two three four five six seven eight nine ten");
        let settings = markdown::Settings {
            palette: crate::retro::Theme::Phosphor.palette(&cosmic::Theme::default()),
            show_markers: false,
            font: cosmic::font::mono(),
        };
        c.update(120.0, cosmic::font::mono(), 15.0, 22.5, &settings);
        // Narrow width wraps into several visual lines.
        assert!(c.height() > 22.5 * 2.0);
        c.perform(Action::Move(Motion::DocumentEnd));
        match c.highlight() {
            Highlight::Caret(p) => assert!(p.y > 22.5),
            Highlight::Range(_) => panic!("no selection expected"),
        }
        c.perform(Action::SelectAll);
        match c.highlight() {
            Highlight::Range(r) => assert!(r.len() >= 2),
            Highlight::Caret(_) => panic!("selection expected"),
        }
        assert!(!c.is_empty());
        assert!(RichContent::with_text("").is_empty());
    }
}
