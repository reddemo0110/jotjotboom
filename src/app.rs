// SPDX-License-Identifier: GPL-3.0-only

use crate::blocks::{Block, Blocks};
use crate::config::Config;
use crate::debug_script::{self, Step};
use crate::fl;
use crate::images::{self, Align, FrameStyle, ImageRef, PickerEntry, Processed, UriList};
use crate::markdown;
use crate::note::{self, Note, NoteSummary};
use crate::retro::{self, Palette};
use crate::store::{Store, View};
use chrono::{DateTime, Datelike, Local, Utc};
use cosmic::Application as _;
use cosmic::app::context_drawer;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::keyboard::{self, key::Physical};
use cosmic::iced::{Alignment, Event, Length, Point, Subscription, event, mouse, window};
use cosmic::prelude::*;
use cosmic::widget::menu::action::MenuAction as _;
use cosmic::widget::{self, about::About, icon, menu, nav_bar, text_editor};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const APP_ICON: &[u8] = include_bytes!("../resources/icons/hicolor/scalable/apps/icon.svg");
/// How long after the last keystroke we write the note to disk.
const AUTOSAVE_DELAY: Duration = Duration::from_millis(600);
/// Undo steps kept per open note.
const UNDO_DEPTH: usize = 200;
/// Typing separated by less than this is one undo step.
const UNDO_GROUP_IDLE: Duration = Duration::from_millis(700);
const NAV_WIDTH: f32 = 230.0;
const NOTE_LIST_WIDTH: f32 = 320.0;
/// Pointer travel (px) that turns a press on a picture into a drag.
const DRAG_THRESHOLD: f32 = 6.0;

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
    /// Editor typeface, per-pane text sizes (px) and dock scale — all persisted.
    editor_font: retro::EditorFont,
    /// Sidebar + list face, and the pane-title face (a designer pairing sets all three).
    ui_font: retro::EditorFont,
    title_font: retro::EditorFont,
    font_size: u16,
    sidebar_size: u16,
    list_size: u16,
    dock_size: retro::DockSize,
    /// What a finished task shows inside its brackets.
    task_marker: String,
    /// Maximum width of the note text column.
    measure: retro::Measure,
    /// Which sections of the Appearance drawer are unfolded.
    appearance_open: [bool; 4],
    /// Processed images keyed by path|mtime|style|theme.
    image_cache: HashMap<String, ImageState>,
    /// Block index of the image whose ⋯ menu is open.
    image_menu: Option<usize>,
    /// Drag-resize in progress.
    resizing: Option<Resize>,
    /// The Ctrl+Shift+H shortcuts overlay.
    show_shortcuts: bool,
    /// A picture being dragged to another line (press, then move).
    dragging: Option<ImageDrag>,
    /// Last known cursor position (window coords), for drag deltas.
    mouse_x: f32,
    mouse_y: f32,
    /// Width being dragged, shown live before it is written to the note.
    live_width: Option<(usize, u32)>,
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
    /// Tags whose sub-tags are folded away.
    collapsed: HashSet<String>,
    /// Tag whose right-click menu is open.
    tag_menu: Option<String>,
    /// Tag being renamed and the draft name.
    tag_rename: Option<(String, String)>,
    rename_id: widget::Id,
    query: String,
    search_id: widget::Id,
    notes_scroll_id: widget::Id,
    /// The dock's `+` is expanded, showing new-note / new-folder.
    dock_open: bool,
    new_folder: String,
    folder_id: widget::Id,
    notes: Vec<NoteSummary>,
    current: Option<Note>,
    /// The open note's body as text/image blocks.
    blocks: Blocks,
    undo: Vec<Snapshot>,
    redo: Vec<Snapshot>,
    /// Set when the last autosave failed; cleared by the next success.
    save_error: Option<String>,
    /// Show "saved hh:mm" next to the tick until this instant.
    saved_info_until: Option<Instant>,
    last_undo_kind: EditKind,
    last_undo_at: Instant,
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
    /// ↑ / ↓ outside the editor: open the note above / below, like mail.
    NavigateNotes(i32),
    SetTheme(retro::Theme),
    SetFont(retro::EditorFont),
    /// Apply a designer pairing (titles, chrome, note) by key.
    SetPairing(String),
    /// Ctrl+click on a `[[link]]` or `#tag` in the rich editor.
    FollowLink(crate::editor::widget::Link),
    RestoreFonts,
    /// Grow (+) or shrink (−) one pane's text, in px.
    SizeStep(Pane, i16),
    ToggleSection(Section),
    SetDockSize(retro::DockSize),
    SetTaskMarker(String),
    SetMeasure(retro::Measure),
    /// Fold or unfold the sub-tags of a tag in the sidebar.
    ToggleTagFold(String),
    /// Right-click menu on a tag (None closes it).
    TagMenu(Option<String>),
    TagRenameStart(String),
    TagRenameInput(String),
    TagRenameCommit,
    TagRenameCancel,
    ToggleMarkers,
    ToggleNav,
    ToggleList,
    ToggleSolo,
    ImagesDropped(Vec<PathBuf>),
    PickImage,
    PickerNavigate(PathBuf),
    PickerChoose(PathBuf),
    DragEnter,
    DragLeave,
    Dropped(Option<UriList>),
    ImageLoaded(String, Result<Processed, String>),
    ImageMenu(Option<usize>),
    SetFrame(usize, FrameStyle),
    SetAlign(usize, Align),
    SetWidth(usize, Option<u32>),
    SetCaption(usize, String),
    RemoveImage(usize),
    ResizeStart(usize),
    /// Left button went down on a picture: a click (menu) or the start of a drag.
    ImagePress(usize),
    /// While dragging, the pointer is over the slot before this body line.
    DragOver(usize),
    MouseMoved(Point),
    MouseReleased,
    OpenImage(String),
    FontLoaded,
    /// The window exists (so the compositor can register fonts): load the bundled faces.
    LoadFonts,

    Editor(usize, text_editor::Action),
    Undo,
    Redo,
    ToggleShortcuts,
    ToggleSavedInfo,
    SavedInfoTick,
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
        let editor_font = retro::EditorFont::from_key(&config.editor_font);
        let ui_font = retro::EditorFont::from_key(&config.ui_font);
        let title_font = title_font_from_config(&config.title_font);
        let font_size = size_from_config(
            config.editor_font_size,
            retro::FONT_SIZE_DEFAULT,
            retro::FONT_SIZE_MIN,
            retro::FONT_SIZE_MAX,
        );
        let sidebar_size = pane_size_from_config(config.sidebar_font_size);
        let list_size = pane_size_from_config(config.list_font_size);
        let dock_size = retro::DockSize::from_key(&config.dock_size);
        let task_marker = task_marker_from_config(&config.task_marker);
        let measure = retro::Measure::from_key(&config.text_width);
        let collapsed: HashSet<String> = config.collapsed_tags.iter().cloned().collect();
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
            editor_font,
            ui_font,
            title_font,
            font_size,
            sidebar_size,
            list_size,
            dock_size,
            task_marker,
            measure,
            appearance_open: [true, false, false, false],
            image_cache: HashMap::new(),
            image_menu: None,
            resizing: None,
            dragging: None,
            show_shortcuts: false,
            mouse_x: 0.0,
            mouse_y: 0.0,
            live_width: None,
            picker_dir: dirs::picture_dir()
                .or_else(dirs::home_dir)
                .unwrap_or_else(|| PathBuf::from("/")),
            picker_entries: Vec::new(),
            drop_hover: false,
            store,
            store_error,
            view: View::All,
            view_counts: [0; 3],
            collapsed,
            tag_menu: None,
            tag_rename: None,
            rename_id: widget::Id::unique(),
            tags: Vec::new(),
            query: String::new(),
            search_id: widget::Id::unique(),
            notes_scroll_id: widget::Id::unique(),
            dock_open: false,
            new_folder: String::new(),
            folder_id: widget::Id::unique(),
            notes: Vec::new(),
            current: None,
            blocks: Blocks::default(),
            undo: Vec::new(),
            redo: Vec::new(),
            save_error: None,
            saved_info_until: None,
            last_undo_kind: EditKind::Other,
            last_undo_at: Instant::now(),
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

        // The Appearance drawer sits beside the note, not over it.
        app.core.window.context_is_overlay = false;
        let title = app.update_title();
        // Fonts are (re)loaded on `window::Event::Opened` too: a LoadFont
        // action issued before the compositor exists is silently dropped.
        (app, Task::batch([load_fonts(), title]))
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
            menu::Item::Button(fl!("theme-colours"), None, MenuAction::Themes),
            menu::Item::CheckBox(
                fl!("show-markers"),
                None,
                self.show_markers,
                MenuAction::ToggleMarkers,
            ),
            menu::Item::Divider,
            menu::Item::Button(fl!("shortcuts"), None, MenuAction::Shortcuts),
            menu::Item::Button(fl!("about"), None, MenuAction::About),
        ];

        let menu_bar = menu::bar(vec![
            menu::Tree::with_children(
                menu::root(fl!("file")).apply(Element::from),
                menu::items(
                    &self.key_binds,
                    vec![
                        menu::Item::Button(fl!("new-note"), None, MenuAction::NewNote),
                        menu::Item::Button(fl!("new-folder"), None, MenuAction::NewFolder),
                        menu::Item::Button(fl!("dock-image"), None, MenuAction::AddImage),
                        menu::Item::Divider,
                        menu::Item::Button(fl!("search-notes"), None, MenuAction::FocusSearch),
                        menu::Item::Button(fl!("pin-note"), None, MenuAction::Pin),
                        menu::Item::Divider,
                        menu::Item::Button(fl!("trash-note"), None, MenuAction::TrashNote),
                    ],
                ),
            ),
            menu::Tree::with_children(
                menu::root(fl!("edit")).apply(Element::from),
                menu::items(
                    &self.key_binds,
                    vec![
                        menu::Item::Button(fl!("undo"), None, MenuAction::Undo),
                        menu::Item::Button(fl!("redo"), None, MenuAction::Redo),
                    ],
                ),
            ),
            menu::Tree::with_children(
                menu::root(fl!("format")).apply(Element::from),
                menu::items(
                    &self.key_binds,
                    Format::ALL
                        .into_iter()
                        .map(|f| menu::Item::Button(f.label(), None, MenuAction::Format(f)))
                        .collect(),
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
                self.with_shortcut(fl!("show-nav"), MenuAction::ToggleNav),
                self.show_nav,
                Message::ToggleNav,
            ),
            toggle(
                "view-list-symbolic",
                self.with_shortcut(fl!("show-list"), MenuAction::ToggleList),
                self.show_list,
                Message::ToggleList,
            ),
        ]
    }

    fn header_end(&self) -> Vec<Element<'_, Self::Message>> {
        let mut items: Vec<Element<'_, Message>> = Vec::new();
        let in_trash = matches!(self.view, View::Trash);
        let sc = |label: String, action: MenuAction| self.with_shortcut(label, action);

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
                    "pin-symbolic",
                    sc(pin_label, MenuAction::Pin),
                    Message::TogglePin,
                ));
                items.push(header_button(
                    "user-trash-symbolic",
                    sc(fl!("trash-note"), MenuAction::TrashNote),
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
                sc(fl!("new-note"), MenuAction::NewNote),
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
            .title(fl!("appearance")),
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

        // Flat layout: panes butt against each other with hairlines between.
        let nav = widget::column::with_capacity(3)
            .push(
                widget::container(self.views_frame(&p))
                    // Three rows plus the header; grows with the sidebar text.
                    .height(Length::Fixed(
                        (f32::from(self.sidebar_size) * 1.3 + 8.0) * 3.0 + 66.0,
                    ))
                    .width(Length::Fill),
            )
            .push(retro::hrule(&p))
            .push(self.tags_frame(&p))
            .width(Length::Fixed(NAV_WIDTH))
            .height(Length::Fill);

        let list = widget::container(self.notes_frame(&p))
            .width(Length::Fixed(NOTE_LIST_WIDTH))
            .height(Length::Fill);

        let mut editor_col = widget::column::with_capacity(3)
            .push(self.editor_frame(&p))
            .width(Length::Fill)
            .height(Length::Fill);
        if !self.backlinks.is_empty() {
            editor_col = editor_col.push(retro::hrule(&p)).push(
                widget::container(self.backlinks_frame(&p))
                    .height(Length::Fixed(58.0))
                    .width(Length::Fill),
            );
        }

        let mut panes = widget::row::with_capacity(5)
            .width(Length::Fill)
            .height(Length::Fill);
        if self.show_nav {
            panes = panes.push(nav).push(retro::vrule(&p));
        }
        if self.show_list {
            panes = panes.push(list).push(retro::vrule(&p));
        }
        panes = panes.push(editor_col);

        let content = widget::container(panes)
            .width(Length::Fill)
            .height(Length::Fill);
        if self.show_shortcuts {
            cosmic::iced::widget::stack([content.into(), self.shortcuts_overlay(&p)])
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        } else {
            content.into()
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let mut subscriptions = vec![
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| Message::UpdateConfig(update.config)),
            event::listen_with(|event, status, _window| match event {
                // Arrows nobody consumed (the editor is not focused) walk the list.
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(keyboard::key::Named::ArrowDown),
                    modifiers,
                    ..
                }) if status == event::Status::Ignored && modifiers.is_empty() => {
                    Some(Message::NavigateNotes(1))
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(keyboard::key::Named::ArrowUp),
                    modifiers,
                    ..
                }) if status == event::Status::Ignored && modifiers.is_empty() => {
                    Some(Message::NavigateNotes(-1))
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key,
                    modifiers,
                    physical_key,
                    ..
                }) => Some(Message::Key(modifiers, key, physical_key)),
                Event::Window(window::Event::FileDropped(paths)) => {
                    Some(Message::ImagesDropped(paths))
                }
                Event::Window(window::Event::Opened { .. }) => Some(Message::LoadFonts),
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    Some(Message::MouseMoved(position))
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    Some(Message::MouseReleased)
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
        if self.saved_info_until.is_some() {
            subscriptions.push(
                cosmic::iced::time::every(Duration::from_millis(500))
                    .map(|_| Message::SavedInfoTick),
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
        if self.show_shortcuts {
            self.show_shortcuts = false;
        } else if self.dock_open {
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

            Message::ImageMenu(block) => {
                self.image_menu = if self.image_menu == block {
                    None
                } else {
                    block
                };
            }

            Message::SetFrame(block, frame) => {
                self.image_menu = None;
                self.edit_ref(block, |r| r.frame = frame);
            }

            Message::SetAlign(block, align) => {
                self.image_menu = None;
                self.edit_ref(block, |r| r.align = align);
            }

            Message::SetWidth(block, width) => {
                self.image_menu = None;
                self.edit_ref(block, |r| r.width = width);
            }

            Message::SetCaption(block, caption) => {
                let caption = caption.replace(['[', ']', '\n'], "");
                self.edit_ref_kind(block, EditKind::Typing, |r| r.alt = caption);
            }

            Message::SetFont(font) => {
                self.editor_font = font;
                if let Some(handler) = &self.config_handler
                    && let Err(why) = self.config.set_editor_font(handler, font.key().to_owned())
                {
                    tracing::warn!(%why, "saving editor font");
                }
            }

            Message::FollowLink(link) => match link {
                crate::editor::widget::Link::Note(title) => {
                    let found = self
                        .store
                        .as_ref()
                        .and_then(|s| s.find_by_title(&title).ok().flatten());
                    if let Some(id) = found {
                        return self.update(Message::Select(id));
                    }
                    tracing::info!(title, "no note with that title");
                }
                crate::editor::widget::Link::Tag(tag) => {
                    if let Some(tag) = note::normalize_tag(tag.trim_start_matches('#')) {
                        return self.update(Message::SetView(View::Tag(tag)));
                    }
                }
            },

            Message::SetPairing(key) => {
                if let Some(pair) = retro::Pairing::from_key(&key) {
                    self.apply_pairing(pair);
                }
            }

            Message::RestoreFonts => {
                self.apply_pairing(retro::Pairing::default_pairing());
            }

            Message::SizeStep(pane, step) => {
                let (current, min, max) = match pane {
                    Pane::Editor => (self.font_size, retro::FONT_SIZE_MIN, retro::FONT_SIZE_MAX),
                    Pane::Sidebar => (
                        self.sidebar_size,
                        retro::PANE_SIZE_MIN,
                        retro::PANE_SIZE_MAX,
                    ),
                    Pane::List => (self.list_size, retro::PANE_SIZE_MIN, retro::PANE_SIZE_MAX),
                };
                let next = (i32::from(current) + i32::from(step))
                    .clamp(i32::from(min), i32::from(max)) as u16;
                let saved = match (pane, &self.config_handler) {
                    (Pane::Editor, Some(h)) => {
                        self.font_size = next;
                        self.config.set_editor_font_size(h, next).map(|_| ())
                    }
                    (Pane::Sidebar, Some(h)) => {
                        self.sidebar_size = next;
                        self.config.set_sidebar_font_size(h, next).map(|_| ())
                    }
                    (Pane::List, Some(h)) => {
                        self.list_size = next;
                        self.config.set_list_font_size(h, next).map(|_| ())
                    }
                    (Pane::Editor, None) => {
                        self.font_size = next;
                        Ok(())
                    }
                    (Pane::Sidebar, None) => {
                        self.sidebar_size = next;
                        Ok(())
                    }
                    (Pane::List, None) => {
                        self.list_size = next;
                        Ok(())
                    }
                };
                if let Err(why) = saved {
                    tracing::warn!(%why, ?pane, "saving text size");
                }
            }

            Message::SetMeasure(measure) => {
                self.measure = measure;
                if let Some(handler) = &self.config_handler
                    && let Err(why) = self
                        .config
                        .set_text_width(handler, measure.key().to_owned())
                {
                    tracing::warn!(%why, "saving text width");
                }
            }

            Message::SetTaskMarker(marker) => {
                self.task_marker = marker.clone();
                if let Some(handler) = &self.config_handler
                    && let Err(why) = self.config.set_task_marker(handler, marker)
                {
                    tracing::warn!(%why, "saving task marker");
                }
            }

            Message::ToggleSection(section) => {
                let i = section as usize;
                self.appearance_open[i] = !self.appearance_open[i];
            }

            Message::SetDockSize(size) => {
                self.dock_size = size;
                if let Some(handler) = &self.config_handler
                    && let Err(why) = self.config.set_dock_size(handler, size.key().to_owned())
                {
                    tracing::warn!(%why, "saving dock size");
                }
            }

            Message::TagMenu(tag) => {
                self.tag_menu = tag;
                self.tag_rename = None;
            }

            Message::TagRenameStart(tag) => {
                self.tag_menu = None;
                self.tag_rename = Some((tag.clone(), tag));
                return widget::text_input::focus(self.rename_id.clone());
            }

            Message::TagRenameInput(draft) => {
                if let Some(r) = &mut self.tag_rename {
                    r.1 = draft;
                }
            }

            Message::TagRenameCancel => {
                self.tag_rename = None;
                self.tag_menu = None;
            }

            Message::TagRenameCommit => {
                let Some((old, draft)) = self.tag_rename.take() else {
                    return Task::none();
                };
                let Some(new) = note::normalize_tag(&draft) else {
                    return Task::none();
                };
                if new == old {
                    return Task::none();
                }
                // Files are rewritten on disk: the open note must be saved first.
                self.flush();
                let under =
                    |t: &str| t == old || (t.starts_with(&old) && t[old.len()..].starts_with('/'));
                let remap = |t: &str| format!("{new}{}", &t[old.len()..]);
                let current_affected = self
                    .current
                    .as_ref()
                    .is_some_and(|n| note::extract_tags(&n.body).iter().any(|t| under(t)));
                let Some(store) = self.store.as_mut() else {
                    return Task::none();
                };
                match store.rename_tag(&old, &new) {
                    Ok(n) => tracing::info!(old, new, notes = n, "renamed tag"),
                    Err(err) => {
                        tracing::error!(%err, old, new, "renaming tag");
                        return Task::none();
                    }
                }
                if let View::Tag(t) = &self.view
                    && under(t)
                {
                    self.view = View::Tag(remap(t));
                }
                self.collapsed = self
                    .collapsed
                    .iter()
                    .map(|t| if under(t) { remap(t) } else { t.clone() })
                    .collect();
                if current_affected && let Some(id) = self.current.as_ref().map(|n| n.id.clone()) {
                    self.open_note(&id);
                }
                self.refresh_tags();
                self.refresh_list();
            }

            Message::ToggleTagFold(tag) => {
                if !self.collapsed.remove(&tag) {
                    self.collapsed.insert(tag);
                }
                if let Some(handler) = &self.config_handler {
                    let mut list: Vec<String> = self.collapsed.iter().cloned().collect();
                    list.sort();
                    if let Err(why) = self.config.set_collapsed_tags(handler, list) {
                        tracing::warn!(%why, "saving folded tags");
                    }
                }
            }

            Message::ToggleShortcuts => {
                self.show_shortcuts = !self.show_shortcuts;
            }

            Message::ToggleSavedInfo => {
                self.saved_info_until = match self.saved_info_until {
                    Some(_) => None,
                    None => Some(Instant::now() + Duration::from_secs(4)),
                };
            }

            Message::SavedInfoTick => {
                if self.saved_info_until.is_some_and(|t| Instant::now() >= t) {
                    self.saved_info_until = None;
                }
            }

            Message::Undo => {
                if let Some(snap) = self.undo.pop() {
                    let now = self.snapshot();
                    self.redo.push(now);
                    self.restore(snap);
                    return self.focus_editor();
                }
            }

            Message::Redo => {
                if let Some(snap) = self.redo.pop() {
                    let now = self.snapshot();
                    self.undo.push(now);
                    self.restore(snap);
                    return self.focus_editor();
                }
            }

            Message::RemoveImage(block) => {
                self.image_menu = None;
                self.record(EditKind::Other);
                self.blocks.remove_image(block);
                self.dirty = true;
                self.last_edit = Instant::now();
                return self.focus_editor();
            }

            Message::ResizeStart(block) => {
                self.image_menu = None;
                let start_w = self
                    .blocks
                    .images()
                    .iter()
                    .find(|(b, _)| *b == block)
                    .and_then(|(_, r)| r.width)
                    .unwrap_or(420);
                self.resizing = Some(Resize {
                    block,
                    start_x: self.mouse_x,
                    start_w,
                });
            }

            Message::ImagePress(block) => {
                let editable = self.current.as_ref().is_some_and(|n| !n.trashed);
                if self.image_menu.is_some() || !editable {
                    return self.update(Message::ImageMenu(Some(block)));
                }
                self.dragging = Some(ImageDrag {
                    block,
                    start: Point::new(self.mouse_x, self.mouse_y),
                    active: false,
                    target: None,
                });
            }

            Message::DragOver(line) => {
                if let Some(d) = &mut self.dragging
                    && d.active
                {
                    d.target = Some(line);
                }
            }

            Message::MouseMoved(position) => {
                self.mouse_x = position.x;
                self.mouse_y = position.y;
                if let Some(d) = &mut self.dragging
                    && !d.active
                    && position.distance(d.start) > DRAG_THRESHOLD
                {
                    d.active = true;
                }
                if let Some(rz) = &self.resizing {
                    let w = (rz.start_w as f32 + (position.x - rz.start_x))
                        .clamp(images::MIN_WIDTH as f32, images::MAX_WIDTH as f32)
                        as u32;
                    self.live_width = Some((rz.block, w));
                }
            }

            Message::MouseReleased => {
                if let Some(rz) = self.resizing.take()
                    && let Some((_, w)) = self.live_width.take()
                {
                    self.edit_ref(rz.block, |r| r.width = Some(w));
                }
                if let Some(d) = self.dragging.take() {
                    if !d.active {
                        return self.update(Message::ImageMenu(Some(d.block)));
                    }
                    if let Some(target) = d.target {
                        let snap = self.snapshot();
                        if self.blocks.move_image(d.block, target).is_some() {
                            self.push_undo(snap);
                            self.dirty = true;
                            self.last_edit = Instant::now();
                            return self.focus_editor();
                        }
                    }
                }
            }

            Message::OpenImage(rel) => {
                if let Some(store) = &self.store {
                    let path = images::resolve(store.notes_dir(), &rel);
                    if let Err(err) = open::that_detached(&path) {
                        tracing::warn!(%err, "opening image");
                    }
                }
            }

            Message::FontLoaded => {}

            Message::LoadFonts => return load_fonts(),

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
                    return self.focus_editor();
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

            Message::Editor(block, action) => {
                use text_editor::{Action, Edit, Motion};
                tracing::trace!(
                    block,
                    focused = self.blocks.focused,
                    ?action,
                    "editor action"
                );
                // A widget losing focus emits ClearSelection; only real
                // interaction with a block moves the caret there.
                let claims_focus =
                    !matches!(action, Action::ClearSelection | Action::Scroll { .. });
                if claims_focus {
                    self.blocks.focused = block;
                }

                self.image_menu = None;
                let editable = self.current.as_ref().is_some_and(|n| !n.trashed);
                // Edges of a block: step over images, or delete the one above.
                if let Some(content) = self.blocks.text(block) {
                    let cursor = content.cursor();
                    let at_start = cursor.position.line == 0
                        && cursor.position.column == 0
                        && cursor.selection.is_none();
                    let on_last_line = cursor.position.line + 1 >= content.line_count();
                    match &action {
                        Action::Edit(Edit::Backspace) if at_start && editable => {
                            if let Some(prev) = block.checked_sub(1)
                                && matches!(
                                    self.blocks.items.get(prev),
                                    Some(Block::Image(_) | Block::Rule(_))
                                )
                            {
                                self.record(EditKind::Other);
                                self.blocks.remove_block(prev);
                                self.dirty = true;
                                self.last_edit = Instant::now();
                                return self.focus_editor();
                            }
                        }
                        Action::Move(Motion::Up) if cursor.position.line == 0 => {
                            if let Some(prev) = self.blocks.text_before(block) {
                                self.blocks.focused = prev;
                                if let Some(c) = self.blocks.text_mut(prev) {
                                    c.perform(Action::Move(Motion::DocumentEnd));
                                }
                                return self.focus_editor();
                            }
                        }
                        Action::Move(Motion::Down) if on_last_line => {
                            if let Some(next) = self.blocks.text_after(block) {
                                self.blocks.focused = next;
                                if let Some(c) = self.blocks.text_mut(next) {
                                    c.perform(Action::Move(Motion::DocumentStart));
                                }
                                return self.focus_editor();
                            }
                        }
                        _ => {}
                    }
                }
                let is_edit = action.is_edit();
                let is_click = matches!(action, Action::Click(_));
                let finishes_line = matches!(action, Action::Edit(Edit::Enter | Edit::Paste(_)));
                let closes_bracket = matches!(action, Action::Edit(Edit::Insert(']')));
                if is_edit && editable {
                    let kind = match &action {
                        Action::Edit(Edit::Insert(_) | Edit::Enter | Edit::Indent) => {
                            EditKind::Typing
                        }
                        Action::Edit(Edit::Backspace | Edit::Delete | Edit::Unindent) => {
                            EditKind::Deleting
                        }
                        _ => EditKind::Other,
                    };
                    self.record(kind);
                }
                if let Some(content) = self.blocks.text_mut(block) {
                    content.perform(action);
                }
                if is_edit && editable {
                    self.dirty = true;
                    self.last_edit = Instant::now();
                }
                // A finished `---` line (Enter / paste) becomes a rule block.
                if finishes_line && editable && self.blocks.needs_resplit() && self.blocks.resplit()
                {
                    return self.focus_editor();
                }
                // `[]` at the start of a line becomes a task box.
                if closes_bracket
                    && editable
                    && let Some(content) = self.blocks.text_mut(block)
                    && crate::blocks::expand_task_shorthand(content)
                {
                    // A fresh task box renders at once; no raw `- [ ] ` first.
                    content.render_now();
                }
                // Clicking a task box ticks / unticks it.
                if is_click && editable {
                    let before = self.snapshot();
                    let marker = self.task_marker.clone();
                    if let Some(content) = self.blocks.text_mut(block)
                        && crate::blocks::toggle_task_at_cursor(content, &marker)
                    {
                        content.render_now();
                        self.push_undo(before);
                        self.last_undo_kind = EditKind::Other;
                        self.dirty = true;
                        self.last_edit = Instant::now();
                    }
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

            Message::NavigateNotes(delta) => {
                let len = self.notes.len();
                if len == 0 {
                    return Task::none();
                }
                let at = self
                    .current
                    .as_ref()
                    .and_then(|c| self.notes.iter().position(|n| n.id == c.id));
                let next = match at {
                    Some(i) => (i as i64 + i64::from(delta)).clamp(0, len as i64 - 1) as usize,
                    None if delta > 0 => 0,
                    None => len - 1,
                };
                let id = self.notes[next].id.clone();
                // Keep the selection in view (rows vary in height; this is close enough).
                let y = if len > 1 {
                    next as f32 / (len - 1) as f32
                } else {
                    0.0
                };
                let scroll = cosmic::iced::widget::scrollable::snap_to(
                    self.notes_scroll_id.clone(),
                    cosmic::iced::widget::scrollable::RelativeOffset {
                        x: None,
                        y: Some(y),
                    },
                );
                return Task::batch([self.update(Message::Select(id)), scroll]);
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
                    return Task::batch([self.update_title(), self.focus_editor()]);
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
                    return self.focus_editor();
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
                if self.dragging.is_some()
                    && matches!(key, keyboard::Key::Named(keyboard::key::Named::Escape))
                {
                    self.dragging = None;
                    return Task::none();
                }
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
                self.editor_font = retro::EditorFont::from_key(&config.editor_font);
                self.ui_font = retro::EditorFont::from_key(&config.ui_font);
                self.title_font = title_font_from_config(&config.title_font);
                self.font_size = size_from_config(
                    config.editor_font_size,
                    retro::FONT_SIZE_DEFAULT,
                    retro::FONT_SIZE_MIN,
                    retro::FONT_SIZE_MAX,
                );
                self.sidebar_size = pane_size_from_config(config.sidebar_font_size);
                self.list_size = pane_size_from_config(config.list_font_size);
                self.dock_size = retro::DockSize::from_key(&config.dock_size);
                self.task_marker = task_marker_from_config(&config.task_marker);
                self.measure = retro::Measure::from_key(&config.text_width);
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

    /// The face pane titles are set in: VT323 as-is, anything else bold.
    fn title_font(&self) -> cosmic::font::Font {
        let f = self.title_font.font();
        if self.title_font.has_bold() {
            cosmic::font::Font {
                weight: cosmic::iced::font::Weight::Bold,
                ..f
            }
        } else {
            f
        }
    }

    fn ui_font(&self) -> cosmic::font::Font {
        self.ui_font.font()
    }

    fn apply_pairing(&mut self, pair: &retro::Pairing) {
        self.editor_font = pair.body;
        self.ui_font = pair.ui;
        self.title_font = pair.title;
        if let Some(handler) = &self.config_handler {
            for (why, _) in [
                self.config
                    .set_editor_font(handler, pair.body.key().to_owned())
                    .err(),
                self.config
                    .set_ui_font(handler, pair.ui.key().to_owned())
                    .err(),
                self.config
                    .set_title_font(handler, pair.title.key().to_owned())
                    .err(),
            ]
            .into_iter()
            .flatten()
            .map(|e| (e, ()))
            {
                tracing::warn!(%why, "saving font pairing");
            }
        }
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
        let sz = f32::from(self.sidebar_size);
        let mut col = widget::column::with_capacity(3).spacing(2);
        for (view, label, count) in rows {
            let selected = self.view == view;
            let marker = if selected { "▌" } else { " " };
            let ui = self.ui_font();
            let row = widget::row::with_capacity(3)
                .push(retro::accent(p, marker).size(sz))
                .push(retro::text(p, label).font(ui).size(sz).width(Length::Fill))
                .push(retro::dim(p, count.to_string()).font(ui).size(sz - 1.0))
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
        retro::pane(p, self.title_font(), fl!("frame-views"), None, col, p.bg)
    }

    fn tags_frame<'a>(&'a self, p: &Palette) -> Element<'a, Message> {
        let sz = f32::from(self.sidebar_size);
        let mut col = widget::column::with_capacity(self.tags.len()).spacing(1);
        for (name, count) in &self.tags {
            // Hidden while any ancestor is folded.
            let folded_away = name
                .match_indices('/')
                .any(|(i, _)| self.collapsed.contains(&name[..i]));
            if folded_away {
                continue;
            }
            let depth = name.matches('/').count();
            let leaf = name.rsplit('/').next().unwrap_or(name);
            let has_children = self.tags.iter().any(|(other, _)| {
                other.len() > name.len() + 1 && other.starts_with(&format!("{name}/"))
            });
            let folded = self.collapsed.contains(name);
            let selected = matches!(&self.view, View::Tag(t) if t == name);
            let count_text = if *count > 0 {
                count.to_string()
            } else {
                String::new()
            };
            let chevron: Element<'a, Message> = if has_children {
                widget::button::custom(
                    retro::dim(p, if folded { "▸" } else { "▾" })
                        .size(sz - 1.0)
                        .class(cosmic::theme::Text::Color(p.dim)),
                )
                .padding([0, 3])
                .class(retro::row_class(p, false))
                .on_press(Message::ToggleTagFold(name.clone()))
                .into()
            } else {
                widget::container(retro::dim(p, " ").size(sz - 1.0))
                    .padding([0, 3])
                    .into()
            };
            let row = widget::row::with_capacity(5)
                .push(widget::Space::new().width(Length::Fixed(14.0 * depth as f32)))
                .push(chevron)
                .push(retro::accent2(p, "#").size(sz))
                .push(
                    retro::text(p, leaf.to_owned())
                        .font(self.ui_font())
                        .size(sz)
                        .width(Length::Fill),
                )
                .push(
                    retro::dim(p, count_text)
                        .font(self.ui_font())
                        .size(sz - 1.0),
                )
                .spacing(4)
                .align_y(Alignment::Center);
            let button = widget::button::custom(row)
                .padding([3, 8, 3, 4])
                .width(Length::Fill)
                .class(retro::row_class(p, selected))
                .on_press(Message::SetView(View::Tag(name.clone())));
            // Right-click: a small menu (rename); the rename is an inline input.
            let item =
                widget::mouse_area(button).on_right_press(Message::TagMenu(Some(name.clone())));
            let popup: Option<Element<'a, Message>> =
                if let Some((t, draft)) = self.tag_rename.as_ref().filter(|(t, _)| t == name) {
                    let _ = t;
                    Some(
                        widget::container(
                            widget::text_input(fl!("tag-rename-placeholder"), draft)
                                .id(self.rename_id.clone())
                                .font(retro::mono())
                                .size(sz)
                                .padding([3, 8])
                                .width(Length::Fixed(200.0))
                                .leading_icon(
                                    widget::container(retro::accent2(p, "#"))
                                        .padding([0, 0, 0, 8])
                                        .into(),
                                )
                                .style(retro::search_class(p))
                                .on_input(Message::TagRenameInput)
                                .on_submit(|_| Message::TagRenameCommit),
                        )
                        .padding(4)
                        .class(retro::dock_class(p))
                        .into(),
                    )
                } else if self.tag_menu.as_deref() == Some(name.as_str()) {
                    Some(
                        widget::container(
                            widget::button::custom(retro::text(p, fl!("tag-rename")).size(sz))
                                .padding([3, 10])
                                .width(Length::Fixed(160.0))
                                .class(retro::row_class(p, false))
                                .on_press(Message::TagRenameStart(name.clone())),
                        )
                        .padding(4)
                        .class(retro::dock_class(p))
                        .into(),
                    )
                } else {
                    None
                };
            match popup {
                Some(popup) => {
                    col = col.push(
                        widget::popover(item)
                            .popup(popup)
                            .position(widget::popover::Position::Bottom)
                            .on_close(Message::TagRenameCancel),
                    );
                }
                None => col = col.push(item),
            }
        }
        let body: Element<'_, Message> = if self.tags.is_empty() {
            widget::container(retro::dim(p, fl!("no-tags-yet")))
                .padding([8, 8])
                .into()
        } else {
            widget::scrollable(col).height(Length::Fill).into()
        };
        retro::pane(p, self.title_font(), fl!("frame-tags"), None, body, p.bg)
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
            .font(self.ui_font())
            .size(f32::from(self.list_size))
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
                    widget::button::custom(note_row(
                        p,
                        note,
                        selected,
                        self.list_size,
                        self.ui_font(),
                    ))
                    .padding([7, 8])
                    .width(Length::Fill)
                    .class(retro::row_class(p, selected))
                    .on_press(Message::Select(note.id.clone())),
                );
            }
            widget::scrollable(col)
                .id(self.notes_scroll_id.clone())
                .height(Length::Fill)
                .into()
        };

        let content = widget::column::with_capacity(2)
            .push(search)
            .push(body)
            .spacing(8)
            .width(Length::Fill)
            .height(Length::Fill);
        retro::pane(
            p,
            self.title_font(),
            fl!("frame-notes"),
            Some(badge),
            content,
            p.panel,
        )
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
            return retro::pane(
                p,
                self.title_font(),
                fl!("app-title").to_lowercase(),
                None,
                content,
                p.panel,
            );
        };
        // Quiet by default: a tick. Click it for the last save time; a failed
        // save replaces it with a warning until the next one succeeds.
        let last_saved = format_time(note.modified);
        let badge: Element<'a, Message> = if self.drop_hover {
            retro::dim(p, fl!("badge-drop")).into()
        } else if note.trashed {
            retro::dim(p, fl!("badge-in-trash")).into()
        } else if self.save_error.is_some() {
            widget::tooltip(
                widget::button::custom(
                    retro::accent(p, fl!("badge-not-saved", time = last_saved))
                        .size(12)
                        .wrapping(cosmic::iced::widget::text::Wrapping::None),
                )
                .padding([0, 4])
                .class(retro::row_class(p, false))
                .on_press(Message::ToggleSavedInfo),
                retro::dim(p, self.save_error.clone().unwrap_or_default()),
                widget::tooltip::Position::Bottom,
            )
            .into()
        } else {
            let label = if self.saved_info_until.is_some() {
                fl!("badge-saved", time = last_saved)
            } else {
                "✓".to_owned()
            };
            widget::button::custom(
                retro::dim(p, label)
                    .size(12)
                    .wrapping(cosmic::iced::widget::text::Wrapping::None),
            )
            .padding([0, 4])
            .class(retro::row_class(p, false))
            .on_press(Message::ToggleSavedInfo)
            .into()
        };

        let text_area: Element<'a, Message> = self.blocks_view(p, note.trashed);
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

        let framed = retro::pane_el(
            p,
            self.title_font(),
            note.title.clone(),
            Some(badge),
            column,
            p.panel,
        );
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
        retro::pane(
            p,
            self.title_font(),
            fl!("linked-from").to_lowercase(),
            None,
            widget::scrollable(row).direction(
                cosmic::iced::widget::scrollable::Direction::Horizontal(
                    cosmic::iced::widget::scrollable::Scrollbar::default(),
                ),
            ),
            p.panel,
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
        let ds = self.dock_size;
        // A wrapping row, so the bigger sizes fold onto extra lines instead
        // of running off the pane.
        let mut items: Vec<Element<'a, Message>> = Vec::with_capacity(16);

        for format in Format::ALL {
            let button = widget::button::custom(retro::text(p, format.glyph()).size(ds.glyph()))
                .padding(ds.pad())
                .class(retro::row_class(p, false))
                .on_press_maybe(editable.then_some(Message::Format(format)));
            items.push(
                widget::tooltip(
                    button,
                    retro::dim(
                        p,
                        self.with_shortcut(format.label(), MenuAction::Format(format)),
                    ),
                    widget::tooltip::Position::Top,
                )
                .into(),
            );
        }

        items.push(divider().into());
        items.push(
            widget::tooltip(
                widget::button::custom(retro::accent(p, "+").size(ds.glyph() + 2.0))
                    .padding([ds.pad()[0].saturating_sub(2), ds.pad()[1]])
                    .class(retro::row_class(p, self.dock_open))
                    .on_press(Message::ToggleDock),
                retro::dim(p, self.with_shortcut(fl!("dock-plus"), MenuAction::NewNote)),
                widget::tooltip::Position::Top,
            )
            .into(),
        );

        items.push(
            widget::tooltip(
                widget::button::custom(retro::accent(p, "⧉").size(ds.glyph() + 1.0))
                    .padding([ds.pad()[0].saturating_sub(1), ds.pad()[1]])
                    .class(retro::row_class(p, false))
                    .on_press_maybe(editable.then_some(Message::PickImage)),
                retro::dim(
                    p,
                    self.with_shortcut(fl!("dock-image"), MenuAction::AddImage),
                ),
                widget::tooltip::Position::Top,
            )
            .into(),
        );

        items.push(divider().into());
        items.push(
            widget::tooltip(
                widget::button::custom(retro::accent2(p, "◐").size(ds.glyph() + 1.0))
                    .padding([ds.pad()[0].saturating_sub(1), ds.pad()[1]])
                    .class(retro::row_class(
                        p,
                        self.core.window.show_context && self.context_page == ContextPage::Themes,
                    ))
                    .on_press(Message::ToggleContextPage(ContextPage::Themes)),
                retro::dim(
                    p,
                    self.with_shortcut(fl!("theme-colours"), MenuAction::Themes),
                ),
                widget::tooltip::Position::Top,
            )
            .into(),
        );
        let row = widget::flex_row(items)
            .spacing(1)
            .align_items(Alignment::Center);

        let pill = |content: Element<'a, Message>| {
            widget::container(
                widget::container(content)
                    .padding(ds.pill())
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
    /// The Appearance drawer: Colour, Font and Size, each foldable.
    fn theme_picker(&self) -> Element<'_, Message> {
        let system = self.core.system_theme();
        let label_color: cosmic::iced::Color = system.cosmic().background(false).on.into();
        let header = |label: String, section: Section| {
            let open = self.appearance_open[section as usize];
            widget::button::custom(
                widget::row::with_capacity(2)
                    .push(widget::text(if open { "▾" } else { "▸" }).size(16))
                    .push(widget::text::heading(label))
                    .spacing(8)
                    .align_y(Alignment::Center)
                    .width(Length::Fill),
            )
            .padding([6, 4])
            .width(Length::Fill)
            .class(cosmic::theme::Button::Text)
            .on_press(Message::ToggleSection(section))
        };
        let mut col = widget::column::with_capacity(40)
            .spacing(6)
            .width(Length::Fill);

        // Colour.
        col = col.push(header(fl!("section-colour"), Section::Colour));
        if self.appearance_open[Section::Colour as usize] {
            col = col.push(widget::text::caption(fl!("theme-picker-hint")));
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
        }

        // Font.
        col = col.push(
            widget::container(header(fl!("section-font"), Section::Font)).padding([8, 0, 0, 0]),
        );
        if self.appearance_open[Section::Font as usize] {
            col = col.push(widget::text::caption(fl!("font-pairings-hint")));
            let current = retro::PAIRINGS.iter().find(|pr| {
                pr.title == self.title_font && pr.ui == self.ui_font && pr.body == self.editor_font
            });
            for pair in &retro::PAIRINGS {
                let selected = current == Some(pair);
                let title_font = if pair.title.has_bold() {
                    cosmic::font::Font {
                        weight: cosmic::iced::font::Weight::Bold,
                        ..pair.title.font()
                    }
                } else {
                    pair.title.font()
                };
                let sample = widget::column::with_capacity(4)
                    .push(widget::text(pair.name).font(title_font).size(20))
                    .push(
                        widget::text(fl!("pairing-sample-ui"))
                            .font(pair.ui.font())
                            .size(13),
                    )
                    .push(
                        widget::text(fl!("pairing-sample-body"))
                            .font(pair.body.font())
                            .size(15),
                    )
                    .push(widget::text::caption(pair.blurb))
                    .spacing(3);
                col = col.push(
                    widget::button::custom(widget::container(sample).width(Length::Fill))
                        .padding([8, 10])
                        .width(Length::Fill)
                        .class(if selected {
                            cosmic::theme::Button::Suggested
                        } else {
                            cosmic::theme::Button::Standard
                        })
                        .on_press(Message::SetPairing(pair.key.to_owned())),
                );
            }
            col = col.push(
                widget::button::custom(widget::text(fl!("font-restore")))
                    .padding([6, 12])
                    .class(cosmic::theme::Button::Standard)
                    .on_press(Message::RestoreFonts),
            );
            col = col.push(
                widget::container(widget::text::heading(fl!("font-editor-only")))
                    .padding([8, 0, 0, 0]),
            );
            for font in retro::EditorFont::ALL {
                let selected = self.editor_font == font;
                let sample = widget::column::with_capacity(2)
                    .push(widget::text(font.label()).font(font.font()).size(17))
                    .push(widget::text::caption(font.blurb()))
                    .spacing(2);
                col = col.push(
                    widget::button::custom(widget::container(sample).width(Length::Fill))
                        .padding([6, 10])
                        .width(Length::Fill)
                        .class(if selected {
                            cosmic::theme::Button::Suggested
                        } else {
                            cosmic::theme::Button::Standard
                        })
                        .on_press(Message::SetFont(font)),
                );
            }
        }

        // Size: one card per pane with a live sample, then the dock.
        col = col.push(
            widget::container(header(fl!("section-size"), Section::Size)).padding([8, 0, 0, 0]),
        );
        if self.appearance_open[Section::Size as usize] {
            let cards = [
                (
                    Pane::Sidebar,
                    fl!("size-sidebar"),
                    fl!("size-sample-sidebar"),
                    self.sidebar_size,
                    retro::mono(),
                    retro::PANE_SIZE_MIN,
                    retro::PANE_SIZE_MAX,
                ),
                (
                    Pane::List,
                    fl!("size-list"),
                    fl!("size-sample-list"),
                    self.list_size,
                    retro::mono(),
                    retro::PANE_SIZE_MIN,
                    retro::PANE_SIZE_MAX,
                ),
                (
                    Pane::Editor,
                    fl!("size-editor"),
                    fl!("size-sample-editor"),
                    self.font_size,
                    self.editor_font.font(),
                    retro::FONT_SIZE_MIN,
                    retro::FONT_SIZE_MAX,
                ),
            ];
            for (pane, label, sample, size, font, min, max) in cards {
                let step = |glyph: &'static str, delta: i16, enabled: bool| {
                    widget::button::custom(widget::text(glyph).size(16))
                        .padding([0, 10])
                        .class(cosmic::theme::Button::Standard)
                        .on_press_maybe(enabled.then_some(Message::SizeStep(pane, delta)))
                };
                let controls = widget::row::with_capacity(4)
                    .push(widget::text::heading(label).width(Length::Fill))
                    .push(step("−", -1, size > min))
                    .push(widget::text(format!("{size} px")).size(14))
                    .push(step("+", 1, size < max))
                    .spacing(8)
                    .align_y(Alignment::Center);
                let card = widget::column::with_capacity(2)
                    .push(controls)
                    .push(widget::text(sample).font(font).size(f32::from(size)))
                    .spacing(6)
                    .width(Length::Fill);
                col = col.push(
                    widget::container(card)
                        .padding([8, 12])
                        .width(Length::Fill)
                        .class(cosmic::theme::Container::Card),
                );
            }

            col = col.push(
                widget::container(widget::text::heading(fl!("dock-size"))).padding([6, 0, 0, 0]),
            );
            let mut sizes = widget::row::with_capacity(4).spacing(6);
            for size in retro::DockSize::ALL {
                sizes = sizes.push(
                    widget::button::custom(widget::text(size.label()))
                        .padding([4, 10])
                        .class(if self.dock_size == size {
                            cosmic::theme::Button::Suggested
                        } else {
                            cosmic::theme::Button::Standard
                        })
                        .on_press(Message::SetDockSize(size)),
                );
            }
            col = col.push(sizes);

            col = col.push(
                widget::container(widget::text::heading(fl!("text-width"))).padding([6, 0, 0, 0]),
            );
            col = col.push(widget::text::caption(fl!("text-width-hint")));
            let mut widths = widget::row::with_capacity(4).spacing(6);
            for m in retro::Measure::ALL {
                widths = widths.push(widget::tooltip(
                    widget::button::custom(widget::text(m.label()).size(15))
                        .padding([4, 10])
                        .class(if self.measure == m {
                            cosmic::theme::Button::Suggested
                        } else {
                            cosmic::theme::Button::Standard
                        })
                        .on_press(Message::SetMeasure(m)),
                    widget::text::caption(m.blurb()),
                    widget::tooltip::Position::Top,
                ));
            }
            col = col.push(widths);
        }

        // Tasks: what a finished one is marked with.
        col = col.push(
            widget::container(header(fl!("section-tasks"), Section::Tasks)).padding([8, 0, 0, 0]),
        );
        if self.appearance_open[Section::Tasks as usize] {
            col = col.push(widget::text::caption(fl!("task-marker-hint")));
            let mut marks: Vec<Element<'_, Message>> =
                Vec::with_capacity(retro::TASK_MARKERS.len());
            for (mark, blurb) in retro::TASK_MARKERS {
                let selected = self.task_marker == mark;
                marks.push(
                    widget::tooltip(
                        widget::button::custom(
                            widget::text(format!("[{mark}]"))
                                .font(retro::mono())
                                .size(16),
                        )
                        .padding([4, 8])
                        .class(if selected {
                            cosmic::theme::Button::Suggested
                        } else {
                            cosmic::theme::Button::Standard
                        })
                        .on_press(Message::SetTaskMarker(mark.to_owned())),
                        widget::text::caption(blurb),
                        widget::tooltip::Position::Top,
                    )
                    .into(),
                );
            }
            col = col.push(widget::flex_row(marks).spacing(6));
            let p = self.palette();
            let sample = widget::column::with_capacity(2)
                .push(
                    retro::text(&p, fl!("task-sample-open"))
                        .font(self.editor_font.font())
                        .size(f32::from(self.font_size)),
                )
                .push(
                    retro::text(
                        &p,
                        fl!("task-sample-done", mark = self.task_marker.as_str()),
                    )
                    .font(self.editor_font.font())
                    .size(f32::from(self.font_size))
                    .class(cosmic::theme::Text::Color(p.fg.scale_alpha(0.45))),
                )
                .spacing(4);
            col = col.push(
                widget::container(sample)
                    .padding([8, 12])
                    .width(Length::Fill)
                    .class(cosmic::theme::Container::Card),
            );
        }
        widget::scrollable(col).into()
    }

    /// Apply a dock format action to the editor buffer.
    fn apply_format(&mut self, format: Format) {
        use text_editor::{Action, Edit, Motion};
        if self.blocks.focused_text().is_none() {
            return;
        }
        self.record(EditKind::Other);
        let Some(editor) = self.blocks.focused_text() else {
            return;
        };
        let perform = |editor: &mut crate::editor::Content, action: Action| editor.perform(action);
        let insert_str = |editor: &mut crate::editor::Content, s: &str| {
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
                if let Some(selection) = editor.selection() {
                    // Already wrapped? Then this press unwraps it.
                    let inner = selection
                        .strip_prefix(before)
                        .and_then(|s| s.strip_suffix(after))
                        .filter(|s| !s.is_empty());
                    let text = match inner {
                        Some(inner) => inner.to_owned(),
                        None => format!("{before}{selection}{after}"),
                    };
                    perform(editor, Action::Edit(Edit::Paste(Arc::new(text))));
                } else {
                    insert_str(editor, before);
                    insert_str(editor, after);
                    for _ in 0..after.chars().count() {
                        perform(editor, Action::Move(Motion::Left));
                    }
                }
            }
            Format::H1 | Format::H2 | Format::Bullet | Format::Todo | Format::Quote => {
                let prefix = match format {
                    Format::H1 => "# ",
                    Format::H2 => "## ",
                    Format::Bullet => "- ",
                    Format::Quote => "> ",
                    _ => "- [ ] ",
                };
                // Every line the selection touches (or just the cursor's).
                let cursor = editor.cursor();
                let (first, last) = match cursor.selection {
                    Some(sel) => (
                        sel.line.min(cursor.position.line),
                        sel.line.max(cursor.position.line),
                    ),
                    None => (cursor.position.line, cursor.position.line),
                };
                // Toggle: strip the same prefix if the line already carries it,
                // otherwise replace a different line prefix and add ours. With
                // several lines, the first line decides for all of them.
                let first_text = editor
                    .line(first)
                    .map(|l| l.text.into_owned())
                    .unwrap_or_default();
                let remove_only = line_prefix(&first_text).map(|(_, f)| f) == Some(format);
                for l in first..=last {
                    let line = editor
                        .line(l)
                        .map(|t| t.text.into_owned())
                        .unwrap_or_default();
                    editor.move_to(text_editor::Cursor {
                        position: text_editor::Position { line: l, column: 0 },
                        selection: None,
                    });
                    if let Some((len, _)) = line_prefix(&line) {
                        for _ in 0..line[..len].chars().count() {
                            perform(editor, Action::Edit(Edit::Delete));
                        }
                    }
                    if !remove_only {
                        insert_str(editor, prefix);
                    }
                }
                perform(editor, Action::Move(Motion::End));
            }
            Format::Tag => insert_str(editor, "#"),
            Format::Rule => {
                // A rule wants a line of its own; it is drawn as a full-width line.
                let cursor = editor.cursor();
                let line_empty = editor
                    .line(cursor.position.line)
                    .is_none_or(|l| l.text.trim().is_empty());
                if line_empty {
                    perform(editor, Action::Move(Motion::End));
                    insert_str(editor, "---\n");
                } else {
                    perform(editor, Action::Move(Motion::End));
                    insert_str(editor, "\n---\n");
                }
            }
        }
        self.dirty = true;
        self.last_edit = Instant::now();
        if self.blocks.needs_resplit() {
            self.blocks.resplit();
        }
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

    // ----- shortcuts -----

    /// The key binding shown for an action, if any ("Ctrl + B").
    fn shortcut_for(&self, action: MenuAction) -> Option<String> {
        let mut found: Vec<String> = self
            .key_binds
            .iter()
            .filter(|(_, a)| **a == action)
            .map(|(k, _)| k.to_string())
            .collect();
        found.sort_by_key(|k| k.len());
        found.into_iter().next()
    }

    /// "Label  ·  Ctrl + B" for tooltips.
    fn with_shortcut(&self, label: String, action: MenuAction) -> String {
        match self.shortcut_for(action) {
            Some(key) => format!("{label}  ·  {key}"),
            None => label,
        }
    }

    /// Everything with a shortcut, grouped the way the menus are.
    fn shortcut_table(&self) -> Vec<(String, Vec<(String, MenuAction)>)> {
        let format: Vec<(String, MenuAction)> = Format::ALL
            .into_iter()
            .map(|f| (f.label(), MenuAction::Format(f)))
            .collect();
        vec![
            (
                fl!("file"),
                vec![
                    (fl!("new-note"), MenuAction::NewNote),
                    (fl!("new-folder"), MenuAction::NewFolder),
                    (fl!("dock-image"), MenuAction::AddImage),
                    (fl!("search-notes"), MenuAction::FocusSearch),
                    (fl!("pin-note"), MenuAction::Pin),
                    (fl!("trash-note"), MenuAction::TrashNote),
                ],
            ),
            (
                fl!("edit"),
                vec![
                    (fl!("undo"), MenuAction::Undo),
                    (fl!("redo"), MenuAction::Redo),
                ],
            ),
            (fl!("format"), format),
            (
                fl!("view"),
                vec![
                    (fl!("show-nav"), MenuAction::ToggleNav),
                    (fl!("show-list"), MenuAction::ToggleList),
                    (fl!("editor-only"), MenuAction::Solo),
                    (fl!("theme-colours"), MenuAction::Themes),
                    (fl!("show-markers"), MenuAction::ToggleMarkers),
                    (fl!("shortcuts"), MenuAction::Shortcuts),
                ],
            ),
        ]
    }

    /// Full-window overlay listing every shortcut; × or Esc closes it.
    fn shortcuts_overlay<'a>(&'a self, p: &Palette) -> Element<'a, Message> {
        let mut columns = widget::row::with_capacity(2)
            .spacing(28)
            .align_y(Alignment::Start);
        let groups = self.shortcut_table();
        let half = groups.len().div_ceil(2);
        for chunk in groups.chunks(half) {
            let mut col = widget::column::with_capacity(24)
                .spacing(3)
                .width(Length::Fixed(300.0));
            for (group, entries) in chunk {
                col = col.push(
                    widget::container(retro::title(p, group.clone()).size(20))
                        .padding([8, 0, 2, 0]),
                );
                for (label, action) in entries {
                    let key = self.shortcut_for(*action).unwrap_or_else(|| "—".to_owned());
                    col = col.push(
                        widget::row::with_capacity(2)
                            .push(retro::text(p, label.clone()).width(Length::Fill))
                            .push(retro::accent(p, key))
                            .spacing(12),
                    );
                }
            }
            columns = columns.push(col);
        }
        let close = widget::button::custom(retro::accent(p, "×").size(18))
            .padding([0, 8])
            .class(retro::row_class(p, false))
            .on_press(Message::ToggleShortcuts);
        let header = widget::row::with_capacity(2)
            .push(retro::dim(p, fl!("shortcuts-hint")).width(Length::Fill))
            .push(close)
            .align_y(Alignment::Center);
        let body = widget::column::with_capacity(2)
            .push(header)
            .push(columns)
            .spacing(6)
            .padding([2, 6, 6, 6]);
        let card = retro::frame_sized(
            p,
            fl!("shortcuts").to_lowercase(),
            self.shortcut_for(MenuAction::Shortcuts),
            body,
            Length::Shrink,
            21.0,
        );
        let backdrop = p.bg.scale_alpha(0.88);
        widget::container(widget::container(card).max_width(700.0))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .padding(24)
            .class(cosmic::theme::Container::custom(move |_| {
                widget::container::Style {
                    background: Some(cosmic::iced::Background::Color(backdrop)),
                    ..Default::default()
                }
            }))
            .into()
    }

    // ----- blocks & images -----

    /// Focus the text block that currently has the caret.
    fn focus_editor(&self) -> Task<cosmic::Action<Message>> {
        match self.blocks.focused_id() {
            Some(id) => widget::text_input::focus(id),
            None => Task::none(),
        }
    }

    /// The note body as a column of text editors and inline images.
    fn blocks_view<'a>(&'a self, p: &Palette, trashed: bool) -> Element<'a, Message> {
        let items = &self.blocks.items;
        let last_text = self.blocks.last_text();
        // While a picture is being dragged the note becomes a drop target:
        // plain lines with hover slots instead of editors.
        let drag = self.dragging.filter(|d| d.active);
        let offsets = self.blocks.line_offsets();
        let target = drag.and_then(|d| d.target);
        let text_el = |i: usize, content: &'a crate::editor::Content, id: widget::Id| {
            if drag.is_some() {
                self.drag_lines(p, content, offsets[i], target, i == last_text)
            } else {
                self.text_block(p, i, content, id, trashed, i == last_text)
            }
        };
        let mut col = widget::column::with_capacity(items.len() + 1)
            .spacing(6)
            .width(Length::Fill);
        let mut i = 0;
        while i < items.len() {
            match &items[i] {
                Block::Text { content, id } => {
                    col = col.push(text_el(i, content, id.clone()));
                    i += 1;
                }
                Block::Rule(_) => {
                    if drag.is_some() && target == Some(offsets[i]) {
                        col = col.push(retro::drop_line(p, fl!("drop-here")));
                    }
                    let rule = retro::rule_block(p);
                    if drag.is_some() {
                        let slot = |line: usize| {
                            widget::mouse_area(
                                widget::Space::new()
                                    .width(Length::Fill)
                                    .height(Length::Fill),
                            )
                            .on_move(move |_| Message::DragOver(line))
                            .interaction(mouse::Interaction::Grabbing)
                        };
                        let halves = widget::column::with_capacity(2)
                            .push(slot(offsets[i]))
                            .push(slot(offsets[i] + 1))
                            .width(Length::Fill)
                            .height(Length::Fill);
                        col = col.push(
                            cosmic::iced::widget::stack([rule, halves.into()]).width(Length::Fill),
                        );
                    } else {
                        col = col.push(rule);
                    }
                    i += 1;
                }
                Block::Image(r) => {
                    if drag.is_some() && target == Some(offsets[i]) {
                        col = col.push(retro::drop_line(p, fl!("drop-here")));
                    }
                    let card = match drag {
                        Some(d) => self.drag_image_card(p, r, i, offsets[i], d.block == i),
                        None => self.image_card(p, r, i),
                    };
                    match r.align {
                        Align::Center => {
                            col = col.push(
                                widget::container(card)
                                    .width(Length::Fill)
                                    .align_x(Alignment::Center)
                                    .padding([4, 10]),
                            );
                            i += 1;
                        }
                        Align::Left | Align::Right => {
                            let paired = match items.get(i + 1) {
                                Some(Block::Text { content, id }) => {
                                    Some(text_el(i + 1, content, id.clone()))
                                }
                                _ => None,
                            };
                            let has_pair = paired.is_some();
                            let card = widget::container(card).padding([4, 10]);
                            let row = match (r.align, paired) {
                                (Align::Left, Some(text)) => {
                                    widget::row::with_capacity(2).push(card).push(text)
                                }
                                (_, Some(text)) => {
                                    widget::row::with_capacity(2).push(text).push(card)
                                }
                                (Align::Left, None) => widget::row::with_capacity(1).push(card),
                                (_, None) => widget::row::with_capacity(2)
                                    .push(widget::Space::new().width(Length::Fill))
                                    .push(card),
                            };
                            col = col
                                .push(row.spacing(8).align_y(Alignment::Start).width(Length::Fill));
                            i += if has_pair { 2 } else { 1 };
                        }
                    }
                }
            }
        }
        if drag.is_some() && target == offsets.last().copied() {
            col = col.push(retro::drop_line(p, fl!("drop-here")));
        }
        // The measure: the column never grows past the chosen width and sits
        // centred with margins; a narrower pane simply wraps the text.
        let mut column = widget::container(col).width(Length::Fill);
        if let Some(max) = self.measure.max_width() {
            column = column.max_width(max);
        }
        let col = widget::container(column)
            .width(Length::Fill)
            .align_x(Alignment::Center);
        widget::scrollable(col)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }

    fn text_block<'a>(
        &'a self,
        p: &Palette,
        block: usize,
        content: &'a crate::editor::Content,
        id: widget::Id,
        trashed: bool,
        is_last: bool,
    ) -> Element<'a, Message> {
        let settings = markdown::Settings {
            palette: *p,
            show_markers: self.show_markers,
            font: self.editor_font.font(),
        };
        let mut editor = crate::editor::RichEditor::new(content, settings)
            .id(id)
            .placeholder(if block == 0 {
                fl!("untitled")
            } else {
                String::new()
            })
            .size(f32::from(self.font_size))
            .padding([6, 10]);
        if is_last {
            editor = editor.min_height(220.0);
        }
        if !trashed {
            editor = editor
                .on_action(move |a| Message::Editor(block, a))
                .on_link(Message::FollowLink);
        }
        editor.into()
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
        let cols = if r.frame == FrameStyle::Ascii {
            images::ascii_layout(r.width).0
        } else {
            0
        };
        Some((
            format!(
                "{}|{mtime}|{}|{theme}|{cols}",
                path.display(),
                r.frame.key()
            ),
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
        let refs: Vec<ImageRef> = self
            .blocks
            .images()
            .into_iter()
            .map(|(_, r)| r.clone())
            .collect();
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
            let cols = images::ascii_layout(r.width).0;
            tasks.push(Task::perform(
                async move {
                    tokio::task::spawn_blocking(move || {
                        images::load_and_process(&path, style, palette, cols)
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

    /// Copy a file into assets/ and drop it into the note at the caret.
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
        let r = ImageRef {
            line: 0,
            alt,
            path: rel,
            frame: FrameStyle::default(),
            align: Align::default(),
            width: None,
        };
        self.record(EditKind::Other);
        self.blocks.insert_image(r);
        self.dirty = true;
        self.last_edit = Instant::now();
        self.focus_editor()
    }

    /// Change one image's attributes in place.
    fn edit_ref(&mut self, block: usize, f: impl FnOnce(&mut ImageRef)) {
        self.edit_ref_kind(block, EditKind::Other, f);
    }

    fn edit_ref_kind(&mut self, block: usize, kind: EditKind, f: impl FnOnce(&mut ImageRef)) {
        if self.blocks.image_mut(block).is_none() {
            return;
        }
        self.record(kind);
        if let Some(r) = self.blocks.image_mut(block) {
            f(r);
            self.dirty = true;
            self.last_edit = Instant::now();
        }
    }

    // ----- undo -----

    fn snapshot(&self) -> Snapshot {
        let cursor = self
            .blocks
            .text(self.blocks.focused)
            .map(|c| c.cursor())
            .unwrap_or(text_editor::Cursor {
                position: text_editor::Position { line: 0, column: 0 },
                selection: None,
            });
        Snapshot {
            body: self.blocks.body(),
            focused: self.blocks.focused,
            cursor,
        }
    }

    /// Push an undo step before an edit, unless it continues the current
    /// typing/deleting burst. Any edit invalidates the redo stack.
    fn record(&mut self, kind: EditKind) {
        let continues = kind != EditKind::Other
            && kind == self.last_undo_kind
            && self.last_undo_at.elapsed() < UNDO_GROUP_IDLE;
        self.last_undo_at = Instant::now();
        self.last_undo_kind = kind;
        self.redo.clear();
        if continues {
            return;
        }
        let snap = self.snapshot();
        self.push_undo(snap);
    }

    /// Push a snapshot as an undo step (skipping an exact repeat).
    fn push_undo(&mut self, snap: Snapshot) {
        if self.undo.last().is_some_and(|last| last.body == snap.body) {
            return;
        }
        self.undo.push(snap);
        if self.undo.len() > UNDO_DEPTH {
            self.undo.remove(0);
        }
    }

    fn restore(&mut self, snap: Snapshot) {
        self.blocks
            .rebuild(&snap.body, snap.focused, Some(snap.cursor));
        self.image_menu = None;
        self.last_undo_kind = EditKind::Other;
        self.dirty = true;
        self.last_edit = Instant::now();
    }

    /// A text block while a picture is being dragged: one plain line per body
    /// line, each split into a top half (drop above) and a bottom half (drop
    /// below), with the drop line drawn exactly where the picture will land.
    fn drag_lines<'a>(
        &'a self,
        p: &Palette,
        content: &'a crate::editor::Content,
        offset: usize,
        target: Option<usize>,
        is_last: bool,
    ) -> Element<'a, Message> {
        let slot = |line: usize, height: Length| {
            widget::mouse_area(widget::Space::new().width(Length::Fill).height(height))
                .on_move(move |_| Message::DragOver(line))
                .interaction(mouse::Interaction::Grabbing)
        };
        let text = content.text();
        let text = text.strip_suffix('\n').unwrap_or(&text);
        let mut col =
            widget::column::with_capacity(text.lines().count() * 2 + 2).width(Length::Fill);
        let count = if text.is_empty() {
            // A gap between pictures owns no line; hovering it means "here".
            col = col.push(slot(
                offset,
                Length::Fixed(if is_last { 120.0 } else { 14.0 }),
            ));
            0
        } else {
            let mut count = 0;
            for (k, line) in text.split('\n').enumerate() {
                if target == Some(offset + k) {
                    col = col.push(retro::drop_line(p, fl!("drop-here")));
                }
                let shown = if line.is_empty() {
                    " ".to_owned()
                } else {
                    line.to_owned()
                };
                let line_el = retro::text(p, shown)
                    .font(self.editor_font.font())
                    .size(f32::from(self.font_size))
                    .line_height(1.5)
                    .width(Length::Fill);
                let halves = widget::column::with_capacity(2)
                    .push(slot(offset + k, Length::Fill))
                    .push(slot(offset + k + 1, Length::Fill))
                    .width(Length::Fill)
                    .height(Length::Fill);
                col = col.push(
                    cosmic::iced::widget::stack([line_el.into(), halves.into()])
                        .width(Length::Fill),
                );
                count = k + 1;
            }
            count
        };
        if is_last && count > 0 {
            // Room below the last line so the end of the note is a target too.
            col = col.push(slot(offset + count, Length::Fixed(120.0)));
        }
        widget::container(col)
            .padding([6, 10])
            .width(Length::Fill)
            .into()
    }

    /// An image while a picture is being dragged: top half drops above it,
    /// bottom half below; the one being moved is outlined.
    fn drag_image_card<'a>(
        &'a self,
        p: &Palette,
        r: &'a ImageRef,
        block: usize,
        offset: usize,
        dragged: bool,
    ) -> Element<'a, Message> {
        let pic = self.image_picture(p, r, block);
        let slot = |line: usize| {
            widget::mouse_area(
                widget::Space::new()
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .on_move(move |_| Message::DragOver(line))
            .interaction(mouse::Interaction::Grabbing)
        };
        let halves = widget::column::with_capacity(2)
            .push(slot(offset))
            .push(slot(offset + 1))
            .width(Length::Fill)
            .height(Length::Fill);
        let stacked = cosmic::iced::widget::stack([pic, halves.into()]).width(Length::Fill);
        let el: Element<'a, Message> = if dragged {
            retro::lifted(p, stacked.into())
        } else {
            stacked.into()
        };
        self.size_card(r, block, el).into()
    }

    /// The card at its chosen width (live while resizing).
    fn size_card<'a>(
        &self,
        r: &ImageRef,
        block: usize,
        content: Element<'a, Message>,
    ) -> widget::Container<'a, Message, cosmic::Theme> {
        let width = self
            .live_width
            .filter(|(b, _)| *b == block)
            .map(|(_, w)| w)
            .or(r.width);
        match (r.align, width) {
            (Align::Center, Some(w)) => widget::container(content)
                .max_width(w as f32)
                .width(Length::Fill),
            (Align::Center, None) => widget::container(content).width(Length::Fill),
            (_, w) => widget::container(content).width(Length::Fixed(w.unwrap_or(360) as f32)),
        }
    }

    /// The picture itself in its frame treatment (or its loading / error state).
    fn image_picture<'a>(
        &'a self,
        p: &Palette,
        r: &'a ImageRef,
        block: usize,
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
                    FrameStyle::Box => retro::frame_sized(
                        p,
                        if r.alt.trim().is_empty() {
                            r.file_name().to_owned()
                        } else {
                            r.alt.clone()
                        },
                        Some(format!("{w}×{h}")),
                        img,
                        Length::Shrink,
                        16.0,
                    ),
                    FrameStyle::Tint
                    | FrameStyle::Dither
                    | FrameStyle::Pixel
                    | FrameStyle::Ascii => retro::bordered(p, img.into()),
                    FrameStyle::Bezel => retro::bezel(p, img.into()),
                    FrameStyle::Print => retro::print(p, img.into(), r.alt.clone()),
                    FrameStyle::Film => retro::film(p, img.into(), self.image_number(block)),
                    FrameStyle::Comic => retro::comic(p, img.into(), r.alt.clone()),
                }
            }
            Some(ImageState::Ascii(text)) => {
                let live = self
                    .live_width
                    .filter(|(b, _)| *b == block)
                    .map(|(_, w)| w)
                    .or(r.width);
                retro::ascii_card(p, text.clone(), images::ascii_layout(live).1)
            }
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
        pic
    }

    /// One inline image: frame treatment, ⋯ menu, drag grip, and the popup menu.
    fn image_card<'a>(
        &'a self,
        p: &Palette,
        r: &'a ImageRef,
        block: usize,
    ) -> Element<'a, Message> {
        let pic = self.image_picture(p, r, block);

        // Click the picture for the menu, drag it to move it; drag ◢ to resize.
        let clickable = widget::mouse_area(pic)
            .on_press(Message::ImagePress(block))
            .interaction(mouse::Interaction::Grab);
        let menu_open = self.image_menu == Some(block);
        let dots = widget::container(
            widget::button::custom(retro::accent(p, "⋯").size(14))
                .padding([0, 6])
                .class(retro::row_class(p, menu_open))
                .on_press(Message::ImageMenu(Some(block))),
        )
        .width(Length::Fill)
        .align_x(Alignment::End)
        .padding([14, 6, 0, 0]);
        let grip = widget::container(
            widget::mouse_area(widget::container(retro::accent(p, "◢").size(13)).padding([0, 3]))
                .on_press(Message::ResizeStart(block))
                .interaction(mouse::Interaction::ResizingHorizontally),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::End)
        .align_y(Alignment::End)
        .padding(2);
        let stacked = cosmic::iced::widget::stack([clickable.into(), dots.into(), grip.into()])
            .width(Length::Fill);

        let sized = self.size_card(r, block, stacked.into());

        if menu_open {
            widget::popover(sized)
                .popup(self.image_menu_view(p, r, block))
                .position(widget::popover::Position::Bottom)
                .on_close(Message::ImageMenu(None))
                .into()
        } else {
            sized.into()
        }
    }

    /// 1-based position of this image among the note's images (film frame number).
    fn image_number(&self, block: usize) -> usize {
        self.blocks
            .images()
            .iter()
            .position(|(b, _)| *b == block)
            .map_or(1, |i| i + 1)
    }

    /// The ⋯ menu: frame styles, alignment, width, open, remove.
    fn image_menu_view<'a>(
        &'a self,
        p: &Palette,
        r: &'a ImageRef,
        block: usize,
    ) -> Element<'a, Message> {
        let item = |label: String, selected: bool, msg: Message| {
            widget::button::custom(
                widget::row::with_capacity(2)
                    .push(retro::accent(p, if selected { "▌" } else { " " }))
                    .push(retro::text(p, label))
                    .spacing(6),
            )
            .padding([2, 8])
            .width(Length::Fill)
            .class(retro::row_class(p, selected))
            .on_press(msg)
        };
        let mut col = widget::column::with_capacity(22)
            .spacing(1)
            .width(Length::Fixed(220.0));
        col = col.push(widget::container(retro::dim(p, fl!("menu-caption"))).padding([2, 8]));
        col = col.push(
            widget::container(
                widget::text_input(fl!("caption-placeholder"), &r.alt)
                    .font(retro::mono())
                    .size(13)
                    .padding([3, 8])
                    .style(retro::search_class(p))
                    .on_input(move |t| Message::SetCaption(block, t))
                    .on_submit(|_| Message::ImageMenu(None)),
            )
            .padding([0, 6, 4, 6]),
        );
        col = col.push(widget::container(retro::dim(p, fl!("menu-frame"))).padding([2, 8]));
        for style in FrameStyle::ALL {
            col = col.push(item(
                style.label().to_owned(),
                r.frame == style,
                Message::SetFrame(block, style),
            ));
        }
        col = col.push(widget::container(retro::dim(p, fl!("menu-align"))).padding([6, 8, 2, 8]));
        let mut align_row = widget::row::with_capacity(3).spacing(2);
        for a in Align::ALL {
            align_row = align_row.push(
                widget::button::custom(retro::text(p, a.label()))
                    .padding([2, 8])
                    .class(retro::row_class(p, r.align == a))
                    .on_press(Message::SetAlign(block, a)),
            );
        }
        col = col.push(widget::container(align_row).padding([0, 6]));
        col = col.push(widget::container(retro::dim(p, fl!("menu-width"))).padding([6, 8, 2, 8]));
        let mut width_row = widget::row::with_capacity(4).spacing(2);
        for (label, px) in images::WIDTH_PRESETS {
            width_row = width_row.push(
                widget::button::custom(retro::text(p, label))
                    .padding([2, 6])
                    .class(retro::row_class(p, r.width == Some(px)))
                    .on_press(Message::SetWidth(block, Some(px))),
            );
        }
        width_row = width_row.push(
            widget::button::custom(retro::text(p, fl!("menu-full")))
                .padding([2, 6])
                .class(retro::row_class(p, r.width.is_none()))
                .on_press(Message::SetWidth(block, None)),
        );
        col = col.push(widget::container(width_row).padding([0, 6]));
        col = col.push(
            widget::container(retro::dim(p, fl!("menu-resize-hint")).size(11)).padding([2, 8]),
        );
        col = col.push(
            widget::container(retro::dim(p, fl!("menu-move-hint")).size(11)).padding([0, 8, 2, 8]),
        );
        col = col.push(
            widget::row::with_capacity(2)
                .push(item(
                    fl!("open-image"),
                    false,
                    Message::OpenImage(r.path.clone()),
                ))
                .push(item(fl!("menu-remove"), false, Message::RemoveImage(block)))
                .spacing(2),
        );
        widget::container(col)
            .padding([6, 4])
            .class(retro::dock_class(p))
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
                self.blocks = Blocks::from_body(&note.body);
                self.image_menu = None;
                self.undo.clear();
                self.redo.clear();
                self.last_undo_kind = EditKind::Other;
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
        self.dragging = None;
        self.resizing = None;
        self.live_width = None;
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
        note.body = self.blocks.body();
        if let Err(err) = store.save(note) {
            tracing::error!(%err, "saving note");
            self.save_error = Some(format!("{err:#}"));
            // Keep the edit pending and retry in a few seconds.
            self.dirty = true;
            self.last_edit = Instant::now() + Duration::from_secs(5);
            return;
        }
        self.save_error = None;
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
                    task = self.update(Message::Editor(
                        self.blocks.focused,
                        text_editor::Action::Edit(edit),
                    ));
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
            Step::SelectAll => self.update(Message::Editor(
                self.blocks.focused,
                text_editor::Action::SelectAll,
            )),
            Step::Dock => self.update(Message::ToggleDock),
            Step::Undo => self.update(Message::Undo),
            Step::Shortcuts => self.update(Message::ToggleShortcuts),
            Step::SavedInfo => self.update(Message::ToggleSavedInfo),
            Step::Redo => self.update(Message::Redo),
            Step::Themes => self.update(Message::ToggleContextPage(ContextPage::Themes)),
            Step::Solo => self.update(Message::ToggleSolo),
            Step::Image(path) => self.update(Message::ImagesDropped(vec![PathBuf::from(path)])),
            Step::Pick => self.update(Message::PickImage),
            Step::ImgFrame(n, key) => match (self.nth_image_block(n), FrameStyle::from_key(&key)) {
                (Some(b), Some(f)) => self.update(Message::SetFrame(b, f)),
                _ => Task::none(),
            },
            Step::ImgAlign(n, key) => match (self.nth_image_block(n), Align::from_key(&key)) {
                (Some(b), Some(a)) => self.update(Message::SetAlign(b, a)),
                _ => Task::none(),
            },
            Step::ImgWidth(n, w) => match self.nth_image_block(n) {
                Some(b) => self.update(Message::SetWidth(b, (w > 0).then_some(w))),
                None => Task::none(),
            },
            Step::ImgCaption(n, text) => match self.nth_image_block(n) {
                Some(b) => self.update(Message::SetCaption(b, text)),
                None => Task::none(),
            },
            Step::ImgMenu(n) => match self.nth_image_block(n) {
                Some(b) => self.update(Message::ImageMenu(Some(b))),
                None => Task::none(),
            },
            Step::ImgDrag(n, line) | Step::ImgMove(n, line) => match self.nth_image_block(n) {
                Some(b) => {
                    self.dragging = Some(ImageDrag {
                        block: b,
                        start: Point::ORIGIN,
                        active: true,
                        target: Some(line),
                    });
                    if matches!(step, Step::ImgMove(..)) {
                        self.update(Message::MouseReleased)
                    } else {
                        Task::none()
                    }
                }
                None => Task::none(),
            },
            Step::Theme(key) => self.update(Message::SetTheme(retro::Theme::from_key(&key))),
            Step::Fold(tag) => self.update(Message::ToggleTagFold(tag)),
            Step::Nav(delta) => self.update(Message::NavigateNotes(delta)),
            Step::ToggleBox(line, col) => {
                let block = self.blocks.focused;
                if let Some(c) = self.blocks.text_mut(block) {
                    c.move_to(text_editor::Cursor {
                        position: text_editor::Position { line, column: col },
                        selection: None,
                    });
                }
                let before = self.snapshot();
                let marker = self.task_marker.clone();
                if let Some(c) = self.blocks.text_mut(block)
                    && crate::blocks::toggle_task_at_cursor(c, &marker)
                {
                    self.push_undo(before);
                    self.dirty = true;
                    self.last_edit = Instant::now();
                }
                Task::none()
            }
            Step::TagMenu(tag) => self.update(Message::TagMenu(Some(tag))),
            Step::RenameTag(old, new) => {
                self.tag_rename = Some((old, new));
                self.update(Message::TagRenameCommit)
            }
            Step::Font(key) => self.update(Message::SetFont(retro::EditorFont::from_key(&key))),
            Step::Pairing(key) => self.update(Message::SetPairing(key)),
            Step::Section(name) => self.update(Message::ToggleSection(match name.as_str() {
                "colour" | "color" => Section::Colour,
                "font" => Section::Font,
                "tasks" => Section::Tasks,
                _ => Section::Size,
            })),
            Step::Marker(mark) => self.update(Message::SetTaskMarker(mark)),
            Step::Measure(key) => self.update(Message::SetMeasure(retro::Measure::from_key(&key))),
            Step::Follow(title) => self.update(Message::FollowLink(
                crate::editor::widget::Link::Note(title),
            )),
            Step::Size(pane, delta) => {
                let pane = match pane.as_str() {
                    "sidebar" => Pane::Sidebar,
                    "list" => Pane::List,
                    _ => Pane::Editor,
                };
                self.update(Message::SizeStep(pane, delta))
            }
            Step::DockSize(key) => {
                self.update(Message::SetDockSize(retro::DockSize::from_key(&key)))
            }
            Step::Trash => self.update(Message::TrashCurrent),
            Step::Wait(_) => Task::none(),
            Step::Exit => {
                self.close_current();
                std::process::exit(0);
            }
        }
    }

    fn nth_image_block(&self, n: usize) -> Option<usize> {
        self.blocks.images().get(n).map(|(b, _)| *b)
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

fn note_row<'a>(
    p: &Palette,
    note: &'a NoteSummary,
    selected: bool,
    size: u16,
    ui: cosmic::font::Font,
) -> Element<'a, Message> {
    let fg = if selected { p.selfg } else { p.fg };
    let size = f32::from(size);
    let mut title_row = widget::row::with_capacity(3)
        .spacing(8)
        .align_y(Alignment::Center);
    if note.pinned {
        title_row = title_row.push(retro::accent(p, "▲").size(size));
    }
    title_row = title_row
        .push(
            retro::text(p, note.title.clone())
                .font(cosmic::font::Font {
                    weight: cosmic::iced::font::Weight::Bold,
                    ..ui
                })
                .size(size)
                .class(cosmic::theme::Text::Color(fg))
                .width(Length::Fill),
        )
        .push(
            retro::dim(p, format_date(note.modified))
                .font(ui)
                .size(size - 2.0),
        );

    let mut column = widget::column::with_capacity(2)
        .push(title_row)
        .spacing(3)
        .width(Length::Fill);
    if !note.preview.is_empty() {
        let preview: String = note.preview.chars().take(90).collect();
        column = column.push(retro::dim(p, preview).font(ui).size(size - 1.0));
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
/// Register every bundled font with the renderer (the title font and the
/// editor faces). Safe to repeat: already-loaded byte slices are skipped.
fn load_fonts() -> Task<cosmic::Action<Message>> {
    let load = |bytes: &'static [u8]| {
        cosmic::iced::font::load(bytes).map(|result| {
            if let Err(err) = result {
                tracing::error!(?err, "loading bundled font");
            }
            cosmic::Action::App(Message::FontLoaded)
        })
    };
    let mut tasks = vec![load(retro::TITLE_FONT_BYTES)];
    tasks.extend(retro::EDITOR_FONT_FILES.iter().map(|b| load(b)));
    Task::batch(tasks)
}

