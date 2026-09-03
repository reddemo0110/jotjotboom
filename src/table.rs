// SPDX-License-Identifier: GPL-3.0-only

//! Tables: GitHub pipe tables in the file, a live grid in the editor.
//!
//! A cell starting with `=` is a formula, Keynote style: the grid shows the
//! computed value, editing shows the text. Supported: numbers, `+ - * /`,
//! parentheses, cell refs (`B2`), ranges (`B2:B5`) inside functions, and
//! `SUM AVG MIN MAX COUNT` (case-insensitive; `AVERAGE` works too).
//!
//! Money is contagious, Keynote style: a cell like `$42.50` (also € £ ¥,
//! commas allowed) is still a number to formulas, and a formula that
//! touches money shows money — `=SUM(B2:B4)` over costs reads `$845.50`.
//! A symbol right after the `=` (`=$SUM(…)`) forces the format by hand.
//!
//! Column and row sizes are presentation, not content: they ride in an
//! HTML comment after the table (`<!-- jjb:table cols=… rows=… -->`),
//! which every other markdown renderer hides.

/// One table: raw cell text (formulas keep their `=`), separator-row
/// alignment for round-tripping, and any manual column/row sizes (0 = auto).
#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    /// All rows, header first. Rectangular: every row has `cols()` cells.
    pub rows: Vec<Vec<String>>,
    pub align: Vec<Align>,
    /// Manual column widths in px, parallel to columns; 0 = auto.
    pub widths: Vec<f32>,
    /// Manual row heights in px, parallel to `rows`; 0 = auto.
    pub heights: Vec<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
}

impl Table {
    /// A fresh table with an empty header row and `rows` empty data rows.
    pub fn starter(cols: usize, rows: usize) -> Table {
        Table {
            rows: (0..rows + 1).map(|_| vec![String::new(); cols]).collect(),
            align: vec![Align::Left; cols],
            widths: vec![0.0; cols],
            heights: vec![0.0; rows + 1],
        }
    }

    pub fn cols(&self) -> usize {
        self.align.len()
    }

    pub fn cell(&self, row: usize, col: usize) -> &str {
        self.rows
            .get(row)
            .and_then(|r| r.get(col))
            .map_or("", String::as_str)
    }

    pub fn set_cell(&mut self, row: usize, col: usize, text: String) {
        if let Some(cell) = self.rows.get_mut(row).and_then(|r| r.get_mut(col)) {
            *cell = text;
        }
    }

    /// What the grid shows for a cell: the computed value for a formula
    /// (money-formatted when the formula touched money, or when a symbol
    /// right after the `=` forces it), a normalised money value for a
    /// `$12.3` cell, the text itself otherwise.
    pub fn display(&self, row: usize, col: usize) -> String {
        let raw = self.cell(row, col);
        if let Some(expr) = raw.strip_prefix('=') {
            let (force, expr) = split_force(expr);
            match eval(expr, self, &mut vec![(row, col)]) {
                Ok((v, money)) => match force.or(money) {
                    Some(sym) => fmt_money(v, sym),
                    None => fmt_number(v),
                },
                Err(()) => "#ERR".to_owned(),
            }
        } else if let Some((v, sym)) = parse_money(raw) {
            fmt_money(v, sym)
        } else {
            raw.to_owned()
        }
    }

    pub fn add_row(&mut self) {
        self.rows.push(vec![String::new(); self.cols()]);
        self.heights.push(0.0);
    }

    pub fn add_col(&mut self) {
        for row in &mut self.rows {
            row.push(String::new());
        }
        self.align.push(Align::Left);
        self.widths.push(0.0);
    }

    pub fn remove_row(&mut self, row: usize) {
        if self.rows.len() > 2 && row > 0 && row < self.rows.len() {
            self.rows.remove(row);
            self.heights.remove(row);
        }
    }

    pub fn remove_col(&mut self, col: usize) {
        if self.cols() > 1 && col < self.cols() {
            for row in &mut self.rows {
                row.remove(col);
            }
            self.align.remove(col);
            self.widths.remove(col);
        }
    }

