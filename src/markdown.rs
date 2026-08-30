// SPDX-License-Identifier: GPL-3.0-only

//! Line-oriented markdown scanner and the editor highlighter built on it.
//!
//! The raw markdown stays the source of truth; this only decides how each
//! byte range of a line is painted: bold text bold, tags and links in the
//! secondary accent, and the syntax markers themselves in a "ghost" colour
//! so the text reads as formatted while the cursor can still move over them.

use crate::retro::Palette;
use cosmic::iced::font::{Style, Weight};
use cosmic::iced::{Color, Font};
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Syntax that is not content: `**`, `#`, `[[`, fences, rules.
    Marker,
    Bold,
    Italic,
    BoldItalic,
    Code,
    CodeBlock,
    Heading,
    Tag,
    /// Wiki-link target or markdown link label.
    Link,
    /// A URL / image reference: shown ghosted.
    LinkUrl,
    ListMarker,
    Quote,
    /// Text of a completed task.
    Done,
    Strike,
    /// An unticked task box `[ ] ` (with its trailing space).
    TaskBox,
    /// A ticked task box `[x] ` (with its trailing space).
    TaskDone,
    /// The `> ` that opens a quote line.
    QuoteMarker,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    pub range: Range<usize>,
    pub kind: Kind,
}

/// Scan one line. `in_fence` is whether the line starts inside a ``` block;
/// the returned bool is the state for the next line.
pub fn scan_line(line: &str, in_fence: bool) -> (Vec<Span>, bool) {
    let trimmed = line.trim_start();
    let indent = line.len() - trimmed.len();
    if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
        return (
            vec![Span {
                range: 0..line.len(),
                kind: Kind::Marker,
            }],
            !in_fence,
        );
    }
    if in_fence {
        return (
            vec![Span {
                range: 0..line.len(),
                kind: Kind::CodeBlock,
            }],
            true,
        );
    }
    let t = trimmed.trim_end();
    if t.len() >= 3
        && (t.bytes().all(|b| b == b'-')
            || t.bytes().all(|b| b == b'*')
            || t.bytes().all(|b| b == b'_'))
    {
        return (
            vec![Span {
                range: 0..line.len(),
                kind: Kind::Marker,
            }],
            false,
        );
    }

    let mut spans = Vec::new();
    let mut pos = indent;
    let mut base: Option<Kind> = None;
    let bytes = line.as_bytes();

    // Heading: 1-6 hashes then a space.
    let hashes = bytes[pos..].iter().take_while(|&&b| b == b'#').count();
    if (1..=6).contains(&hashes) && bytes.get(pos + hashes) == Some(&b' ') {
        spans.push(Span {
            range: pos..pos + hashes + 1,
            kind: Kind::Marker,
        });
        pos += hashes + 1;
        base = Some(Kind::Heading);
    } else if bytes.get(pos) == Some(&b'>') {
        let end = if bytes.get(pos + 1) == Some(&b' ') {
            pos + 2
        } else {
            pos + 1
        };
        spans.push(Span {
            range: pos..end,
            kind: Kind::QuoteMarker,
        });
        pos = end;
        base = Some(Kind::Quote);
    } else if matches!(bytes.get(pos), Some(b'-' | b'*' | b'+'))
        && bytes.get(pos + 1) == Some(&b' ')
    {
        spans.push(Span {
            range: pos..pos + 2,
            kind: Kind::ListMarker,
        });
        pos += 2;
        // Task box: `[ ]` open, `[x]` / `[✓]` / `[🦆]` … done. The span
        // keeps its trailing space so a drawn box leaves a gap.
        if let Some((len, done)) = crate::note::task_box(&line[pos..]) {
            spans.push(Span {
                range: pos..pos + len,
                kind: if done { Kind::TaskDone } else { Kind::TaskBox },
            });
            pos += len;
            if done {
                base = Some(Kind::Done);
            }
        }
    } else {
        let digits = bytes[pos..]
            .iter()
            .take_while(|b| b.is_ascii_digit())
            .count();
        if digits > 0
            && matches!(bytes.get(pos + digits), Some(b'.' | b')'))
            && bytes.get(pos + digits + 1) == Some(&b' ')
        {
            spans.push(Span {
                range: pos..pos + digits + 2,
                kind: Kind::ListMarker,
            });
            pos += digits + 2;
        }
    }

    scan_inline(line, pos, base, &mut spans);
    (spans, false)
}

