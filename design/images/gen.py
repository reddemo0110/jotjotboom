import json, html
FONTS = '<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500;600&amp;family=VT323&amp;display=swap">'
MONO = "'IBM Plex Mono', 'DejaVu Sans Mono', monospace"
DISPLAY = "'VT323', 'IBM Plex Mono', monospace"
P = dict(bg='#050806', panel='#070b08', fg='#b9f2bf', dim='#57a36a', mute='#264a2e', accent='#4dff8f', accent2='#9ad3ff', border='#1f5a2f', sel='#0f3a1c')
ASCII = html.escape(open("ascii.txt").read())

STYLE = f'''<style>
  body {{ margin: 0; font-family: {MONO}; -webkit-font-smoothing: antialiased; }}
  a {{ color: {P['accent']}; }} a:hover {{ color: {P['accent2']}; }}
  @keyframes blink {{ 0%, 49% {{ opacity: 1; }} 50%, 100% {{ opacity: 0; }} }}
</style>'''

def page(inner):
    return f'''<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <script src="./support.js"></script>
</head>
<body>
<x-dc>
<helmet>
  {FONTS}
  {STYLE}
</helmet>
{inner}
</x-dc>
</body>
</html>
'''

def frame_title(title, right=""):
    r = f'<div style="position: absolute; top: -9px; right: 14px; padding: 0 6px; background: {P["panel"]}; color: {P["dim"]}; font-size: 12px; line-height: 16px">{right}</div>' if right else ''
    return f'<div style="position: absolute; top: -10px; left: 12px; padding: 0 6px; background: {P["panel"]}; color: {P["accent"]}; font-family: {DISPLAY}; font-size: 20px; line-height: 18px">{title}</div>{r}'

def editor(title, body_html, w=640, h=460, badge="saved 14:02"):
    return f'''
<div style="width: {w}px; height: {h}px; background: {P['bg']}; padding: 22px 14px 14px; box-sizing: border-box; font-family: {MONO}; font-size: 13px; color: {P['fg']}">
  <div style="position: relative; border: 1px solid {P['border']}; border-radius: 6px; background: {P['panel']}; height: 100%; box-sizing: border-box; padding: 18px 16px 12px; display: flex; flex-direction: column; gap: 10px">
    {frame_title(title, badge)}
    {body_html}
  </div>
</div>'''

def para(text):
    return f'<div style="line-height: 22px; font-size: 14px">{text}</div>'

def ghost(s):
    return f'<span style="color: {P["mute"]}; opacity: 0.6">{s}</span>'

HEAD = f'<div style="font-family: {DISPLAY}; font-size: 30px; line-height: 32px; color: {P["accent"]}">{ghost("# ")}Weekend at the coast</div>'
TXT1 = para("Drove out before sunrise and got the tent up just as the sky turned. Two pictures below, one of the ridge.")
TXT2 = para(f"Tomorrow: the lighthouse walk {ghost('#')}<span style=\"color: {P['accent2']}\">#trips/coast</span>")
MD = lambda name: f'<div style="font-size: 12px; line-height: 18px">{ghost("![sunset](assets/" + name + ")")}</div>'

# ---------- frame treatments (image 380 wide inside a 640 editor) ----------
def img_box(src):
    return f'''<div style="position: relative; border: 1px solid {P['border']}; border-radius: 6px; padding: 14px 10px 10px; background: {P['bg']}; width: 380px; box-sizing: border-box">
  {frame_title("sunset.png", "420×280")}
  <img src="{src}" style="display: block; width: 100%; border-radius: 2px">
</div>'''

def img_plain(src, extra=""):
    return f'<img src="{src}" style="display: block; width: 380px; border: 1px solid {P["border"]}; border-radius: 3px; {extra}">'

def img_bezel(src):
    return f'''<div style="width: 380px; padding: 14px; border-radius: 16px; background: #0a0d0a; box-shadow: inset 0 0 0 1px {P['border']}, inset 0 0 18px rgba(0,0,0,0.9), 0 0 24px {P['accent']}22; position: relative; box-sizing: border-box">
  <div style="position: relative; border-radius: 10px; overflow: hidden">
    <img src="{src}" style="display: block; width: 100%; filter: saturate(0.9) contrast(1.05)">
    <div style="position: absolute; inset: 0; background: repeating-linear-gradient(0deg, rgba(0,0,0,0.28) 0px, rgba(0,0,0,0.28) 1px, transparent 1px, transparent 3px); mix-blend-mode: multiply"></div>
    <div style="position: absolute; inset: 0; background: radial-gradient(ellipse at center, transparent 60%, rgba(0,0,0,0.55) 100%)"></div>
  </div>
</div>'''