    /// Parse a run of table lines (and an optional trailing size comment).
    /// The second line must be the separator row.
    pub fn parse(lines: &[&str]) -> Option<Table> {
        if lines.len() < 2 || !separator_line(lines[1]) {
            return None;
        }
        let align: Vec<Align> = split_row(lines[1])
            .iter()
            .map(|c| {
                let c = c.trim();
                match (c.starts_with(':'), c.ends_with(':')) {
                    (true, true) => Align::Center,
                    (false, true) => Align::Right,
                    _ => Align::Left,
                }
            })
            .collect();
        let cols = align.len();
        let mut rows = Vec::new();
        let mut widths = vec![0.0; cols];
        let mut heights = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            if i == 1 {
                continue;
            }
            if let Some(sizes) = line.trim().strip_prefix("<!-- jjb:table") {
                let sizes = sizes.trim_end_matches("-->").trim();
                for part in sizes.split_whitespace() {
                    if let Some(list) = part.strip_prefix("cols=") {
                        for (j, w) in list.split(',').enumerate() {
                            if let (Some(slot), Ok(w)) = (widths.get_mut(j), w.parse()) {
                                *slot = w;
                            }
                        }
                    } else if let Some(list) = part.strip_prefix("rows=") {
                        for (j, h) in list.split(',').enumerate() {
                            if let (Some(slot), Ok(h)) = (heights.get_mut(j), h.parse()) {
                                *slot = h;
                            }
                        }
                    }
                }
                continue;
            }
            let mut cells = split_row(line);
            cells.resize(cols, String::new());
            cells.truncate(cols);
            rows.push(cells);
            heights.push(0.0);
        }
        (!rows.is_empty()).then_some(Table {
            rows,
            align,
            widths,
            heights,
        })
    }

    /// Back to markdown (plus the size comment when any size is manual).
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        let esc = |c: &str| c.replace('|', "\\|");
        for (i, row) in self.rows.iter().enumerate() {
            out.push('|');
            for cell in row {
                out.push(' ');
                out.push_str(&esc(cell));
                out.push_str(" |");
            }
            out.push('\n');
            if i == 0 {
                out.push('|');
                for a in &self.align {
                    out.push_str(match a {
                        Align::Left => " --- |",
                        Align::Center => " :---: |",
                        Align::Right => " ---: |",
                    });
                }
                out.push('\n');
            }
        }
        let manual = |v: &[f32]| v.iter().any(|w| *w > 0.0);
        if manual(&self.widths) || manual(&self.heights) {
            let list = |v: &[f32]| {
                v.iter()
                    .map(|w| format!("{}", w.round() as i64))
                    .collect::<Vec<_>>()
                    .join(",")
            };
            out.push_str("<!-- jjb:table");
            if manual(&self.widths) {
                out.push_str(&format!(" cols={}", list(&self.widths)));
            }
            if manual(&self.heights) {
                out.push_str(&format!(" rows={}", list(&self.heights)));
            }
            out.push_str(" -->\n");
        }
        out.pop();
        out
    }
}

/// `| a | b |` → cells. A `\|` stays a literal pipe.
fn split_row(line: &str) -> Vec<String> {
    let line = line.trim();
    let line = line.strip_prefix('|').unwrap_or(line);
    let line = line.strip_suffix('|').unwrap_or(line);
    let mut cells = Vec::new();
    let mut cur = String::new();
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some('|') => cur.push('|'),
                Some(o) => {
                    cur.push('\\');
                    cur.push(o);
                }
                None => cur.push('\\'),
            },
            '|' => cells.push(std::mem::take(&mut cur).trim().to_owned()),
            c => cur.push(c),
        }
    }
    cells.push(cur.trim().to_owned());
    cells
}

/// Is this a `| --- | :---: |` separator row?
pub fn separator_line(line: &str) -> bool {
    let t = line.trim();
    if !t.starts_with('|') && !t.contains('|') {
        return false;
    }
    let cells = split_row(t);
    !cells.is_empty()
        && cells.iter().all(|c| {
            let c = c.trim();
            !c.is_empty()
                && c.trim_start_matches(':')
                    .trim_end_matches(':')
                    .chars()
                    .all(|ch| ch == '-')
                && c.contains('-')
        })
}

