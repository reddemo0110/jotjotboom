// SPDX-License-Identifier: GPL-3.0-only

use crate::config::Config;
use crate::debug_script::{self, Step};
use crate::fl;
use crate::note::{Note, NoteSummary};
use crate::store::{Store, View};
use chrono::{DateTime, Datelike, Local, Utc};
use cosmic::Application as _;
use cosmic::app::context_drawer;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::keyboard::{self, key::Physical};
use cosmic::iced::widget::scrollable::{Direction, Scrollbar};
use cosmic::iced::{Alignment, Event, Length, Subscription, event};
use cosmic::prelude::*;
use cosmic::widget::menu::action::MenuAction as _;
use cosmic::widget::{self, about::About, icon, menu, nav_bar, text_editor};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const APP_ICON: &[u8] = include_bytes!("../resources/icons/hicolor/scalable/apps/icon.svg");
/// How long after the last keystroke we write the note to disk.
const AUTOSAVE_DELAY: Duration = Duration::from_millis(600);
const NOTE_LIST_WIDTH: f32 = 300.0;

pub struct AppModel {
    core: cosmic::Core,
    context_page: ContextPage,
    about: About,
    nav: nav_bar::Model,
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    config: Config,
    /// Kept for the settings page (notes dir picker) that lands in the polish phase.
    #[allow(dead_code)]
    config_handler: Option<cosmic_config::Config>,

    store: Option<Store>,
    store_error: Option<String>,
    view: View,
    query: String,
    search_id: widget::Id,
    notes: Vec<NoteSummary>,
    current: Option<Note>,
    editor: text_editor::Content,
    dirty: bool,
    last_edit: Instant,
    backlinks: Vec<NoteSummary>,
    /// Debug hook: `JJB_SCREENSHOT=/path.png` saves a frame ~2.5s after launch, then exits.
    /// Note: iced's screenshot path drops text-editor contents and menu labels; for a
    /// faithful capture use `tools/xshot.py` (see CLAUDE.md).
    screenshot_pending: bool,
    /// Debug hook: `JJB_SCRIPT=...` drives the app; see `debug_script.rs`.
    script: Option<debug_script::Runner>,
}

#[derive(Debug, Clone)]
pub enum Message {
    LaunchUrl(String),
    ToggleContextPage(ContextPage),
    UpdateConfig(Config),
    Key(keyboard::Modifiers, keyboard::Key, Physical),

    Editor(text_editor::Action),
    AutosaveTick,
    Select(String),
    NewNote,
    TrashCurrent,
    RestoreCurrent,
    DeleteCurrentForever,
    EmptyTrash,
    TogglePin,
    Search(String),
    ClearSearch,
    FocusSearch,
    TakeScreenshot,
    ScriptTick,
    ScreenshotTaken(Arc<cosmic::iced::window::Screenshot>),
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "io.github.jotjotboom.JotJotBoom";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let config_handler = cosmic_config::Config::new(Self::APP_ID, Config::VERSION).ok();
        let mut config = config_handler
            .as_ref()
            .map(|ctx| match Config::get_entry(ctx) {
                Ok(config) => config,
                Err((errors, config)) => {
                    for why in errors {
                        tracing::warn!(%why, "error loading app config");
                    }
                    config
                }
            })
            .unwrap_or_default();

        if config.device_id.is_empty() {
            let id = crate::note::new_id();
            if let Some(handler) = &config_handler
                && let Err(why) = config.set_device_id(handler, id.clone())
            {
                tracing::warn!(%why, "could not persist device id");
            }
            config.device_id = id;
        }

        let (store, store_error) = match Store::open(
            config.notes_dir(),
            &Config::index_path(Self::APP_ID),
            config.device_id.clone(),
        ) {
            Ok(store) => (Some(store), None),
            Err(err) => {
                tracing::error!(%err, "opening store");
                (None, Some(format!("{err:#}")))
            }
        };