/// Config stores 0 for "default"; anything else is clamped to the allowed range.
fn size_from_config(px: u16, default: u16, min: u16, max: u16) -> u16 {
    if px == 0 { default } else { px.clamp(min, max) }
}

fn pane_size_from_config(px: u16) -> u16 {
    size_from_config(
        px,
        retro::PANE_SIZE_DEFAULT,
        retro::PANE_SIZE_MIN,
        retro::PANE_SIZE_MAX,
    )
}

/// A text column whose size the user can set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Sidebar,
    List,
    Editor,
}

/// Foldable sections of the Appearance drawer (index into `appearance_open`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Colour = 0,
    Font = 1,
    Size = 2,
    Tasks = 3,
}

fn title_font_from_config(s: &str) -> retro::EditorFont {
    if s.is_empty() {
        retro::EditorFont::Vt323
    } else {
        retro::EditorFont::from_key(s)
    }
}

fn task_marker_from_config(s: &str) -> String {
    if s.is_empty() {
        "x".to_owned()
    } else {
        s.to_owned()
    }
}

/// The block prefix a line already carries and the dock action that makes
/// it: `# ` (H1), `## ` (H2), `- ` (bullet), `- [ ] ` / `- [✓] ` (to-do).
fn line_prefix(line: &str) -> Option<(usize, Format)> {
    if line.starts_with("## ") {
        return Some((3, Format::H2));
    }
    if line.starts_with("# ") {
        return Some((2, Format::H1));
    }
    if line.starts_with("> ") {
        return Some((2, Format::Quote));
    }
    let lm = note::list_marker(line)?;
    match note::task_box(&line[lm..]) {
        Some((len, _)) => Some((lm + len, Format::Todo)),
        None => Some((lm, Format::Bullet)),
    }
}

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
    use keyboard::Key;
    use menu::key_bind::Modifier::{Ctrl, Shift};
    let mut binds = HashMap::new();
    let mut bind = |modifiers: &[menu::key_bind::Modifier], key: &str, action: MenuAction| {
        binds.insert(
            menu::KeyBind {
                modifiers: modifiers.to_vec(),
                key: Key::Character(key.into()),
            },
            action,
        );
    };
    // File
    bind(&[Ctrl], "n", MenuAction::NewNote);
    bind(&[Ctrl, Shift], "n", MenuAction::NewFolder);
    bind(&[Ctrl, Shift], "i", MenuAction::AddImage);
    bind(&[Ctrl], "f", MenuAction::FocusSearch);
    bind(&[Ctrl, Shift], "p", MenuAction::Pin);
    bind(&[Ctrl, Shift], "d", MenuAction::TrashNote);
    // Edit
    bind(&[Ctrl], "z", MenuAction::Undo);
    bind(&[Ctrl, Shift], "z", MenuAction::Redo);
    bind(&[Ctrl], "y", MenuAction::Redo);
    // Format
    bind(&[Ctrl], "b", MenuAction::Format(Format::Bold));
    bind(&[Ctrl], "i", MenuAction::Format(Format::Italic));
    bind(&[Ctrl], "e", MenuAction::Format(Format::Code));
    bind(&[Ctrl], "1", MenuAction::Format(Format::H1));
    bind(&[Ctrl], "2", MenuAction::Format(Format::H2));
    bind(&[Ctrl], "l", MenuAction::Format(Format::Bullet));
    bind(&[Ctrl], "t", MenuAction::Format(Format::Todo));
    bind(&[Ctrl], "k", MenuAction::Format(Format::Link));
    bind(&[Ctrl, Shift], "3", MenuAction::Format(Format::Tag));
    bind(&[Ctrl], "r", MenuAction::Format(Format::Rule));
    bind(&[Ctrl, Shift], "q", MenuAction::Format(Format::Quote));
    // View
    bind(&[Ctrl, Shift], "1", MenuAction::ToggleNav);
    bind(&[Ctrl, Shift], "2", MenuAction::ToggleList);
    bind(&[Ctrl, Shift], "0", MenuAction::Solo);
    bind(&[Ctrl, Shift], "c", MenuAction::Themes);
    bind(&[Ctrl, Shift], "m", MenuAction::ToggleMarkers);
    bind(&[Ctrl, Shift], "h", MenuAction::Shortcuts);
    binds
}