/// Does `line` look like a table row at all?
pub fn table_line(line: &str) -> bool {
    line.trim_start().starts_with('|')
}

/// The trailing size comment.
pub fn size_comment(line: &str) -> bool {
    line.trim_start().starts_with("<!-- jjb:table")
}

/// Column index → letters (0 = A, 26 = AA).
pub fn col_name(mut col: usize) -> String {
    let mut name = String::new();
    loop {
        name.insert(0, (b'A' + (col % 26) as u8) as char);
        if col < 26 {
            break;
        }
        col = col / 26 - 1;
    }
    name
}

/// Round-trip friendly number: integers stay integers.
fn fmt_number(v: f64) -> String {
    if !v.is_finite() {
        return "#ERR".to_owned();
    }
    let rounded = (v * 10_000.0).round() / 10_000.0;
    if rounded.fract() == 0.0 && rounded.abs() < 1e15 {
        format!("{}", rounded as i64)
    } else {
        format!("{rounded}")
    }
}

/// The currency symbols a cell may carry.
pub const MONEY: [char; 4] = ['$', '\u{20ac}', '\u{a3}', '\u{a5}'];

/// `$1,234.5` / `-\u{20ac}3` → the number and its symbol.
pub fn parse_money(s: &str) -> Option<(f64, char)> {
    let s = s.trim();
    let (neg, s) = match s.strip_prefix('-') {
        Some(rest) => (true, rest.trim_start()),
        None => (false, s),
    };
    let sym = s.chars().next().filter(|c| MONEY.contains(c))?;
    let v: f64 = s[sym.len_utf8()..].replace(',', "").trim().parse().ok()?;
    Some((if neg { -v } else { v }, sym))
}

/// A leading currency symbol in a formula body (`$SUM(…)`) forces money.
fn split_force(expr: &str) -> (Option<char>, &str) {
    match expr.chars().next() {
        Some(c) if MONEY.contains(&c) => (Some(c), &expr[c.len_utf8()..]),
        _ => (None, expr),
    }
}

/// Two decimals, thousands grouped: `-$1,234.50`.
fn fmt_money(v: f64, sym: char) -> String {
    if !v.is_finite() {
        return "#ERR".to_owned();
    }
    let cents = (v.abs() * 100.0).round() as i64;
    let (int, frac) = (cents / 100, cents % 100);
    let digits = int.to_string();
    let mut grouped = String::new();
    for (i, c) in digits.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(c);
    }
    let int_s: String = grouped.chars().rev().collect();
    let neg = if v < 0.0 && cents != 0 { "-" } else { "" };
    format!("{neg}{sym}{int_s}.{frac:02}")
}

/// The money toggle: cycle a cell's format `plain → $ → \u{20ac} → \u{a3} → \u{a5} → plain`.
/// Formulas cycle the forcing symbol after `=`; anything non-numeric is
/// returned unchanged.
pub fn cycle_money(text: &str) -> String {
    let next = |cur: Option<char>| match cur {
        None => Some(MONEY[0]),
        Some(c) => MONEY
            .iter()
            .position(|m| *m == c)
            .and_then(|i| MONEY.get(i + 1))
            .copied(),
    };
    if let Some(expr) = text.strip_prefix('=') {
        let (force, rest) = split_force(expr);
        match next(force) {
            Some(sym) => format!("={sym}{rest}"),
            None => format!("={rest}"),
        }
    } else if let Some((_, sym)) = parse_money(text) {
        let bare = text.replacen(sym, "", 1);
        let bare = bare.trim();
        match next(Some(sym)) {
            Some(sym) => prefix_money(bare, sym),
            None => bare.to_owned(),
        }
    } else if text.trim().replace(',', "").parse::<f64>().is_ok() {
        prefix_money(text.trim(), MONEY[0])
    } else {
        text.to_owned()
    }
}

/// `-5` → `-$5`, `5` → `$5`.
fn prefix_money(bare: &str, sym: char) -> String {
    match bare.strip_prefix('-') {
        Some(rest) => format!("-{sym}{}", rest.trim_start()),
        None => format!("{sym}{bare}"),
    }
}