def img_polaroid(src):
    return f'''<div style="width: 340px; padding: 12px 12px 42px; background: #e9e6da; transform: rotate(-1.5deg); box-shadow: 0 6px 18px rgba(0,0,0,0.6); position: relative; box-sizing: border-box; margin: 6px 0 4px 8px">
  <img src="{src}" style="display: block; width: 100%">
  <div style="position: absolute; left: 0; right: 0; bottom: 10px; text-align: center; font-family: {DISPLAY}; font-size: 22px; color: #3a3630">the ridge, 6:12am</div>
</div>'''

def img_film(src):
    holes = "".join(f'<span style="display: inline-block; width: 10px; height: 8px; background: {P["bg"]}; border-radius: 2px; margin: 0 5px"></span>' for _ in range(17))
    return f'''<div style="width: 380px; background: #0b0f0c; border: 1px solid {P['border']}; box-sizing: border-box; padding: 4px 0">
  <div style="height: 14px; line-height: 14px; text-align: center; overflow: hidden">{holes}</div>
  <div style="padding: 3px 14px; position: relative"><img src="{src}" style="display: block; width: 100%">
    <div style="position: absolute; right: 18px; bottom: 6px; font-family: {DISPLAY}; font-size: 16px; color: {P['accent']}">▶ 07A</div></div>
  <div style="height: 14px; line-height: 14px; text-align: center; overflow: hidden">{holes}</div>
</div>'''

def img_ascii():
    return f'<pre style="margin: 0; width: 380px; font-family: {MONO}; font-size: 7.6px; line-height: 8px; color: {P["accent"]}; background: {P["bg"]}; border: 1px solid {P["border"]}; padding: 8px; box-sizing: border-box; overflow: hidden">{ASCII}</pre>'

frames = {
 "Main":         ("F1 · Box frame", "Same btop frame as the panes, filename cut into the border, size badge. Zero-surprise default.", img_box("sample.png")),
 "FrameTint":    ("F2 · Phosphor tint", "Greyscale mapped onto the theme's own ramp — follows whichever palette you pick. True colour on hover.", img_plain("tint_phosphor.png")),
 "FrameDither":  ("F3 · Dithered", "Floyd–Steinberg to four theme shades. Handheld/1-bit Mac energy; text stays crisp beside it.", img_plain("dither_phosphor.png")),
 "FrameBezel":   ("F4 · CRT bezel", "Rounded bezel, inner vignette, scanlines and a faint glow. Pairs with the CRT shader later.", img_bezel("sample.png")),
 "FramePolaroid":("F5 · Instant print", "Off-white print pinned to the terminal, hand caption in VT323, slight tilt. The one that breaks the palette on purpose.", img_polaroid("sample.png")),
 "FrameAscii":   ("F6 · ASCII", "Rendered as characters in the accent colour. Lowest fidelity, most terminal. Could be a hover-to-reveal.", img_ascii()),
 "FrameFilm":    ("F7 · Film strip", "Sprocket holes top and bottom, frame number. Good for runs of several shots.", img_film("sample.png")),
 "FramePixel":   ("F8 · Chunky pixels", "Nearest-neighbour downsample to ~84px and back up. Reads as 8-bit without changing colours.", img_plain("pixel.png")),
}

for name, (title, blurb, img_html) in frames.items():
    body = HEAD + TXT1 + MD("sunset.png") + img_html + TXT2
    caption = f'<div style="position: absolute; left: 14px; right: 14px; bottom: 16px; font-size: 12px; color: {P["dim"]}; line-height: 16px"><span style="color: {P["accent"]}">{title}</span> — {blurb}</div>'
    open(f"{name}.dc.html", "w").write(page(editor("Weekend at the coast", body + caption, h=560)))

# ---------- placement options (wider editor) ----------
def thumb(src, w=150, label="sunset.png"):
    return f'''<div style="position: relative; border: 1px solid {P['border']}; border-radius: 5px; padding: 12px 6px 6px; background: {P['bg']}; width: {w}px; box-sizing: border-box; flex-shrink: 0">
  <div style="position: absolute; top: -8px; left: 8px; padding: 0 4px; background: {P['bg']}; color: {P['accent']}; font-family: {DISPLAY}; font-size: 15px; line-height: 14px">{label}</div>
  <img src="{src}" style="display: block; width: 100%; border-radius: 2px"></div>'''

def images_strip(title="images", right="3 · size M"):
    return f'''<div style="position: relative; border: 1px solid {P['border']}; border-radius: 6px; padding: 16px 10px 10px; background: {P['panel']}; display: flex; gap: 12px; align-items: flex-start">
  {frame_title(title, right)}
  {thumb("sample.png")}{thumb("dither_phosphor.png", label="ridge.png")}{thumb("tint_phosphor.png", label="camp.png")}
  <div style="margin-left: auto; align-self: center; color: {P['dim']}; font-size: 12px; text-align: right; line-height: 18px">size<br><span style="color: {P['accent']}">S</span> · <span style="color: {P['fg']}">M</span> · L</div>
</div>'''

