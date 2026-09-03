// SPDX-License-Identifier: GPL-3.0-only

mod anim;
mod app;
mod blocks;
mod coffee;
mod config;
mod debug_script;
mod table;
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
mod secrets;
mod store;

fn main() -> cosmic::iced::Result {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "jotjotboom=info,warn".into()),
        )
        .init();

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

    // Starts the application's event loop with `()` as the application's flags.
    cosmic::app::run::<app::AppModel>(settings, ())
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
