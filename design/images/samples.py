"""Render a sample photo-ish scene and the retro treatments the app would apply."""
from PIL import Image, ImageDraw, ImageFilter
import math, random

W, H = 420, 280
random.seed(7)
img = Image.new("RGB", (W, H))
d = ImageDraw.Draw(img)
# sky gradient
for y in range(H):
    t = y / H
    r = int(250 * (1 - t) + 60 * t); g = int(120 * (1 - t) + 30 * t); b = int(90 * (1 - t) + 120 * t)
    d.line([(0, y), (W, y)], fill=(r, g, b))
# sun
d.ellipse([W*0.55-70, 60, W*0.55+70, 200], fill=(255, 230, 150))
for i in range(6):
    y = 110 + i * 14
    d.rectangle([0, y, W, y + 4], fill=(230, 100, 90))
# distant hills
pts = [(0, 200)] + [(x, 190 - 40 * math.sin(x / 55.0) - 10 * math.sin(x / 13.0)) for x in range(0, W + 1, 6)] + [(W, 200), (W, H), (0, H)]
d.polygon(pts, fill=(70, 30, 80))
# near hills
pts = [(0, 230)] + [(x, 225 - 25 * math.sin(x / 40.0 + 1.5)) for x in range(0, W + 1, 6)] + [(W, 230), (W, H), (0, H)]
d.polygon(pts, fill=(30, 15, 45))
# a little house + trees
d.rectangle([250, 205, 300, 240], fill=(90, 60, 70)); d.polygon([(245, 205), (275, 180), (305, 205)], fill=(140, 70, 60))
d.rectangle([268, 222, 282, 240], fill=(250, 220, 120))
for x in (80, 110, 340, 370):
    d.polygon([(x-14, 245), (x, 195), (x+14, 245)], fill=(25, 40, 35)); d.rectangle([x-3, 245, x+3, 258], fill=(40, 25, 20))
img = img.filter(ImageFilter.GaussianBlur(0.4))
img.save("sample.png", optimize=True)

gray = img.convert("L")

def tint(g, bg, fg, mid=None):
    out = Image.new("RGB", g.size)
    px = out.load(); gp = g.load()
    for y in range(g.height):
        for x in range(g.width):
            t = gp[x, y] / 255
            t = t ** 0.9
            px[x, y] = tuple(int(bg[i] * (1 - t) + fg[i] * t) for i in range(3))
    return out

tint(gray, (5, 8, 6), (77, 255, 143)).save("tint_phosphor.png", optimize=True)
tint(gray, (11, 7, 4), (255, 176, 46)).save("tint_amber.png", optimize=True)

# Floyd–Steinberg dither to the 4 Game Boy greens and to 2-bit phosphor
def dither(g, palette):
    src = g.copy().load(); w, h = g.size
    buf = [[src[x, y] for x in range(w)] for y in range(h)]
    out = Image.new("RGB", g.size); px = out.load()
    levels = [int(255 * i / (len(palette) - 1)) for i in range(len(palette))]
    for y in range(h):
        for x in range(w):
            old = buf[y][x]
            idx = min(range(len(levels)), key=lambda i: abs(levels[i] - old))
            new = levels[idx]
            px[x, y] = palette[idx]
            err = old - new
            for dx, dy, wgt in ((1, 0, 7/16), (-1, 1, 3/16), (0, 1, 5/16), (1, 1, 1/16)):
                nx, ny = x + dx, y + dy
                if 0 <= nx < w and 0 <= ny < h:
                    buf[ny][nx] += err * wgt
    return out

gb = [(15, 56, 15), (48, 98, 48), (139, 172, 15), (155, 188, 15)]
dither(gray, gb).save("dither_gameboy.png", optimize=True)
ph = [(5, 8, 6), (31, 90, 47), (87, 163, 106), (185, 242, 191)]
dither(gray, ph).save("dither_phosphor.png", optimize=True)

# chunky pixels: nearest downscale then upscale
small = img.resize((84, 56), Image.NEAREST)
small.resize((W, H), Image.NEAREST).save("pixel.png", optimize=True)

# ASCII (72 cols), character aspect ~2:1
cols = 72; rows = int(H / W * cols * 0.5)
ramp = " .:-=+*#%@"
g2 = gray.resize((cols, rows))
lines = []
for y in range(rows):
    lines.append("".join(ramp[min(9, int(g2.getpixel((x, y)) / 256 * 10))] for x in range(cols)))
open("ascii.txt", "w").write("\n".join(lines))
import os
for f in sorted(os.listdir(".")):
    if f.endswith(".png"): print(f, os.path.getsize(f))
