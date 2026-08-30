// SPDX-License-Identifier: GPL-3.0-only

//! Link cards: a web address or an attached file on a line of its own is
//! shown as a card (title, description, picture, domain — or file name,
//! kind and size) instead of a bare link. The markdown line stays as it is
//! written; the fetched preview is derived data cached under
//! `assets/.links/` and can be deleted at any time.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Where previews are cached, relative to the notes dir.
pub const CACHE_DIR: &str = "assets/.links";
const MAX_HTML: u64 = 1_500_000;
const MAX_IMAGE: u64 = 8_000_000;
const THUMB: u32 = 320;
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux) JotJotBoom/0.2 link-preview";

/// One `[text](target)` or bare `https://…` line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkRef {
    /// The link text; empty for a bare address.
    pub text: String,
    /// A web address or a path relative to the notes dir.
    pub target: String,
}

impl LinkRef {
    pub fn is_web(&self) -> bool {
        is_web_url(&self.target)
    }

    pub fn to_markdown(&self) -> String {
        if self.text.is_empty() {
            self.target.clone()
        } else {
            format!("[{}]({})", self.text, self.target)
        }
    }

    /// `typ.io/fonts/open_sans` for a web link; the file name otherwise.
    pub fn short(&self) -> String {
        if self.is_web() {
            let s = self
                .target
                .trim_start_matches("https://")
                .trim_start_matches("http://")
                .trim_start_matches("www.")
                .trim_end_matches('/');
            truncate(s, 60)
        } else {
            self.file_name().to_owned()
        }
    }

    pub fn file_name(&self) -> &str {
        self.target.rsplit('/').next().unwrap_or(&self.target)
    }

    /// The card's headline: the link text, else the file name.
    pub fn label(&self) -> String {
        if self.text.is_empty() {
            self.file_name().to_owned()
        } else {
            self.text.clone()
        }
    }
}

pub fn is_web_url(s: &str) -> bool {
    (s.starts_with("https://") || s.starts_with("http://"))
        && s.len() > 8
        && !s.contains(char::is_whitespace)
}

/// Parse a line that is only a link. Wiki links and images are not links
/// here; a link inside a sentence stays inline.
pub fn parse_line(line: &str) -> Option<LinkRef> {
    let t = line.trim();
    if is_web_url(t) {
        return Some(LinkRef {
            text: String::new(),
            target: t.to_owned(),
        });
    }
    let rest = t.strip_prefix('[')?;
    if rest.starts_with('[') {
        return None;
    }
    let close = rest.find("](")?;
    let text = rest[..close].trim();
    let after = &rest[close + 2..];
    let end = after.rfind(')')?;
    if end + 1 != after.len() {
        return None;
    }
    let target = after[..end].trim();
    if target.is_empty() || text.is_empty() || text.contains('[') {
        return None;
    }
    let ok = is_web_url(target) || (!target.contains("://") && !target.starts_with('#'));
    ok.then(|| LinkRef {
        text: text.to_owned(),
        target: target.to_owned(),
    })
}

// ---------- attached files ----------

/// What a file card shows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileInfo {
    /// `PDF`, `ZIP`, `TXT`, … from the extension.
    pub kind: String,
    /// `4.5 MB`, or empty when the file is missing.
    pub size: String,
    pub exists: bool,
}

pub fn file_info(path: &Path) -> FileInfo {
    let kind = path
        .extension()
        .and_then(|e| e.to_str())
        .map_or_else(|| "FILE".to_owned(), str::to_ascii_uppercase);
    match std::fs::metadata(path) {
        Ok(m) => FileInfo {
            kind,
            size: human_size(m.len()),
            exists: true,
        },
        Err(_) => FileInfo {
            kind,
            size: String::new(),
            exists: false,
        },
    }
}

