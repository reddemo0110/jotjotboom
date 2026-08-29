// SPDX-License-Identifier: GPL-3.0-only

//! The note body as editable blocks: runs of text (each its own text editor)
//! with images sitting between them exactly where their `![…]` line is.

use crate::images::{self, ImageRef, Segment};
use cosmic::widget::{self, text_editor};

pub enum Block {
    Text {
        content: text_editor::Content,
        id: widget::Id,
    },
    Image(ImageRef),
}

pub struct Blocks {
    pub items: Vec<Block>,
    /// Index of the text block that has (or last had) keyboard focus.
    pub focused: usize,
}

impl Default for Blocks {
    fn default() -> Self {
        Self::from_body("")
    }
}

impl Blocks {
    pub fn from_body(body: &str) -> Self {
        let items = images::split(body)
            .into_iter()
            .map(|seg| match seg {
                Segment::Text(t) => Block::Text {
                    content: text_editor::Content::with_text(&t),
                    id: widget::Id::unique(),
                },
                Segment::Image(r) => Block::Image(r),
            })
            .collect();
        Self { items, focused: 0 }
    }

    /// Reassemble the markdown body.
    pub fn body(&self) -> String {
        let segments: Vec<Segment> = self
            .items
            .iter()
            .map(|b| match b {
                Block::Text { content, .. } => Segment::Text(content_text(content)),
                Block::Image(r) => Segment::Image(r.clone()),
            })
            .collect();
        images::join(&segments)
    }

    pub fn segments(&self) -> Vec<Segment> {
        self.items
            .iter()
            .map(|b| match b {
                Block::Text { content, .. } => Segment::Text(content_text(content)),
                Block::Image(r) => Segment::Image(r.clone()),
            })
            .collect()
    }

    /// Image references with their block index.
    pub fn images(&self) -> Vec<(usize, &ImageRef)> {
        self.items
            .iter()
            .enumerate()
            .filter_map(|(i, b)| match b {
                Block::Image(r) => Some((i, r)),
                _ => None,
            })
            .collect()
    }

    pub fn image_mut(&mut self, block: usize) -> Option<&mut ImageRef> {
        match self.items.get_mut(block) {
            Some(Block::Image(r)) => Some(r),
            _ => None,
        }
    }

    pub fn text_mut(&mut self, block: usize) -> Option<&mut text_editor::Content> {
        match self.items.get_mut(block) {
            Some(Block::Text { content, .. }) => Some(content),
            _ => None,
        }
    }

    pub fn text(&self, block: usize) -> Option<&text_editor::Content> {
        match self.items.get(block) {
            Some(Block::Text { content, .. }) => Some(content),
            _ => None,
        }
    }

    pub fn focused_text(&mut self) -> Option<&mut text_editor::Content> {
        self.text_mut(self.focused)
    }

    pub fn id(&self, block: usize) -> Option<widget::Id> {
        match self.items.get(block) {
            Some(Block::Text { id, .. }) => Some(id.clone()),
            _ => None,
        }
    }

    pub fn focused_id(&self) -> Option<widget::Id> {
        self.id(self.focused)
    }

    /// Previous / next text block relative to `block`.
    pub fn text_before(&self, block: usize) -> Option<usize> {
        (0..block)
            .rev()
            .find(|&i| matches!(self.items[i], Block::Text { .. }))
    }

    pub fn text_after(&self, block: usize) -> Option<usize> {
        (block + 1..self.items.len()).find(|&i| matches!(self.items[i], Block::Text { .. }))
    }

    pub fn last_text(&self) -> usize {
        (0..self.items.len())
            .rev()
            .find(|&i| matches!(self.items[i], Block::Text { .. }))
            .unwrap_or(0)
    }

    /// Rebuild from a body, keeping focus on the text block at `focus` with
    /// the cursor at `cursor` when possible.
    pub fn rebuild(&mut self, body: &str, focus: usize, cursor: Option<text_editor::Cursor>) {
        let fresh = Blocks::from_body(body);
        self.items = fresh.items;
        self.focused = if matches!(self.items.get(focus), Some(Block::Text { .. })) {
            focus
        } else {
            self.text_before(focus.min(self.items.len())).unwrap_or(0)
        };
        if let (Some(c), Some(content)) = (cursor, self.focused_text()) {
            content.move_to(c);
        }
    }

    /// Insert an image after the cursor line of the focused block. Returns
    /// the index of the text block that follows it.
    pub fn insert_image(&mut self, r: ImageRef) -> usize {
        let focused = self.focused;
        let Some(content) = self.text(focused) else {
            return focused;
        };
        let text = content_text(content);
        let cursor = content.cursor().position.line;
        let lines: Vec<&str> = text.split('\n').collect();
        let split_at = if lines.get(cursor).is_some_and(|l| l.trim().is_empty()) {
            cursor
        } else {
            cursor + 1
        };
        let before = lines[..split_at.min(lines.len())].join("\n");
        let after = lines[split_at.min(lines.len())..].join("\n");
        let mut segs = self.segments();
        segs.splice(
            focused..=focused,
            [
                Segment::Text(before),
                Segment::Image(r),
                Segment::Text(after),
            ],
        );
        let body = images::join(&segs);
        self.rebuild(
            &body,
            focused + 2,
            Some(text_editor::Cursor {
                position: text_editor::Position { line: 0, column: 0 },
                selection: None,
            }),
        );
        focused + 2
    }