/// The fill handle's smart copy: shift every cell reference in a formula
/// by (rows, cols) — `=SUM(C2*C3)` dragged one row down becomes
/// `=SUM(C3*C4)`. Values and anything that is not a formula come back
/// unchanged; a reference pushed off the table's edge becomes `#REF`.
pub fn translate_formula(text: &str, dr: isize, dc: isize) -> String {
    let Some(expr) = text.strip_prefix('=') else {
        return text.to_owned();
    };
    let mut out = String::from("=");
    let chars: Vec<(usize, char)> = expr.char_indices().collect();
    let mut i = 0;
    while i < chars.len() {
        let (_, c) = chars[i];
        if c.is_ascii_alphabetic() {
            let start = chars[i].0;
            while i < chars.len() && chars[i].1.is_ascii_alphabetic() {
                i += 1;
            }
            let letters_end = chars.get(i).map_or(expr.len(), |(b, _)| *b);
            let digit_start = i;
            while i < chars.len() && chars[i].1.is_ascii_digit() {
                i += 1;
            }
            let end = chars.get(i).map_or(expr.len(), |(b, _)| *b);
            if digit_start < i
                && let Ok((row, col)) =
                    parse_ref(&expr[start..letters_end], &expr[letters_end..end])
            {
                let (nr, nc) = (row as isize + dr, col as isize + dc);
                if nr < 0 || nc < 0 {
                    out.push_str("#REF");
                } else {
                    out.push_str(&col_name(nc as usize));
                    out.push_str(&(nr + 1).to_string());
                }
            } else {
                out.push_str(&expr[start..end]);
            }
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}

// ---------- the formula engine ----------

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Num(f64),
    Ref(usize, usize),
    /// `A1:B3`, only meaningful as a function argument.
    Range(usize, usize, usize, usize),
    Func(String),
    Plus,
    Minus,
    Star,
    Slash,
    Open,
    Close,
    Comma,
}

fn lex(expr: &str) -> Result<Vec<Tok>, ()> {
    let mut toks = Vec::new();
    let bytes = expr.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        match c {
            ' ' | '\t' => i += 1,
            '+' => {
                toks.push(Tok::Plus);
                i += 1;
            }
            '-' => {
                toks.push(Tok::Minus);
                i += 1;
            }
            '*' => {
                toks.push(Tok::Star);
                i += 1;
            }
            '/' => {
                toks.push(Tok::Slash);
                i += 1;
            }
            '(' => {
                toks.push(Tok::Open);
                i += 1;
            }
            ')' => {
                toks.push(Tok::Close);
                i += 1;
            }
            ',' | ';' => {
                toks.push(Tok::Comma);
                i += 1;
            }
            '0'..='9' | '.' => {
                let start = i;
                while i < bytes.len() && matches!(bytes[i] as char, '0'..='9' | '.') {
                    i += 1;
                }
                toks.push(Tok::Num(expr[start..i].parse().map_err(|_| ())?));
            }
            'a'..='z' | 'A'..='Z' => {
                let start = i;
                while i < bytes.len() && (bytes[i] as char).is_ascii_alphabetic() {
                    i += 1;
                }
                let letters = &expr[start..i];
                let digit_start = i;
                while i < bytes.len() && (bytes[i] as char).is_ascii_digit() {
                    i += 1;
                }
                if digit_start < i {
                    // A cell ref, maybe the start of a range.
                    let a = parse_ref(letters, &expr[digit_start..i])?;
                    if bytes.get(i) == Some(&b':') {
                        i += 1;
                        let ls = i;
                        while i < bytes.len() && (bytes[i] as char).is_ascii_alphabetic() {
                            i += 1;
                        }
                        let ds = i;
                        while i < bytes.len() && (bytes[i] as char).is_ascii_digit() {
                            i += 1;
                        }
                        if ls == ds || ds == i {
                            return Err(());
                        }
                        let b = parse_ref(&expr[ls..ds], &expr[ds..i])?;
                        toks.push(Tok::Range(
                            a.0.min(b.0),
                            a.1.min(b.1),
                            a.0.max(b.0),
                            a.1.max(b.1),
                        ));
                    } else {
                        toks.push(Tok::Ref(a.0, a.1));
                    }
                } else {
                    toks.push(Tok::Func(letters.to_ascii_uppercase()));
                }
            }
            _ => return Err(()),
        }
    }
    Ok(toks)
}