pub fn kind_label(kind: &str) -> String {
    match kind {
        "PDF" => "PDF document".to_owned(),
        "ZIP" | "TAR" | "GZ" | "7Z" | "XZ" => "Archive".to_owned(),
        "TXT" | "MD" => "Text".to_owned(),
        "DOC" | "DOCX" | "ODT" => "Word document".to_owned(),
        "XLS" | "XLSX" | "ODS" | "CSV" => "Spreadsheet".to_owned(),
        "PPT" | "PPTX" | "ODP" => "Presentation".to_owned(),
        "MP3" | "FLAC" | "OGG" | "WAV" | "M4A" => "Audio".to_owned(),
        "MP4" | "MKV" | "WEBM" | "MOV" => "Video".to_owned(),
        "EPUB" | "MOBI" => "E-book".to_owned(),
        other => format!("{other} file"),
    }
}

pub fn human_size(bytes: u64) -> String {
    let b = bytes as f64;
    if b < 1024.0 {
        format!("{bytes} B")
    } else if b < 1024.0 * 1024.0 {
        format!("{:.0} KB", b / 1024.0)
    } else if b < 1024.0 * 1024.0 * 1024.0 {
        format!("{:.1} MB", b / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", b / (1024.0 * 1024.0 * 1024.0))
    }
}

// ---------- web previews ----------

/// What a web card shows, once fetched.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Preview {
    pub title: String,
    pub description: String,
    /// Absolute path of the cached thumbnail, if the page had a picture.
    pub image: Option<PathBuf>,
}

fn cache_key(url: &str) -> String {
    blake3::hash(url.as_bytes()).to_hex()[..20].to_owned()
}

fn cache_paths(notes_dir: &Path, url: &str) -> (PathBuf, PathBuf) {
    let dir = notes_dir.join(CACHE_DIR);
    let key = cache_key(url);
    (
        dir.join(format!("{key}.txt")),
        dir.join(format!("{key}.png")),
    )
}

/// A preview fetched earlier, if any.
pub fn load_cached(notes_dir: &Path, url: &str) -> Option<Preview> {
    let (meta, img) = cache_paths(notes_dir, url);
    let text = std::fs::read_to_string(meta).ok()?;
    let mut p = Preview::default();
    for line in text.lines() {
        if let Some(v) = line.strip_prefix("title=") {
            v.clone_into(&mut p.title);
        } else if let Some(v) = line.strip_prefix("description=") {
            v.clone_into(&mut p.description);
        }
    }
    p.image = img.exists().then_some(img);
    Some(p)
}

/// Forget a cached preview so the next look fetches it again.
pub fn forget(notes_dir: &Path, url: &str) {
    let (meta, img) = cache_paths(notes_dir, url);
    let _ = std::fs::remove_file(meta);
    let _ = std::fs::remove_file(img);
}

/// Fetch a page, read its Open Graph / title / description, cache a
/// thumbnail of its picture. Blocking — run on a worker thread.
///
/// `JJB_LINK_FIXTURE=/path/page.html` serves that file for every address
/// instead of going online (the headless harness has no network); an
/// `og:image` there may be a local path next to the fixture.
pub fn fetch(notes_dir: &Path, url: &str) -> Result<Preview> {
    let fixture = std::env::var_os("JJB_LINK_FIXTURE").map(PathBuf::from);
    let html = match &fixture {
        Some(path) => std::fs::read_to_string(path).context("reading link fixture")?,
        None => get_text(url)?,
    };
    let scraped = scrape(&html);
    let mut preview = Preview {
        title: scraped.title,
        description: scraped.description,
        image: None,
    };
    if let Some(src) = scraped.image {
        let bytes = match &fixture {
            Some(path) if !is_web_url(&src) => {
                let local = path.parent().unwrap_or(Path::new(".")).join(&src);
                std::fs::read(local).ok()
            }
            _ => get_bytes(&absolute(url, &src)).ok(),
        };
        if let Some(bytes) = bytes
            && let Ok(img) = image::load_from_memory(&bytes)
        {
            let (_, img_path) = cache_paths(notes_dir, url);
            if let Some(dir) = img_path.parent() {
                std::fs::create_dir_all(dir).context("creating link cache")?;
            }
            let thumb = img.thumbnail(THUMB, THUMB);
            if thumb.save(&img_path).is_ok() {
                preview.image = Some(img_path);
            }
        }
    }
    let (meta, _) = cache_paths(notes_dir, url);
    if let Some(dir) = meta.parent() {
        std::fs::create_dir_all(dir).context("creating link cache")?;
    }
    std::fs::write(
        meta,
        format!(
            "url={url}\ntitle={}\ndescription={}\n",
            preview.title.replace('\n', " "),
            preview.description.replace('\n', " ")
        ),
    )
    .context("writing link preview")?;
    Ok(preview)
}

