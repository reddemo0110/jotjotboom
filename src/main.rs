// SPDX-License-Identifier: GPL-3.0-only

mod anim;
mod app;
mod blocks;
mod coffee;
mod config;
mod debug_script;
mod desktop;
mod dockicon;
mod editor;
mod glyph;
mod i18n;
mod icon;
mod images;
mod links;
mod markdown;
mod note;
mod probe;
mod retro;
mod search_provider;
mod secrets;
mod store;
mod sync;
mod table;

use cosmic::Application as _;

const USAGE: &str = "\
Usage: jotjotboom [OPTIONS] [FILE]...

Retro-flavoured markdown notes. With no arguments, opens (or raises) the
notes window. Files inside the notes folder open in place; other markdown
files are imported as copies.

Options:
  --new              Start on a fresh note
  --search TEXT...   Open with TEXT in the search box
  --search-provider  Run the GNOME Shell search service (D-Bus activated)
  -V, --version      Print the version
  -h, --help         Show this help";

fn main() -> cosmic::iced::Result {
    // Off COSMIC there is no cosmic-theme config, and libcosmic logs an
    // error per missing key at every start; that is expected there.
    let default_filter = if desktop::Desktop::current() == desktop::Desktop::Cosmic {
        "jotjotboom=info,warn"
    } else {
        "jotjotboom=info,warn,cosmic::theme=off"
    };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| default_filter.into()),
        )
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("{USAGE}");
        return Ok(());
    }
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("jotjotboom {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // `jotjotboom --search-provider` is the headless GNOME Shell search
    // service, D-Bus activated by the shell; no window, no store.
    let Some(flags) = app::Flags::from_args(args) else {
        if let Err(err) = search_provider::serve(app::AppModel::APP_ID) {
            tracing::error!(%err, "search provider");
            std::process::exit(1);
        }
        return Ok(());
    };

    // Get the system's preferred languages.
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    // Enable localizations to be applied.
    i18n::init(&requested_languages);

    // Settings for configuring the application window and iced runtime.
    let mut settings = cosmic::app::Settings::default().size_limits(
        cosmic::iced::Limits::NONE
            .min_width(480.0)
            .min_height(320.0),
    );
    // Reopen at the size the window closed at.
    if let Some((w, h)) = last_window_size() {
        settings = settings.size(cosmic::iced::Size::new(w, h));
    }

    // Off COSMIC, borrow the desktop's fonts, icon theme and title-bar
    // buttons (GNOME via the portal) rather than assume COSMIC's.
    let desk = desktop::settings();
    if let Some(theme) = &desk.icon_theme {
        settings = settings.default_icon_theme(theme.clone());
    }
    desktop::reassert();

    // One window per user: a second launch (from the dock, `jotjotboom
    // file.md`, the search provider) hands its arguments to the running
    // instance over D-Bus and exits.
    cosmic::app::run_single_instance::<app::AppModel>(settings, flags)
}

/// The window size saved on the last run, if any.
fn last_window_size() -> Option<(f32, f32)> {
    use cosmic::cosmic_config::CosmicConfigEntry;
    let ctx = cosmic::cosmic_config::Config::new(
        "io.github.jotjotboom.JotJotBoom",
        config::Config::VERSION,
    )
    .ok()?;
    let cfg = config::Config::get_entry(&ctx).unwrap_or_else(|(_, cfg)| cfg);
    (cfg.window_width >= 480 && cfg.window_height >= 320)
        .then(|| (cfg.window_width as f32, cfg.window_height as f32))
}
