// SPDX-License-Identifier: GPL-3.0-only

//! Images in notes: the `![alt](assets/x.png){frame=tint size=m}` line format,
//! the asset store, and the retro treatments applied before display.

use crate::retro::Palette;
use anyhow::{Context, Result};
use cosmic::iced::clipboard::mime::AllowedMimeTypes;
use image::{ImageBuffer, Rgba, RgbaImage};
use std::borrow::Cow;
use std::path::{Path, PathBuf};

pub const ASSETS_DIR: &str = "assets";
/// Largest edge we keep for display; treatments run on this size.
const DISPLAY_MAX: u32 = 720;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum FrameStyle {
    #[default]
    Box,
    Tint,
    Dither,
    Bezel,
    Print,
    Ascii,
    Film,
    Pixel,
}

impl FrameStyle {
    pub const ALL: [FrameStyle; 8] = [
        FrameStyle::Box,
        FrameStyle::Tint,
        FrameStyle::Dither,
        FrameStyle::Bezel,
        FrameStyle::Print,
        FrameStyle::Ascii,
        FrameStyle::Film,
        FrameStyle::Pixel,
    ];

    pub fn key(self) -> &'static str {
        match self {
            FrameStyle::Box => "box",
            FrameStyle::Tint => "tint",
            FrameStyle::Dither => "dither",
            FrameStyle::Bezel => "bezel",
            FrameStyle::Print => "print",
            FrameStyle::Ascii => "ascii",
            FrameStyle::Film => "film",
            FrameStyle::Pixel => "pixel",
        }
    }

    pub fn from_key(key: &str) -> Option<FrameStyle> {
        FrameStyle::ALL.into_iter().find(|f| f.key() == key)
    }

    pub fn label(self) -> &'static str {
        match self {
            FrameStyle::Box => "box frame",
            FrameStyle::Tint => "phosphor tint",
            FrameStyle::Dither => "dithered",
            FrameStyle::Bezel => "CRT bezel",
            FrameStyle::Print => "instant print",
            FrameStyle::Ascii => "ASCII",
            FrameStyle::Film => "film strip",
            FrameStyle::Pixel => "chunky pixels",
        }
    }

    pub fn next(self) -> FrameStyle {
        let i = FrameStyle::ALL.iter().position(|f| *f == self).unwrap_or(0);
        FrameStyle::ALL[(i + 1) % FrameStyle::ALL.len()]
    }

    /// Whether the pixels depend on the theme palette (cache key needs it).
    pub fn themed(self) -> bool {
        matches!(
            self,
            FrameStyle::Tint | FrameStyle::Dither | FrameStyle::Ascii
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Size {
    S,
    #[default]
    M,
    L,
}

impl Size {
    pub fn key(self) -> &'static str {
        match self {
            Size::S => "s",
            Size::M => "m",
            Size::L => "l",
        }
    }
    pub fn from_key(key: &str) -> Option<Size> {
        match key {
            "s" => Some(Size::S),
            "m" => Some(Size::M),
            "l" => Some(Size::L),
            _ => None,
        }
    }
    pub fn next(self) -> Size {
        match self {
            Size::S => Size::M,
            Size::M => Size::L,
            Size::L => Size::S,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Size::S => "S",
            Size::M => "M",
            Size::L => "L",
        }
    }
}

/// One `![alt](path){attrs}` reference in a note body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageRef {
    /// 0-based line in the body.
    pub line: usize,
    pub alt: String,
    /// As written (relative to the notes dir, usually `assets/…`).
    pub path: String,
    pub frame: FrameStyle,
    pub size: Size,
}

impl ImageRef {
    /// The markdown line for this reference (attributes omitted when default).
    pub fn to_markdown(&self) -> String {
        let mut attrs = Vec::new();
        if self.frame != FrameStyle::default() {
            attrs.push(format!("frame={}", self.frame.key()));
        }
        if self.size != Size::default() {
            attrs.push(format!("size={}", self.size.key()));
        }
        if attrs.is_empty() {
            format!("![{}]({})", self.alt, self.path)
        } else {
            format!("![{}]({}){{{}}}", self.alt, self.path, attrs.join(" "))
        }
    }

    pub fn file_name(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or(&self.path)
    }
}

/// Find every image line. Only whole-line references count (an image in the
/// middle of a sentence stays plain markdown).
pub fn parse_refs(body: &str) -> Vec<ImageRef> {
    body.lines()
        .enumerate()
        .filter_map(|(line, text)| {
            parse_line(text).map(|(alt, path, frame, size)| ImageRef {
                line,
                alt,
                path,
                frame,
                size,
            })
        })
        .collect()
}

