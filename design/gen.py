"""Generates the design-canvas artboards for the retro/btop direction."""
import json

FONTS = '<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500;600&amp;family=VT323&amp;display=swap">'
MONO = "'IBM Plex Mono', 'DejaVu Sans Mono', 'Liberation Mono', monospace"
DISPLAY = "'VT323', 'IBM Plex Mono', 'DejaVu Sans Mono', monospace"
SYS = "system-ui, 'Fira Sans', 'Noto Sans', sans-serif"

# ---------- shared logic (theme tweak -> palette) ----------
LOGIC = r"""
class Component extends DCLogic {
  renderVals() {
    const theme = this.props.theme ?? 'phosphor';
    const crt = this.props.crt ?? true;
    const focus = this.props.focus ?? false;
    const themes = {
      phosphor:    { bg:'#050806', panel:'#070b08', fg:'#b9f2bf', dim:'#3f7a4a', mute:'#264a2e', accent:'#4dff8f', accent2:'#9ad3ff', border:'#1f5a2f', sel:'#0f3a1c', selfg:'#d7ffe0', cursor:'#4dff8f' },
      amber:       { bg:'#0b0704', panel:'#0e0905', fg:'#ffc978', dim:'#8a5a1f', mute:'#4a3010', accent:'#ffb02e', accent2:'#ffe9a8', border:'#5c3c12', sel:'#3a2408', selfg:'#fff1d6', cursor:'#ffb02e' },
      wordperfect: { bg:'#0000aa', panel:'#0000aa', fg:'#e8e8e8', dim:'#8a8aff', mute:'#4a4aff', accent:'#ffff55', accent2:'#55ffff', border:'#9a9aff', sel:'#0000ff', selfg:'#ffffff', cursor:'#ffffff' },
      cosmic:      { bg:'#1c1c1e', panel:'#232326', fg:'#e4e4e7', dim:'#8a8a92', mute:'#4a4a52', accent:'#8ab4f8', accent2:'#c4a7ff', border:'#3a3a40', sel:'#2f3a4d', selfg:'#ffffff', cursor:'#8ab4f8' },
    };
    const t = { ...themes[theme] };
    t.glow = crt ? `0 0 6px ${t.accent}66` : 'none';
    t.titleGlow = crt ? `0 0 10px ${t.accent}99` : 'none';
    t.dimmed = focus ? 0.32 : 1;
    return { t, crt, focus };
  }
}
"""

def props(extra=""):
    return ('{"theme":{"editor":"enum","default":"phosphor","options":["phosphor","amber","wordperfect","cosmic"],"section":"Look"},'
            '"crt":{"editor":"boolean","default":true,"section":"Look"},'
            '"focus":{"editor":"boolean","default":false,"section":"Look"}' + extra + '}')

# ---------- SVG icons (stroke, 16px grid) ----------
def ico(path, size=16, color="currentColor", sw=1.5):
    return (f'<svg width="{size}" height="{size}" viewBox="0 0 16 16" fill="none" stroke="{color}" '
            f'stroke-width="{sw}" stroke-linecap="round" stroke-linejoin="round" style="flex-shrink: 0">{path}</svg>')
I_NAV = ico('<rect x="2" y="3" width="12" height="10" rx="1.5"></rect><path d="M6 3v10"></path>')
I_MIN = ico('<path d="M4 8h8"></path>')
I_MAX = ico('<rect x="3.5" y="3.5" width="9" height="9" rx="1"></rect>')
I_CLOSE = ico('<path d="M4 4l8 8M12 4l-8 8"></path>')
I_PLUS = ico('<path d="M8 3v10M3 8h10"></path>')
I_PIN = ico('<path d="M9.5 2.5l4 4-2.5 1 -1.5 4-2.5-2.5L3 13l1.5-4-2.5-2.5 4-1.5z"></path>')
I_TRASH = ico('<path d="M3 4.5h10M6.5 4.5v-1h3v1M4.5 4.5l.7 8h5.6l.7-8"></path>')
I_FOLDER = ico('<path d="M2 4.5h4l1.5 1.5H14v7H2z"></path>', 16)
I_LIST = ico('<path d="M3 4.5h1M6 4.5h7M3 8h1M6 8h7M3 11.5h1M6 11.5h7"></path>')
I_SEARCH = ico('<circle cx="7" cy="7" r="4"></circle><path d="M10 10l3.5 3.5"></path>')