        let about = About::default()
            .name(fl!("app-title"))
            .icon(widget::icon::from_svg_bytes(APP_ICON))
            .version(env!("CARGO_PKG_VERSION"))
            .links([(fl!("repository"), REPOSITORY)])
            .license(env!("CARGO_PKG_LICENSE"));

        let mut app = AppModel {
            core,
            context_page: ContextPage::default(),
            about,
            nav: nav_bar::Model::default(),
            key_binds: key_binds(),
            config,
            config_handler,
            store,
            store_error,
            view: View::All,
            query: String::new(),
            search_id: widget::Id::unique(),
            notes: Vec::new(),
            current: None,
            editor: text_editor::Content::new(),
            dirty: false,
            last_edit: Instant::now(),
            backlinks: Vec::new(),
            screenshot_pending: std::env::var_os("JJB_SCREENSHOT").is_some(),
            script: debug_script::Runner::from_env(),
        };

        app.rebuild_nav();
        app.refresh_list();
        // Open the most recent note so the window isn't empty on launch.
        if let Some(first) = app.notes.first().map(|n| n.id.clone()) {
            app.open_note(&first);
        }

        let command = app.update_title();
        (app, command)
    }

    fn header_start(&self) -> Vec<Element<'_, Self::Message>> {
        let menu_bar = menu::bar(vec![
            menu::Tree::with_children(
                menu::root(fl!("file")).apply(Element::from),
                menu::items(
                    &self.key_binds,
                    vec![
                        menu::Item::Button(fl!("new-note"), None, MenuAction::NewNote),
                        menu::Item::Divider,
                        menu::Item::Button(fl!("trash-note"), None, MenuAction::TrashNote),
                    ],
                ),
            ),
            menu::Tree::with_children(
                menu::root(fl!("view")).apply(Element::from),
                menu::items(
                    &self.key_binds,
                    vec![menu::Item::Button(fl!("about"), None, MenuAction::About)],
                ),
            ),
        ]);

        vec![menu_bar.into()]
    }

    fn header_end(&self) -> Vec<Element<'_, Self::Message>> {
        let mut items: Vec<Element<'_, Message>> = Vec::new();
        let in_trash = matches!(self.view, View::Trash);

        if let Some(note) = &self.current {
            if in_trash {
                items.push(header_button(
                    "edit-undo-symbolic",
                    fl!("restore-note"),
                    Message::RestoreCurrent,
                ));
                items.push(header_button(
                    "edit-delete-symbolic",
                    fl!("delete-forever"),
                    Message::DeleteCurrentForever,
                ));
            } else {
                let pin_label = if note.pinned {
                    fl!("unpin-note")
                } else {
                    fl!("pin-note")
                };
                items.push(header_button(
                    "view-pin-symbolic",
                    pin_label,
                    Message::TogglePin,
                ));
                items.push(header_button(
                    "user-trash-symbolic",
                    fl!("trash-note"),
                    Message::TrashCurrent,
                ));
            }
        }
        if in_trash && !self.notes.is_empty() {
            items.push(header_button(
                "edit-clear-all-symbolic",
                fl!("empty-trash"),
                Message::EmptyTrash,
            ));
        }
        if !in_trash {
            items.push(header_button(
                "list-add-symbolic",
                fl!("new-note"),
                Message::NewNote,
            ));
        }
        items
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    fn context_drawer(&self) -> Option<context_drawer::ContextDrawer<'_, Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => context_drawer::about(
                &self.about,
                |url| Message::LaunchUrl(url.to_string()),
                Message::ToggleContextPage(ContextPage::About),
            ),
        })
    }

    fn view(&self) -> Element<'_, Self::Message> {
        if let Some(err) = &self.store_error {
            return widget::container(widget::text::body(fl!("store-error", error = err.as_str())))
                .padding(24)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .into();
        }

        widget::row::with_capacity(3)
            .push(self.note_list())
            .push(widget::divider::vertical::default())
            .push(self.editor_pane())
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let mut subscriptions = vec![
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| Message::UpdateConfig(update.config)),
            event::listen_with(|event, _status, _window| match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key,
                    modifiers,
                    physical_key,
                    ..
                }) => Some(Message::Key(modifiers, key, physical_key)),
                _ => None,
            }),
        ];

        if self.dirty {
            subscriptions.push(
                cosmic::iced::time::every(Duration::from_millis(200))
                    .map(|_| Message::AutosaveTick),
            );
        }
        if self.screenshot_pending {
            subscriptions.push(
                cosmic::iced::time::every(Duration::from_millis(2500))
                    .map(|_| Message::TakeScreenshot),
            );
        }
        if self
            .script
            .as_ref()
            .is_some_and(debug_script::Runner::is_active)
        {
            subscriptions.push(
                cosmic::iced::time::every(Duration::from_millis(100)).map(|_| Message::ScriptTick),
            );
        }

        Subscription::batch(subscriptions)
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::Editor(action) => {
                let is_edit = action.is_edit();
                self.editor.perform(action);
                if is_edit && self.current.as_ref().is_some_and(|n| !n.trashed) {
                    self.dirty = true;
                    self.last_edit = Instant::now();
                }
            }

            Message::AutosaveTick => {
                if self.dirty && self.last_edit.elapsed() >= AUTOSAVE_DELAY {
                    self.flush();
                    return self.update_title();
                }
            }

            Message::Select(id) => {
                if self.current.as_ref().is_some_and(|n| n.id == id) {
                    return Task::none();
                }
                self.close_current();
                self.open_note(&id);
                return self.update_title();
            }

            Message::NewNote => {
                if matches!(self.view, View::Trash) {
                    self.set_view(View::All);
                }
                self.close_current();
                self.query.clear();
                let created = self.store.as_mut().and_then(|s| match s.create() {
                    Ok(note) => Some(note),
                    Err(err) => {
                        tracing::error!(%err, "creating note");
                        None
                    }
                });
                if let Some(note) = created {
                    self.refresh_list();
                    self.open_note(&note.id);
                }
                return self.update_title();
            }

            Message::TrashCurrent => {
                self.flush();
                if let (Some(store), Some(note)) = (self.store.as_mut(), self.current.take())
                    && let Err(err) = store.trash(&note.id)
                {
                    tracing::error!(%err, "trashing note");
                }
                self.after_store_change();
                return self.update_title();
            }

            Message::RestoreCurrent => {
                if let (Some(store), Some(note)) = (self.store.as_mut(), self.current.take())
                    && let Err(err) = store.restore(&note.id)
                {
                    tracing::error!(%err, "restoring note");
                }
                self.after_store_change();
                return self.update_title();
            }

            Message::DeleteCurrentForever => {
                if let (Some(store), Some(note)) = (self.store.as_mut(), self.current.take())
                    && let Err(err) = store.delete_forever(&note.id)
                {
                    tracing::error!(%err, "deleting note");
                }
                self.after_store_change();
                return self.update_title();
            }

            Message::EmptyTrash => {
                self.current = None;
                if let Some(store) = self.store.as_mut()
                    && let Err(err) = store.empty_trash()
                {
                    tracing::error!(%err, "emptying trash");
                }
                self.after_store_change();
                return self.update_title();
            }

            Message::TogglePin => {
                self.flush();
                let target = self.current.as_ref().map(|n| (n.id.clone(), !n.pinned));
                if let (Some(store), Some((id, pinned))) = (self.store.as_mut(), target) {
                    match store.set_pinned(&id, pinned) {
                        Ok(Some(note)) => {
                            if let Some(current) = self.current.as_mut() {
                                current.pinned = note.pinned;
                                current.modified = note.modified;
                            }
                        }
                        Ok(None) => {}
                        Err(err) => tracing::error!(%err, "pinning note"),
                    }
                }
                self.refresh_list();
            }

            Message::Search(query) => {
                self.query = query;
                self.refresh_list();
            }

            Message::ClearSearch => {
                self.query.clear();
                self.refresh_list();
            }

            Message::FocusSearch => {
                return widget::text_input::focus(self.search_id.clone());
            }

            Message::ScriptTick => {
                let step = self.script.as_mut().and_then(debug_script::Runner::next);
                if let Some(step) = step {
                    tracing::debug!(?step, "JJB_SCRIPT step");
                    return self.run_step(step);
                }
            }

            Message::TakeScreenshot => {
                if self.screenshot_pending {
                    self.screenshot_pending = false;
                    if let Some(id) = self.core.main_window_id() {
                        return cosmic::iced::window::screenshot(id).map(|shot| {
                            cosmic::Action::App(Message::ScreenshotTaken(Arc::new(shot)))
                        });
                    }
                }
            }

            Message::ScreenshotTaken(shot) => {
                if let Some(path) = std::env::var_os("JJB_SCREENSHOT") {
                    match image::save_buffer(
                        &path,
                        &shot.rgba,
                        shot.size.width,
                        shot.size.height,
                        image::ColorType::Rgba8,
                    ) {
                        Ok(()) => {
                            tracing::info!(path = %path.to_string_lossy(), "screenshot saved")
                        }
                        Err(err) => tracing::error!(%err, "saving screenshot"),
                    }
                }
                self.close_current();
                std::process::exit(0);
            }

            Message::Key(modifiers, key, physical) => {
                let action = self
                    .key_binds
                    .iter()
                    .find(|(bind, _)| bind.matches(modifiers, &key, Some(&physical)))
                    .map(|(_, action)| action.message());
                if let Some(message) = action {
                    return self.update(message);
                }
            }

            Message::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    self.context_page = context_page;
                    self.core.window.show_context = true;
                }
            }

            Message::UpdateConfig(config) => {
                // A changed notes_dir takes effect on next launch; everything else is live.
                self.config = config;
            }

            Message::LaunchUrl(url) => match open::that_detached(&url) {
                Ok(()) => {}
                Err(err) => {
                    tracing::warn!(%err, url, "failed to open url");
                }
            },
        }
        Task::none()
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<cosmic::Action<Self::Message>> {
        self.nav.activate(id);
        let view = self.nav.data::<View>(id).cloned().unwrap_or(View::All);
        self.close_current();
        self.view = view;
        self.refresh_list();
        if let Some(first) = self.notes.first().map(|n| n.id.clone()) {
            self.open_note(&first);
        }
        self.update_title()
    }

    fn on_escape(&mut self) -> Task<cosmic::Action<Self::Message>> {
        if self.core.window.show_context {
            self.core.window.show_context = false;
        } else if !self.query.is_empty() {
            self.query.clear();
            self.refresh_list();
        }
        Task::none()
    }

    fn on_app_exit(&mut self) -> Option<Self::Message> {
        self.close_current();
        None
    }
}