fn scan_inline(line: &str, start: usize, base: Option<Kind>, spans: &mut Vec<Span>) {
    let bytes = line.as_bytes();
    let n = bytes.len();
    let mut i = start;
    let mut gap_start = start;

    let push =
        |spans: &mut Vec<Span>, gap_start: &mut usize, from: usize, to: usize, kind: Kind| {
            if let Some(base) = base
                && from > *gap_start
            {
                spans.push(Span {
                    range: *gap_start..from,
                    kind: base,
                });
            }
            if to > from {
                spans.push(Span {
                    range: from..to,
                    kind,
                });
            }
            *gap_start = to;
        };

    while i < n {
        let b = bytes[i];
        match b {
            b'`' => {
                if let Some(rel) = line[i + 1..].find('`') {
                    let j = i + 1 + rel;
                    push(spans, &mut gap_start, i, i + 1, Kind::Marker);
                    push(spans, &mut gap_start, i + 1, j, Kind::Code);
                    push(spans, &mut gap_start, j, j + 1, Kind::Marker);
                    i = j + 1;
                    continue;
                }
                i += 1;
            }
            b'*' | b'_' | b'~' => {
                let run = bytes[i..].iter().take_while(|&&c| c == b).count().min(3);
                let run = if b == b'~' {
                    if run >= 2 { 2 } else { 0 }
                } else {
                    run
                };
                let after = i + run;
                let flanking = run > 0 && after < n && !bytes[after].is_ascii_whitespace();
                if flanking {
                    let delim = &line[i..after];
                    if let Some(rel) = find_closing(line, after, delim) {
                        let k = after + rel;
                        let kind = match (b, run) {
                            (b'~', _) => Kind::Strike,
                            (_, 1) => Kind::Italic,
                            (_, 2) => Kind::Bold,
                            _ => Kind::BoldItalic,
                        };
                        push(spans, &mut gap_start, i, after, Kind::Marker);
                        push(spans, &mut gap_start, after, k, kind);
                        push(spans, &mut gap_start, k, k + run, Kind::Marker);
                        i = k + run;
                        continue;
                    }
                }
                i += run.max(1);
            }
            b'!' if bytes.get(i + 1) == Some(&b'[') => {
                if let Some(mut end) = link_end(line, i + 1) {
                    // Trailing `{frame=… size=…}` attributes belong to the image.
                    if bytes.get(end) == Some(&b'{')
                        && let Some(close) = line[end..].find('}')
                    {
                        end = end + close + 1;
                    }
                    push(spans, &mut gap_start, i, end, Kind::LinkUrl);
                    i = end;
                    continue;
                }
                i += 1;
            }
            b'[' if bytes.get(i + 1) == Some(&b'[') => {
                if let Some(rel) = line[i + 2..].find("]]") {
                    let j = i + 2 + rel;
                    push(spans, &mut gap_start, i, i + 2, Kind::Marker);
                    push(spans, &mut gap_start, i + 2, j, Kind::Link);
                    push(spans, &mut gap_start, j, j + 2, Kind::Marker);
                    i = j + 2;
                    continue;
                }
                i += 1;
            }
            b'[' => {
                if let Some(close) = line[i + 1..].find(']').map(|r| r + i + 1)
                    && bytes.get(close + 1) == Some(&b'(')
                    && let Some(end) = line[close + 2..].find(')').map(|r| r + close + 2)
                {
                    push(spans, &mut gap_start, i, i + 1, Kind::Marker);
                    push(spans, &mut gap_start, i + 1, close, Kind::Link);
                    push(spans, &mut gap_start, close, close + 2, Kind::Marker);
                    push(spans, &mut gap_start, close + 2, end, Kind::LinkUrl);
                    push(spans, &mut gap_start, end, end + 1, Kind::Marker);
                    i = end + 1;
                    continue;
                }
                i += 1;
            }
            b'#' => {
                let boundary = i == start
                    || bytes[i - 1].is_ascii_whitespace()
                    || matches!(bytes[i - 1], b'(' | b'[' | b'{' | b',' | b';');
                if boundary {
                    let tag_len: usize = line[i + 1..]
                        .chars()
                        .take_while(|&c| c.is_alphanumeric() || matches!(c, '_' | '-' | '/'))
                        .map(char::len_utf8)
                        .sum();
                    let tag = &line[i + 1..i + 1 + tag_len];
                    if tag.chars().any(char::is_alphabetic) {
                        push(spans, &mut gap_start, i, i + 1 + tag_len, Kind::Tag);
                        i += 1 + tag_len;
                        continue;
                    }
                }
                i += 1;
            }
            _ => i += 1,
        }
    }
    if let Some(base) = base
        && n > gap_start
    {
        spans.push(Span {
            range: gap_start..n,
            kind: base,
        });
    }
}

/// Find `delim` at or after `from`, where the char before it is not whitespace.
fn find_closing(line: &str, from: usize, delim: &str) -> Option<usize> {
    let mut search = from;
    while let Some(rel) = line[search..].find(delim) {
        let k = search + rel;
        let prev_ws = line[..k]
            .chars()
            .next_back()
            .is_some_and(char::is_whitespace);
        if k > from && !prev_ws {
            return Some(k - from);
        }
        search = k + delim.len();
    }
    None
}

/// For `[label](target)` with `line[start] == '['`, the byte index just past `)`.
fn link_end(line: &str, start: usize) -> Option<usize> {
    let close = line[start..].find(']')? + start;
    if line.as_bytes().get(close + 1) != Some(&b'(') {
        return None;
    }
    let end = line[close + 2..].find(')')? + close + 2;
    Some(end + 1)
}

// ---------- the iced highlighter ----------

#[derive(Clone, PartialEq)]
pub struct Settings {
    pub palette: Palette,
    pub show_markers: bool,
    pub font: Font,
    /// Folder icons by tag; a tag wearing one shows it instead of its `#`.
    pub tag_icons: std::sync::Arc<std::collections::HashMap<String, crate::pixel::Icon>>,
}