/// One undo step: the whole body plus where the caret was.
#[derive(Debug, Clone)]
pub struct Snapshot {
    body: String,
    focused: usize,
    cursor: text_editor::Cursor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditKind {
    Typing,
    Deleting,
    Other,
}

/// A picture being dragged to a new line.
#[derive(Debug, Clone, Copy)]
pub struct ImageDrag {
    block: usize,
    /// Where the button went down (window coords).
    start: Point,
    /// The pointer has travelled past `DRAG_THRESHOLD`: a drag, not a click.
    active: bool,
    /// Body line the picture will sit before when dropped.
    target: Option<usize>,
}

/// A drag-resize in progress on an image block.
#[derive(Debug, Clone, Copy)]
pub struct Resize {
    block: usize,
    start_x: f32,
    start_w: u32,
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
    Quote,
}

impl Format {
    pub const ALL: [Format; 11] = [
        Format::Bold,
        Format::Italic,
        Format::Code,
        Format::H1,
        Format::H2,
        Format::Bullet,
        Format::Todo,
        Format::Quote,
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
            Format::Quote => "quote",
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
            Format::Quote => "❝",
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
            Format::Quote => fl!("dock-quote"),
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
    Undo,
    Redo,
    NewFolder,
    AddImage,
    Pin,
    Format(Format),
    Shortcuts,
    Themes,
    ToggleMarkers,
    ToggleNav,
    ToggleList,
    Solo,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
            MenuAction::NewNote => Message::NewNote,
            MenuAction::Undo => Message::Undo,
            MenuAction::Redo => Message::Redo,
            MenuAction::NewFolder => Message::ToggleDock,
            MenuAction::AddImage => Message::PickImage,
            MenuAction::Pin => Message::TogglePin,
            MenuAction::Format(f) => Message::Format(*f),
            MenuAction::Shortcuts => Message::ToggleShortcuts,
            MenuAction::TrashNote => Message::TrashCurrent,
            MenuAction::FocusSearch => Message::FocusSearch,
            MenuAction::Themes => Message::ToggleContextPage(ContextPage::Themes),
            MenuAction::ToggleMarkers => Message::ToggleMarkers,
            MenuAction::ToggleNav => Message::ToggleNav,
            MenuAction::ToggleList => Message::ToggleList,
            MenuAction::Solo => Message::ToggleSolo,
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
