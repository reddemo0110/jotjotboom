// SPDX-License-Identifier: GPL-3.0-only

use crate::config::Config;
use crate::debug_script::{self, Step};
use crate::fl;
use crate::images::{self, FrameStyle, ImageRef, PickerEntry, Processed, UriList};
use crate::markdown;
use crate::note::{Note, NoteSummary};
use crate::retro::{self, Palette};
use crate::store::{Store, View};
use chrono::{DateTime, Datelike, Local, Utc};
use cosmic::Application as _;
use cosmic::app::context_drawer;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::keyboard::{self, key::Physical};
use cosmic::iced::{Alignment, Event, Length, Subscription, event, window};
use cosmic::prelude::*;
use cosmic::widget::menu::action::MenuAction as _;
use cosmic::widget::{self, about::About, icon, menu, nav_bar, text_editor};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const APP_ICON: &[u8] = include_bytes!("../resources/icons/hicolor/scalable/apps/icon.svg");
/// How long after the last keystroke we write the note to disk.
const AUTOSAVE_DELAY: Duration = Duration::from_millis(600);
const NAV_WIDTH: f32 = 230.0;
const NOTE_LIST_WIDTH: f32 = 320.0;
const GAP: u16 = 14;

pub struct AppModel {
    core: cosmic::Core,
    context_page: ContextPage,
    about: About,
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    config: Config,
    config_handler: Option<cosmic_config::Config>,
    theme: retro::Theme,
    show_markers: bool,
    show_nav: bool,
    show_list: bool,
    placement: Placement,
    /// Processed images keyed by path|mtime|style|theme.
    image_cache: HashMap<String, ImageState>,
    /// In-app image picker (context drawer).
    picker_dir: PathBuf,
    picker_entries: Vec<PickerEntry>,
    /// A file drag is hovering the editor.
    drop_hover: bool,

    store: Option<Store>,
    store_error: Option<String>,
    view: View,
    /// Note counts for the fixed views: all, untagged, trash.
    view_counts: [usize; 3],
    /// Tag tree (full path, count), depth-first.
    tags: Vec<(String, usize)>,
    query: String,
    search_id: widget::Id,
    editor_id: widget::Id,
    /// The dock's `+` is expanded, showing new-note / new-folder.
    dock_open: bool,
    new_folder: String,
    folder_id: widget::Id,
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
    SetTheme(retro::Theme),
    ToggleMarkers,
    ToggleNav,
    ToggleList,
    ToggleSolo,
    SetPlacement(Placement),
    ImagesDropped(Vec<PathBuf>),
    PickImage,
    PickerNavigate(PathBuf),
    PickerChoose(PathBuf),
    DragEnter,
    DragLeave,
    Dropped(Option<UriList>),
    ImageLoaded(String, Result<Processed, String>),
    CycleFrame(usize),
    CycleSize(usize),
    OpenImage(String),
    FontLoaded,

    Editor(text_editor::Action),
    AutosaveTick,
    SetView(View),
    Select(String),
    NewNote,
    TrashCurrent,
    RestoreCurrent,
    DeleteCurrentForever,
    EmptyTrash,
    TogglePin,
    Search(String),
    FocusSearch,
    Format(Format),
    ToggleDock,
    NewFolderName(String),
    CreateFolder,
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

        let theme = retro::Theme::from_key(&config.theme);
        let show_markers = config.show_markers;
        let (show_nav, show_list) = (!config.hide_nav, !config.hide_list);
        let placement = Placement::from_key(&config.image_placement);
        let mut app = AppModel {
            core,
            context_page: ContextPage::default(),
            about,
            key_binds: key_binds(),
            config,
            config_handler,
            theme,
            show_markers,
            show_nav,
            show_list,
            placement,
            image_cache: HashMap::new(),
            picker_dir: dirs::picture_dir()
                .or_else(dirs::home_dir)
                .unwrap_or_else(|| PathBuf::from("/")),
            picker_entries: Vec::new(),
            drop_hover: false,
            store,
            store_error,
            view: View::All,
            view_counts: [0; 3],
            tags: Vec::new(),
            query: String::new(),
            search_id: widget::Id::unique(),
            editor_id: widget::Id::unique(),
            dock_open: false,
            new_folder: String::new(),
            folder_id: widget::Id::unique(),
            notes: Vec::new(),
            current: None,
            editor: text_editor::Content::new(),
            dirty: false,
            last_edit: Instant::now(),
            backlinks: Vec::new(),
            screenshot_pending: std::env::var_os("JJB_SCREENSHOT").is_some(),
            script: debug_script::Runner::from_env(),
        };

        app.refresh_tags();
        app.refresh_list();
        // Open the most recent note so the window isn't empty on launch.
        if let Some(first) = app.notes.first().map(|n| n.id.clone()) {
            app.open_note(&first);
        }