#[derive(Clone, Copy, Debug)]
pub struct Highlight {
    pub color: Option<Color>,
    pub font: Option<Font>,
}

/// Colour and font for a span kind under `settings` — shared by iced's
/// highlighter path and the rich editor.
pub fn style_for(kind: Kind, settings: &Settings) -> Highlight {
    {
        let p = &settings.palette;
        let base = settings.font;
        let bold = Font {
            weight: Weight::Bold,
            ..base
        };
        let italic = Font {
            style: Style::Italic,
            ..base
        };
        let bold_italic = Font {
            weight: Weight::Bold,
            style: Style::Italic,
            ..base
        };
        let ghost = if settings.show_markers {
            p.dim
        } else {
            p.mute.scale_alpha(0.45)
        };
        let (color, font) = match kind {
            Kind::Marker | Kind::QuoteMarker => (ghost, None),
            Kind::Bold => (p.fg, Some(bold)),
            Kind::Italic => (p.fg, Some(italic)),
            Kind::BoldItalic => (p.fg, Some(bold_italic)),
            Kind::Code | Kind::CodeBlock => (p.accent2, None),
            Kind::Heading => (p.accent, Some(bold)),
            Kind::Tag | Kind::Link => (p.accent2, None),
            Kind::LinkUrl => (ghost, None),
            Kind::ListMarker => (p.accent, None),
            Kind::Quote => (p.dim, Some(italic)),
            // Finished tasks fade into the theme rather than change colour.
            Kind::Done | Kind::Strike => (p.fg.scale_alpha(0.45), None),
            Kind::TaskBox => (p.dim, None),
            Kind::TaskDone => (p.accent, Some(bold)),
        };
        Highlight {
            color: Some(color),
            font,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(line: &str) -> Vec<(&str, Kind)> {
        let (spans, _) = scan_line(line, false);
        spans
            .iter()
            .map(|s| (&line[s.range.clone()], s.kind))
            .collect()
    }

    #[test]
    fn heading_with_inline() {
        assert_eq!(
            kinds("## Big **bold** day"),
            vec![
                ("## ", Kind::Marker),
                ("Big ", Kind::Heading),
                ("**", Kind::Marker),
                ("bold", Kind::Bold),
                ("**", Kind::Marker),
                (" day", Kind::Heading),
            ]
        );
    }

    #[test]
    fn emphasis_code_tags_links() {
        assert_eq!(
            kinds("a *b* `c` #tag [[Note|x]] [l](u) ~~s~~ x*y"),
            vec![
                ("*", Kind::Marker),
                ("b", Kind::Italic),
                ("*", Kind::Marker),
                ("`", Kind::Marker),
                ("c", Kind::Code),
                ("`", Kind::Marker),
                ("#tag", Kind::Tag),
                ("[[", Kind::Marker),
                ("Note|x", Kind::Link),
                ("]]", Kind::Marker),
                ("[", Kind::Marker),
                ("l", Kind::Link),
                ("](", Kind::Marker),
                ("u", Kind::LinkUrl),
                (")", Kind::Marker),
                ("~~", Kind::Marker),
                ("s", Kind::Strike),
                ("~~", Kind::Marker),
            ]
        );
    }

    #[test]
    fn image_lines_ghost_their_attributes() {
        assert_eq!(
            kinds("![alt](assets/a.png){frame=tint size=l} after"),
            vec![("![alt](assets/a.png){frame=tint size=l}", Kind::LinkUrl)]
        );
    }

    #[test]
    fn unmatched_markers_are_literal() {
        assert_eq!(kinds("2 * 3 and a_b and **open"), vec![]);
        assert_eq!(kinds("not#tag #123"), vec![]);
    }

    #[test]
    fn lists_tasks_quotes_rules_fences() {
        assert_eq!(
            kinds("- [x] done ✓"),
            vec![
                ("- ", Kind::ListMarker),
                ("[x] ", Kind::TaskDone),
                ("done ✓", Kind::Done)
            ]
        );
        assert_eq!(kinds("3. third"), vec![("3. ", Kind::ListMarker)]);
        assert_eq!(
            kinds("> quoted"),
            vec![("> ", Kind::QuoteMarker), ("quoted", Kind::Quote)]
        );
        assert_eq!(kinds("---"), vec![("---", Kind::Marker)]);
        let (_, fence) = scan_line("```rust", false);
        assert!(fence);
        let (spans, still) = scan_line("let x = 1; **not bold**", true);
        assert_eq!(spans[0].kind, Kind::CodeBlock);
        assert!(still);
    }

    #[test]
    fn unicode_is_safe() {
        // Ranges must land on char boundaries.
        for line in [
            "héllo **wörld** #tägs",
            "日本語 *強調* [[リンク]]",
            "— ünïcödé —",
        ] {
            let (spans, _) = scan_line(line, false);
            for s in spans {
                assert!(line.is_char_boundary(s.range.start) && line.is_char_boundary(s.range.end));
            }
        }
    }
}