fn parse_line(text: &str) -> Option<(String, String, FrameStyle, Size)> {
    let t = text.trim();
    let rest = t.strip_prefix("![")?;
    let close = rest.find("](")?;
    let alt = rest[..close].to_owned();
    let after = &rest[close + 2..];
    let end = after.find(')')?;
    let path = after[..end].trim().to_owned();
    if path.is_empty() || path.contains("://") {
        return None;
    }
    let tail = after[end + 1..].trim();
    let mut frame = FrameStyle::default();
    let mut size = Size::default();
    if let Some(attrs) = tail.strip_prefix('{').and_then(|a| a.strip_suffix('}')) {
        for attr in attrs.split_whitespace() {
            match attr.split_once('=') {
                Some(("frame", v)) => frame = FrameStyle::from_key(v).unwrap_or_default(),
                Some(("size", v)) => size = Size::from_key(v).unwrap_or_default(),
                _ => {}
            }
        }
    } else if !tail.is_empty() {
        return None;
    }
    Some((alt, path, frame, size))
}

/// Replace the line `line` of `body` with `new_line`, keeping everything else.
pub fn replace_line(body: &str, line: usize, new_line: &str) -> String {
    let mut out = String::with_capacity(body.len() + new_line.len());
    for (i, l) in body.split_inclusive('\n').enumerate() {
        if i == line {
            out.push_str(new_line);
            if l.ends_with('\n') {
                out.push('\n');
            }
        } else {
            out.push_str(l);
        }
    }
    out
}

// ---------- drag and drop payload ----------

/// A `text/uri-list` drop (files dragged from a file manager).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UriList(pub Vec<PathBuf>);

impl AllowedMimeTypes for UriList {
    fn allowed() -> Cow<'static, [String]> {
        Cow::Owned(vec!["text/uri-list".to_owned()])
    }
}

impl TryFrom<(Vec<u8>, String)> for UriList {
    type Error = anyhow::Error;

    fn try_from((data, mime): (Vec<u8>, String)) -> Result<Self> {
        anyhow::ensure!(
            mime.starts_with("text/uri-list"),
            "unsupported mime type {mime}"
        );
        let text = String::from_utf8_lossy(&data);
        let paths = text
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .filter_map(|l| l.strip_prefix("file://").map(percent_decode))
            .map(|p| {
                // file://host/path — drop a host component if present.
                let p = p.strip_prefix("localhost").unwrap_or(&p).to_owned();
                PathBuf::from(p)
            })
            .collect();
        Ok(UriList(paths))
    }
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(v) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// A directory listing for the in-app picker: folders first, then images.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PickerEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
}

pub fn list_dir(dir: &Path) -> Vec<PickerEntry> {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut entries: Vec<PickerEntry> = rd
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let path = e.path();
            let name = path.file_name()?.to_str()?.to_owned();
            if name.starts_with('.') {
                return None;
            }
            let is_dir = path.is_dir();
            (is_dir || is_image_file(&path)).then_some(PickerEntry { path, name, is_dir })
        })
        .collect();
    entries.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    entries
}

// ---------- asset store ----------

/// Copy `src` into `<notes dir>/assets/`, returning the relative path to
/// reference from markdown. Names are kept readable and made unique.
pub fn import_asset(notes_dir: &Path, src: &Path) -> Result<String> {
    let assets = notes_dir.join(ASSETS_DIR);
    std::fs::create_dir_all(&assets).context("creating assets dir")?;
    let stem = src
        .file_stem()
        .and_then(|s| s.to_str())
        .map(crate::note::slug_filename)
        .unwrap_or_else(|| "image".to_owned())
        .replace(' ', "-")
        .to_lowercase();
    let ext = src
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_else(|| "png".to_owned());
    let mut name = format!("{stem}.{ext}");
    let mut n = 2;
    while assets.join(&name).exists() {
        name = format!("{stem}-{n}.{ext}");
        n += 1;
    }
    std::fs::copy(src, assets.join(&name)).with_context(|| format!("copying {}", src.display()))?;
    Ok(format!("{ASSETS_DIR}/{name}"))
}

/// Write raw image bytes (e.g. from the clipboard) as a new PNG asset.
/// Clipboard paste is not wired yet; kept for that.
#[allow(dead_code)]
pub fn import_bytes(notes_dir: &Path, bytes: &[u8], stem: &str) -> Result<String> {
    let assets = notes_dir.join(ASSETS_DIR);
    std::fs::create_dir_all(&assets).context("creating assets dir")?;
    let img = image::load_from_memory(bytes).context("decoding pasted image")?;
    let mut name = format!("{stem}.png");
    let mut n = 2;
    while assets.join(&name).exists() {
        name = format!("{stem}-{n}.png");
        n += 1;
    }
    img.save(assets.join(&name))
        .context("saving pasted image")?;
    Ok(format!("{ASSETS_DIR}/{name}"))
}

