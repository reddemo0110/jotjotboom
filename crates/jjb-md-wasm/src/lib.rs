// SPDX-License-Identifier: GPL-3.0-only

//! The markdown scanner for the browser: the same `scan_line` the COSMIC
//! editor uses, compiled to WASM so the CodeMirror editor paints exactly the
//! spans the native app does. Results travel as JSON strings; the JS side
//! parses them (`JSON.parse`) — no serde-wasm-bindgen, no glue types.
//!
//! Byte offsets are UTF-8 byte offsets into the line, as native; the JS
//! side converts to UTF-16 indexes where CodeMirror needs them.

use jjb_core::markdown::{Kind, Span, scan_line as native_scan_line};
use serde::Serialize;
use wasm_bindgen::prelude::*;

/// One scanned line: its spans and the fence state for the next line.
#[derive(Serialize)]
pub struct Scanned {
    pub spans: Vec<Span>,
    pub in_fence: bool,
}

/// Scan one line; `in_fence` is whether it starts inside a ``` block.
/// Returns `{"spans":[{"range":{"start":0,"end":2},"kind":"marker"},…],"in_fence":false}`.
#[wasm_bindgen]
pub fn scan_line(line: &str, in_fence: bool) -> String {
    let (spans, in_fence) = native_scan_line(line, in_fence);
    serde_json::to_string(&Scanned { spans, in_fence }).unwrap_or_default()
}

/// Scan a whole text, line by line, threading the fence state; one entry
/// per line, in order.
#[wasm_bindgen]
pub fn scan_text(text: &str) -> String {
    let mut in_fence = false;
    let mut out = Vec::new();
    for line in text.lines() {
        let (spans, next) = native_scan_line(line, in_fence);
        out.push(Scanned {
            spans,
            in_fence: next,
        });
        in_fence = next;
    }
    serde_json::to_string(&out).unwrap_or_default()
}

/// The span kinds, as the JSON spells them (for building a CSS class map).
#[wasm_bindgen]
pub fn kinds() -> String {
    const ALL: [Kind; 19] = [
        Kind::Marker,
        Kind::Bold,
        Kind::Italic,
        Kind::BoldItalic,
        Kind::Code,
        Kind::CodeBlock,
        Kind::Heading,
        Kind::Tag,
        Kind::Link,
        Kind::LinkUrl,
        Kind::ListMarker,
        Kind::NumMarker,
        Kind::Quote,
        Kind::Done,
        Kind::Strike,
        Kind::Mark,
        Kind::TaskBox,
        Kind::TaskDone,
        Kind::QuoteMarker,
    ];
    serde_json::to_string(&ALL).unwrap_or_default()
}
