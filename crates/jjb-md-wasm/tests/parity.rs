// SPDX-License-Identifier: GPL-3.0-only

//! The WASM build of the scanner must give the same spans as the native
//! one. This builds the crate for `wasm32-unknown-unknown`, runs
//! `wasm-bindgen` for Node, feeds a fixture through Node, and compares with
//! `jjb_core::markdown::scan_line` in this process.
//!
//! Needs the wasm32 target, the `wasm-bindgen` CLI (same version as the
//! crate) and `node`. Missing any of them, the test skips with a note —
//! unless `JJB_WASM_STRICT=1`, when it fails.

use std::path::{Path, PathBuf};
use std::process::Command;

const FIXTURE: &str = "\
# Heading with a #tag and **bold**
## Second *italic* and ***both***
plain line with `code` and ~~struck~~ words
- [ ] an open task with a [[wiki link]]
- [x] a done task [link](https://example.com)
1. numbered ==marked== item
2) another with #travel/japan/kyoto
> a quote line
---
```rust
let x = **not bold** in a fence;
```
after the fence **bold again**
![alt](assets/pic.jpg){frame=box}
| a | b |
|---|---|
| 1 | =SUM(A1:A2) |
caf\u{e9} \u{1F986} multibyte **bold** after emoji
";

fn find(program: &str, extra: &[PathBuf]) -> Option<PathBuf> {
    let mut dirs: Vec<PathBuf> = std::env::var_os("PATH")
        .map(|p| std::env::split_paths(&p).collect())
        .unwrap_or_default();
    dirs.extend(extra.iter().cloned());
    dirs.into_iter()
        .map(|d| d.join(program))
        .find(|p| p.is_file())
}

fn skip(reason: &str) -> bool {
    if std::env::var_os("JJB_WASM_STRICT").is_some() {
        panic!("{reason}");
    }
    eprintln!("skipping wasm parity: {reason}");
    true
}

#[test]
fn wasm_scanner_matches_native() {
    let home = dirs_home();
    let extra = [
        home.join(".cargo/bin"),
        home.join(".local/opt/node/bin"),
        home.join(".local/bin"),
    ];
    let Some(cargo) = std::env::var_os("CARGO").map(PathBuf::from) else {
        skip("no $CARGO");
        return;
    };
    let Some(bindgen) = find("wasm-bindgen", &extra) else {
        skip("wasm-bindgen CLI not installed (cargo install wasm-bindgen-cli --locked)");
        return;
    };
    let Some(node) = find("node", &extra) else {
        skip("node not installed");
        return;
    };

    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    // A target dir of our own: the outer `cargo test` holds the lock on the
    // workspace's.
    let target = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest.join("../../target"))
        .join("wasm-parity");
    let build = Command::new(&cargo)
        .args([
            "build",
            "-p",
            "jjb-md-wasm",
            "--target",
            "wasm32-unknown-unknown",
            "--release",
        ])
        .env("CARGO_TARGET_DIR", &target)
        .current_dir(manifest)
        .output()
        .expect("running cargo");
    if !build.status.success() {
        let err = String::from_utf8_lossy(&build.stderr);
        if err.contains("wasm32-unknown-unknown") && err.contains("target may not be installed") {
            skip("wasm32 target not installed (rustup target add wasm32-unknown-unknown)");
            return;
        }
        panic!("wasm build failed:\n{err}");
    }
    let wasm = target.join("wasm32-unknown-unknown/release/jjb_md_wasm.wasm");
    let out = target.join("pkg");
    let bg = Command::new(&bindgen)
        .args(["--target", "nodejs", "--out-dir"])
        .arg(&out)
        .arg(&wasm)
        .output()
        .expect("running wasm-bindgen");
    assert!(
        bg.status.success(),
        "wasm-bindgen failed (CLI and crate versions must match):\n{}",
        String::from_utf8_lossy(&bg.stderr)
    );

    // Node: scan the fixture line by line and print the JSON array.
    let script = format!(
        "const m = require({pkg:?}); const fs = require('fs');\n\
         const text = fs.readFileSync(0, 'utf8');\n\
         let fence = false; const out = [];\n\
         for (const line of text.split('\\n').slice(0, -1)) {{\n\
           const r = JSON.parse(m.scan_line(line, fence));\n\
           out.push(r); fence = r.in_fence;\n\
         }}\n\
         const whole = JSON.parse(m.scan_text(text));\n\
         process.stdout.write(JSON.stringify({{ lines: out, whole }}));",
        pkg = out.join("jjb_md_wasm.js").to_string_lossy()
    );
    let mut child = Command::new(&node)
        .arg("-e")
        .arg(&script)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("running node");
    {
        use std::io::Write;
        child
            .stdin
            .take()
            .unwrap()
            .write_all(FIXTURE.as_bytes())
            .unwrap();
    }
    let run = child.wait_with_output().unwrap();
    assert!(
        run.status.success(),
        "node failed:\n{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let got: serde_json::Value = serde_json::from_slice(&run.stdout).expect("node output is JSON");

    // Native, same threading of the fence state.
    let mut fence = false;
    let mut want = Vec::new();
    for line in FIXTURE.lines() {
        let (spans, next) = jjb_core::markdown::scan_line(line, fence);
        want.push(serde_json::json!({ "spans": spans, "in_fence": next }));
        fence = next;
    }
    let want = serde_json::Value::Array(want);
    assert_eq!(
        got["lines"], want,
        "line-by-line spans differ between WASM and native"
    );
    assert_eq!(got["whole"], want, "scan_text differs from scan_line");
    assert!(
        want.as_array()
            .unwrap()
            .iter()
            .any(|l| !l["spans"].as_array().unwrap().is_empty()),
        "fixture produced no spans at all"
    );
}

fn dirs_home() -> PathBuf {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_default()
}