        let title = app.update_title();
        let font = cosmic::iced::font::load(retro::TITLE_FONT_BYTES).map(|result| {
            if let Err(err) = result {
                tracing::error!(?err, "loading title font");
            }
            cosmic::Action::App(Message::FontLoaded)
        });
        (app, Task::batch([font, title]))
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(retro::app_style(&self.palette()))
    }

    fn header_start(&self) -> Vec<Element<'_, Self::Message>> {
        let view_items = vec![
            menu::Item::CheckBox(fl!("show-nav"), None, self.show_nav, MenuAction::ToggleNav),
            menu::Item::CheckBox(
                fl!("show-list"),
                None,
                self.show_list,
                MenuAction::ToggleList,
            ),
            menu::Item::Button(fl!("editor-only"), None, MenuAction::Solo),
            menu::Item::Divider,
            menu::Item::CheckBox(
                fl!("images-rail"),
                None,
                self.placement == Placement::Rail,
                MenuAction::Placement(Placement::Rail),
            ),
            menu::Item::CheckBox(
                fl!("images-top"),
                None,
                self.placement == Placement::Top,
                MenuAction::Placement(Placement::Top),
            ),
            menu::Item::CheckBox(
                fl!("images-bottom"),
                None,
                self.placement == Placement::Bottom,
                MenuAction::Placement(Placement::Bottom),
            ),
            menu::Item::Divider,
            menu::Item::Button(fl!("theme-colours"), None, MenuAction::Themes),
            menu::Item::CheckBox(
                fl!("show-markers"),
                None,
                self.show_markers,
                MenuAction::ToggleMarkers,
            ),
            menu::Item::Divider,
            menu::Item::Button(fl!("about"), None, MenuAction::About),
        ];

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
                menu::items(&self.key_binds, view_items),
            ),
        ]);

        let toggle = |icon_name: &'static str, label: String, on: bool, msg: Message| {
            widget::tooltip(
                widget::button::icon(icon::from_name(icon_name))
                    .selected(on)
                    .on_press(msg),
                widget::text::body(label),
                widget::tooltip::Position::Bottom,
            )
            .into()
        };
        vec![
            menu_bar.into(),
            toggle(
                "sidebar-places-symbolic",
                fl!("show-nav"),
                self.show_nav,
                Message::ToggleNav,
            ),
            toggle(
                "view-list-symbolic",
                fl!("show-list"),
                self.show_list,
                Message::ToggleList,
            ),
        ]
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
                items.push(header_button("pin-symbolic", pin_label, Message::TogglePin));
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
        None
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
            ContextPage::Themes => context_drawer::context_drawer(
                self.theme_picker(),
                Message::ToggleContextPage(ContextPage::Themes),
            )
            .title(fl!("theme-colours")),
            ContextPage::Picker => context_drawer::context_drawer(
                self.file_picker(),
                Message::ToggleContextPage(ContextPage::Picker),
            )
            .title(fl!("pick-image")),
        })
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let p = self.palette();
        if let Some(err) = &self.store_error {
            return widget::container(retro::text(&p, fl!("store-error", error = err.as_str())))
                .padding(24)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .into();
        }

        let nav = widget::column::with_capacity(2)
            .push(
                widget::container(self.views_frame(&p))
                    .height(Length::Fixed(140.0))
                    .width(Length::Fill),
            )
            .push(self.tags_frame(&p))
            .spacing(GAP)
            .width(Length::Fixed(NAV_WIDTH))
            .height(Length::Fill);

        let list = widget::container(self.notes_frame(&p))
            .width(Length::Fixed(NOTE_LIST_WIDTH))
            .height(Length::Fill);

        let mut editor_col = widget::column::with_capacity(2)
            .push(self.editor_frame(&p))
            .spacing(GAP)
            .width(Length::Fill)
            .height(Length::Fill);
        if !self.backlinks.is_empty() {
            editor_col = editor_col.push(
                widget::container(self.backlinks_frame(&p))
                    .height(Length::Fixed(64.0))
                    .width(Length::Fill),
            );
        }

        let mut panes = widget::row::with_capacity(3)
            .spacing(GAP)
            .width(Length::Fill)
            .height(Length::Fill);
        if self.show_nav {
            panes = panes.push(nav);
        }
        if self.show_list {
            panes = panes.push(list);
        }
        panes = panes.push(editor_col);

        widget::container(panes)
            .padding([GAP + 4, GAP, GAP, GAP])
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
                Event::Window(window::Event::FileDropped(paths)) => {
                    Some(Message::ImagesDropped(paths))
                }
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
        let task = self.update_inner(message);
        let loads = self.image_loads();
        if loads.is_empty() {
            task
        } else {
            Task::batch(std::iter::once(task).chain(loads))
        }
    }

    fn on_escape(&mut self) -> Task<cosmic::Action<Self::Message>> {
        if self.dock_open {
            self.dock_open = false;
        } else if self.core.window.show_context {
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
    fn update_inner(&mut self, message: Message) -> Task<cosmic::Action<Message>> {
        match message {
            Message::SetPlacement(placement) => {
                self.placement = placement;
                if let Some(handler) = &self.config_handler
                    && let Err(why) = self
                        .config
                        .set_image_placement(handler, placement.key().to_owned())
                {
                    tracing::warn!(%why, "could not persist image placement");
                }
            }

            Message::ImagesDropped(paths) => {
                let mut task = Task::none();
                for path in paths.into_iter().filter(|p| images::is_image_file(p)) {
                    task = self.import_image(&path);
                }
                return task;
            }

            Message::PickImage => {
                if self.current.as_ref().is_none_or(|n| n.trashed) {
                    return Task::none();
                }
                self.picker_entries = images::list_dir(&self.picker_dir);
                self.context_page = ContextPage::Picker;
                self.core.window.show_context = true;
            }

            Message::PickerNavigate(dir) => {
                self.picker_dir = dir;
                self.picker_entries = images::list_dir(&self.picker_dir);
            }

            Message::PickerChoose(path) => {
                self.core.window.show_context = false;
                return self.import_image(&path);
            }

            Message::DragEnter => self.drop_hover = true,
            Message::DragLeave => self.drop_hover = false,

            Message::Dropped(list) => {
                self.drop_hover = false;
                let mut task = Task::none();
                if let Some(UriList(paths)) = list {
                    for path in paths.into_iter().filter(|p| images::is_image_file(p)) {
                        task = self.import_image(&path);
                    }
                }
                return task;
            }

            Message::ImageLoaded(key, result) => {
                let state = match result {
                    Ok(Processed::Pixels {
                        width,
                        height,
                        rgba,
                    }) => ImageState::Ready(
                        widget::image::Handle::from_rgba(width, height, rgba),
                        width,
                        height,
                    ),
                    Ok(Processed::Ascii(text)) => ImageState::Ascii(text),
                    Err(err) => ImageState::Failed(err),
                };
                self.image_cache.insert(key, state);
            }

            Message::CycleFrame(index) => self.edit_image_ref(index, |r| r.frame = r.frame.next()),
            Message::CycleSize(index) => self.edit_image_ref(index, |r| r.size = r.size.next()),

            Message::OpenImage(rel) => {
                if let Some(store) = &self.store {
                    let path = images::resolve(store.notes_dir(), &rel);
                    if let Err(err) = open::that_detached(&path) {
                        tracing::warn!(%err, "opening image");
                    }
                }
            }

            Message::FontLoaded => {}

            Message::SetTheme(theme) => {
                self.theme = theme;
                if let Some(handler) = &self.config_handler
                    && let Err(why) = self.config.set_theme(handler, theme.key().to_owned())
                {
                    tracing::warn!(%why, "could not persist theme");
                }
            }

            Message::ToggleNav => {
                self.show_nav = !self.show_nav;
                self.persist_layout();
            }

            Message::ToggleList => {
                self.show_list = !self.show_list;
                self.persist_layout();
            }

            Message::ToggleSolo => {
                let solo = !self.show_nav && !self.show_list;
                self.show_nav = solo;
                self.show_list = solo;
                self.persist_layout();
                if !solo {
                    return widget::text_input::focus(self.editor_id.clone());
                }
            }

            Message::ToggleMarkers => {
                self.show_markers = !self.show_markers;
                if let Some(handler) = &self.config_handler
                    && let Err(why) = self.config.set_show_markers(handler, self.show_markers)
                {
                    tracing::warn!(%why, "could not persist marker setting");
                }
            }

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

            Message::SetView(view) => {
                if self.view == view {
                    return Task::none();
                }
                self.close_current();
                self.view = view;
                self.refresh_list();
                if let Some(first) = self.notes.first().map(|n| n.id.clone()) {
                    self.open_note(&first);
                }
                return self.update_title();
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
                self.dock_open = false;
                if matches!(self.view, View::Trash) {
                    self.view = View::All;
                }
                self.close_current();
                self.query.clear();
                // A note started inside a folder carries that tag from the outset.
                let tag_line = match &self.view {
                    View::Tag(t) => Some(format!("\n\n#{t}\n")),
                    _ => None,
                };
                let created = self.store.as_mut().and_then(|s| match s.create() {
                    Ok(mut note) => {
                        if let Some(line) = tag_line {
                            note.body = line;
                            if let Err(err) = s.save(&mut note) {
                                tracing::error!(%err, "pre-filling folder tag");
                            }
                        }
                        Some(note)
                    }
                    Err(err) => {
                        tracing::error!(%err, "creating note");
                        None
                    }
                });
                if let Some(note) = created {
                    self.refresh_tags();
                    self.refresh_list();
                    self.open_note(&note.id);
                    return Task::batch([
                        self.update_title(),
                        widget::text_input::focus(self.editor_id.clone()),
                    ]);
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

            Message::FocusSearch => {
                return widget::text_input::focus(self.search_id.clone());
            }

            Message::Format(format) => {
                if self.current.as_ref().is_some_and(|n| !n.trashed) {
                    self.apply_format(format);
                    return widget::text_input::focus(self.editor_id.clone());
                }
            }

            Message::ToggleDock => {
                self.dock_open = !self.dock_open;
                if self.dock_open {
                    return widget::text_input::focus(self.folder_id.clone());
                }
            }

            Message::NewFolderName(name) => {
                self.new_folder = name;
            }

            Message::CreateFolder => {
                let name = std::mem::take(&mut self.new_folder);
                let created = self.store.as_mut().and_then(|s| match s.add_folder(&name) {
                    Ok(tag) => tag,
                    Err(err) => {
                        tracing::error!(%err, "creating folder");
                        None
                    }
                });
                self.dock_open = false;
                if let Some(tag) = created {
                    self.refresh_tags();
                    return self.update(Message::SetView(View::Tag(tag)));
                }
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
                self.theme = retro::Theme::from_key(&config.theme);
                self.show_markers = config.show_markers;
                self.show_nav = !config.hide_nav;
                self.show_list = !config.hide_list;
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

    fn palette(&self) -> Palette {
        self.theme.palette(self.core.system_theme())
    }

    // ----- frames -----

    fn views_frame<'a>(&'a self, p: &Palette) -> Element<'a, Message> {
        let rows = [
            (View::All, fl!("nav-all-notes"), self.view_counts[0]),
            (View::Untagged, fl!("nav-untagged"), self.view_counts[1]),
            (View::Trash, fl!("nav-trash"), self.view_counts[2]),
        ];
        let mut col = widget::column::with_capacity(3).spacing(2);
        for (view, label, count) in rows {
            let selected = self.view == view;
            let marker = if selected { "▌" } else { " " };
            let row = widget::row::with_capacity(3)
                .push(retro::accent(p, marker))
                .push(retro::text(p, label).width(Length::Fill))
                .push(retro::dim(p, count.to_string()))
                .spacing(8)
                .align_y(Alignment::Center);
            col = col.push(
                widget::button::custom(row)
                    .padding([4, 8])
                    .width(Length::Fill)
                    .class(retro::row_class(p, selected))
                    .on_press(Message::SetView(view)),
            );
        }
        retro::frame(p, fl!("frame-views"), None, col)
    }

    fn tags_frame<'a>(&'a self, p: &Palette) -> Element<'a, Message> {
        let mut col = widget::column::with_capacity(self.tags.len()).spacing(1);
        for (name, count) in &self.tags {
            let depth = name.matches('/').count();
            let leaf = name.rsplit('/').next().unwrap_or(name);
            let prefix = if depth == 0 {
                String::new()
            } else {
                format!("{}└─ ", "   ".repeat(depth - 1))
            };
            let selected = matches!(&self.view, View::Tag(t) if t == name);
            let count_text = if *count > 0 {
                count.to_string()
            } else {
                String::new()
            };
            let row = widget::row::with_capacity(4)
                .push(retro::dim(p, prefix).class(cosmic::theme::Text::Color(p.mute)))
                .push(retro::accent2(p, "#"))
                .push(retro::text(p, leaf.to_owned()).width(Length::Fill))
                .push(retro::dim(p, count_text))
                .align_y(Alignment::Center);
            col = col.push(
                widget::button::custom(row)
                    .padding([3, 8])
                    .width(Length::Fill)
                    .class(retro::row_class(p, selected))
                    .on_press(Message::SetView(View::Tag(name.clone()))),
            );
        }
        let body: Element<'_, Message> = if self.tags.is_empty() {
            widget::container(retro::dim(p, fl!("no-tags-yet")))
                .padding([8, 8])
                .into()
        } else {
            widget::scrollable(col).height(Length::Fill).into()
        };
        retro::frame(p, fl!("frame-tags"), None, body)
    }

    fn notes_frame<'a>(&'a self, p: &Palette) -> Element<'a, Message> {
        let total = match &self.view {
            View::All => self.view_counts[0],
            View::Untagged => self.view_counts[1],
            View::Trash => self.view_counts[2],
            View::Tag(_) if self.query.is_empty() => self.notes.len(),
            View::Tag(t) => self
                .store
                .as_ref()
                .and_then(|s| s.list(&View::Tag(t.clone())).ok())
                .map_or(self.notes.len(), |v| v.len()),
        };
        let badge = if self.query.is_empty() {
            fl!("notes-count", count = total)
        } else {
            fl!("notes-of", shown = self.notes.len(), total = total)
        };

        let search = widget::text_input(fl!("search-placeholder"), &self.query)
            .id(self.search_id.clone())
            .font(retro::mono())
            .size(13)
            .padding([5, 8])
            .leading_icon(
                widget::container(retro::accent(p, "/"))
                    .padding([0, 0, 0, 8])
                    .into(),
            )
            .style(retro::search_class(p))
            .on_input(Message::Search);

        let body: Element<'_, Message> = if self.notes.is_empty() {
            let text = if self.query.is_empty() {
                fl!("no-notes-here")
            } else {
                fl!("no-results", query = self.query.as_str())
            };
            widget::container(retro::dim(p, text))
                .padding(12)
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .into()
        } else {
            let mut col = widget::column::with_capacity(self.notes.len()).spacing(4);
            for note in &self.notes {
                let selected = self.current.as_ref().is_some_and(|c| c.id == note.id);
                col = col.push(
                    widget::button::custom(note_row(p, note, selected))
                        .padding([7, 8])
                        .width(Length::Fill)
                        .class(retro::row_class(p, selected))
                        .on_press(Message::Select(note.id.clone())),
                );
            }
            widget::scrollable(col).height(Length::Fill).into()
        };

        let content = widget::column::with_capacity(2)
            .push(search)
            .push(body)
            .spacing(8)
            .width(Length::Fill)
            .height(Length::Fill);
        retro::frame(p, fl!("frame-notes"), Some(badge), content)
    }

    fn editor_frame<'a>(&'a self, p: &Palette) -> Element<'a, Message> {
        let Some(note) = &self.current else {
            let hint = widget::container(retro::dim(p, fl!("no-note-selected")))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center);
            let content = widget::column::with_capacity(2)
                .push(hint)
                .push(self.dock(p))
                .spacing(8)
                .width(Length::Fill)
                .height(Length::Fill);
            return retro::frame(p, fl!("app-title").to_lowercase(), None, content);
        };
        let badge = if self.drop_hover {
            fl!("badge-drop")
        } else if self.dirty {
            fl!("badge-editing")
        } else if note.trashed {
            fl!("badge-in-trash")
        } else {
            fl!("badge-saved", time = format_time(note.modified))
        };

        let settings = markdown::Settings {
            palette: *p,
            show_markers: self.show_markers,
            font: retro::mono(),
        };
        let mut editor = cosmic::iced::widget::text_editor(&self.editor)
            .id(self.editor_id.clone())
            .placeholder(fl!("untitled"))
            .font(retro::mono())
            .size(15)
            .line_height(1.5)
            .padding([6, 10])
            .style(retro::editor_style(*p))
            .height(Length::Fill)
            .highlight_with::<markdown::MarkdownHighlighter>(settings, markdown::to_format);
        if !note.trashed {
            editor = editor.on_action(Message::Editor);
        }

        let refs = images::parse_refs(&self.editor.text());
        let placement = self.note_placement(note);
        let text_area: Element<'a, Message> = if refs.is_empty() {
            editor.into()
        } else {
            match placement {
                Placement::Rail => widget::row::with_capacity(2)
                    .push(editor)
                    .push(
                        widget::container(widget::scrollable(self.image_stack(p, &refs, 232.0)))
                            .width(Length::Fixed(240.0))
                            .height(Length::Fill)
                            .padding([4, 0, 0, 4]),
                    )
                    .spacing(8)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into(),
                Placement::Top => widget::column::with_capacity(2)
                    .push(self.image_strip(p, &refs))
                    .push(editor)
                    .spacing(8)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into(),
                Placement::Bottom => widget::column::with_capacity(2)
                    .push(editor)
                    .push(self.image_strip(p, &refs))
                    .spacing(8)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into(),
            }
        };

        let mut column = widget::column::with_capacity(3)
            .push(text_area)
            .spacing(8)
            .width(Length::Fill)
            .height(Length::Fill);
        if note.trashed {
            column =
                column.push(widget::container(retro::dim(p, fl!("trash-hint"))).padding([6, 10]));
        }
        column = column.push(self.dock(p));

        let framed = retro::frame(p, note.title.clone(), Some(badge), column);
        widget::dnd_destination::dnd_destination_for_data::<UriList, Message>(
            framed,
            |data, _action| Message::Dropped(data),
        )
        .on_enter(|_, _, _| Message::DragEnter)
        .on_leave(|| Message::DragLeave)
        .into()
    }

    fn backlinks_frame<'a>(&'a self, p: &Palette) -> Element<'a, Message> {
        let mut row = widget::row::with_capacity(self.backlinks.len() + 1)
            .spacing(6)
            .align_y(Alignment::Center)
            .push(retro::dim(p, "←").class(cosmic::theme::Text::Color(p.mute)));
        for link in &self.backlinks {
            row = row.push(
                widget::button::custom(retro::accent2(p, link.title.clone()))
                    .padding([2, 6])
                    .class(retro::row_class(p, false))
                    .on_press(Message::Select(link.id.clone())),
            );
        }
        retro::frame(
            p,
            fl!("linked-from").to_lowercase(),
            None,
            widget::scrollable(row).direction(
                cosmic::iced::widget::scrollable::Direction::Horizontal(
                    cosmic::iced::widget::scrollable::Scrollbar::default(),
                ),
            ),
        )
    }

    /// The dock: format actions for the open note, `+`, and the theme
    /// picker, as a centred pill at the foot of the editor. The `+` section
    /// opens as a second pill underneath so the main row never overflows.
    fn dock<'a>(&'a self, p: &Palette) -> Element<'a, Message> {
        let editable = self.current.as_ref().is_some_and(|n| !n.trashed);
        let divider = || {
            widget::container(retro::dim(p, "│").class(cosmic::theme::Text::Color(p.mute)))
                .padding([0, 1])
        };
        let mut row = widget::row::with_capacity(16)
            .spacing(1)
            .align_y(Alignment::Center);

        for format in Format::ALL {
            let button = widget::button::custom(retro::text(p, format.glyph()).size(14))
                .padding([3, 7])
                .class(retro::row_class(p, false))
                .on_press_maybe(editable.then_some(Message::Format(format)));
            row = row.push(widget::tooltip(
                button,
                retro::dim(p, format.label()),
                widget::tooltip::Position::Top,
            ));
        }

        row = row.push(divider()).push(widget::tooltip(
            widget::button::custom(retro::accent(p, "+").size(16))
                .padding([1, 7])
                .class(retro::row_class(p, self.dock_open))
                .on_press(Message::ToggleDock),
            retro::dim(p, fl!("dock-plus")),
            widget::tooltip::Position::Top,
        ));

        row = row.push(widget::tooltip(
            widget::button::custom(retro::accent(p, "⧉").size(15))
                .padding([2, 7])
                .class(retro::row_class(p, false))
                .on_press_maybe(editable.then_some(Message::PickImage)),
            retro::dim(p, fl!("dock-image")),
            widget::tooltip::Position::Top,
        ));

        row = row.push(divider()).push(widget::tooltip(
            widget::button::custom(retro::accent2(p, "◐").size(15))
                .padding([2, 7])
                .class(retro::row_class(
                    p,
                    self.core.window.show_context && self.context_page == ContextPage::Themes,
                ))
                .on_press(Message::ToggleContextPage(ContextPage::Themes)),
            retro::dim(p, fl!("theme-colours")),
            widget::tooltip::Position::Top,
        ));

        let pill = |content: Element<'a, Message>| {
            widget::container(
                widget::container(content)
                    .padding([3, 6])
                    .class(retro::dock_class(p)),
            )
            .width(Length::Fill)
            .align_x(Alignment::Center)
        };

        let mut dock = widget::column::with_capacity(2)
            .push(pill(row.into()))
            .spacing(6)
            .width(Length::Fill);

        if self.dock_open {
            let create = widget::row::with_capacity(3)
                .push(
                    widget::button::custom(retro::text(p, fl!("dock-new-note")))
                        .padding([3, 8])
                        .class(retro::row_class(p, false))
                        .on_press(Message::NewNote),
                )
                .push(
                    widget::text_input(fl!("folder-name-placeholder"), &self.new_folder)
                        .id(self.folder_id.clone())
                        .font(retro::mono())
                        .size(13)
                        .padding([3, 8])
                        .width(Length::Fixed(170.0))
                        .leading_icon(
                            widget::container(retro::accent2(p, "#"))
                                .padding([0, 0, 0, 8])
                                .into(),
                        )
                        .style(retro::search_class(p))
                        .on_input(Message::NewFolderName)
                        .on_submit(|_| Message::CreateFolder),
                )
                .push(
                    widget::button::custom(retro::text(p, fl!("dock-new-folder")))
                        .padding([3, 8])
                        .class(retro::row_class(p, false))
                        .on_press_maybe(
                            (!self.new_folder.trim().is_empty()).then_some(Message::CreateFolder),
                        ),
                )
                .spacing(4)
                .align_y(Alignment::Center);
            dock = dock.push(pill(create.into()));
        }

        dock.into()
    }

    /// The theme picker shown in the context drawer.
    fn theme_picker(&self) -> Element<'_, Message> {
        let system = self.core.system_theme();
        let label_color: cosmic::iced::Color = system.cosmic().background(false).on.into();
        let mut col = widget::column::with_capacity(retro::Theme::ALL.len() + 1)
            .push(widget::text::caption(fl!("theme-picker-hint")))
            .spacing(6)
            .width(Length::Fill);
        for theme in retro::Theme::ALL {
            let palette = theme.palette(system);
            col = col.push(retro::swatch(
                theme,
                &palette,
                self.theme == theme,
                label_color,
                Message::SetTheme(theme),
            ));
        }
        widget::scrollable(col).into()
    }

    /// Apply a dock format action to the editor buffer.
    fn apply_format(&mut self, format: Format) {
        use text_editor::{Action, Edit, Motion};
        let perform = |editor: &mut text_editor::Content, action: Action| editor.perform(action);
        let insert_str = |editor: &mut text_editor::Content, s: &str| {
            for c in s.chars() {
                editor.perform(Action::Edit(if c == '\n' {
                    Edit::Enter
                } else {
                    Edit::Insert(c)
                }));
            }
        };

        match format {
            Format::Bold | Format::Italic | Format::Code | Format::Link => {
                let (before, after) = match format {
                    Format::Bold => ("**", "**"),
                    Format::Italic => ("*", "*"),
                    Format::Code => ("`", "`"),
                    _ => ("[[", "]]"),
                };
                if let Some(selection) = self.editor.selection() {
                    let text = format!("{before}{selection}{after}");
                    perform(&mut self.editor, Action::Edit(Edit::Paste(Arc::new(text))));
                } else {
                    insert_str(&mut self.editor, before);
                    insert_str(&mut self.editor, after);
                    for _ in 0..after.chars().count() {
                        perform(&mut self.editor, Action::Move(Motion::Left));
                    }
                }
            }
            Format::H1 | Format::H2 | Format::Bullet | Format::Todo => {
                let prefix = match format {
                    Format::H1 => "# ",
                    Format::H2 => "## ",
                    Format::Bullet => "- ",
                    _ => "- [ ] ",
                };
                let cursor = self.editor.cursor();
                let line = self
                    .editor
                    .line(cursor.position.line)
                    .map(|l| l.text.into_owned())
                    .unwrap_or_default();
                perform(&mut self.editor, Action::Move(Motion::Home));
                // Toggle: strip the same prefix if the line already carries it,
                // otherwise replace a different line prefix and add ours.
                let existing = ["- [ ] ", "- [x] ", "## ", "# ", "- "]
                    .into_iter()
                    .find(|p| line.starts_with(p));
                if let Some(existing) = existing {
                    for _ in 0..existing.chars().count() {
                        perform(&mut self.editor, Action::Edit(Edit::Delete));
                    }
                }
                if existing != Some(prefix) {
                    insert_str(&mut self.editor, prefix);
                }
                perform(&mut self.editor, Action::Move(Motion::End));
            }
            Format::Tag => insert_str(&mut self.editor, "#"),
            Format::Rule => insert_str(&mut self.editor, "\n---\n"),
        }
        self.dirty = true;
        self.last_edit = Instant::now();
    }

    /// In-app image picker: folders first, then image files.
    fn file_picker(&self) -> Element<'_, Message> {
        let mut col = widget::column::with_capacity(4)
            .spacing(8)
            .width(Length::Fill);

        let mut places = widget::row::with_capacity(3).spacing(6);
        for (icon_name, label, dir) in [
            ("user-home-symbolic", fl!("picker-home"), dirs::home_dir()),
            (
                "folder-pictures-symbolic",
                fl!("picker-pictures"),
                dirs::picture_dir(),
            ),
            (
                "folder-download-symbolic",
                fl!("picker-downloads"),
                dirs::download_dir(),
            ),
        ] {
            if let Some(dir) = dir {
                places = places.push(
                    widget::button::text(label)
                        .leading_icon(icon::from_name(icon_name))
                        .on_press(Message::PickerNavigate(dir)),
                );
            }
        }
        col = col.push(places);
        col = col.push(widget::text::caption(self.picker_dir.display().to_string()));

        let mut list = widget::list_column();
        if let Some(parent) = self.picker_dir.parent() {
            list = list.add(
                widget::list::button(
                    widget::row::with_capacity(2)
                        .push(icon::from_name("go-up-symbolic").size(16))
                        .push(widget::text::body(".."))
                        .spacing(10)
                        .align_y(Alignment::Center),
                )
                .on_press(Message::PickerNavigate(parent.to_owned())),
            );
        }
        for entry in &self.picker_entries {
            let icon_name = if entry.is_dir {
                "folder-symbolic"
            } else {
                "image-x-generic-symbolic"
            };
            let msg = if entry.is_dir {
                Message::PickerNavigate(entry.path.clone())
            } else {
                Message::PickerChoose(entry.path.clone())
            };
            list = list.add(
                widget::list::button(
                    widget::row::with_capacity(2)
                        .push(icon::from_name(icon_name).size(16))
                        .push(widget::text::body(entry.name.clone()))
                        .spacing(10)
                        .align_y(Alignment::Center),
                )
                .on_press(msg),
            );
        }
        if self.picker_entries.is_empty() {
            col = col.push(widget::text::caption(fl!("picker-empty")));
        }
        col = col.push(list);
        widget::scrollable(col).into()
    }

    // ----- images -----

    /// Per-note `images:` frontmatter overrides the global placement.
    fn note_placement(&self, note: &Note) -> Placement {
        note.extra_frontmatter
            .iter()
            .find_map(|l| l.strip_prefix("images:").map(str::trim))
            .and_then(Placement::from_key_opt)
            .unwrap_or(self.placement)
    }

    fn image_key(&self, r: &ImageRef) -> Option<(String, PathBuf)> {
        let store = self.store.as_ref()?;
        let path = images::resolve(store.notes_dir(), &r.path);
        let mtime = std::fs::metadata(&path)
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map_or(0, |d| d.as_nanos());
        let theme = if r.frame.themed() {
            self.theme.key()
        } else {
            ""
        };
        Some((
            format!("{}|{mtime}|{}|{theme}", path.display(), r.frame.key()),
            path,
        ))
    }

    /// Spawn loads for every referenced image not yet in the cache.
    fn image_loads(&mut self) -> Vec<Task<cosmic::Action<Message>>> {
        let Some(note) = &self.current else {
            return Vec::new();
        };
        if note.trashed {
            return Vec::new();
        }
        let palette = self.palette();
        let refs = images::parse_refs(&self.editor.text());
        let mut tasks = Vec::new();
        for r in refs {
            let Some((key, path)) = self.image_key(&r) else {
                continue;
            };
            if self.image_cache.contains_key(&key) {
                continue;
            }
            self.image_cache.insert(key.clone(), ImageState::Loading);
            let style = r.frame;
            tasks.push(Task::perform(
                async move {
                    tokio::task::spawn_blocking(move || {
                        images::load_and_process(&path, style, palette)
                    })
                    .await
                    .map_err(|e| e.to_string())
                    .and_then(|r| r.map_err(|e| format!("{e:#}")))
                },
                move |result| cosmic::Action::App(Message::ImageLoaded(key.clone(), result)),
            ));
        }
        tasks
    }

    /// Copy a file into assets/ and add its line to the note at the cursor.
    fn import_image(&mut self, path: &std::path::Path) -> Task<cosmic::Action<Message>> {
        if self.current.as_ref().is_none_or(|n| n.trashed) {
            return Task::none();
        }
        let Some(store) = &self.store else {
            return Task::none();
        };
        let rel = match images::import_asset(store.notes_dir(), path) {
            Ok(rel) => rel,
            Err(err) => {
                tracing::error!(%err, "importing image");
                return Task::none();
            }
        };
        let alt = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("image")
            .to_owned();
        let line = ImageRef {
            line: 0,
            alt,
            path: rel,
            frame: FrameStyle::default(),
            size: images::Size::default(),
        }
        .to_markdown();
        use text_editor::{Action, Edit, Motion};
        let cursor = self.editor.cursor();
        let current_line_empty = self
            .editor
            .line(cursor.position.line)
            .is_none_or(|l| l.text.trim().is_empty());
        self.editor.perform(Action::Move(Motion::End));
        if !current_line_empty {
            self.editor.perform(Action::Edit(Edit::Enter));
        }
        for c in line.chars() {
            self.editor.perform(Action::Edit(Edit::Insert(c)));
        }
        self.editor.perform(Action::Edit(Edit::Enter));
        self.dirty = true;
        self.last_edit = Instant::now();
        widget::text_input::focus(self.editor_id.clone())
    }

    /// Rewrite the n-th image line after mutating its reference.
    fn edit_image_ref(&mut self, index: usize, f: impl FnOnce(&mut ImageRef)) {
        let body = self.editor.text();
        let refs = images::parse_refs(&body);
        let Some(mut r) = refs.get(index).cloned() else {
            return;
        };
        f(&mut r);
        let new_body = images::replace_line(&body, r.line, &r.to_markdown());
        let cursor = self.editor.cursor();
        self.editor = text_editor::Content::with_text(&new_body);
        self.editor.move_to(cursor);
        self.dirty = true;
        self.last_edit = Instant::now();
    }

    fn image_stack<'a>(
        &'a self,
        p: &Palette,
        refs: &[ImageRef],
        width: f32,
    ) -> Element<'a, Message> {
        let mut col = widget::column::with_capacity(refs.len())
            .spacing(14)
            .width(Length::Fixed(width));
        for (i, r) in refs.iter().enumerate() {
            let w = match r.size {
                images::Size::S => width * 0.55,
                images::Size::M => width * 0.8,
                images::Size::L => width,
            };
            col = col.push(self.image_card(p, r, i, w));
        }
        col.into()
    }

    fn image_strip<'a>(&'a self, p: &Palette, refs: &[ImageRef]) -> Element<'a, Message> {
        let mut row = widget::row::with_capacity(refs.len())
            .spacing(14)
            .align_y(Alignment::Start);
        for (i, r) in refs.iter().enumerate() {
            let w = match r.size {
                images::Size::S => 130.0,
                images::Size::M => 210.0,
                images::Size::L => 320.0,
            };
            row = row.push(self.image_card(p, r, i, w));
        }
        widget::container(widget::scrollable(row).direction(
            cosmic::iced::widget::scrollable::Direction::Horizontal(
                cosmic::iced::widget::scrollable::Scrollbar::default(),
            ),
        ))
        .padding([6, 4])
        .width(Length::Fill)
        .into()
    }

    /// One image with its frame treatment and the chips to change it.
    fn image_card<'a>(
        &'a self,
        p: &Palette,
        r: &ImageRef,
        index: usize,
        width: f32,
    ) -> Element<'a, Message> {
        let state = self
            .image_key(r)
            .and_then(|(k, _)| self.image_cache.get(&k));
        let pic: Element<'a, Message> = match state {
            Some(ImageState::Ready(handle, w, h)) => {
                let (w, h) = (*w, *h);
                let img = widget::image(handle.clone())
                    .width(Length::Fill)
                    .content_fit(cosmic::iced::ContentFit::Contain);
                match r.frame {
                    FrameStyle::Box => {
                        retro::frame(p, r.file_name().to_owned(), Some(format!("{w}×{h}")), img)
                    }
                    FrameStyle::Tint | FrameStyle::Dither | FrameStyle::Pixel => {
                        retro::bordered(p, img.into())
                    }
                    FrameStyle::Bezel => retro::bezel(p, img.into()),
                    FrameStyle::Print => retro::print(p, img.into(), r.alt.clone()),
                    FrameStyle::Film => retro::film(p, img.into(), index + 1),
                    FrameStyle::Ascii => retro::bordered(p, img.into()),
                }
            }
            Some(ImageState::Ascii(text)) => retro::ascii_card(p, text.clone()),
            Some(ImageState::Failed(err)) => widget::container(retro::dim(p, format!("⚠ {err}")))
                .padding(8)
                .width(Length::Fill)
                .into(),
            Some(ImageState::Loading) | None => widget::container(retro::dim(p, "…"))
                .padding(8)
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .into(),
        };

        let chip = |label: String, msg: Message| {
            widget::button::custom(retro::dim(p, label).size(11))
                .padding([1, 5])
                .class(retro::row_class(p, false))
                .on_press(msg)
        };
        let chips = widget::row::with_capacity(4)
            .push(chip(
                format!("{} ▸", r.frame.label()),
                Message::CycleFrame(index),
            ))
            .push(chip(r.size.label().to_owned(), Message::CycleSize(index)))
            .push(chip(fl!("open-image"), Message::OpenImage(r.path.clone())))
            .spacing(2)
            .align_y(Alignment::Center);

        widget::column::with_capacity(2)
            .push(pic)
            .push(chips)
            .spacing(3)
            .width(Length::Fixed(width))
            .into()
    }

    fn persist_layout(&mut self) {
        if let Some(handler) = &self.config_handler {
            if let Err(why) = self.config.set_hide_nav(handler, !self.show_nav) {
                tracing::warn!(%why, "could not persist nav visibility");
            }
            if let Err(why) = self.config.set_hide_list(handler, !self.show_list) {
                tracing::warn!(%why, "could not persist list visibility");
            }
        }
    }

    // ----- state -----

    /// Reload the tag tree and view counts from the store.
    fn refresh_tags(&mut self) {
        let Some(store) = &self.store else { return };
        let tags = store.tags().unwrap_or_default();
        self.tags = tag_tree(&tags);
        self.view_counts = [
            store.list(&View::All).map_or(0, |v| v.len()),
            store.list(&View::Untagged).map_or(0, |v| v.len()),
            store.list(&View::Trash).map_or(0, |v| v.len()),
        ];
        // The tag we were looking at may be gone.
        if let View::Tag(t) = &self.view
            && !self.tags.iter().any(|(n, _)| n == t)
        {
            self.view = View::All;
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
        self.refresh_tags();
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
                Ok(true) => {
                    self.refresh_tags();
                    self.refresh_list();
                }
                Ok(false) => {}
                Err(err) => tracing::error!(%err, "dropping empty note"),
            }
        }
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
        self.refresh_tags();
        self.refresh_list();
    }

    /// Apply one `JJB_SCRIPT` step by routing it through the normal messages.
    fn run_step(&mut self, step: Step) -> Task<cosmic::Action<Message>> {
        match step {
            Step::New => self.update(Message::NewNote),
            Step::Type(text) => {
                // Type one line per tick so the editor sees the same incremental
                // edits a keyboard would produce.
                let (now, rest) = match text.find('\n') {
                    Some(i) => (text[..=i].to_owned(), text[i + 1..].to_owned()),
                    None => (text, String::new()),
                };
                if !rest.is_empty()
                    && let Some(runner) = self.script.as_mut()
                {
                    runner.push_front(Step::Type(rest));
                }
                let text = now;
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
            Step::Folder(name) => {
                self.new_folder = name;
                self.update(Message::CreateFolder)
            }
            Step::Format(key) => match Format::ALL.into_iter().find(|f| f.key() == key) {
                Some(format) => self.update(Message::Format(format)),
                None => Task::none(),
            },
            Step::SelectAll => self.update(Message::Editor(text_editor::Action::SelectAll)),
            Step::Dock => self.update(Message::ToggleDock),
            Step::Themes => self.update(Message::ToggleContextPage(ContextPage::Themes)),
            Step::Solo => self.update(Message::ToggleSolo),
            Step::Image(path) => self.update(Message::ImagesDropped(vec![PathBuf::from(path)])),
            Step::Pick => self.update(Message::PickImage),
            Step::CycleFrame(i) => self.update(Message::CycleFrame(i)),
            Step::CycleSize(i) => self.update(Message::CycleSize(i)),
            Step::Placement(key) => self.update(Message::SetPlacement(Placement::from_key(&key))),
            Step::Theme(key) => self.update(Message::SetTheme(retro::Theme::from_key(&key))),
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

fn note_row<'a>(p: &Palette, note: &'a NoteSummary, selected: bool) -> Element<'a, Message> {
    let fg = if selected { p.selfg } else { p.fg };
    let mut title_row = widget::row::with_capacity(3)
        .spacing(8)
        .align_y(Alignment::Center);
    if note.pinned {
        title_row = title_row.push(retro::accent(p, "▲"));
    }
    title_row = title_row
        .push(
            retro::text(p, note.title.clone())
                .font(cosmic::font::Font {
                    weight: cosmic::iced::font::Weight::Bold,
                    ..retro::mono()
                })
                .class(cosmic::theme::Text::Color(fg))
                .width(Length::Fill),
        )
        .push(retro::dim(p, format_date(note.modified)).size(11));

    let mut column = widget::column::with_capacity(2)
        .push(title_row)
        .spacing(3)
        .width(Length::Fill);
    if !note.preview.is_empty() {
        let preview: String = note.preview.chars().take(90).collect();
        column = column.push(retro::dim(p, preview));
    }
    column.into()
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

fn format_time(when: DateTime<Utc>) -> String {
    when.with_timezone(&Local).format("%H:%M").to_string()
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
    binds.insert(
        menu::KeyBind {
            modifiers: vec![Modifier::Ctrl],
            key: keyboard::Key::Character("1".into()),
        },
        MenuAction::ToggleNav,
    );
    binds.insert(
        menu::KeyBind {
            modifiers: vec![Modifier::Ctrl],
            key: keyboard::Key::Character("2".into()),
        },
        MenuAction::ToggleList,
    );
    binds.insert(
        menu::KeyBind {
            modifiers: vec![Modifier::Ctrl],
            key: keyboard::Key::Character("0".into()),
        },
        MenuAction::Solo,
    );
    binds
}

/// Where images sit relative to the text.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Placement {
    #[default]
    Rail,
    Top,
    Bottom,
}

impl Placement {
    pub fn key(self) -> &'static str {
        match self {
            Placement::Rail => "rail",
            Placement::Top => "top",
            Placement::Bottom => "bottom",
        }
    }
    pub fn from_key_opt(key: &str) -> Option<Placement> {
        match key {
            "rail" => Some(Placement::Rail),
            "top" => Some(Placement::Top),
            "bottom" => Some(Placement::Bottom),
            _ => None,
        }
    }
    pub fn from_key(key: &str) -> Placement {
        Placement::from_key_opt(key).unwrap_or_default()
    }
}