impl AppModel {
    fn note_list(&self) -> Element<'_, Message> {
        let spacing = cosmic::theme::spacing();
        let search = widget::search_input(fl!("search-placeholder"), &self.query)
            .id(self.search_id.clone())
            .on_input(Message::Search)
            .on_clear(Message::ClearSearch);

        let body: Element<'_, Message> = if self.notes.is_empty() {
            let text = if self.query.is_empty() {
                fl!("no-notes-here")
            } else {
                fl!("no-results", query = self.query.as_str())
            };
            widget::container(widget::text::caption(text))
                .padding(spacing.space_m)
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .into()
        } else {
            let mut list =
                widget::list_column().list_item_padding([spacing.space_xs, spacing.space_s]);
            for note in &self.notes {
                let selected = self.current.as_ref().is_some_and(|c| c.id == note.id);
                list = list.add(
                    widget::list::button(note_row(note))
                        .on_press(Message::Select(note.id.clone()))
                        .selected(selected),
                );
            }
            widget::scrollable(list).height(Length::Fill).into()
        };

        let count = widget::text::caption(fl!("notes-count", count = self.notes.len()));

        widget::column::with_capacity(3)
            .push(widget::container(search).padding([
                spacing.space_xs,
                spacing.space_xs,
                0,
                spacing.space_xs,
            ]))
            .push(body)
            .push(widget::container(count).padding([spacing.space_xxs, spacing.space_s]))
            .spacing(spacing.space_xs)
            .width(Length::Fixed(NOTE_LIST_WIDTH))
            .height(Length::Fill)
            .into()
    }

    fn editor_pane(&self) -> Element<'_, Message> {
        let spacing = cosmic::theme::spacing();
        let Some(note) = &self.current else {
            return widget::container(widget::text::body(fl!("no-note-selected")))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .into();
        };

        let mut editor = text_editor::text_editor(&self.editor)
            .placeholder(fl!("untitled"))
            .font(cosmic::font::mono())
            .size(15)
            .line_height(1.5)
            .padding(spacing.space_m)
            .height(Length::Fill);
        if !note.trashed {
            editor = editor.on_action(Message::Editor);
        }

        let mut column = widget::column::with_capacity(3)
            .push(editor)
            .width(Length::Fill)
            .height(Length::Fill);

        if note.trashed {
            column = column.push(
                widget::container(widget::text::caption(fl!("trash-hint")))
                    .padding([spacing.space_xs, spacing.space_m]),
            );
        }

        if !self.backlinks.is_empty() {
            let mut row = widget::row::with_capacity(self.backlinks.len() + 1)
                .spacing(spacing.space_xs)
                .align_y(Alignment::Center)
                .push(widget::text::caption_heading(fl!("linked-from")));
            for link in &self.backlinks {
                row = row.push(
                    widget::button::link(link.title.clone())
                        .on_press(Message::Select(link.id.clone())),
                );
            }
            column = column.push(
                widget::container(
                    widget::scrollable(row).direction(Direction::Horizontal(Scrollbar::default())),
                )
                .padding([spacing.space_xs, spacing.space_m]),
            );
        }

        column.into()
    }

    /// Rebuild the nav bar: fixed views, then the tag tree.
    fn rebuild_nav(&mut self) {
        self.nav.clear();
        self.nav
            .insert()
            .text(fl!("nav-all-notes"))
            .icon(icon::from_name("view-list-symbolic"))
            .data(View::All);
        self.nav
            .insert()
            .text(fl!("nav-untagged"))
            .icon(icon::from_name("folder-symbolic"))
            .data(View::Untagged);
        self.nav
            .insert()
            .text(fl!("nav-trash"))
            .icon(icon::from_name("user-trash-symbolic"))
            .data(View::Trash);

        let tags = self
            .store
            .as_ref()
            .and_then(|s| s.tags().ok())
            .unwrap_or_default();
        let mut first_tag = true;
        for (name, count) in tag_tree(&tags) {
            let depth = name.matches('/').count() as u16;
            let leaf = name.rsplit('/').next().unwrap_or(&name).to_owned();
            let label = if count > 0 {
                format!("{leaf}  {count}")
            } else {
                leaf
            };
            let mut entry = self
                .nav
                .insert()
                .text(label)
                .indent(depth)
                .data(View::Tag(name));
            if first_tag {
                entry = entry.divider_above(true);
                first_tag = false;
            }
            if depth == 0 {
                entry.icon(icon::from_name("folder-symbolic"));
            }
        }

        // Re-activate the entry matching the current view (a tag may have vanished).
        let active = self
            .nav
            .iter()
            .find(|id| self.nav.data::<View>(*id) == Some(&self.view))
            .or_else(|| self.nav.iter().next());
        if let Some(id) = active {
            self.nav.activate(id);
            self.view = self.nav.data::<View>(id).cloned().unwrap_or(View::All);
        }
    }

    fn set_view(&mut self, view: View) {
        self.view = view;
        let found = self
            .nav
            .iter()
            .find(|id| self.nav.data::<View>(*id) == Some(&self.view));
        if let Some(id) = found {
            self.nav.activate(id);
        }
    }

    fn refresh_list(&mut self) {
        let Some(store) = &self.store else { return };
        self.notes = match store.search(&self.query, &self.view) {
            Ok(notes) => notes,
            Err(err) => {
                tracing::error!(%err, "listing notes");
                Vec::new()
            }
        };
    }

    /// Tags or trash membership may have changed: rebuild nav + list.
    fn after_store_change(&mut self) {
        self.rebuild_nav();
        self.refresh_list();
        if self.current.is_none()
            && let Some(first) = self.notes.first().map(|n| n.id.clone())
        {
            self.open_note(&first);
        }
    }

    fn open_note(&mut self, id: &str) {
        let Some(store) = self.store.as_mut() else {
            return;
        };
        match store.load(id) {
            Ok(Some(note)) => {
                self.editor = text_editor::Content::with_text(&note.body);
                self.backlinks = store.backlinks(&note.title).unwrap_or_default();
                self.current = Some(note);
                self.dirty = false;
            }
            Ok(None) => {
                self.current = None;
                self.refresh_list();
            }
            Err(err) => tracing::error!(%err, id, "loading note"),
        }
    }

    /// Write pending edits and let go of the current note.
    fn close_current(&mut self) {
        self.flush();
        let Some(note) = self.current.take() else {
            return;
        };
        self.backlinks.clear();
        if let Some(store) = self.store.as_mut() {
            match store.delete_if_empty(&note.id) {
                Ok(true) => self.after_store_change_keep_selection(),
                Ok(false) => {}
                Err(err) => tracing::error!(%err, "dropping empty note"),
            }
        }
    }

    fn after_store_change_keep_selection(&mut self) {
        self.rebuild_nav();
        self.refresh_list();
    }

    /// Persist the editor buffer if it has unsaved changes.
    fn flush(&mut self) {
        if !self.dirty {
            return;
        }
        self.dirty = false;
        let (Some(store), Some(note)) = (self.store.as_mut(), self.current.as_mut()) else {
            return;
        };
        let old_title = note.title.clone();
        note.body = self.editor.text();
        if let Err(err) = store.save(note) {
            tracing::error!(%err, "saving note");
            return;
        }
        if note.title != old_title {
            self.backlinks = store.backlinks(&note.title).unwrap_or_default();
        }
        // Tags may have changed; keep nav + list in step.
        self.rebuild_nav();
        self.refresh_list();
    }

    /// Apply one `JJB_SCRIPT` step by routing it through the normal messages.
    fn run_step(&mut self, step: Step) -> Task<cosmic::Action<Message>> {
        match step {
            Step::New => self.update(Message::NewNote),
            Step::Type(text) => {
                let mut task = Task::none();
                for c in text.chars() {
                    let edit = if c == '\n' {
                        text_editor::Edit::Enter
                    } else {
                        text_editor::Edit::Insert(c)
                    };
                    task = self.update(Message::Editor(text_editor::Action::Edit(edit)));
                }
                task
            }
            Step::Search(text) => self.update(Message::Search(text)),
            Step::Select(n) => match self.notes.get(n).map(|s| s.id.clone()) {
                Some(id) => self.update(Message::Select(id)),
                None => Task::none(),
            },
            Step::Pin => self.update(Message::TogglePin),
            Step::Trash => self.update(Message::TrashCurrent),
            Step::Wait(_) => Task::none(),
            Step::Exit => {
                self.close_current();
                std::process::exit(0);
            }
        }
    }

    fn update_title(&mut self) -> Task<cosmic::Action<Message>> {
        let mut window_title = fl!("app-title");
        if let Some(note) = &self.current {
            window_title.push_str(" — ");
            window_title.push_str(&note.title);
        }
        if let Some(id) = self.core.main_window_id() {
            self.set_window_title(window_title, id)
        } else {
            Task::none()
        }
    }
}