fn get_text(url: &str) -> Result<String> {
    let mut resp = ureq::get(url)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "text/html,application/xhtml+xml")
        .call()
        .with_context(|| format!("fetching {url}"))?;
    let ct = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_owned();
    if !(ct.contains("html") || ct.contains("xml") || ct.is_empty()) {
        anyhow::bail!("not a web page ({ct})");
    }
    resp.body_mut()
        .with_config()
        .limit(MAX_HTML)
        .read_to_string()
        .context("reading page")
}

fn get_bytes(url: &str) -> Result<Vec<u8>> {
    let mut resp = ureq::get(url)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "image/*")
        .call()
        .with_context(|| format!("fetching {url}"))?;
    resp.body_mut()
        .with_config()
        .limit(MAX_IMAGE)
        .read_to_vec()
        .context("reading image")
}

/// Resolve `src` against the page address (`//cdn…`, `/path`, `rel`).
fn absolute(page: &str, src: &str) -> String {
    if is_web_url(src) {
        return src.to_owned();
    }
    let scheme_end = page.find("://").map_or(0, |i| i + 3);
    let scheme = &page[..scheme_end];
    let rest = &page[scheme_end..];
    let host = rest.split('/').next().unwrap_or(rest);
    if let Some(s) = src.strip_prefix("//") {
        return format!("{scheme}{s}");
    }
    if let Some(s) = src.strip_prefix('/') {
        return format!("{scheme}{host}/{s}");
    }
    let base = match page.rfind('/') {
        Some(i) if i >= scheme_end => &page[..=i],
        _ => return format!("{scheme}{host}/{src}"),
    };
    format!("{base}{src}")
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Scraped {
    pub title: String,
    pub description: String,
    pub image: Option<String>,
}

/// Open Graph first, then Twitter cards, then plain `<title>` and
/// `<meta name=description>`. No HTML parser: tags are scanned for.
pub fn scrape(html: &str) -> Scraped {
    let mut og_title = None;
    let mut og_desc = None;
    let mut og_image = None;
    let mut tw_title = None;
    let mut tw_desc = None;
    let mut tw_image = None;
    let mut meta_desc = None;
    for tag in tags(html, "meta") {
        let key = attr(tag, "property")
            .or_else(|| attr(tag, "name"))
            .unwrap_or_default()
            .to_ascii_lowercase();
        let Some(content) = attr(tag, "content") else {
            continue;
        };
        let content = decode_entities(content.trim());
        if content.is_empty() {
            continue;
        }
        match key.as_str() {
            "og:title" => og_title.get_or_insert(content),
            "og:description" => og_desc.get_or_insert(content),
            "og:image" | "og:image:url" | "og:image:secure_url" => og_image.get_or_insert(content),
            "twitter:title" => tw_title.get_or_insert(content),
            "twitter:description" => tw_desc.get_or_insert(content),
            "twitter:image" | "twitter:image:src" => tw_image.get_or_insert(content),
            "description" => meta_desc.get_or_insert(content),
            _ => continue,
        };
    }
    let title_tag = title_text(html);
    Scraped {
        title: truncate(
            &og_title.or(tw_title).or(title_tag).unwrap_or_default(),
            160,
        ),
        description: truncate(&og_desc.or(tw_desc).or(meta_desc).unwrap_or_default(), 300),
        image: og_image.or(tw_image),
    }
}

/// Every `<name …>` tag body (without the angle brackets), case-insensitive.
fn tags<'a>(html: &'a str, name: &str) -> Vec<&'a str> {
    let lower = html.to_ascii_lowercase();
    let needle = format!("<{name}");
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(i) = lower[from..].find(&needle) {
        let start = from + i + needle.len();
        if !lower[start..].starts_with(|c: char| c.is_whitespace() || c == '/' || c == '>') {
            from = start;
            continue;
        }
        let Some(len) = html[start..].find('>') else {
            break;
        };
        out.push(&html[start..start + len]);
        from = start + len;
    }
    out
}