pub fn resolve(notes_dir: &Path, rel: &str) -> PathBuf {
    let p = Path::new(rel);
    if p.is_absolute() {
        p.to_owned()
    } else {
        notes_dir.join(p)
    }
}

pub fn is_image_file(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
            .as_deref(),
        Some("png" | "jpg" | "jpeg" | "webp" | "gif" | "bmp")
    )
}

// ---------- treatments ----------

/// What the renderer gets back for one image.
#[derive(Debug, Clone)]
pub enum Processed {
    Pixels {
        width: u32,
        height: u32,
        rgba: Vec<u8>,
    },
    Ascii(String),
}

/// Load, downscale for display, and apply the frame's pixel treatment.
pub fn load_and_process(path: &Path, style: FrameStyle, palette: Palette) -> Result<Processed> {
    let img = image::open(path).with_context(|| format!("opening {}", path.display()))?;
    let (w, h) = (img.width(), img.height());
    let scale = (DISPLAY_MAX as f32 / w.max(h) as f32).min(1.0);
    let img = if scale < 1.0 {
        img.resize(
            (w as f32 * scale) as u32,
            (h as f32 * scale) as u32,
            image::imageops::FilterType::Triangle,
        )
    } else {
        img
    };
    let rgba = img.to_rgba8();
    Ok(process(rgba, style, &palette))
}

pub fn process(rgba: RgbaImage, style: FrameStyle, p: &Palette) -> Processed {
    let out = match style {
        FrameStyle::Tint => tint(&rgba, p),
        FrameStyle::Dither => dither(&rgba, p),
        FrameStyle::Pixel => pixelate(&rgba),
        FrameStyle::Bezel => scanlines(rgba),
        FrameStyle::Ascii => return Processed::Ascii(ascii(&rgba)),
        FrameStyle::Box | FrameStyle::Print | FrameStyle::Film => rgba,
    };
    Processed::Pixels {
        width: out.width(),
        height: out.height(),
        rgba: out.into_raw(),
    }
}

fn luma(px: &Rgba<u8>) -> f32 {
    (0.2126 * px[0] as f32 + 0.7152 * px[1] as f32 + 0.0722 * px[2] as f32) / 255.0
}

fn c8(c: cosmic::iced::Color) -> [f32; 3] {
    [c.r * 255.0, c.g * 255.0, c.b * 255.0]
}

fn lerp(a: [f32; 3], b: [f32; 3], t: f32) -> Rgba<u8> {
    Rgba([
        (a[0] + (b[0] - a[0]) * t) as u8,
        (a[1] + (b[1] - a[1]) * t) as u8,
        (a[2] + (b[2] - a[2]) * t) as u8,
        255,
    ])
}

/// Greyscale mapped onto bg → fg of the palette (with a slight lift).
fn tint(src: &RgbaImage, p: &Palette) -> RgbaImage {
    let (bg, fg) = (c8(p.bg), c8(p.accent));
    ImageBuffer::from_fn(src.width(), src.height(), |x, y| {
        let t = luma(src.get_pixel(x, y)).powf(0.9);
        let mut px = lerp(bg, fg, t);
        px[3] = src.get_pixel(x, y)[3];
        px
    })
}

/// Floyd–Steinberg to four palette shades: bg, mute, dim, fg.
fn dither(src: &RgbaImage, p: &Palette) -> RgbaImage {
    let shades = [c8(p.bg), c8(p.mute), c8(p.dim), c8(p.fg)];
    let (w, h) = (src.width() as usize, src.height() as usize);
    let mut buf: Vec<f32> = src.pixels().map(|px| luma(px) * 255.0).collect();
    let mut out = ImageBuffer::new(src.width(), src.height());
    for y in 0..h {
        for x in 0..w {
            let old = buf[y * w + x];
            let idx = ((old / 255.0) * 3.0).round().clamp(0.0, 3.0) as usize;
            let new = idx as f32 * 255.0 / 3.0;
            let err = old - new;
            let s = shades[idx];
            out.put_pixel(
                x as u32,
                y as u32,
                Rgba([
                    s[0] as u8,
                    s[1] as u8,
                    s[2] as u8,
                    src.get_pixel(x as u32, y as u32)[3],
                ]),
            );
            let mut spread = |dx: isize, dy: isize, wgt: f32| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && (nx as usize) < w && (ny as usize) < h {
                    buf[ny as usize * w + nx as usize] += err * wgt;
                }
            };
            spread(1, 0, 7.0 / 16.0);
            spread(-1, 1, 3.0 / 16.0);
            spread(0, 1, 5.0 / 16.0);
            spread(1, 1, 1.0 / 16.0);
        }
    }
    out
}