/// A processed image ready for display (or on its way).
#[derive(Debug, Clone)]
pub enum ImageState {
    Loading,
    Ready(widget::image::Handle, u32, u32),
    Ascii(String),
    Failed(String),
}

/// Markdown formatting actions offered by the dock.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Format {
    Bold,
    Italic,
    Code,
    H1,
    H2,
    Bullet,
    Todo,
    Link,
    Tag,
    Rule,
}

impl Format {
    pub const ALL: [Format; 10] = [
        Format::Bold,
        Format::Italic,
        Format::Code,
        Format::H1,
        Format::H2,
        Format::Bullet,
        Format::Todo,
        Format::Link,
        Format::Tag,
        Format::Rule,
    ];

    fn key(self) -> &'static str {
        match self {
            Format::Bold => "bold",
            Format::Italic => "italic",
            Format::Code => "code",
            Format::H1 => "h1",
            Format::H2 => "h2",
            Format::Bullet => "bullet",
            Format::Todo => "todo",
            Format::Link => "link",
            Format::Tag => "tag",
            Format::Rule => "rule",
        }
    }

    fn glyph(self) -> &'static str {
        match self {
            Format::Bold => "B",
            Format::Italic => "I",
            Format::Code => "`",
            Format::H1 => "H1",
            Format::H2 => "H2",
            Format::Bullet => "•",
            Format::Todo => "☐",
            Format::Link => "[[ ]]",
            Format::Tag => "#",
            Format::Rule => "—",
        }
    }

    fn label(self) -> String {
        match self {
            Format::Bold => fl!("dock-bold"),
            Format::Italic => fl!("dock-italic"),
            Format::Code => fl!("dock-code"),
            Format::H1 => fl!("dock-h1"),
            Format::H2 => fl!("dock-h2"),
            Format::Bullet => fl!("dock-bullet"),
            Format::Todo => fl!("dock-todo"),
            Format::Link => fl!("dock-link"),
            Format::Tag => fl!("dock-tag"),
            Format::Rule => fl!("dock-rule"),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
    Themes,
    Picker,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
    NewNote,
    TrashNote,
    FocusSearch,
    Themes,
    ToggleMarkers,
    ToggleNav,
    ToggleList,
    Solo,
    Placement(Placement),
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
            MenuAction::NewNote => Message::NewNote,
            MenuAction::TrashNote => Message::TrashCurrent,
            MenuAction::FocusSearch => Message::FocusSearch,
            MenuAction::Themes => Message::ToggleContextPage(ContextPage::Themes),
            MenuAction::ToggleMarkers => Message::ToggleMarkers,
            MenuAction::ToggleNav => Message::ToggleNav,
            MenuAction::ToggleList => Message::ToggleList,
            MenuAction::Solo => Message::ToggleSolo,
            MenuAction::Placement(p) => Message::SetPlacement(*p),
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