    /// Remove the image block at `block`, merging the text around it.
    pub fn remove_image(&mut self, block: usize) {
        if !matches!(self.items.get(block), Some(Block::Image(_))) {
            return;
        }
        let mut segs = self.segments();
        segs.remove(block);
        // Merge the two text segments that now touch.
        if block > 0
            && block < segs.len()
            && let (Segment::Text(a), Segment::Text(b)) = (&segs[block - 1], &segs[block])
        {
            let merged = if a.is_empty() {
                b.clone()
            } else if b.is_empty() {
                a.clone()
            } else {
                format!("{a}\n{b}")
            };
            segs.splice(block - 1..=block, [Segment::Text(merged)]);
        }
        let body = images::join(&segs);
        let focus = block.saturating_sub(1);
        self.rebuild(&body, focus, None);
        if let Some(content) = self.focused_text() {
            content.perform(text_editor::Action::Move(text_editor::Motion::DocumentEnd));
        }
    }
    /// Body line at which each block starts. Empty text blocks own no line
    /// (`images::join` skips them), so they share the next block's offset.
    /// The final entry is the total line count.
    pub fn line_offsets(&self) -> Vec<usize> {
        let mut out = Vec::with_capacity(self.items.len() + 1);
        let mut line = 0;
        for b in &self.items {
            out.push(line);
            line += match b {
                Block::Text { content, .. } => {
                    let t = content_text(content);
                    if t.is_empty() {
                        0
                    } else {
                        t.split('\n').count()
                    }
                }
                Block::Image(_) => 1,
            };
        }
        out.push(line);
        out
    }

    /// Move the image at `block` so that its line sits just before body line
    /// `target` (`target == line count` puts it last). Returns the image's
    /// new block index, or `None` when nothing changed.
    pub fn move_image(&mut self, block: usize, target: usize) -> Option<usize> {
        if !matches!(self.items.get(block), Some(Block::Image(_))) {
            return None;
        }
        let offsets = self.line_offsets();
        let from = offsets[block];
        let total = *offsets.last()?;
        let target = target.min(total);
        if target == from || target == from + 1 {
            return None;
        }
        let body = self.body();
        let mut lines: Vec<&str> = body.lines().collect();
        let md = lines.remove(from);
        let at = if target > from { target - 1 } else { target };
        lines.insert(at, md);
        let ordinal = lines[..at]
            .iter()
            .filter(|l| images::parse_line(l).is_some())
            .count();
        let mut new_body = lines.join("\n");
        new_body.push('\n');
        // Focus the text that now follows the picture.
        let fresh = Blocks::from_body(&new_body);
        let new_block = fresh.images().get(ordinal).map(|(b, _)| *b)?;
        let focus = fresh
            .text_after(new_block)
            .unwrap_or_else(|| fresh.last_text());
        self.rebuild(&new_body, focus, None);
        Some(new_block)
    }
}

/// `Content::text()` appends a trailing newline; drop it so joins stay exact.
fn content_text(content: &text_editor::Content) -> String {
    let mut t = content.text();
    if t.ends_with('\n') {
        t.pop();
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_round_trips_and_images_insert_remove() {
        let mut b = Blocks::from_body("hello\nworld\n");
        assert_eq!(b.body(), "hello\nworld\n");
        assert_eq!(b.items.len(), 1);
        // cursor on line 0 ("hello") → image goes after it
        let r = images::parse_line("![p](assets/p.png)").unwrap();
        let next = b.insert_image(r);
        assert_eq!(next, 2);
        assert_eq!(b.body(), "hello\n![p](assets/p.png)\nworld\n");
        assert_eq!(b.images().len(), 1);
        b.remove_image(1);
        assert_eq!(b.body(), "hello\nworld\n");
    }

    #[test]
    fn move_image_between_lines() {
        let body = "one\ntwo\n![p](assets/p.png)\nthree\n\nfour\n";
        let mut b = Blocks::from_body(body);
        assert_eq!(b.line_offsets(), vec![0, 2, 3, 6]);
        // Same place (before or after its own line) is a no-op.
        assert_eq!(b.move_image(1, 2), None);
        assert_eq!(b.move_image(1, 3), None);
        assert_eq!(b.body(), body);
        // Up to the top.
        assert_eq!(b.move_image(1, 0), Some(1));
        assert_eq!(b.body(), "![p](assets/p.png)\none\ntwo\nthree\n\nfour\n");
        assert_eq!(b.line_offsets(), vec![0, 0, 1, 6]);
        // Into the middle of a paragraph splits it.
        assert_eq!(b.move_image(1, 4), Some(1));
        assert_eq!(b.body(), "one\ntwo\nthree\n![p](assets/p.png)\n\nfour\n");
        assert_eq!(b.focused, 2);
        // To the very end.
        assert_eq!(b.move_image(1, 6), Some(1));
        assert_eq!(b.body(), "one\ntwo\nthree\n\nfour\n![p](assets/p.png)\n");
        assert_eq!(b.focused, 2);
        assert_eq!(b.line_offsets(), vec![0, 5, 6, 6]);
        // Not an image block.
        assert_eq!(b.move_image(0, 3), None);
    }

    #[test]
    fn typing_after_an_inserted_image_goes_below_it() {
        use text_editor::{Action, Edit, Motion};
        let mut b = Blocks::from_body("Weekend\n\nDrove out early.\n");
        b.focused_text()
            .unwrap()
            .perform(Action::Move(Motion::DocumentEnd));
        let r = images::parse_line("![s](assets/s.png)").unwrap();
        let next = b.insert_image(r);
        assert_eq!(b.focused, next);
        for c in "Tomorrow".chars() {
            b.focused_text()
                .unwrap()
                .perform(Action::Edit(Edit::Insert(c)));
        }
        assert_eq!(
            b.body(),
            "Weekend\n\nDrove out early.\n![s](assets/s.png)\nTomorrow\n"
        );
    }
}