# ---------- COSMIC header bar (native, never themed) ----------
def header(with_menu=True):
    menu = ('<div style="display: flex; gap: 18px; font-size: 14px; color: #c9c9cc">'
            '<span>File</span><span>View</span></div>') if with_menu else ''
    return f'''
<div style="height: 46px; background: #1f1f22; border-bottom: 1px solid #2c2c30; display: flex; align-items: center; padding: 0 12px; gap: 16px; font-family: {SYS}; color: #c9c9cc">
  <div style="display: flex; align-items: center; color: #c9c9cc">{I_NAV}</div>
  {menu}
  <div style="flex-grow: 1"></div>
  <div style="display: flex; align-items: center; gap: 14px; color: #c9c9cc">{I_PIN}{I_TRASH}{I_PLUS}
    <span style="width: 1px; height: 18px; background: #3a3a40"></span>{I_MIN}{I_MAX}{I_CLOSE}</div>
</div>'''

# ---------- btop-style frame ----------
def frame(title, body, right=None, grow=False, width=None, height=None, extra_style=""):
    size = ""
    if width: size += f" width: {width}px; flex-shrink: 0;"
    if height: size += f" height: {height}px; flex-shrink: 0;"
    if grow: size += " flex-grow: 1; min-height: 0;"
    right_html = (f'<div style="position: absolute; top: -9px; right: 14px; padding: 0 6px; background: {{{{t.panel}}}}; '
                  f'color: {{{{t.dim}}}}; font-size: 12px; line-height: 16px">{right}</div>') if right else ''
    return f'''
<div style="position: relative; border: 1px solid {{{{t.border}}}}; border-radius: 6px; background: {{{{t.panel}}}}; padding: 16px 12px 10px; display: flex; flex-direction: column; gap: 8px; min-width: 0;{size}{extra_style}">
  <div style="position: absolute; top: -10px; left: 12px; padding: 0 6px; background: {{{{t.panel}}}}; color: {{{{t.accent}}}}; font-family: {DISPLAY}; font-size: 20px; line-height: 18px; letter-spacing: 0.5px; text-shadow: {{{{t.titleGlow}}}}">{title}</div>
  {right_html}
  {body}
</div>'''

def crt_overlay():
    return '''
<sc-if value="{{crt}}" hint-placeholder-val="{{ true }}">
  <div style="position: absolute; inset: 0; pointer-events: none; background: repeating-linear-gradient(0deg, rgba(0,0,0,0.22) 0px, rgba(0,0,0,0.22) 1px, transparent 1px, transparent 3px); mix-blend-mode: multiply"></div>
  <div style="position: absolute; inset: 0; pointer-events: none; background: radial-gradient(ellipse at center, transparent 55%, rgba(0,0,0,0.45) 100%)"></div>
</sc-if>'''

# ---------- content blocks ----------
def views_body(tui=True):
    rows = [("notes", "3", True), ("untagged", "1", False), ("trash", "0", False)]
    out = []
    for name, n, sel in rows:
        bg = "{{t.sel}}" if sel else "transparent"
        fg = "{{t.selfg}}" if sel else "{{t.fg}}"
        marker = "▌" if sel else "&nbsp;"
        out.append(f'<div style="display: flex; align-items: center; gap: 8px; padding: 4px 8px; border-radius: 3px; background: {bg}; color: {fg}">'
                   f'<span style="color: {{{{t.accent}}}}; width: 8px">{marker}</span><span style="flex-grow: 1">{name}</span>'
                   f'<span style="color: {{{{t.dim}}}}">{n}</span></div>')
    return '<div style="display: flex; flex-direction: column; gap: 2px">' + "".join(out) + '</div>'

def tags_body():
    # box-drawing tree, monospace
    lines = [
        ("", "errand", "1"), ("", "home", ""), ("└─ ", "kitchen", "2"),
        ("", "tips", ""), ("└─ ", "tags", "1"), ("", "welcome", "1"),
    ]
    out = []
    for pre, name, n in lines:
        out.append(f'<div style="display: flex; align-items: center; gap: 0; padding: 3px 8px; color: {{{{t.fg}}}}">'
                   f'<span style="color: {{{{t.mute}}}}; white-space: pre">{pre}</span>'
                   f'<span style="color: {{{{t.accent2}}}}">#</span><span style="flex-grow: 1">{name}</span>'
                   f'<span style="color: {{{{t.dim}}}}">{n}</span></div>')
    return '<div style="display: flex; flex-direction: column; gap: 1px">' + "".join(out) + '</div>'

def search_line():
    return (f'<div style="display: flex; align-items: center; gap: 8px; padding: 6px 8px; border: 1px solid {{{{t.mute}}}}; border-radius: 3px; color: {{{{t.dim}}}}">'
            f'<span style="color: {{{{t.accent}}}}">/</span><span style="color: {{{{t.fg}}}}">kitchen</span>'
            f'<span style="display: inline-block; width: 8px; height: 15px; background: {{{{t.cursor}}}}; animation: blink 1.06s steps(1) infinite"></span>'
            f'<span style="flex-grow: 1"></span><span style="font-size: 11px">2 of 3</span></div>')