/// `B2` → (row 1, col 1). Rows are 1-based in formulas, header = row 1.
fn parse_ref(letters: &str, digits: &str) -> Result<(usize, usize), ()> {
    let mut col = 0usize;
    for c in letters.chars() {
        col = col * 26 + (c.to_ascii_uppercase() as usize - 'A' as usize + 1);
    }
    let row: usize = digits.parse().map_err(|_| ())?;
    if col == 0 || row == 0 {
        return Err(());
    }
    Ok((row - 1, col - 1))
}

struct Parser<'a> {
    toks: &'a [Tok],
    pos: usize,
    table: &'a Table,
    visiting: &'a mut Vec<(usize, usize)>,
    /// The first currency symbol met among referenced cells: money spreads.
    money: Option<char>,
}

/// Evaluate `expr` against the table. `visiting` carries the chain of
/// formula cells being resolved, so circular references fail instead of
/// recursing forever.
fn eval(
    expr: &str,
    table: &Table,
    visiting: &mut Vec<(usize, usize)>,
) -> Result<(f64, Option<char>), ()> {
    let toks = lex(expr)?;
    let mut p = Parser {
        toks: &toks,
        pos: 0,
        table,
        visiting,
        money: None,
    };
    let v = p.expr()?;
    if p.pos != p.toks.len() {
        return Err(());
    }
    Ok((v, p.money))
}