fn header_button<'a>(
    icon_name: &'static str,
    label: String,
    on_press: Message,
) -> Element<'a, Message> {
    widget::tooltip(
        widget::button::icon(icon::from_name(icon_name)).on_press(on_press),
        widget::text::body(label),
        widget::tooltip::Position::Bottom,
    )
    .into()
}

fn note_row(note: &NoteSummary) -> Element<'_, Message> {
    let spacing = cosmic::theme::spacing();
    let title = if note.pinned {
        format!("📌 {}", note.title)
    } else {
        note.title.clone()
    };
    let mut column = widget::column::with_capacity(3)
        .push(widget::text::heading(title))
        .spacing(spacing.space_xxxs);
    if !note.preview.is_empty() {
        column = column.push(widget::text::caption(note.preview.clone()));
    }
    column = column.push(widget::text::caption(format_date(note.modified)));
    column.width(Length::Fill).into()
}

/// Relative dates: time today, day+month this year, full date otherwise.
fn format_date(when: DateTime<Utc>) -> String {
    let local = when.with_timezone(&Local);
    let now = Local::now();
    if local.date_naive() == now.date_naive() {
        local.format("%H:%M").to_string()
    } else if local.year() == now.year() {
        local.format("%-d %b").to_string()
    } else {
        local.format("%-d %b %Y").to_string()
    }
}