def note_rows():
    notes = [
        ("Shopping list", "13:43", "Things to grab before the weekend — milk, eggs, see Welcome to JotJotBoom", True, False),
        ("Groceries", "13:44", "eggs #home/kitchen #errand see Shopping list", False, True),
    ]
    out = []
    for title, when, preview, sel, pinned in notes:
        bg = "{{t.sel}}" if sel else "transparent"
        fg = "{{t.selfg}}" if sel else "{{t.fg}}"
        pin = f'<span style="color: {{{{t.accent}}}}; font-size: 11px">▲ </span>' if pinned else ''
        out.append(f'''<div style="display: flex; flex-direction: column; gap: 3px; padding: 7px 8px; border-radius: 3px; background: {bg}; color: {fg}">
  <div style="display: flex; gap: 8px; align-items: baseline"><span style="font-weight: 600; flex-grow: 1">{pin}{title}</span><span style="color: {{{{t.dim}}}}; font-size: 11px">{when}</span></div>
  <div style="color: {{{{t.dim}}}}; font-size: 12px; line-height: 16px; overflow: hidden; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical">{preview}</div>
</div>''')
    return '<div style="display: flex; flex-direction: column; gap: 4px">' + "".join(out) + '</div>'

def editor_body():
    dim = '{{t.mute}}'
    acc = '{{t.accent}}'
    acc2 = '{{t.accent2}}'
    p = lambda content, focused=False: (
        f'<div style="opacity: {"1" if focused else "{{t.dimmed}}"}; transition: opacity 0.2s">{content}</div>')
    body = "".join([
        p(f'<div style="font-family: {DISPLAY}; font-size: 34px; line-height: 36px; color: {acc}; text-shadow: {{{{t.titleGlow}}}}"><span style="color: {dim}; font-size: 22px; vertical-align: 6px"># </span>Shopping list</div>'),
        p('<div style="height: 12px"></div>'),
        p(f'<div>Things to grab before the weekend <span style="color: {acc2}">#home/kitchen</span></div>'),
        p('<div style="height: 12px"></div>'),
        p(f'<div><span style="color: {dim}">- [ ]</span> milk</div>'),
        p(f'<div><span style="color: {dim}">- [</span><span style="color: {acc}">x</span><span style="color: {dim}">]</span> <span style="text-decoration: line-through; color: {{{{t.dim}}}}">eggs</span></div>'),
        p(f'<div><span style="color: {dim}">-</span> see <span style="color: {dim}">[[</span><span style="color: {acc2}; text-decoration: underline; text-underline-offset: 3px">Welcome to JotJotBoom</span><span style="color: {dim}">]]</span></div>'),
        p('<div style="height: 12px"></div>'),
        p(f'<div><span style="color: {dim}">**</span><span style="font-weight: 600">Remember</span><span style="color: {dim}">**</span> the market closes at <span style="color: {dim}">*</span><span style="font-style: italic">noon</span><span style="color: {dim}">*</span>.'
          f'<span style="display: inline-block; width: 9px; height: 19px; vertical-align: -4px; margin-left: 1px; background: {{{{t.cursor}}}}; box-shadow: {{{{t.glow}}}}; animation: blink 1.06s steps(1) infinite"></span></div>', focused=True),
    ])
    return f'<div style="flex-grow: 1; padding: 6px 10px; font-size: 15px; line-height: 24px; color: {{{{t.fg}}}}; text-shadow: {{{{t.glow}}}}">{body}</div>'

def backlinks_body():
    return (f'<div style="display: flex; gap: 10px; align-items: center; color: {{{{t.fg}}}}; padding: 0 8px">'
            f'<span style="color: {{{{t.mute}}}}">←</span><span style="color: {{{{t.accent2}}}}; text-decoration: underline; text-underline-offset: 3px">Groceries</span></div>')

STYLE = f'''
<style>
  body {{ margin: 0; font-family: {MONO}; -webkit-font-smoothing: antialiased; }}
  a {{ color: #4dff8f; }} a:hover {{ color: #9ad3ff; }}
  @keyframes blink {{ 0%, 49% {{ opacity: 1; }} 50%, 100% {{ opacity: 0; }} }}
</style>'''

