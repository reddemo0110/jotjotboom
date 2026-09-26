use std::{
    env, fs,
    path::{Path, PathBuf},
};
use xdgen::{App, Context, FluentString};

fn main() {
    let ctx = Context::new("i18n", env::var("CARGO_PKG_NAME").unwrap()).unwrap();
    let app = App::new(FluentString("app-title"))
        .comment(FluentString("app-comment"))
        .keywords(FluentString("app-keywords"));

    let desktop_entry = app.expand_desktop("resources/app.desktop", &ctx).unwrap();
    let metainfo = app
        .expand_metainfo("resources/app.metainfo.xml", &ctx)
        .unwrap();

    // Into the workspace's target dir (the justfile installs from there),
    // not the package's: `apps/jjb-cosmic/../../target/xdgen`.
    let output = match env::var_os("CARGO_TARGET_DIR") {
        Some(dir) => PathBuf::from(dir).join("xdgen"),
        None => Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("../../target/xdgen"),
    };
    let output = output.as_path();
    fs::create_dir_all(output).unwrap();
    fs::write(output.join("app.desktop"), desktop_entry).unwrap();
    fs::write(output.join("app.metainfo.xml"), metainfo).unwrap();
}