/// Expand `a/b/c` tags into a depth-first tree, inserting implicit parents
/// (`a`, `a/b`) that no note carries directly with a count of 0.
fn tag_tree(tags: &[(String, usize)]) -> Vec<(String, usize)> {
    let mut out: Vec<(String, usize)> = Vec::new();
    for (name, count) in tags {
        let mut prefix = String::new();
        let segments: Vec<&str> = name.split('/').collect();
        for (i, segment) in segments.iter().enumerate() {
            if !prefix.is_empty() {
                prefix.push('/');
            }
            prefix.push_str(segment);
            let is_leaf = i + 1 == segments.len();
            match out.iter_mut().find(|(n, _)| *n == prefix) {
                Some(existing) if is_leaf => existing.1 = *count,
                Some(_) => {}
                None => out.push((prefix.clone(), if is_leaf { *count } else { 0 })),
            }
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn key_binds() -> HashMap<menu::KeyBind, MenuAction> {
    use menu::key_bind::Modifier;
    let mut binds = HashMap::new();
    binds.insert(
        menu::KeyBind {
            modifiers: vec![Modifier::Ctrl],
            key: keyboard::Key::Character("n".into()),
        },
        MenuAction::NewNote,
    );
    binds.insert(
        menu::KeyBind {
            modifiers: vec![Modifier::Ctrl],
            key: keyboard::Key::Character("f".into()),
        },
        MenuAction::FocusSearch,
    );
    binds.insert(
        menu::KeyBind {
            modifiers: vec![Modifier::Ctrl, Modifier::Shift],
            key: keyboard::Key::Character("d".into()),
        },
        MenuAction::TrashNote,
    );
    binds
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
    NewNote,
    TrashNote,
    FocusSearch,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
            MenuAction::NewNote => Message::NewNote,
            MenuAction::TrashNote => Message::TrashCurrent,
            MenuAction::FocusSearch => Message::FocusSearch,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_tree_inserts_parents() {
        let tags = vec![
            ("work/incab".to_string(), 2),
            ("work".to_string(), 1),
            ("zed/a/b".to_string(), 1),
        ];
        let tree = tag_tree(&tags);
        assert_eq!(
            tree,
            vec![
                ("work".to_string(), 1),
                ("work/incab".to_string(), 2),
                ("zed".to_string(), 0),
                ("zed/a".to_string(), 0),
                ("zed/a/b".to_string(), 1),
            ]
        );
    }
}