def page(inner, props_json):
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
<script data-dc-script data-props='{props_json}'>{LOGIC}</script>
</body>
</html>
'''

# ---------- Option A: full terminal ----------
nav_col = (f'<div style="display: flex; flex-direction: column; gap: 14px; width: 230px; flex-shrink: 0">'
           + frame("views", views_body(), height=132)
           + frame("tags", tags_body(), grow=True) + '</div>')
list_col = frame("notes", search_line() + note_rows(), right="2 of 3", width=340)
editor_col = ('<div style="display: flex; flex-direction: column; gap: 14px; flex-grow: 1; min-width: 0">'
              + frame("Shopping list", editor_body(), right="autosaved 13:43", grow=True)
              + frame("linked from", backlinks_body(), height=58) + '</div>')
optionA = f'''
<div style="width: 1280px; height: 800px; background: {{{{t.bg}}}}; position: relative; overflow: hidden; display: flex; flex-direction: column; font-family: {MONO}; font-size: 13px">
  {header()}
  <div style="flex-grow: 1; min-height: 0; padding: 18px 14px 14px; display: flex; gap: 14px; background: {{{{t.bg}}}}">
    {nav_col}
    {list_col}
    {editor_col}
  </div>
  {crt_overlay()}
</div>'''
open("Main.dc.html", "w").write(page(optionA, props()))

# ---------- Option B: COSMIC chrome, terminal panes ----------
def cosmic_nav():
    def item(icon, label, count="", sel=False, indent=0):
        bg = "#3a3a40" if sel else "transparent"
        return (f'<div style="display: flex; align-items: center; gap: 10px; padding: 8px 12px; margin-left: {indent}px; border-radius: 8px; background: {bg}; color: #e4e4e7">'
                f'{icon}<span style="flex-grow: 1">{label}</span><span style="color: #8a8a92; font-size: 12px">{count}</span></div>')
    rows = [item(I_LIST, "Notes", "", True), item(I_FOLDER, "Untagged"), item(I_TRASH, "Trash"),
            '<div style="height: 1px; background: #3a3a40; margin: 8px 12px"></div>',
            item(I_FOLDER, "errand", "1"), item(I_FOLDER, "home"), item("", "kitchen", "2", indent=26),
            item(I_FOLDER, "tips"), item("", "tags", "1", indent=26), item(I_FOLDER, "welcome", "1")]
    return (f'<div style="width: 250px; flex-shrink: 0; background: #262629; border-radius: 10px; padding: 8px; display: flex; flex-direction: column; gap: 2px; font-family: {SYS}; font-size: 14px">'
            + "".join(rows) + '</div>')

optionB = f'''
<div style="width: 1280px; height: 800px; background: #1c1c1e; position: relative; overflow: hidden; display: flex; flex-direction: column; font-family: {MONO}; font-size: 13px">
  {header()}
  <div style="flex-grow: 1; min-height: 0; padding: 12px; display: flex; gap: 12px">
    {cosmic_nav()}
    <div style="flex-grow: 1; min-width: 0; display: flex; gap: 14px; padding: 12px 10px 8px; border-radius: 10px; background: {{{{t.bg}}}}; position: relative; overflow: hidden">
      {frame("notes", search_line() + note_rows(), right="2 of 3", width=330)}
      <div style="display: flex; flex-direction: column; gap: 14px; flex-grow: 1; min-width: 0">
        {frame("Shopping list", editor_body(), right="autosaved 13:43", grow=True)}
        {frame("linked from", backlinks_body(), height=58)}
      </div>
      {crt_overlay()}
    </div>
  </div>
</div>'''
open("CosmicChrome.dc.html", "w").write(page(optionB, props()))

canvas = {
  "artboards": [
    {"file": "Main.dc.html", "title": "Option A — full terminal", "x": 0, "y": 0, "w": 1280, "h": 800},
    {"file": "CosmicChrome.dc.html", "title": "Option B — COSMIC chrome, terminal panes", "x": 1380, "y": 0, "w": 1280, "h": 800},
  ],
  "annotations": [
    {"id": "brief", "x": 0, "y": -330, "w": 620,
     "text": "Retro + btop direction for JotJotBoom.\n\nBoth boards share the tweaks up top: Theme (phosphor / amber / wordperfect / cosmic), CRT scanlines on/off, Focus mode (dims every paragraph but the one with the cursor).\n\nA: everything but the COSMIC header lives in btop-style frames, tag tree drawn with box-drawing connectors. Most retro; breaks the handover's 'chrome stays native' rule.\nB: nav bar stays stock COSMIC; only the notes list and editor are framed. Reads as a COSMIC app with a terminal inside it."},
  ],
  "launch": {"view": "canvas"},
}
json.dump(canvas, open("canvas.json", "w"), indent=2)
print("written")
