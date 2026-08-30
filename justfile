# Name of the application's binary.
name := 'jotjotboom'
# The unique ID of the application.
appid := 'io.github.jotjotboom.JotJotBoom'

# Path to root file system, which defaults to `/`.
rootdir := ''
# The prefix for the `/usr` directory.
prefix := '/usr'
# The location of the cargo target directory.
cargo-target-dir := env('CARGO_TARGET_DIR', 'target')

# Application's appstream metadata
appdata := appid + '.metainfo.xml'
# Application's desktop entry
desktop := appid + '.desktop'
# Application's icon.
icon-svg := appid + '.svg'

# Install destinations
base-dir := absolute_path(clean(rootdir / prefix))
appdata-dst := base-dir / 'share' / 'appdata' / appdata
bin-dst := base-dir / 'bin' / name
desktop-dst := base-dir / 'share' / 'applications' / desktop
icons-dst := base-dir / 'share' / 'icons' / 'hicolor'
icon-svg-dst := icons-dst / 'scalable' / 'apps' / icon-svg

# Default recipe which runs `just build-release`
default: build-release

# Runs `cargo clean`
clean:
    cargo clean

# Removes vendored dependencies
clean-vendor:
    rm -rf .cargo vendor vendor.tar

# `cargo clean` and removes vendored dependencies
clean-dist: clean clean-vendor

# Compiles with debug profile
build-debug *args:
    cargo build --locked {{args}}

# Compiles with release profile
build-release *args: (build-debug '--release' args)

# Compiles release profile with vendored dependencies
build-vendored *args: vendor-extract (build-release '--frozen --offline' args)

# Runs a clippy check
check *args:
    cargo clippy --all-features --locked {{args}} -- -W clippy::pedantic

# Runs a clippy check with JSON message format
check-json: (check '--message-format=json')

# Run the application for testing purposes
run *args:
    env RUST_BACKTRACE=full cargo run --release --locked {{args}}

# Installs files
install:
    install -Dm0755 {{ cargo-target-dir / 'release' / name }} {{bin-dst}}
    install -Dm0644 {{ 'target' / 'xdgen' / 'app.desktop' }} {{desktop-dst}}
    install -Dm0644 {{ 'target' / 'xdgen' / 'app.metainfo.xml' }} {{appdata-dst}}
    install -Dm0644 {{ 'resources' / 'icons' / 'hicolor' / 'scalable' / 'apps' / 'icon.svg' }} {{icon-svg-dst}}

# Uninstalls installed files
uninstall:
    rm {{bin-dst}} {{desktop-dst}} {{icon-svg-dst}}

# Per-user install (no sudo): ~/.local/bin, launcher entry, icon.
# The desktop entry gets an absolute Exec so it works even if ~/.local/bin
# is not on PATH.
user-base := env('HOME') / '.local'
install-user: build-release
    install -Dm0755 {{ cargo-target-dir / 'release' / name }} {{ user-base / 'bin' / name }}
    install -Dm0644 {{ 'resources' / 'icons' / 'hicolor' / 'scalable' / 'apps' / 'icon.svg' }} {{ user-base / 'share' / 'icons' / 'hicolor' / 'scalable' / 'apps' / icon-svg }}
    install -Dm0644 {{ 'target' / 'xdgen' / 'app.metainfo.xml' }} {{ user-base / 'share' / 'metainfo' / appdata }}
    sed 's|^Exec=.*|Exec={{ user-base / 'bin' / name }} %F|' {{ 'target' / 'xdgen' / 'app.desktop' }} > {{ user-base / 'share' / 'applications' / desktop }}
    -update-desktop-database {{ user-base / 'share' / 'applications' }}
    -gtk-update-icon-cache -q -t -f {{ user-base / 'share' / 'icons' / 'hicolor' }}
    @echo "Installed. Find JotJotBoom in the app library; right-click it in the dock to pin."

# Removes the per-user install
uninstall-user:
    rm -f {{ user-base / 'bin' / name }} {{ user-base / 'share' / 'applications' / desktop }} {{ user-base / 'share' / 'icons' / 'hicolor' / 'scalable' / 'apps' / icon-svg }} {{ user-base / 'share' / 'metainfo' / appdata }}
    -update-desktop-database {{ user-base / 'share' / 'applications' }}

# Vendor dependencies locally
vendor:
    mkdir -p .cargo
    cargo vendor | head -n -1 > .cargo/config.toml
    echo 'directory = "vendor"' >> .cargo/config.toml
    tar pcf vendor.tar vendor
    rm -rf vendor

# Extracts vendored dependencies
vendor-extract:
    rm -rf vendor
    tar pxf vendor.tar

# Bump cargo version, create git commit, and create tag
tag version:
    find -type f -name Cargo.toml -exec sed -i '0,/^version/s/^version.*/version = "{{version}}"/' '{}' \; -exec git add '{}' \;
    cargo check
    cargo clean
    git add Cargo.lock
    git commit -m 'release: {{version}}'
    git commit --amend
    git tag -a {{version}} -m ''


# Copy the example notes (and their photos) into the notes folder; never overwrites.
install-examples notes_dir=(env('HOME') / 'Documents' / 'JotJotBoom'):
    mkdir -p '{{notes_dir}}/assets'
    cp -n examples/notes/*.md '{{notes_dir}}/'
    cp -n examples/notes/assets/*.jpg '{{notes_dir}}/assets/'
    @echo "Examples installed into {{notes_dir}} — open JotJotBoom and look for #examples."