impl Parser<'_> {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }

    fn next(&mut self) -> Option<Tok> {
        let t = self.toks.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn expr(&mut self) -> Result<f64, ()> {
        let mut v = self.term()?;
        loop {
            match self.peek() {
                Some(Tok::Plus) => {
                    self.pos += 1;
                    v += self.term()?;
                }
                Some(Tok::Minus) => {
                    self.pos += 1;
                    v -= self.term()?;
                }
                _ => return Ok(v),
            }
        }
    }

    fn term(&mut self) -> Result<f64, ()> {
        let mut v = self.factor()?;
        loop {
            match self.peek() {
                Some(Tok::Star) => {
                    self.pos += 1;
                    v *= self.factor()?;
                }
                Some(Tok::Slash) => {
                    self.pos += 1;
                    v /= self.factor()?;
                }
                _ => return Ok(v),
            }
        }
    }

    fn factor(&mut self) -> Result<f64, ()> {
        match self.next().ok_or(())? {
            Tok::Num(n) => Ok(n),
            Tok::Minus => Ok(-self.factor()?),
            Tok::Ref(r, c) => self.cell_value(r, c)?.ok_or(()),
            Tok::Open => {
                let v = self.expr()?;
                match self.next() {
                    Some(Tok::Close) => Ok(v),
                    _ => Err(()),
                }
            }
            Tok::Func(name) => {
                if self.next() != Some(Tok::Open) {
                    return Err(());
                }
                let mut values: Vec<f64> = Vec::new();
                if self.peek() != Some(&Tok::Close) {
                    loop {
                        if let Some(Tok::Range(r0, c0, r1, c1)) = self.peek().cloned() {
                            self.pos += 1;
                            for r in r0..=r1 {
                                for c in c0..=c1 {
                                    if let Some(v) = self.cell_value(r, c)? {
                                        values.push(v);
                                    }
                                }
                            }
                        } else {
                            values.push(self.expr()?);
                        }
                        match self.next() {
                            Some(Tok::Comma) => continue,
                            Some(Tok::Close) => break,
                            _ => return Err(()),
                        }
                    }
                } else {
                    self.pos += 1;
                }
                let n = values.len() as f64;
                match name.as_str() {
                    "SUM" => Ok(values.iter().sum()),
                    "AVG" | "AVERAGE" | "MEAN" => {
                        if values.is_empty() {
                            Err(())
                        } else {
                            Ok(values.iter().sum::<f64>() / n)
                        }
                    }
                    "MIN" => values.iter().copied().reduce(f64::min).ok_or(()),
                    "MAX" => values.iter().copied().reduce(f64::max).ok_or(()),
                    "COUNT" => Ok(n),
                    _ => Err(()),
                }
            }
            _ => Err(()),
        }
    }

    /// A referenced cell's numeric value: numbers parse, formulas resolve
    /// (watching for cycles), text and empties are `None`.
    fn cell_value(&mut self, row: usize, col: usize) -> Result<Option<f64>, ()> {
        if row >= self.table.rows.len() || col >= self.table.cols() {
            return Ok(None);
        }
        let raw = self.table.cell(row, col).trim();
        if let Some(inner) = raw.strip_prefix('=') {
            if self.visiting.contains(&(row, col)) {
                return Err(());
            }
            let (force, inner) = split_force(inner);
            self.visiting.push((row, col));
            let v = eval(inner, self.table, self.visiting);
            self.visiting.pop();
            let (v, money) = v?;
            self.money = self.money.or(force).or(money);
            return Ok(Some(v));
        }
        if let Some((v, sym)) = parse_money(raw) {
            self.money = self.money.or(Some(sym));
            return Ok(Some(v));
        }
        Ok(raw.parse().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid(cells: &[&[&str]]) -> Table {
        let cols = cells[0].len();
        Table {
            rows: cells
                .iter()
                .map(|r| r.iter().map(|c| (*c).to_owned()).collect())
                .collect(),
            align: vec![Align::Left; cols],
            widths: vec![0.0; cols],
            heights: vec![0.0; cells.len()],
        }
    }

    #[test]
    fn parse_and_round_trip() {
        let md = "| Item | Cost |\n| --- | ---: |\n| Tea | 4.5 |\n| Buns | 3 |";
        let lines: Vec<&str> = md.lines().collect();
        let t = Table::parse(&lines).unwrap();
        assert_eq!(t.cols(), 2);
        assert_eq!(t.rows.len(), 3);
        assert_eq!(t.cell(2, 0), "Buns");
        assert_eq!(t.align[1], Align::Right);
        let back = t.to_markdown();
        let again = Table::parse(&back.lines().collect::<Vec<_>>()).unwrap();
        assert_eq!(t, again);
    }

    #[test]
    fn sizes_ride_the_comment() {
        let mut t = Table::starter(2, 1);
        t.widths[0] = 120.0;
        t.heights[1] = 40.0;
        let md = t.to_markdown();
        assert!(md.contains("<!-- jjb:table cols=120,0 rows=0,40 -->"));
        let again = Table::parse(&md.lines().collect::<Vec<_>>()).unwrap();
        assert_eq!(again.widths[0], 120.0);
        assert_eq!(again.heights[1], 40.0);
    }

    #[test]
    fn escaped_pipes_survive() {
        let mut t = Table::starter(1, 1);
        t.set_cell(1, 0, "a|b".into());
        let md = t.to_markdown();
        let again = Table::parse(&md.lines().collect::<Vec<_>>()).unwrap();
        assert_eq!(again.cell(1, 0), "a|b");
    }

    #[test]
    fn arithmetic_and_refs() {
        let t = grid(&[&["h", "h"], &["4", "=A2*2"], &["=A2+B2", "10"]]);
        assert_eq!(t.display(1, 1), "8");
        assert_eq!(t.display(2, 0), "12");
        let t = grid(&[&["h"], &["=2+3*4"], &["=(2+3)*4"], &["=-2+10"], &["=10/4"]]);
        assert_eq!(t.display(1, 0), "14");
        assert_eq!(t.display(2, 0), "20");
        assert_eq!(t.display(3, 0), "8");
        assert_eq!(t.display(4, 0), "2.5");
    }

    #[test]
    fn functions_over_ranges() {
        let t = grid(&[
            &["Item", "Cost"],
            &["Tea", "4.5"],
            &["Buns", "3"],
            &["Jam", "2.5"],
            &["", "=SUM(B2:B4)"],
            &["", "=AVG(B2:B4)"],
            &["", "=MIN(B2:B4)"],
            &["", "=MAX(B2:B4)"],
            &["", "=COUNT(B2:B4)"],
        ]);
        assert_eq!(t.display(4, 1), "10");
        assert_eq!(t.display(5, 1), "3.3333");
        assert_eq!(t.display(6, 1), "2.5");
        assert_eq!(t.display(7, 1), "4.5");
        assert_eq!(t.display(8, 1), "3");
        // Text cells are skipped, not errors.
        let t = grid(&[&["h"], &["x"], &["2"], &["=SUM(A1:A3)"]]);
        assert_eq!(t.display(3, 0), "2");
    }

    #[test]
    fn errors_and_cycles() {
        let t = grid(&[&["h"], &["=NOPE(1)"], &["=1+"], &["=A4"], &["=A3"]]);
        assert_eq!(t.display(1, 0), "#ERR");
        assert_eq!(t.display(2, 0), "#ERR");
        // A3 and A4 reference each other: both error, neither hangs.
        assert_eq!(t.display(3, 0), "#ERR");
        let t = grid(&[&["h"], &["=A2"]]);
        assert_eq!(t.display(1, 0), "#ERR");
    }

    #[test]
    fn money_is_contagious() {
        let t = grid(&[
            &["Item", "Cost"],
            &["Tea", "$4.5"],
            &["Buns", "$1,200"],
            &["", "=SUM(B2:B3)"],
            &["", "=SUM(B2:B3)/2"],
            &["plain", "=2+3"],
        ]);
        assert_eq!(t.display(1, 1), "$4.50");
        assert_eq!(t.display(2, 1), "$1,200.00");
        assert_eq!(t.display(3, 1), "$1,204.50");
        assert_eq!(t.display(4, 1), "$602.25");
        assert_eq!(t.display(5, 1), "5");
        // A forced symbol wins even with no money in the refs.
        let t = grid(&[&["h"], &["3"], &["=\u{20ac}A2*2"]]);
        assert_eq!(t.display(2, 0), "\u{20ac}6.00");
    }

    #[test]
    fn fill_translates_refs() {
        assert_eq!(translate_formula("=SUM(C2*C3)", 1, 0), "=SUM(C3*C4)");
        assert_eq!(translate_formula("=SUM(B2:B4)", 2, 0), "=SUM(B4:B6)");
        assert_eq!(translate_formula("=A1+10", 0, 1), "=B1+10");
        assert_eq!(translate_formula("=\u{20ac}SUM(A1)", 1, 0), "=\u{20ac}SUM(A2)");
        // Values copy verbatim; off-the-edge refs flag themselves.
        assert_eq!(translate_formula("$4.50", 3, 0), "$4.50");
        assert_eq!(translate_formula("=A1", -1, 0), "=#REF");
        // Function names keep their letters.
        assert_eq!(translate_formula("=MAX(B2,3)", 1, 0), "=MAX(B3,3)");
    }

    #[test]
    fn money_cycle() {
        assert_eq!(cycle_money("42.5"), "$42.5");
        assert_eq!(cycle_money("$42.5"), "\u{20ac}42.5");
        assert_eq!(cycle_money("\u{a5}42.5"), "42.5");
        assert_eq!(cycle_money("-7"), "-$7");
        assert_eq!(cycle_money("-$7"), "-\u{20ac}7");
        assert_eq!(cycle_money("=SUM(B2:B4)"), "=$SUM(B2:B4)");
        assert_eq!(cycle_money("=$SUM(B2:B4)"), "=\u{20ac}SUM(B2:B4)");
        assert_eq!(cycle_money("=\u{a3}SUM(B2:B4)"), "=\u{a5}SUM(B2:B4)");
        // Words are left alone.
        assert_eq!(cycle_money("hello"), "hello");
    }

    #[test]
    fn col_names() {
        assert_eq!(col_name(0), "A");
        assert_eq!(col_name(25), "Z");
        assert_eq!(col_name(26), "AA");
    }
}