TXTS = HEAD + TXT1 + MD("sunset.png") + MD("ridge.png") + MD("camp.png") + TXT2 + para("Packed the stove, forgot the matches. The couple in the next bay lent us a lighter and a story about a seal.")

placements = {
 "PlaceBottom": ("P1 · Bottom strip", "Images gather in a strip at the foot of the note, above the dock. Text stays a clean column; click a thumbnail to open it large.",
     f'<div style="display: flex; flex-direction: column; gap: 10px; height: 100%">{TXTS}<div style="flex-grow: 1"></div>{images_strip()}</div>'),
 "PlaceTop": ("P2 · Top strip", "Hero shots above the text, like a photo header. Best for notes that are mostly about the pictures.",
     f'<div style="display: flex; flex-direction: column; gap: 10px; height: 100%">{images_strip("images", "3 · size L")}{TXTS}</div>'),
 "PlaceRail": ("P3 · Right rail", "A margin column of figures beside the text, in note order. The closest to a classic notes layout we can do without the new editor.",
     f'''<div style="display: flex; gap: 14px; height: 100%"><div style="flex: 1 1 0; min-width: 0; display: flex; flex-direction: column; gap: 10px">{TXTS}</div>
      <div style="width: 190px; flex-shrink: 0; display: flex; flex-direction: column; gap: 14px; padding-top: 4px">{thumb("sample.png", 180)}{thumb("dither_phosphor.png", 180, "ridge.png")}{thumb("tint_phosphor.png", 180, "camp.png")}</div></div>'''),
 "PlaceInline": ("P4 · Inline (later)", "Pictures sit in the flow exactly where the ![…] line is. Needs the custom cosmic-text editor (build step 3); shown so the strip/rail can be judged against it.",
     f'''<div style="display: flex; flex-direction: column; gap: 10px">{HEAD}{TXT1}{thumb("sample.png", 260)}{para("The ridge at first light:")}{thumb("dither_phosphor.png", 260, "ridge.png")}{TXT2}</div>'''),
}
for name, (title, blurb, body) in placements.items():
    caption = f'<div style="position: absolute; left: 14px; right: 14px; bottom: 14px; font-size: 12px; color: {P["dim"]}; line-height: 16px; background: {P["panel"]}; padding-top: 6px"><span style="color: {P["accent"]}">{title}</span> — {blurb}</div>'
    open(f"{name}.dc.html", "w").write(page(editor("Weekend at the coast", body + caption, w=820, h=600)))

# ---------- canvas ----------
fr = list(frames.keys()); pl = list(placements.keys())
boards = []
for i, n in enumerate(fr):
    boards.append({"file": f"{n}.dc.html", "title": frames[n][0], "x": (i % 4) * 740, "y": (i // 4) * 700, "w": 640, "h": 560, "page": "frames"})
for i, n in enumerate(pl):
    boards.append({"file": f"{n}.dc.html", "title": placements[n][0], "x": (i % 2) * 920, "y": (i // 2) * 740, "w": 820, "h": 600, "page": "placement"})
canvas = {
  "pages": [{"id": "frames", "name": "Frame styles"}, {"id": "placement", "name": "Placement in the note"}],
  "artboards": boards,
  "annotations": [
    {"id": "how-to-read", "page": "frames", "x": 0, "y": -330, "w": 700,
     "text": "IMAGES IN JOTJOTBOOM — pick from two axes.\n\nPage 1 (this one): how a picture is FRAMED. Pick any number; they become per-image options in the editor (right-click or the dock), with one as the default.\nPage 2: WHERE pictures sit relative to the text. Pick one default; the inline option is shown for comparison but arrives with the custom editor.\n\nAdding images will work three ways: drag a file onto the note, paste from the clipboard, or the ⧉ button on the dock (file picker). The file is copied to ~/Documents/JotJotBoom/assets/ and the note gets an ![alt](assets/…) line, ghosted like other syntax.\n\nAll previews use the real conversions (tint, dither, ASCII, pixel) I'd ship, on the same sample photo."},
    {"id": "sizes", "page": "placement", "x": 0, "y": -170, "w": 620,
     "text": "Sizes: each image can be S (thumbnail), M, or L (full width of the strip/rail), per image, remembered in the note as ![alt](assets/x.png){size=m}. Frames from page 1 apply inside any placement."}
  ],
  "launch": {"view": "canvas", "page": "frames"},
}
json.dump(canvas, open("canvas.json", "w"), indent=2)
print("written", len(boards), "artboards")
