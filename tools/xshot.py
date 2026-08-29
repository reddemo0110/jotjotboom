#!/usr/bin/env python3
"""Headless-ish visual check for JotJotBoom.

Runs the app on Xwayland (DISPLAY, WAYLAND_DISPLAY unset), optionally drives
it through the app's JJB_SCRIPT hook, and captures the window with XGetImage.

    tools/xshot.py out.png                                   # launch, wait, capture
    tools/xshot.py out.png --script 'new;type:Hello #demo;wait:1500' --wait 6

Needs python-xlib and Pillow. iced's own window::screenshot drops editor text
and menu labels, and XTEST/portal input injection hangs unattended on COSMIC,
which is why this exists. See src/debug_script.rs for the step language.
"""
import argparse, os, subprocess, sys, time
from Xlib import X, XK, display
from PIL import Image

class _Timeout(Exception):
    pass

def _find_window(w, depth=0):
    """Recursive search for the app's top-level window (depth-limited: Xwayland
    puts it two levels below root)."""
    if depth > 3:
        return None, None
    try:
        attrs = w.get_attributes()
        geom = w.get_geometry()
        cls = w.get_wm_class()
    except Exception:
        return None, None
    if attrs.map_state == X.IsViewable and geom.width > 100 and cls and any("jotjotboom" in c.lower() for c in cls):
        return w, geom
    try:
        children = w.query_tree().children
    except Exception:
        return None, None
    for c in children:
        found, g = _find_window(c, depth + 1)
        if found is not None:
            return found, g
    return None, None

def find_window(display_name, tries=6, timeout=3):
    """Walk the X tree with a hard timeout per attempt; python-xlib occasionally
    blocks on a window that is mid-creation, so reconnect and retry."""
    import signal
    def on_alarm(signum, frame):
        raise _Timeout()
    old = signal.signal(signal.SIGALRM, on_alarm)
    try:
        for attempt in range(tries):
            d = display.Display(display_name)
            signal.alarm(timeout)
            try:
                win, geom = _find_window(d.screen().root)
                signal.alarm(0)
                if win is not None:
                    return d, win, geom
            except _Timeout:
                print(f"x walk timed out (attempt {attempt + 1}), retrying", file=sys.stderr, flush=True)
            finally:
                signal.alarm(0)
            try:
                d.close()
            except Exception:
                pass
            time.sleep(1)
    finally:
        signal.signal(signal.SIGALRM, old)
    return None, None, None

def log(*a):
    if os.environ.get("XSHOT_DEBUG"):
        print(*a, file=sys.stderr, flush=True)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("out")
    ap.add_argument("--binary", default="target/debug/jotjotboom")
    ap.add_argument("--display", default=os.environ.get("DISPLAY", ":1"))
    ap.add_argument("--wait", type=float, default=4.0, help="seconds after launch before acting")
    ap.add_argument("--script", default=None, help="JJB_SCRIPT steps, e.g. 'new;type:Hi;wait:1500'")
    ap.add_argument("--settle", type=float, default=1.5, help="seconds before capture")
    ap.add_argument("--keep", action="store_true", help="leave the app running")
    args = ap.parse_args()

    env = dict(os.environ, DISPLAY=args.display)
    env.pop("WAYLAND_DISPLAY", None)
    env.setdefault("RUST_LOG", "jotjotboom=info,warn")
    if args.script:
        env["JJB_SCRIPT"] = args.script
    proc = subprocess.Popen([args.binary], env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    # A key left held on the X server (e.g. by a broken injection tool) would
    # auto-repeat into our window; switch X auto-repeat off while we capture.
    kb = None
    try:
        kb = display.Display(args.display)
        kb.change_keyboard_control(auto_repeat_mode=X.AutoRepeatModeOff)
        kb.sync()
    except Exception as e:
        log("could not disable autorepeat:", e)
    try:
        time.sleep(args.wait)
        log("launched, searching")
        d, win, geom = find_window(args.display)
        log("search done:", win is not None)
        if win is None:
            print("no JotJotBoom window found", file=sys.stderr)
            return 1
        log("settling")
        time.sleep(args.settle)
        geom = win.get_geometry()
        raw = win.get_image(0, 0, geom.width, geom.height, X.ZPixmap, 0xFFFFFFFF)
        Image.frombytes("RGB", (geom.width, geom.height), raw.data, "raw", "BGRX").save(args.out)
        print("saved", args.out, f"{geom.width}x{geom.height}")
        return 0
    finally:
        if kb is not None:
            try:
                kb.change_keyboard_control(auto_repeat_mode=X.AutoRepeatModeDefault)
                kb.sync()
            except Exception:
                pass
        if not args.keep:
            proc.terminate()
            try:
                proc.wait(timeout=5)
            except subprocess.TimeoutExpired:
                proc.kill()

if __name__ == "__main__":
    sys.exit(main())