/// The value of `name="…"` (or `'…'`, or bare) in a tag body.
fn attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let lower = tag.to_ascii_lowercase();
    let mut from = 0;
    while let Some(i) = lower[from..].find(name) {
        let at = from + i;
        let before_ok = at == 0 || !lower.as_bytes()[at - 1].is_ascii_alphanumeric();
        let after = &tag[at + name.len()..];
        let after_t = after.trim_start();
        if before_ok && let Some(rest) = after_t.strip_prefix('=') {
            let rest = rest.trim_start();
            return Some(match rest.chars().next() {
                Some(q @ ('"' | '\'')) => {
                    let body = &rest[1..];
                    &body[..body.find(q).unwrap_or(body.len())]
                }
                _ => rest.split(char::is_whitespace).next().unwrap_or(""),
            });
        }
        from = at + name.len();
    }
    None
}

fn title_text(html: &str) -> Option<String> {
    let lower = html.to_ascii_lowercase();
    let start = lower.find("<title")?;
    let open_end = start + lower[start..].find('>')?;
    let close = open_end + lower[open_end..].find("</title")?;
    let raw = html[open_end + 1..close].trim();
    let t = decode_entities(raw);
    (!t.is_empty()).then_some(t)
}

/// `&amp;` and friends, plus numeric references.
pub fn decode_entities(s: &str) -> String {
    if !s.contains('&') {
        return s.split_whitespace().collect::<Vec<_>>().join(" ");
    }
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        let Some(end) = rest.find(';').filter(|e| *e <= 10) else {
            out.push('&');
            rest = &rest[1..];
            continue;
        };
        let ent = &rest[1..end];
        let decoded = match ent {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            "nbsp" => Some(' '),
            "ndash" => Some('–'),
            "mdash" => Some('—'),
            "hellip" => Some('…'),
            "rsquo" => Some('’'),
            "lsquo" => Some('‘'),
            "rdquo" => Some('”'),
            "ldquo" => Some('“'),
            _ => ent
                .strip_prefix('#')
                .and_then(|n| {
                    n.strip_prefix(['x', 'X'])
                        .and_then(|h| u32::from_str_radix(h, 16).ok())
                        .or_else(|| n.parse().ok())
                })
                .and_then(char::from_u32),
        };
        match decoded {
            Some(c) => {
                out.push(c);
                rest = &rest[end + 1..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_owned();
    }
    let mut t: String = s.chars().take(max.saturating_sub(1)).collect();
    t.push('…');
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_lines_parse_and_round_trip() {
        let bare = parse_line("  https://typ.io/fonts/open_sans ").unwrap();
        assert_eq!(bare.text, "");
        assert!(bare.is_web());
        assert_eq!(bare.to_markdown(), "https://typ.io/fonts/open_sans");
        assert_eq!(bare.short(), "typ.io/fonts/open_sans");

        let titled =
            parse_line("[Open Sans in action | Typ.io](https://typ.io/fonts/open_sans)").unwrap();
        assert_eq!(titled.text, "Open Sans in action | Typ.io");
        assert_eq!(
            titled.to_markdown(),
            "[Open Sans in action | Typ.io](https://typ.io/fonts/open_sans)"
        );

        let file = parse_line("[brief.pdf](assets/brief.pdf)").unwrap();
        assert!(!file.is_web());
        assert_eq!(file.file_name(), "brief.pdf");
        assert_eq!(file.label(), "brief.pdf");

        assert!(parse_line("see https://example.com for more").is_none());
        assert!(parse_line("[[Wiki link]]").is_none());
        assert!(parse_line("![pic](assets/pic.png)").is_none());
        assert!(parse_line("[a](https://x.io) and [b](https://y.io)").is_none());
        assert!(parse_line("[anchor](#top)").is_none());
        assert!(parse_line("https://").is_none());
    }

    #[test]
    fn scrapes_open_graph_then_falls_back() {
        let html = r#"<html><head><TITLE>Fallback &amp; title</TITLE>
            <meta name="description" content="Plain description">
            <meta property="og:title" content="Open Sans in action | Typ.io" />
            <meta content='It goes well with Montserrat &#8212; and more' property='og:description'>
            <meta property="og:image" content="/img/open_sans.png">
            </head><body></body></html>"#;
        let s = scrape(html);
        assert_eq!(s.title, "Open Sans in action | Typ.io");
        assert_eq!(s.description, "It goes well with Montserrat — and more");
        assert_eq!(s.image.as_deref(), Some("/img/open_sans.png"));

        let s = scrape("<title>Just a   title</title><meta name=description content=bare>");
        assert_eq!(s.title, "Just a title");
        assert_eq!(s.description, "bare");
        assert!(s.image.is_none());
        // A <metadata> tag is not a <meta> tag.
        assert!(
            scrape("<metadata property=\"og:title\" content=\"x\">")
                .title
                .is_empty()
        );
    }

    #[test]
    fn urls_resolve_against_the_page() {
        let page = "https://typ.io/fonts/open_sans";
        assert_eq!(absolute(page, "/img/a.png"), "https://typ.io/img/a.png");
        assert_eq!(absolute(page, "//cdn.x.io/a.png"), "https://cdn.x.io/a.png");
        assert_eq!(absolute(page, "a.png"), "https://typ.io/fonts/a.png");
        assert_eq!(absolute("https://typ.io", "a.png"), "https://typ.io/a.png");
        assert_eq!(absolute(page, "https://o.io/b.png"), "https://o.io/b.png");
    }

    #[test]
    fn entities_and_sizes() {
        assert_eq!(
            decode_entities("a &lt;b&gt; &#x41;&#66; &bogus; c"),
            "a <b> AB &bogus; c"
        );
        assert_eq!(human_size(900), "900 B");
        assert_eq!(human_size(4_718_592), "4.5 MB");
        assert_eq!(kind_label("PDF"), "PDF document");
        assert_eq!(truncate("abcdef", 4), "abc…");
    }

    #[test]
    fn cache_round_trips_and_fixture_fetch_works() {
        let dir = std::env::temp_dir().join(format!("jjb-links-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let fixture = dir.join("page.html");
        std::fs::write(
            &fixture,
            "<title>Fixture page</title><meta property=\"og:description\" content=\"Hello\">\
             <meta property=\"og:image\" content=\"pic.png\">",
        )
        .unwrap();
        image::RgbaImage::from_pixel(8, 8, image::Rgba([200, 40, 40, 255]))
            .save(dir.join("pic.png"))
            .unwrap();
        // The fixture switch is process-wide; the test is the only user.
        unsafe { std::env::set_var("JJB_LINK_FIXTURE", &fixture) };
        let url = "https://example.com/x";
        assert!(load_cached(&dir, url).is_none());
        let p = fetch(&dir, url).unwrap();
        unsafe { std::env::remove_var("JJB_LINK_FIXTURE") };
        assert_eq!(p.title, "Fixture page");
        assert_eq!(p.description, "Hello");
        assert!(p.image.as_ref().is_some_and(|i| i.exists()));
        assert_eq!(load_cached(&dir, url), Some(p));
        forget(&dir, url);
        assert!(load_cached(&dir, url).is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