/// Nearest-neighbour down to ~96px on the long edge and back up.
fn pixelate(src: &RgbaImage) -> RgbaImage {
    let (w, h) = (src.width(), src.height());
    let scale = 96.0 / w.max(h) as f32;
    let (sw, sh) = (
        ((w as f32 * scale) as u32).max(1),
        ((h as f32 * scale) as u32).max(1),
    );
    let small = image::imageops::resize(src, sw, sh, image::imageops::FilterType::Nearest);
    image::imageops::resize(&small, w, h, image::imageops::FilterType::Nearest)
}

/// Darken every third row and vignette the corners — the inside of a CRT bezel.
fn scanlines(mut img: RgbaImage) -> RgbaImage {
    let (w, h) = (img.width() as f32, img.height() as f32);
    for (x, y, px) in img.enumerate_pixels_mut() {
        let mut k = if y % 3 == 0 { 0.72 } else { 1.0 };
        let dx = (x as f32 / w - 0.5) * 2.0;
        let dy = (y as f32 / h - 0.5) * 2.0;
        let r = (dx * dx + dy * dy).sqrt();
        if r > 0.7 {
            k *= 1.0 - ((r - 0.7) / 0.72).min(1.0) * 0.55;
        }
        px[0] = (px[0] as f32 * k) as u8;
        px[1] = (px[1] as f32 * k) as u8;
        px[2] = (px[2] as f32 * k) as u8;
    }
    img
}

/// Characters for luminance, ~72 columns, 2:1 cell aspect.
fn ascii(src: &RgbaImage) -> String {
    const RAMP: &[u8] = b" .:-=+*#%@";
    let cols = 72u32.min(src.width().max(1));
    let rows =
        ((src.height() as f32 / src.width().max(1) as f32) * cols as f32 * 0.5).max(1.0) as u32;
    let small = image::imageops::resize(src, cols, rows, image::imageops::FilterType::Triangle);
    let mut out = String::with_capacity((cols as usize + 1) * rows as usize);
    for y in 0..rows {
        for x in 0..cols {
            let l = luma(small.get_pixel(x, y));
            let i = ((l * (RAMP.len() - 1) as f32).round() as usize).min(RAMP.len() - 1);
            out.push(RAMP[i] as char);
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_serialise_refs() {
        let body = "# T\n![sunset](assets/sunset.png)\ntext ![inline](x.png) more\n![ridge](assets/ridge.png){frame=tint size=l}\n![web](https://x/y.png)\n";
        let refs = parse_refs(body);
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].line, 1);
        assert_eq!(refs[0].frame, FrameStyle::Box);
        assert_eq!(refs[1].frame, FrameStyle::Tint);
        assert_eq!(refs[1].size, Size::L);
        assert_eq!(refs[0].to_markdown(), "![sunset](assets/sunset.png)");
        assert_eq!(
            refs[1].to_markdown(),
            "![ridge](assets/ridge.png){frame=tint size=l}"
        );
        let mut r = refs[0].clone();
        r.frame = r.frame.next();
        let new = replace_line(body, r.line, &r.to_markdown());
        assert!(new.contains("![sunset](assets/sunset.png){frame=tint}\n"));
        assert_eq!(new.lines().count(), body.lines().count());
    }

    #[test]
    fn treatments_keep_dimensions() {
        let p = crate::retro::Theme::Phosphor.palette(&cosmic::Theme::default());
        let img = ImageBuffer::from_fn(40, 30, |x, y| {
            Rgba([(x * 6) as u8, (y * 8) as u8, 128, 255])
        });
        for style in FrameStyle::ALL {
            match process(img.clone(), style, &p) {
                Processed::Pixels {
                    width,
                    height,
                    rgba,
                } => {
                    assert_eq!((width, height), (40, 30), "{style:?}");
                    assert_eq!(rgba.len(), 40 * 30 * 4);
                }
                Processed::Ascii(text) => assert!(text.lines().count() >= 1),
            }
        }
    }

    #[test]
    fn uri_list_parses_file_urls() {
        let data = b"# comment\r\nfile:///home/me/My%20Pics/a%20b.png\r\nfile://localhost/tmp/c.jpg\r\nhttp://x/y.png\r\n".to_vec();
        let list = UriList::try_from((data, "text/uri-list".to_owned())).unwrap();
        assert_eq!(
            list.0,
            vec![
                PathBuf::from("/home/me/My Pics/a b.png"),
                PathBuf::from("/tmp/c.jpg")
            ]
        );
        assert!(UriList::try_from((vec![], "text/plain".to_owned())).is_err());
    }

    #[test]
    fn asset_import_names() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("My Photo.PNG");
        image::RgbaImage::new(4, 4).save(&src).unwrap();
        let notes = tmp.path().join("notes");
        assert_eq!(import_asset(&notes, &src).unwrap(), "assets/my-photo.png");
        assert_eq!(import_asset(&notes, &src).unwrap(), "assets/my-photo-2.png");
        assert!(is_image_file(&src));
        assert!(!is_image_file(Path::new("a.txt")));
    }
}
