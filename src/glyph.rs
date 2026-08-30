// SPDX-License-Identifier: GPL-3.0-only

//! Folder icons a tag can wear instead of its `#`, in seven styles the user
//! picks between: Boxicons Solid (MIT), Iconoir (MIT), Solar Bold (CC BY 4.0,
//! 480 Design), Myna UI Solid (MIT), Majesticons Solid (MIT), Pixelarticons
//! (MIT) and Duoicons (MIT). Bundled as SVG path data from Iconify; drawn in
//! the theme's tag colour in the sidebar, the picker and over the tag's hash
//! in a rendered note. Boxicons covers every meaning; a set that lacks one
//! borrows the Boxicons drawing rather than leaving a gap.

use cosmic::iced::Color;
use cosmic::widget::svg;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Which drawing style the folder icons use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum IconSet {
    #[default]
    Boxicons,
    Iconoir,
    Solar,
    MynaUi,
    Majesticons,
    Pixelarticons,
    DuoIcons,
}

impl IconSet {
    pub const ALL: [IconSet; 7] = [
        IconSet::Boxicons,
        IconSet::Iconoir,
        IconSet::Solar,
        IconSet::MynaUi,
        IconSet::Majesticons,
        IconSet::Pixelarticons,
        IconSet::DuoIcons,
    ];

    pub fn key(self) -> &'static str {
        match self {
            IconSet::Boxicons => "boxicons",
            IconSet::Iconoir => "iconoir",
            IconSet::Solar => "solar",
            IconSet::MynaUi => "mynaui",
            IconSet::Majesticons => "majesticons",
            IconSet::Pixelarticons => "pixelarticons",
            IconSet::DuoIcons => "duoicons",
        }
    }

    pub fn from_key(key: &str) -> IconSet {
        IconSet::ALL
            .into_iter()
            .find(|s| s.key() == key)
            .unwrap_or_default()
    }

    pub fn label(self) -> &'static str {
        match self {
            IconSet::Boxicons => "Boxicons",
            IconSet::Iconoir => "Iconoir",
            IconSet::Solar => "Solar",
            IconSet::MynaUi => "Myna UI",
            IconSet::Majesticons => "Majesticons",
            IconSet::Pixelarticons => "Pixelarticons",
            IconSet::DuoIcons => "Duoicons",
        }
    }

    pub fn blurb(self) -> &'static str {
        match self {
            IconSet::Boxicons => "solid shapes, bold at small sizes",
            IconSet::Iconoir => "thin outlines, lighter on the eye",
            IconSet::Solar => "rounded solids, soft and friendly",
            IconSet::MynaUi => "clean modern solids",
            IconSet::Majesticons => "geometric solids with a quirk",
            IconSet::Pixelarticons => "crisp pixel art, the retro pick",
            IconSet::DuoIcons => "two-tone and playful; a small set, so many borrow Boxicons",
        }
    }

    /// Licence line for the credits.
    pub fn licence(self) -> &'static str {
        match self {
            IconSet::Solar => "CC BY 4.0",
            _ => "MIT",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Icon {
    Coffee,
    Book,
    Camera,
    Home,
    Work,
    Music,
    Heart,
    Star,
    Plane,
    Food,
    Idea,
    Code,
    Money,
    Gift,
    Leaf,
    Gear,
    Flag,
    Pin,
    Bug,
    Game,
    Beer,
    Cart,
    Car,
    Bell,
    Calendar,
    Envelope,
    Phone,
    Moon,
    Sun,
    Cloud,
    Film,
    Pencil,
    Key,
    Lock,
    Brain,
    Cat,
    Dog,
    Palette,
    Wrench,
    Trophy,
    Rocket,
    Wine,
    Pizza,
    Bank,
    Medal,
    Truck,
    Bag,
    Movie,
    Bookmark,
    Folder,
    User,
    Pram,
    Paint,
    Tree,
    Ship,
    Train,
    Bed,
    Cake,
    Drink,
}

impl Icon {
    pub const ALL: [Icon; 59] = [
        Icon::Coffee,
        Icon::Book,
        Icon::Camera,
        Icon::Home,
        Icon::Work,
        Icon::Music,
        Icon::Heart,
        Icon::Star,
        Icon::Plane,
        Icon::Food,
        Icon::Idea,
        Icon::Code,
        Icon::Money,
        Icon::Gift,
        Icon::Leaf,
        Icon::Gear,
        Icon::Flag,
        Icon::Pin,
        Icon::Bug,
        Icon::Game,
        Icon::Beer,
        Icon::Cart,
        Icon::Car,
        Icon::Bell,
        Icon::Calendar,
        Icon::Envelope,
        Icon::Phone,
        Icon::Moon,
        Icon::Sun,
        Icon::Cloud,
        Icon::Film,
        Icon::Pencil,
        Icon::Key,
        Icon::Lock,
        Icon::Brain,
        Icon::Cat,
        Icon::Dog,
        Icon::Palette,
        Icon::Wrench,
        Icon::Trophy,
        Icon::Rocket,
        Icon::Wine,
        Icon::Pizza,
        Icon::Bank,
        Icon::Medal,
        Icon::Truck,
        Icon::Bag,
        Icon::Movie,
        Icon::Bookmark,
        Icon::Folder,
        Icon::User,
        Icon::Pram,
        Icon::Paint,
        Icon::Tree,
        Icon::Ship,
        Icon::Train,
        Icon::Bed,
        Icon::Cake,
        Icon::Drink,
    ];

    pub fn key(self) -> &'static str {
        match self {
            Icon::Coffee => "coffee",
            Icon::Book => "book",
            Icon::Camera => "camera",
            Icon::Home => "home",
            Icon::Work => "work",
            Icon::Music => "music",
            Icon::Heart => "heart",
            Icon::Star => "star",
            Icon::Plane => "plane",
            Icon::Food => "food",
            Icon::Idea => "idea",
            Icon::Code => "code",
            Icon::Money => "money",
            Icon::Gift => "gift",
            Icon::Leaf => "leaf",
            Icon::Gear => "gear",
            Icon::Flag => "flag",
            Icon::Pin => "pin",
            Icon::Bug => "bug",
            Icon::Game => "game",
            Icon::Beer => "beer",
            Icon::Cart => "cart",
            Icon::Car => "car",
            Icon::Bell => "bell",
            Icon::Calendar => "calendar",
            Icon::Envelope => "envelope",
            Icon::Phone => "phone",
            Icon::Moon => "moon",
            Icon::Sun => "sun",
            Icon::Cloud => "cloud",
            Icon::Film => "film",
            Icon::Pencil => "pencil",
            Icon::Key => "key",
            Icon::Lock => "lock",
            Icon::Brain => "brain",
            Icon::Cat => "cat",
            Icon::Dog => "dog",
            Icon::Palette => "palette",
            Icon::Wrench => "wrench",
            Icon::Trophy => "trophy",
            Icon::Rocket => "rocket",
            Icon::Wine => "wine",
            Icon::Pizza => "pizza",
            Icon::Bank => "bank",
            Icon::Medal => "medal",
            Icon::Truck => "truck",
            Icon::Bag => "bag",
            Icon::Movie => "movie",
            Icon::Bookmark => "bookmark",
            Icon::Folder => "folder",
            Icon::User => "user",
            Icon::Pram => "pram",
            Icon::Paint => "paint",
            Icon::Tree => "tree",
            Icon::Ship => "ship",
            Icon::Train => "train",
            Icon::Bed => "bed",
            Icon::Cake => "cake",
            Icon::Drink => "drink",
        }
    }

    pub fn from_key(key: &str) -> Option<Icon> {
        Icon::ALL.into_iter().find(|i| i.key() == key)
    }

    pub fn label(self) -> &'static str {
        match self {
            Icon::Coffee => "coffee",
            Icon::Book => "book",
            Icon::Camera => "camera",
            Icon::Home => "home",
            Icon::Work => "briefcase",
            Icon::Music => "music",
            Icon::Heart => "heart",
            Icon::Star => "star",
            Icon::Plane => "plane",
            Icon::Food => "food",
            Icon::Idea => "light bulb",
            Icon::Code => "terminal",
            Icon::Money => "money",
            Icon::Gift => "gift",
            Icon::Leaf => "leaf",
            Icon::Gear => "cog",
            Icon::Flag => "flag",
            Icon::Pin => "map pin",
            Icon::Bug => "bug",
            Icon::Game => "game pad",
            Icon::Beer => "beer",
            Icon::Cart => "cart",
            Icon::Car => "car",
            Icon::Bell => "bell",
            Icon::Calendar => "calendar",
            Icon::Envelope => "envelope",
            Icon::Phone => "phone",
            Icon::Moon => "moon",
            Icon::Sun => "sun",
            Icon::Cloud => "cloud",
            Icon::Film => "film",
            Icon::Pencil => "pencil",
            Icon::Key => "key",
            Icon::Lock => "lock",
            Icon::Brain => "brain",
            Icon::Cat => "cat",
            Icon::Dog => "dog",
            Icon::Palette => "palette",
            Icon::Wrench => "wrench",
            Icon::Trophy => "trophy",
            Icon::Rocket => "rocket",
            Icon::Wine => "wine",
            Icon::Pizza => "pizza",
            Icon::Bank => "bank",
            Icon::Medal => "medal",
            Icon::Truck => "truck",
            Icon::Bag => "shopping bag",
            Icon::Movie => "movie camera",
            Icon::Bookmark => "bookmark",
            Icon::Folder => "folder",
            Icon::User => "person",
            Icon::Pram => "pram",
            Icon::Paint => "paint",
            Icon::Tree => "tree",
            Icon::Ship => "ship",
            Icon::Train => "train",
            Icon::Bed => "bed",
            Icon::Cake => "cake",
            Icon::Drink => "drink",
        }
    }

    /// The SVG body (`currentColor`) in `set`; an icon the set lacks falls
    /// back to Boxicons rather than vanishing.
    fn body(self, set: IconSet) -> (&'static str, u32, u32) {
        let own = match set {
            IconSet::Boxicons => Some(self.boxicons()),
            IconSet::Iconoir => self.iconoir(),
            IconSet::Solar => self.solar(),
            IconSet::MynaUi => self.mynaui(),
            IconSet::Majesticons => self.majesticons(),
            IconSet::Pixelarticons => self.pixelarticons(),
            IconSet::DuoIcons => self.duoicons(),
        };
        own.unwrap_or_else(|| self.boxicons())
    }

    fn boxicons(self) -> (&'static str, u32, u32) {
        match self {
            Icon::Coffee => (
                "<path fill=\"currentColor\" d=\"M5 2h2v3H5zm4 0h2v3H9zm4 0h2v3h-2zm6 7h-2V8a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v10a3 3 0 0 0 3 3h8a3 3 0 0 0 3-3h2c1.103 0 2-.897 2-2v-5c0-1.103-.897-2-2-2m-2 7v-5h2l.002 5z\"/>",
                24,
                24,
            ),
            Icon::Book => (
                "<path fill=\"currentColor\" d=\"M6.012 18H21V4a2 2 0 0 0-2-2H6c-1.206 0-3 .799-3 3v14c0 2.201 1.794 3 3 3h15v-2H6.012C5.55 19.988 5 19.805 5 19s.55-.988 1.012-1M8 6h9v2H8z\"/>",
                24,
                24,
            ),
            Icon::Camera => (
                "<path fill=\"currentColor\" d=\"M12 9c-1.626 0-3 1.374-3 3s1.374 3 3 3s3-1.374 3-3s-1.374-3-3-3\"/><path fill=\"currentColor\" d=\"M20 5h-2.586l-2.707-2.707A1 1 0 0 0 14 2h-4a1 1 0 0 0-.707.293L6.586 5H4c-1.103 0-2 .897-2 2v11c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V7c0-1.103-.897-2-2-2m-8 12c-2.71 0-5-2.29-5-5s2.29-5 5-5s5 2.29 5 5s-2.29 5-5 5\"/>",
                24,
                24,
            ),
            Icon::Home => (
                "<path fill=\"currentColor\" d=\"m21.743 12.331l-9-10c-.379-.422-1.107-.422-1.486 0l-9 10a1 1 0 0 0-.17 1.076c.16.361.518.593.913.593h2v7a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-4h4v4a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-7h2a.998.998 0 0 0 .743-1.669\"/>",
                24,
                24,
            ),
            Icon::Work => (
                "<path fill=\"currentColor\" d=\"M20 6h-3V4c0-1.103-.897-2-2-2H9c-1.103 0-2 .897-2 2v2H4c-1.103 0-2 .897-2 2v3h20V8c0-1.103-.897-2-2-2M9 4h6v2H9zm5 10h-4v-2H2v7c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2v-7h-8z\"/>",
                24,
                24,
            ),
            Icon::Music => (
                "<path fill=\"currentColor\" d=\"M6 18.573c2.206 0 4-1.794 4-4V4.428L19 7.7v7.43a3.95 3.95 0 0 0-2-.557c-2.206 0-4 1.794-4 4s1.794 4 4 4s4-1.794 4-4V7a1 1 0 0 0-.658-.939l-11-4A1 1 0 0 0 8 3v8.13a3.95 3.95 0 0 0-2-.557c-2.206 0-4 1.794-4 4s1.794 4 4 4\"/>",
                24,
                24,
            ),
            Icon::Heart => (
                "<path fill=\"currentColor\" d=\"M20.205 4.791a5.94 5.94 0 0 0-4.209-1.754A5.9 5.9 0 0 0 12 4.595a5.9 5.9 0 0 0-3.996-1.558a5.94 5.94 0 0 0-4.213 1.758c-2.353 2.363-2.352 6.059.002 8.412L12 21.414l8.207-8.207c2.354-2.353 2.355-6.049-.002-8.416\"/>",
                24,
                24,
            ),
            Icon::Star => (
                "<path fill=\"currentColor\" d=\"M21.947 9.179a1 1 0 0 0-.868-.676l-5.701-.453l-2.467-5.461a.998.998 0 0 0-1.822-.001L8.622 8.05l-5.701.453a1 1 0 0 0-.619 1.713l4.213 4.107l-1.49 6.452a1 1 0 0 0 1.53 1.057L12 18.202l5.445 3.63a1.001 1.001 0 0 0 1.517-1.106l-1.829-6.4l4.536-4.082c.297-.268.406-.686.278-1.065\"/>",
                24,
                24,
            ),
            Icon::Plane => (
                "<path fill=\"currentColor\" d=\"M3.414 13.778L2 15.192l4.949 2.121l2.122 4.95l1.414-1.414l-.707-3.536L13.091 14l3.61 7.704l1.339-1.339l-1.19-10.123l2.828-2.829a2 2 0 1 0-2.828-2.828l-2.903 2.903L3.824 6.297L2.559 7.563l7.644 3.67l-3.253 3.253z\"/>",
                24,
                24,
            ),
            Icon::Food => (
                "<path fill=\"currentColor\" d=\"M21 10H3a1 1 0 0 0-1 1a10 10 0 0 0 5 8.66V21a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1v-1.34A10 10 0 0 0 22 11a1 1 0 0 0-1-1M9 9V7.93a4.5 4.5 0 0 0-1.28-3.15A2.5 2.5 0 0 1 7 3V2H5v1a4.5 4.5 0 0 0 1.28 3.17A2.5 2.5 0 0 1 7 7.93V9zm4 0V7.93a4.5 4.5 0 0 0-1.28-3.15A2.5 2.5 0 0 1 11 3V2H9v1a4.5 4.5 0 0 0 1.28 3.15A2.5 2.5 0 0 1 11 7.93V9zm4 0V7.93a4.5 4.5 0 0 0-1.28-3.15A2.5 2.5 0 0 1 15 3V2h-2v1a4.5 4.5 0 0 0 1.28 3.15A2.5 2.5 0 0 1 15 7.93V9z\"/>",
                24,
                24,
            ),
            Icon::Idea => (
                "<path fill=\"currentColor\" d=\"M9 20h6v2H9zm7.906-6.288C17.936 12.506 19 11.259 19 9c0-3.859-3.141-7-7-7S5 5.141 5 9c0 2.285 1.067 3.528 2.101 4.73c.358.418.729.851 1.084 1.349c.144.206.38.996.591 1.921h-.792v2h8.032v-2h-.79c.213-.927.45-1.719.593-1.925c.352-.503.726-.94 1.087-1.363\"/>",
                24,
                24,
            ),
            Icon::Code => (
                "<path fill=\"currentColor\" d=\"M20 4H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2M6.414 15.707L5 14.293L7.293 12L5 9.707l1.414-1.414L10.121 12zM19 16h-7v-2h7z\"/>",
                24,
                24,
            ),
            Icon::Money => (
                "<path fill=\"currentColor\" d=\"M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10s10-4.486 10-10S17.514 2 12 2m1 14.915V18h-2v-1.08c-2.339-.367-3-2.002-3-2.92h2c.011.143.159 1 2 1c1.38 0 2-.585 2-1c0-.324 0-1-2-1c-3.48 0-4-1.88-4-3c0-1.288 1.029-2.584 3-2.915V6.012h2v1.109c1.734.41 2.4 1.853 2.4 2.879h-1l-1 .018C13.386 9.638 13.185 9 12 9c-1.299 0-2 .516-2 1c0 .374 0 1 2 1c3.48 0 4 1.88 4 3c0 1.288-1.029 2.584-3 2.915\"/>",
                24,
                24,
            ),
            Icon::Gift => (
                "<path fill=\"currentColor\" d=\"M5 12H4v8a2 2 0 0 0 2 2h5V12zm13 0h-5v10h5a2 2 0 0 0 2-2v-8zm.791-5A5 5 0 0 0 19 5.5C19 3.57 17.43 2 15.5 2c-1.622 0-2.705 1.482-3.404 3.085C11.407 3.57 10.269 2 8.5 2C6.57 2 5 3.57 5 5.5c0 .596.079 1.089.209 1.5H2v4h9V9h2v2h9V7zM7 5.5C7 4.673 7.673 4 8.5 4c.888 0 1.714 1.525 2.198 3H8c-.374 0-1 0-1-1.5M15.5 4c.827 0 1.5.673 1.5 1.5C17 7 16.374 7 16 7h-2.477c.51-1.576 1.251-3 1.977-3\"/>",
                24,
                24,
            ),
            Icon::Leaf => (
                "<path fill=\"currentColor\" d=\"m22 3.41l-.12-1.26l-1.2.4a13.84 13.84 0 0 1-6.41.64a11.87 11.87 0 0 0-6.68.9A7.23 7.23 0 0 0 3.3 9.5a9 9 0 0 0 .39 4.58a16.6 16.6 0 0 1 1.18-2.2a9.85 9.85 0 0 1 4.07-3.43a11.16 11.16 0 0 1 5.06-1A12.1 12.1 0 0 0 9.34 9.2a9.5 9.5 0 0 0-1.86 1.53a11.4 11.4 0 0 0-1.39 1.91a16.4 16.4 0 0 0-1.57 4.54A26.4 26.4 0 0 0 4 22h2a31 31 0 0 1 .59-4.32a9.25 9.25 0 0 0 4.52 1.11a11 11 0 0 0 4.28-.87C23 14.67 22 3.86 22 3.41\"/>",
                24,
                24,
            ),
            Icon::Gear => (
                "<path fill=\"currentColor\" d=\"m2.344 15.271l2 3.46a1 1 0 0 0 1.366.365l1.396-.806c.58.457 1.221.832 1.895 1.112V21a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1v-1.598a8 8 0 0 0 1.895-1.112l1.396.806c.477.275 1.091.11 1.366-.365l2-3.46a1.004 1.004 0 0 0-.365-1.366l-1.372-.793a7.7 7.7 0 0 0-.002-2.224l1.372-.793c.476-.275.641-.89.365-1.366l-2-3.46a1 1 0 0 0-1.366-.365l-1.396.806A8 8 0 0 0 15 4.598V3a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v1.598A8 8 0 0 0 7.105 5.71L5.71 4.904a1 1 0 0 0-1.366.365l-2 3.46a1.004 1.004 0 0 0 .365 1.366l1.372.793a7.7 7.7 0 0 0 0 2.224l-1.372.793c-.476.275-.641.89-.365 1.366M12 8c2.206 0 4 1.794 4 4s-1.794 4-4 4s-4-1.794-4-4s1.794-4 4-4\"/>",
                24,
                24,
            ),
            Icon::Flag => (
                "<path fill=\"currentColor\" d=\"M19 4H6V2H4v18H3v2h4v-2H6v-5h13a1 1 0 0 0 1-1V5a1 1 0 0 0-1-1\"/>",
                24,
                24,
            ),
            Icon::Pin => (
                "<path fill=\"currentColor\" d=\"M12 2C7.589 2 4 5.589 4 9.995C3.971 16.44 11.696 21.784 12 22c0 0 8.029-5.56 8-12c0-4.411-3.589-8-8-8m0 12c-2.21 0-4-1.79-4-4s1.79-4 4-4s4 1.79 4 4s-1.79 4-4 4\"/>",
                24,
                24,
            ),
            Icon::Bug => (
                "<path fill=\"currentColor\" d=\"M6.787 7h10.426c-.108-.158-.201-.331-.318-.481l2.813-2.812l-1.414-1.414l-2.846 2.846a7 7 0 0 0-.723-.454a5.78 5.78 0 0 0-5.45 0c-.25.132-.488.287-.722.453L5.707 2.293L4.293 3.707l2.813 2.812c-.118.151-.21.323-.319.481M5.756 9H2v2h2.307c-.065.495-.107.997-.107 1.5c0 .507.042 1.013.107 1.511H2v2h2.753c.013.039.021.08.034.118c.188.555.421 1.093.695 1.6c.044.081.095.155.141.234l-2.33 2.33l1.414 1.414l2.11-2.111a7.5 7.5 0 0 0 2.068 1.619c.479.253.982.449 1.496.58c.204.052.411.085.618.118V16h2v5.914a6 6 0 0 0 .618-.118a6.8 6.8 0 0 0 1.496-.58c.465-.246.914-.55 1.333-.904c.258-.218.5-.462.734-.716l2.111 2.111l1.414-1.414l-2.33-2.33c.047-.08.098-.155.142-.236c.273-.505.507-1.043.694-1.599c.013-.039.021-.079.034-.118H22v-2h-2.308c.065-.499.107-1.004.107-1.511c0-.503-.042-1.005-.106-1.5H22V9z\"/>",
                24,
                24,
            ),
            Icon::Game => (
                "<path fill=\"currentColor\" d=\"m21.986 9.74l-.008-.088A5.003 5.003 0 0 0 17 5H7a4.97 4.97 0 0 0-4.987 4.737q-.014.117-.013.253v6.51c0 .925.373 1.828 1.022 2.476A3.52 3.52 0 0 0 5.5 20c1.8 0 2.504-1 3.5-3c.146-.292.992-2 3-2c1.996 0 2.853 1.707 3 2c1.004 2 1.7 3 3.5 3c.925 0 1.828-.373 2.476-1.022A3.52 3.52 0 0 0 22 16.5V10q0-.141-.014-.26M7 12.031a2 2 0 1 1-.001-3.999A2 2 0 0 1 7 12.031m10-5a1 1 0 1 1 0 2a1 1 0 1 1 0-2m-2 4a1 1 0 1 1 0-2a1 1 0 1 1 0 2m2 2a1 1 0 1 1 0-2a1 1 0 1 1 0 2m2-2a1 1 0 1 1 0-2a1 1 0 1 1 0 2\"/>",
                24,
                24,
            ),
            Icon::Beer => (
                "<path fill=\"currentColor\" d=\"M20 6h-2V4a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v15c0 1.654 1.346 3 3 3h10c1.654 0 3-1.346 3-3v-1h2c1.103 0 2-.897 2-2V8c0-1.103-.897-2-2-2M8 17H6V7h2zm6 0h-2V7h2zm6-1h-2V8h2z\"/>",
                24,
                24,
            ),
            Icon::Cart => (
                "<path fill=\"currentColor\" d=\"M21.822 7.431A1 1 0 0 0 21 7H7.333L6.179 4.23A1.99 1.99 0 0 0 4.333 3H2v2h2.333l4.744 11.385A1 1 0 0 0 10 17h8c.417 0 .79-.259.937-.648l3-8a1 1 0 0 0-.115-.921\"/><circle cx=\"10.5\" cy=\"19.5\" r=\"1.5\" fill=\"currentColor\"/><circle cx=\"17.5\" cy=\"19.5\" r=\"1.5\" fill=\"currentColor\"/>",
                24,
                24,
            ),
            Icon::Car => (
                "<path fill=\"currentColor\" d=\"m20.772 10.155l-1.368-4.104A2.995 2.995 0 0 0 16.559 4H7.441a2.995 2.995 0 0 0-2.845 2.051l-1.368 4.104A2 2 0 0 0 2 12v5c0 .738.404 1.376 1 1.723V21a1 1 0 0 0 1 1h1a1 1 0 0 0 1-1v-2h12v2a1 1 0 0 0 1 1h1a1 1 0 0 0 1-1v-2.277A1.99 1.99 0 0 0 22 17v-5a2 2 0 0 0-1.228-1.845M7.441 6h9.117c.431 0 .813.274.949.684L18.613 10H5.387l1.105-3.316A1 1 0 0 1 7.441 6M5.5 16a1.5 1.5 0 1 1 .001-3.001A1.5 1.5 0 0 1 5.5 16m13 0a1.5 1.5 0 1 1 .001-3.001A1.5 1.5 0 0 1 18.5 16\"/>",
                24,
                24,
            ),
            Icon::Bell => (
                "<path fill=\"currentColor\" d=\"M12 22a2.98 2.98 0 0 0 2.818-2H9.182A2.98 2.98 0 0 0 12 22m7-7.414V10c0-3.217-2.185-5.927-5.145-6.742C13.562 2.52 12.846 2 12 2s-1.562.52-1.855 1.258C7.185 4.074 5 6.783 5 10v4.586l-1.707 1.707A1 1 0 0 0 3 17v1a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1v-1a1 1 0 0 0-.293-.707z\"/>",
                24,
                24,
            ),
            Icon::Calendar => (
                "<path fill=\"currentColor\" d=\"M21 20V6c0-1.103-.897-2-2-2h-2V2h-2v2H9V2H7v2H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2M9 18H7v-2h2zm0-4H7v-2h2zm4 4h-2v-2h2zm0-4h-2v-2h2zm4 4h-2v-2h2zm0-4h-2v-2h2zm2-5H5V7h14z\"/>",
                24,
                24,
            ),
            Icon::Envelope => (
                "<path fill=\"currentColor\" d=\"M20 4H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2m0 4.7l-8 5.334L4 8.7V6.297l8 5.333l8-5.333z\"/>",
                24,
                24,
            ),
            Icon::Phone => (
                "<path fill=\"currentColor\" d=\"m20.487 17.14l-4.065-3.696a1 1 0 0 0-1.391.043l-2.393 2.461c-.576-.11-1.734-.471-2.926-1.66c-1.192-1.193-1.553-2.354-1.66-2.926l2.459-2.394a1 1 0 0 0 .043-1.391L6.859 3.513a1 1 0 0 0-1.391-.087l-2.17 1.861a1 1 0 0 0-.29.649c-.015.25-.301 6.172 4.291 10.766C11.305 20.707 16.323 21 17.705 21c.202 0 .326-.006.359-.008a1 1 0 0 0 .648-.291l1.86-2.171a.997.997 0 0 0-.085-1.39\"/>",
                24,
                24,
            ),
            Icon::Moon => (
                "<path fill=\"currentColor\" d=\"M12 11.807A9 9 0 0 1 10.049 2a9.94 9.94 0 0 0-5.12 2.735c-3.905 3.905-3.905 10.237 0 14.142c3.906 3.906 10.237 3.905 14.143 0a9.95 9.95 0 0 0 2.735-5.119A9 9 0 0 1 12 11.807\"/>",
                24,
                24,
            ),
            Icon::Sun => (
                "<path fill=\"currentColor\" d=\"M6.995 12c0 2.761 2.246 5.007 5.007 5.007s5.007-2.246 5.007-5.007s-2.246-5.007-5.007-5.007S6.995 9.239 6.995 12M11 19h2v3h-2zm0-17h2v3h-2zm-9 9h3v2H2zm17 0h3v2h-3zM5.637 19.778l-1.414-1.414l2.121-2.121l1.414 1.414zM16.242 6.344l2.122-2.122l1.414 1.414l-2.122 2.122zM6.344 7.759L4.223 5.637l1.415-1.414l2.12 2.122zm13.434 10.605l-1.414 1.414l-2.122-2.122l1.414-1.414z\"/>",
                24,
                24,
            ),
            Icon::Cloud => (
                "<path fill=\"currentColor\" d=\"M18.944 11.112C18.507 7.67 15.56 5 12 5C9.244 5 6.85 6.611 5.757 9.15C3.609 9.792 2 11.82 2 14c0 2.757 2.243 5 5 5h11c2.206 0 4-1.794 4-4a4.01 4.01 0 0 0-3.056-3.888\"/>",
                24,
                24,
            ),
            Icon::Film => (
                "<path fill=\"currentColor\" d=\"M19 4v1h-2V3H7v2H5V3H3v18h2v-2h2v2h10v-2h2v2h2V3h-2zM5 7h2v2H5zm0 4h2v2H5zm0 6v-2h2v2zm12 0v-2h2v2zm2-4h-2v-2h2zm-2-4V7h2v2z\"/>",
                24,
                24,
            ),
            Icon::Pencil => (
                "<path fill=\"currentColor\" d=\"M8.707 19.707L18 10.414L13.586 6l-9.293 9.293a1 1 0 0 0-.263.464L3 21l5.242-1.03c.176-.044.337-.135.465-.263M21 7.414a2 2 0 0 0 0-2.828L19.414 3a2 2 0 0 0-2.828 0L15 4.586L19.414 9z\"/>",
                24,
                24,
            ),
            Icon::Key => (
                "<path fill=\"currentColor\" d=\"M3.433 17.325L3.079 19.8a1 1 0 0 0 1.131 1.131l2.475-.354C7.06 20.524 8 18 8 18s.472.405.665.466c.412.13.813-.274.948-.684L10 16.01s.577.292.786.335c.266.055.524-.109.707-.293a1 1 0 0 0 .241-.391L12 14.01s.675.187.906.214c.263.03.519-.104.707-.293l1.138-1.137a5.5 5.5 0 0 0 5.581-1.338a5.507 5.507 0 0 0 0-7.778a5.507 5.507 0 0 0-7.778 0a5.5 5.5 0 0 0-1.338 5.581l-7.501 7.5a1 1 0 0 0-.282.566M18.504 5.506a2.92 2.92 0 0 1 0 4.122l-4.122-4.122a2.92 2.92 0 0 1 4.122 0\"/>",
                24,
                24,
            ),
            Icon::Lock => (
                "<path fill=\"currentColor\" d=\"M12 2C9.243 2 7 4.243 7 7v3H6a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8a2 2 0 0 0-2-2h-1V7c0-2.757-2.243-5-5-5M9 7c0-1.654 1.346-3 3-3s3 1.346 3 3v3H9zm4 10.723V20h-2v-2.277a1.993 1.993 0 0 1 .567-3.677A2 2 0 0 1 14 16a1.99 1.99 0 0 1-1 1.723\"/>",
                24,
                24,
            ),
            Icon::Brain => (
                "<path fill=\"currentColor\" d=\"M3.299 17.596c.432 1.332 1.745 2.182 3.146 2.182H6.5A2.78 2.78 0 0 0 9.223 22c.457 0 .884-.115 1.262-.313a.99.99 0 0 0 .515-.882V3.027a1 1 0 0 0-.785-.983a2.32 2.32 0 0 0-1.479.201c-.744.356-1.18 1.151-1.18 1.978v.055a2.778 2.778 0 0 0-2.744 4.433A3.33 3.33 0 0 0 2 12c0 1.178.611 2.211 1.533 2.812c-.43.771-.571 1.746-.234 2.784m15.889-8.885a2.778 2.778 0 0 0-2.744-4.433v-.055c0-.826-.437-1.622-1.181-1.978a2.32 2.32 0 0 0-1.478-.201a1 1 0 0 0-.785.983v17.777c0 .365.192.712.516.882c.378.199.804.314 1.261.314a2.78 2.78 0 0 0 2.723-2.223h.056c1.4 0 2.714-.85 3.146-2.182c.337-1.038.196-2.013-.234-2.784A3.35 3.35 0 0 0 22 12a3.33 3.33 0 0 0-2.812-3.289\"/>",
                24,
                24,
            ),
            Icon::Cat => (
                "<path fill=\"currentColor\" d=\"M17 14a5 5 0 0 0 2.71-.81L20 13a3 3 0 0 0 .45-.37l.21-.2a4.5 4.5 0 0 0 .48-.58l.06-.08a4.3 4.3 0 0 0 .41-.76a2 2 0 0 0 .09-.23a4 4 0 0 0 .2-.63l.06-.25A5.5 5.5 0 0 0 22 9V2l-3 3h-4l-3-3v7a5 5 0 0 0 5 5m2-7a1 1 0 1 1-1 1a1 1 0 0 1 1-1m-4 0a1 1 0 1 1-1 1a1 1 0 0 1 1-1\"/><path fill=\"currentColor\" d=\"M11 22v-5H8v5H5V11.9a3.49 3.49 0 0 1-2.48-1.64A3.6 3.6 0 0 1 2 8.5A3.65 3.65 0 0 1 6 5a1.89 1.89 0 0 0 2-2a1 1 0 0 1 1-1a1 1 0 0 1 1 1a3.89 3.89 0 0 1-4 4C4.19 7 4 8.16 4 8.51S4.18 10 6 10h5.09A6 6 0 0 0 19 14.65V22h-3v-5h-2v5z\"/>",
                24,
                24,
            ),
            Icon::Dog => (
                "<path fill=\"currentColor\" d=\"M21 6h-2l-1.27-1.27A2.5 2.5 0 0 0 16 4h-2.5A2.64 2.64 0 0 0 11 2v6.36a4.38 4.38 0 0 0 1.13 2.72a6.57 6.57 0 0 0 4.13 1.82l3.45-1.38a3 3 0 0 0 1.73-1.84L22 8.15a1 1 0 0 0 0-.31V7a1 1 0 0 0-1-1m-5 2a1 1 0 1 1 1-1a1 1 0 0 1-1 1\"/><path fill=\"currentColor\" d=\"M11.38 11.74A5.24 5.24 0 0 1 10.07 9H6a1.88 1.88 0 0 1-2-2a1 1 0 0 0-2 0a4.7 4.7 0 0 0 .48 2A3.6 3.6 0 0 0 4 10.53V22h3v-5h6v5h3v-8.13a7.35 7.35 0 0 1-4.62-2.13\"/>",
                24,
                24,
            ),
            Icon::Palette => (
                "<path fill=\"currentColor\" d=\"M9.38 21.646A10 10 0 0 0 12 22l.141-.001a3 3 0 0 0 2.515-1.425c.542-.876.6-1.953.153-2.88l-.198-.415c-.453-.942-.097-1.796.388-2.281s1.341-.841 2.28-.388h.001l.413.199a2.99 2.99 0 0 0 2.881-.153A3 3 0 0 0 22 12.141a10 10 0 0 0-.353-2.76c-1.038-3.827-4.353-6.754-8.246-7.285c-3.149-.427-6.241.602-8.471 2.833S1.666 10.247 2.096 13.4c.53 3.894 3.458 7.208 7.284 8.246M15.5 6a1.5 1.5 0 1 1 0 3a1.5 1.5 0 0 1 0-3m-5-1a1.5 1.5 0 1 1 0 3a1.5 1.5 0 0 1 0-3M9 15.506a1.5 1.5 0 1 1-3 0a1.5 1.5 0 0 1 3 0m-2.5-6.5a1.5 1.5 0 1 1-.001 3.001A1.5 1.5 0 0 1 6.5 9.006\"/>",
                24,
                24,
            ),
            Icon::Wrench => (
                "<path fill=\"currentColor\" d=\"m21.512 6.112l-3.89 3.889l-3.535-3.536l3.889-3.889a6.501 6.501 0 0 0-8.484 8.486l-6.276 6.275a1 1 0 0 0 0 1.414l2.122 2.122a1 1 0 0 0 1.414 0l6.275-6.276a6.5 6.5 0 0 0 7.071-1.414a6.5 6.5 0 0 0 1.414-7.071\"/>",
                24,
                24,
            ),
            Icon::Trophy => (
                "<path fill=\"currentColor\" d=\"M21 4h-3V3a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1v1H3a1 1 0 0 0-1 1v3c0 4.31 1.8 6.91 4.82 7A6 6 0 0 0 11 17.91V20H9v2h6v-2h-2v-2.09A6 6 0 0 0 17.18 15c3-.1 4.82-2.7 4.82-7V5a1 1 0 0 0-1-1M4 8V6h2v6.83C4.22 12.08 4 9.3 4 8m14 4.83V6h2v2c0 1.3-.22 4.08-2 4.83\"/>",
                24,
                24,
            ),
            Icon::Rocket => (
                "<path fill=\"currentColor\" d=\"M15.78 15.84S18.64 13 19.61 12c3.07-3 1.54-9.18 1.54-9.18S15 1.29 12 4.36C9.66 6.64 8.14 8.22 8.14 8.22S4.3 7.42 2 9.72L14.25 22c2.3-2.33 1.53-6.16 1.53-6.16m-1.5-9a2 2 0 0 1 2.83 0a2 2 0 1 1-2.83 0M3 21a7.8 7.8 0 0 0 5-2l-3-3c-2 1-2 5-2 5\"/>",
                24,
                24,
            ),
            Icon::Wine => (
                "<path fill=\"currentColor\" d=\"M11 17.916V20H9v2h6v-2h-2v-2.084c3.162-.402 5.849-2.66 6.713-5.793c.264-.952.312-2.03.143-3.206l-.866-6.059A1 1 0 0 0 18 2H6a1 1 0 0 0-.99.858l-.865 6.058c-.169 1.177-.121 2.255.143 3.206c.863 3.134 3.55 5.392 6.712 5.794M17.133 4l.57 4H6.296l.571-4z\"/>",
                24,
                24,
            ),
            Icon::Pizza => (
                "<path fill=\"currentColor\" d=\"M9.76 2.021a.995.995 0 0 0-.989.703L3.579 19.166a1 1 0 0 0 1.255 1.255l16.442-5.192a.99.99 0 0 0 .702-.988C21.6 7.666 16.334 2.4 9.76 2.021M10 16a2 2 0 1 1 .001-4.001A2 2 0 0 1 10 16m6-2a2 2 0 1 1 .001-4.001A2 2 0 0 1 16 14\"/>",
                24,
                24,
            ),
            Icon::Bank => (
                "<path fill=\"currentColor\" d=\"M2 8v4.001h1V18H2v3h16l3 .001V21h1v-3h-1v-5.999h1V8L12 2zm4 10v-5.999h2V18zm5 0v-5.999h2V18zm7 0h-2v-5.999h2zM14 8a2 2 0 1 1-4.001-.001A2 2 0 0 1 14 8\"/>",
                24,
                24,
            ),
            Icon::Medal => (
                "<path fill=\"currentColor\" d=\"M17 2h-4v4.059a8.95 8.95 0 0 1 4 1.459zm-6 0H7v5.518a8.95 8.95 0 0 1 4-1.459zm1 20a7 7 0 1 0 0-14a7 7 0 0 0 0 14m-1.225-8.519L12 11l1.225 2.481l2.738.397l-1.981 1.932l.468 2.727L12 17.25l-2.449 1.287l.468-2.727l-1.981-1.932z\"/>",
                24,
                24,
            ),
            Icon::Truck => (
                "<path fill=\"currentColor\" d=\"M19.15 8a2 2 0 0 0-1.72-1H15V5a1 1 0 0 0-1-1H4a2 2 0 0 0-2 2v10a2 2 0 0 0 1 1.73a3.49 3.49 0 0 0 7 .27h3.1a3.48 3.48 0 0 0 6.9 0a2 2 0 0 0 2-2v-3a1.1 1.1 0 0 0-.14-.52zM15 9h2.43l1.8 3H15zM6.5 19A1.5 1.5 0 1 1 8 17.5A1.5 1.5 0 0 1 6.5 19m10 0a1.5 1.5 0 1 1 1.5-1.5a1.5 1.5 0 0 1-1.5 1.5\"/>",
                24,
                24,
            ),
            Icon::Bag => (
                "<path fill=\"currentColor\" d=\"M5 22h14a2 2 0 0 0 2-2V9a1 1 0 0 0-1-1h-3v-.777c0-2.609-1.903-4.945-4.5-5.198A5.005 5.005 0 0 0 7 7v1H4a1 1 0 0 0-1 1v11a2 2 0 0 0 2 2m12-12v2h-2v-2zM9 7c0-1.654 1.346-3 3-3s3 1.346 3 3v1H9zm-2 3h2v2H7z\"/>",
                24,
                24,
            ),
            Icon::Movie => (
                "<path fill=\"currentColor\" d=\"M18 11c0-.959-.68-1.761-1.581-1.954C16.779 8.445 17 7.75 17 7c0-2.206-1.794-4-4-4c-1.516 0-2.822.857-3.5 2.104C8.822 3.857 7.516 3 6 3C3.794 3 2 4.794 2 7c0 .902.312 1.726.817 2.396A2 2 0 0 0 2 11v8c0 1.103.897 2 2 2h12c1.103 0 2-.897 2-2v-2.637l4 2v-7l-4 2zm-5-6c1.103 0 2 .897 2 2s-.897 2-2 2s-2-.897-2-2s.897-2 2-2M6 5c1.103 0 2 .897 2 2s-.897 2-2 2s-2-.897-2-2s.897-2 2-2\"/>",
                24,
                24,
            ),
            Icon::Bookmark => (
                "<path fill=\"currentColor\" d=\"M19 10.132v-6c0-1.103-.897-2-2-2H7c-1.103 0-2 .897-2 2V22l7-4.666L19 22z\"/>",
                24,
                24,
            ),
            Icon::Folder => (
                "<path fill=\"currentColor\" d=\"M20 5h-9.586L8.707 3.293A1 1 0 0 0 8 3H4c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V7c0-1.103-.897-2-2-2\"/>",
                24,
                24,
            ),
            Icon::User => (
                "<path fill=\"currentColor\" d=\"M7.5 6.5C7.5 8.981 9.519 11 12 11s4.5-2.019 4.5-4.5S14.481 2 12 2S7.5 4.019 7.5 6.5M20 21h1v-1c0-3.859-3.141-7-7-7h-4c-3.86 0-7 3.141-7 7v1z\"/>",
                24,
                24,
            ),
            Icon::Pram => (
                "<path fill=\"currentColor\" d=\"M21.666 12.277a8 8 0 0 0 .171-.665l.008-.05c.02-.098.029-.199.045-.298c.025-.157.055-.313.07-.471a7.98 7.98 0 0 0-2.303-6.45A7.98 7.98 0 0 0 14 2v8H6.517l-.858-2H2v2h2.341l1.828 4.266A3.5 3.5 0 0 0 4 17.5C4 19.43 5.57 21 7.5 21c1.759 0 3.204-1.309 3.449-3h2.102c.245 1.691 1.69 3 3.449 3c1.93 0 3.5-1.57 3.5-3.5c0-.63-.181-1.213-.473-1.725c.042-.041.089-.077.131-.119c.36-.361.688-.759.977-1.184c.288-.43.536-.886.736-1.359c.016-.037.026-.076.041-.113h.001l.015-.042q.133-.329.235-.668zM7.5 19c-.827 0-1.5-.673-1.5-1.5S6.673 16 7.5 16s1.5.673 1.5 1.5S8.327 19 7.5 19m9 0c-.827 0-1.5-.673-1.5-1.5s.673-1.5 1.5-1.5s1.5.673 1.5 1.5s-.673 1.5-1.5 1.5\"/>",
                24,
                24,
            ),
            Icon::Paint => (
                "<path fill=\"currentColor\" d=\"M21.084 2.914c-1.178-1.179-3.234-1.179-4.412 0l-8.379 8.379a1 1 0 0 0 0 1.414l3 3a.997.997 0 0 0 1.414 0l8.379-8.379a3.123 3.123 0 0 0-.002-4.414m-1.412 3L12 13.586L10.414 12l7.672-7.672a1.146 1.146 0 0 1 1.586.002a1.123 1.123 0 0 1 0 1.584M8 15c-1.265-.634-3.5 0-3.5 2c0 1.197.5 2-1.5 3c0 0 3.25 2.25 5.5 0c1.274-1.274 1.494-4-.5-5\"/>",
                24,
                24,
            ),
            Icon::Tree => (
                "<path fill=\"currentColor\" d=\"m20 18l-4-5h3l-4-5h2l-5-6l-5 6h2l-4 5h3l-4 5h7v4h2v-4z\"/>",
                24,
                24,
            ),
            Icon::Ship => (
                "<path fill=\"currentColor\" d=\"M16.997 20c-.899 0-1.288-.311-1.876-.781c-.68-.543-1.525-1.219-3.127-1.219s-2.446.676-3.125 1.22c-.587.469-.975.78-1.874.78c-.897 0-1.285-.311-1.872-.78C4.444 18.676 3.601 18 2 18v2c.898 0 1.286.311 1.873.78c.679.544 1.523 1.22 3.122 1.22c1.601 0 2.445-.676 3.124-1.219c.588-.47.976-.781 1.875-.781c.9 0 1.311.328 1.878.781c.679.543 1.524 1.219 3.125 1.219s2.446-.676 3.125-1.219C20.689 20.328 21.1 20 22 20v-2c-1.602 0-2.447.676-3.127 1.219c-.588.47-.977.781-1.876.781M6 8.5L4 9l2 8h.995c1.601 0 2.445-.676 3.124-1.219c.588-.47.976-.781 1.875-.781c.9 0 1.311.328 1.878.781c.679.543 1.524 1.219 3.125 1.219H18l.027-.107l.313-1.252L20 9l-2-.5V5.001a1 1 0 0 0-.804-.981L13 3.181V2h-2v1.181l-4.196.839A1 1 0 0 0 6 5.001zm2-2.681l4-.8l4 .8V8l-4-1l-4 1z\"/>",
                24,
                24,
            ),
            Icon::Train => (
                "<path fill=\"currentColor\" d=\"M16.375 2H7.621c-.224 0-1.399.065-2.503 1.351C4.031 4.616 4 5.862 4 6v11a2 2 0 0 0 2 2h1l-2 3h2.353l.667-1h8l.677 1H19l-2-3h1a2 2 0 0 0 2-2V6c.001-.188-.032-1.434-1.129-2.665C17.715 2.037 16.509 2 16.375 2M10 4h4v2h-4zM7.5 17a1.5 1.5 0 1 1 .001-3.001A1.5 1.5 0 0 1 7.5 17m9 0a1.5 1.5 0 1 1 .001-3.001A1.5 1.5 0 0 1 16.5 17m1.5-5H6V8h12z\"/>",
                24,
                24,
            ),
            Icon::Bed => (
                "<path fill=\"currentColor\" d=\"M20 9.556V3h-2v2H6V3H4v6.557C2.81 10.25 2 11.526 2 13v4a1 1 0 0 0 1 1h1v4h2v-4h12v4h2v-4h1a1 1 0 0 0 1-1v-4c0-1.474-.811-2.75-2-3.444M11 9H6V7h5zm7 0h-5V7h5z\"/>",
                24,
                24,
            ),
            Icon::Cake => (
                "<path fill=\"currentColor\" d=\"M16.997 15c-1.601 0-2.446-.676-3.125-1.219c-.567-.453-.977-.781-1.878-.781c-.898 0-1.287.311-1.874.78c-.679.544-1.524 1.22-3.125 1.22s-2.444-.676-3.123-1.22C3.285 13.311 2.897 13 2 13v5c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2v-5c-.899 0-1.288.311-1.876.781c-.68.543-1.525 1.219-3.127 1.219M19 5h-6V2h-2v3H5C3.346 5 2 6.346 2 8v3c1.6 0 2.443.676 3.122 1.22c.587.469.975.78 1.873.78c.899 0 1.287-.311 1.875-.781c.679-.543 1.524-1.219 3.124-1.219c1.602 0 2.447.676 3.127 1.219c.588.47.977.781 1.876.781c.9 0 1.311-.328 1.878-.781C19.554 11.676 20.399 11 22 11V8c0-1.654-1.346-3-3-3\"/>",
                24,
                24,
            ),
            Icon::Drink => (
                "<path fill=\"currentColor\" d=\"M20.832 4.555A1 1 0 0 0 20 3H4a1 1 0 0 0-.832 1.554L11 16.303V20H8v2h8v-2h-3v-3.697zm-2.7.445l-2 3H7.868l-2-3z\"/>",
                24,
                24,
            ),
        }
    }

    fn iconoir(self) -> Option<(&'static str, u32, u32)> {
        match self {
            Icon::Coffee => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\"><path d=\"M17 11.6V15a6 6 0 0 1-6 6H9a6 6 0 0 1-6-6v-3.4a.6.6 0 0 1 .6-.6h12.8a.6.6 0 0 1 .6.6M12 9c0-1 .714-2 2.143-2v0A2.857 2.857 0 0 0 17 4.143V3.5M8 9v-.5a3 3 0 0 1 3-3v0a2 2 0 0 0 2-2V3\"/><path d=\"M16 11h2.5a2.5 2.5 0 0 1 0 5H17\"/></g>",
                24,
                24,
            )),
            Icon::Book => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-width=\"1.5\"><path d=\"M4 19V5a2 2 0 0 1 2-2h13.4a.6.6 0 0 1 .6.6v13.114M6 17h14M6 21h14\"/><path stroke-linejoin=\"round\" d=\"M6 21a2 2 0 1 1 0-4\"/><path d=\"M9 7h6\"/></g>",
                24,
                24,
            )),
            Icon::Camera => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\"><path d=\"M2 19V9a2 2 0 0 1 2-2h.5a2 2 0 0 0 1.6-.8l2.22-2.96A.6.6 0 0 1 8.8 3h6.4a.6.6 0 0 1 .48.24L17.9 6.2a2 2 0 0 0 1.6.8h.5a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2\"/><path d=\"M12 17a4 4 0 1 0 0-8a4 4 0 0 0 0 8\"/></g>",
                24,
                24,
            )),
            Icon::Home => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"m2 8l9.732-4.866a.6.6 0 0 1 .536 0L22 8m-2 3v8a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-8\"/>",
                24,
                24,
            )),
            Icon::Work => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.5\" d=\"M8 7H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-4M8 7V3.6a.6.6 0 0 1 .6-.6h6.8a.6.6 0 0 1 .6.6V7M8 7h8\"/>",
                24,
                24,
            )),
            Icon::Music => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M12 16v3a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2v-1a2 2 0 0 1 2-2zm0 0V8m0 0V4l5-1v4z\"/>",
                24,
                24,
            )),
            Icon::Heart => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M22 8.862a5.95 5.95 0 0 1-1.654 4.13c-2.441 2.531-4.809 5.17-7.34 7.608c-.581.55-1.502.53-2.057-.045l-7.295-7.562c-2.205-2.286-2.205-5.976 0-8.261a5.58 5.58 0 0 1 8.08 0l.266.274l.265-.274A5.6 5.6 0 0 1 16.305 3c1.52 0 2.973.624 4.04 1.732A5.95 5.95 0 0 1 22 8.862Z\"/>",
                24,
                24,
            )),
            Icon::Star => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"m8.587 8.236l2.598-5.232a.911.911 0 0 1 1.63 0l2.598 5.232l5.808.844a.902.902 0 0 1 .503 1.542l-4.202 4.07l.992 5.75c.127.738-.653 1.3-1.32.952L12 18.678l-5.195 2.716c-.666.349-1.446-.214-1.319-.953l.992-5.75l-4.202-4.07a.902.902 0 0 1 .503-1.54z\"/>",
                24,
                24,
            )),
            Icon::Plane => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M10.5 4.5v4.667a.6.6 0 0 1-.282.51l-7.436 4.647a.6.6 0 0 0-.282.508v.9a.6.6 0 0 0 .746.582l6.508-1.628a.6.6 0 0 1 .746.582v2.96a.6.6 0 0 1-.205.451l-2.16 1.89c-.458.402-.097 1.151.502 1.042l3.256-.591a.6.6 0 0 1 .214 0l3.256.591c.599.11.96-.64.502-1.041l-2.16-1.89a.6.6 0 0 1-.205-.452v-2.96a.6.6 0 0 1 .745-.582l6.51 1.628a.6.6 0 0 0 .745-.582v-.9a.6.6 0 0 0-.282-.508l-7.436-4.648a.6.6 0 0 1-.282-.509V4.5a1.5 1.5 0 0 0-3 0\"/>",
                24,
                24,
            )),
            Icon::Food => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M6 20h3m3 0H9m0 0v-5m8 5v-8s2.5-1 2.5-3V4.5m-2.5 4v-4M4.5 11c1 2.128 4.5 4 4.5 4s3.5-1.872 4.5-4c1.08-2.297 0-6.5 0-6.5h-9s-1.08 4.203 0 6.5\"/>",
                24,
                24,
            )),
            Icon::Idea => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M9 18h6m-5 3h4m-5-6c.001-2-.499-2.5-1.5-3.5S6.025 9.487 6 8c-.047-3.05 2-5 6-5c4.001 0 6.049 1.95 6 5c-.023 1.487-.5 2.5-1.5 3.5c-.999 1-1.499 1.5-1.5 3.5\"/>",
                24,
                24,
            )),
            Icon::Code => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M13 17h7M5 7l5 5l-5 5\"/>",
                24,
                24,
            )),
            Icon::Money => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\"><path d=\"M16 13c-2.761 0-5-1.12-5-2.5S13.239 8 16 8s5 1.12 5 2.5s-2.239 2.5-5 2.5m-5 1.5c0 1.38 2.239 2.5 5 2.5s5-1.12 5-2.5m-18-5C3 10.88 5.239 12 8 12c1.126 0 2.165-.186 3-.5M3 13c0 1.38 2.239 2.5 5 2.5c1.126 0 2.164-.186 3-.5\"/><path d=\"M3 5.5v11C3 17.88 5.239 19 8 19c1.126 0 2.164-.186 3-.5m2-10v-3m-2 5v8c0 1.38 2.239 2.5 5 2.5s5-1.12 5-2.5v-8\"/><path d=\"M8 8C5.239 8 3 6.88 3 5.5S5.239 3 8 3s5 1.12 5 2.5S10.761 8 8 8\"/></g>",
                24,
                24,
            )),
            Icon::Gift => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M20 12v9.4a.6.6 0 0 1-.6.6H4.6a.6.6 0 0 1-.6-.6V12m17.4-5H2.6a.6.6 0 0 0-.6.6v3.8a.6.6 0 0 0 .6.6h18.8a.6.6 0 0 0 .6-.6V7.6a.6.6 0 0 0-.6-.6M12 22V7m0 0H7.5a2.5 2.5 0 1 1 0-5C11 2 12 7 12 7m0 0h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7\"/>",
                24,
                24,
            )),
            Icon::Leaf => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\"><path d=\"M7 21s.5-4.5 4-8.5\"/><path d=\"m19.13 4.242l.594 6.175c.374 3.886-2.54 7.346-6.425 7.72c-3.813.367-7.267-2.42-7.634-6.233a6.936 6.936 0 0 1 6.239-7.569l6.571-.632a.6.6 0 0 1 .655.54\"/></g>",
                24,
                24,
            )),
            Icon::Gear => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\"><path d=\"M12 15a3 3 0 1 0 0-6a3 3 0 0 0 0 6\"/><path d=\"m19.622 10.395l-1.097-2.65L20 6l-2-2l-1.735 1.483l-2.707-1.113L12.935 2h-1.954l-.632 2.401l-2.645 1.115L6 4L4 6l1.453 1.789l-1.08 2.657L2 11v2l2.401.656L5.516 16.3L4 18l2 2l1.791-1.46l2.606 1.072L11 22h2l.604-2.387l2.651-1.098C16.697 18.832 18 20 18 20l2-2l-1.484-1.75l1.098-2.652l2.386-.62V11z\"/></g>",
                24,
                24,
            )),
            Icon::Flag => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M8 21v-5m0 0V3.577a.6.6 0 0 1 .916-.51l8.79 5.442a.6.6 0 0 1 .017 1.009z\"/>",
                24,
                24,
            )),
            Icon::Pin => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.5\"><path d=\"M20 10c0 4.418-8 12-8 12s-8-7.582-8-12a8 8 0 1 1 16 0Z\"/><path fill=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M12 11a1 1 0 1 0 0-2a1 1 0 0 0 0 2\"/></g>",
                24,
                24,
            )),
            Icon::Bug => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\"><path d=\"M12 21c-3.866 0-7-4.03-7-9s3.134-9 7-9s7 4.03 7 9s-3.134 9-7 9m6-3.5l2 2m-1-10l2-1m-16 1l-2-1\"/><path d=\"M18 8s-3 1-6 1M6 8s3 1 6 1m0 0v12m-7-7H2m20 0h-3M6 17.5l-2 2\"/></g>",
                24,
                24,
            )),
            Icon::Game => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\"><path d=\"M17.5 17.5c2.5 3.5 6.449.915 5.5-2.5c-1.425-5.129-2.2-7.984-2.603-9.492A2.03 2.03 0 0 0 18.438 4H5.562c-.918 0-1.718.625-1.941 1.515C2.78 8.863 2.033 11.802 1.144 15c-.948 3.415 3 6 5.5 2.5M18 8.5l.011.01M16.49 7l.011.01M16.49 10l.011.01M15 8.5l.011.01M7 7v3M5.5 8.5h3\"/><path d=\"M8 16a2 2 0 1 0 0-4a2 2 0 0 0 0 4m8 0a2 2 0 1 0 0-4a2 2 0 0 0 0 4\"/></g>",
                24,
                24,
            )),
            Icon::Beer => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.5\"><path d=\"M3.04 4.294a.5.5 0 0 1 .191-.479C3.927 3.32 6.314 2 12 2s8.073 1.32 8.769 1.815a.5.5 0 0 1 .192.479l-1.7 12.744a4 4 0 0 1-1.98 2.944l-.32.183a10 10 0 0 1-9.922 0l-.32-.183a4 4 0 0 1-1.98-2.944z\"/><path d=\"M3 5c2.571 2.667 15.429 2.667 18 0M4 13c1.032 1.203 3.925 1.864 7 1.981a25.4 25.4 0 0 0 4-.158c2.266-.279 4.197-.886 5-1.823M4 13c2.286-2.667 13.714-2.667 16 0\"/></g>",
                24,
                24,
            )),
            Icon::Cart => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\"><path fill=\"currentColor\" d=\"M19.5 22a1.5 1.5 0 1 0 0-3a1.5 1.5 0 0 0 0 3m-10 0a1.5 1.5 0 1 0 0-3a1.5 1.5 0 0 0 0 3\"/><path d=\"M5 4h17l-2 11H7zm0 0c-.167-.667-1-2-3-2m18 13H5.23c-1.784 0-2.73.781-2.73 2s.946 2 2.73 2H19.5\"/></g>",
                24,
                24,
            )),
            Icon::Car => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.5\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M8 10h8m-9 4h1m8 0h1\"/><path d=\"M3 18v-6.59a2 2 0 0 1 .162-.787l2.319-5.41A2 2 0 0 1 7.319 4h9.362a2 2 0 0 1 1.838 1.212l2.32 5.41a2 2 0 0 1 .161.789V18M3 18v2.4a.6.6 0 0 0 .6.6h2.8a.6.6 0 0 0 .6-.6V18m-4 0h4m14 0v2.4a.6.6 0 0 1-.6.6h-2.8a.6.6 0 0 1-.6-.6V18m4 0h-4M7 18h10\"/></g>",
                24,
                24,
            )),
            Icon::Bell => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M18 8.4c0-1.697-.632-3.325-1.757-4.525S13.59 2 12 2s-3.117.674-4.243 1.875C6.632 5.075 6 6.703 6 8.4C6 15.867 3 18 3 18h18s-3-2.133-3-9.6M13.73 21a2 2 0 0 1-3.46 0\"/>",
                24,
                24,
            )),
            Icon::Calendar => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M15 4V2m0 2v2m0-2h-4.5M3 10v9a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-9zm0 0V6a2 2 0 0 1 2-2h2m0-2v4m14 4V6a2 2 0 0 0-2-2h-.5\"/>",
                24,
                24,
            )),
            Icon::Envelope => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.5\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m7 9l5 3.5L17 9\"/><path d=\"M2 17V7a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2Z\"/></g>",
                24,
                24,
            )),
            Icon::Phone => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M18.118 14.702L14 15.5c-2.782-1.396-4.5-3-5.5-5.5l.77-4.13L7.815 2H4.064c-1.128 0-2.016.932-1.847 2.047c.42 2.783 1.66 7.83 5.283 11.453c3.805 3.805 9.286 5.456 12.302 6.113c1.165.253 2.198-.655 2.198-1.848v-3.584z\"/>",
                24,
                24,
            )),
            Icon::Moon => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M3 11.507a9.493 9.493 0 0 0 18 4.219c-8.507 0-12.726-4.22-12.726-12.726A9.49 9.49 0 0 0 3 11.507\"/>",
                24,
                24,
            )),
            Icon::Sun => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M12 18a6 6 0 1 0 0-12a6 6 0 0 0 0 12m10-6h1M12 2V1m0 22v-1m8-2l-1-1m1-15l-1 1M4 20l1-1M4 4l1 1m-4 7h1\"/>",
                24,
                24,
            )),
            Icon::Cloud => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M12 4c-6 0-6 4-6 6c-1.667 0-5 1-5 5s3.333 5 5 5h12c1.667 0 5-1 5-5s-3.333-5-5-5c0-2 0-6-6-6Z\"/>",
                24,
                24,
            )),
            Icon::Film => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\"><path d=\"M7 14a2 2 0 1 0 0-4a2 2 0 0 0 0 4m10 0a2 2 0 1 0 0-4a2 2 0 0 0 0 4m-5-5a2 2 0 1 0 0-4a2 2 0 0 0 0 4m0 10a2 2 0 1 0 0-4a2 2 0 0 0 0 4\"/><path d=\"M2 12c0 5.523 4.477 10 10 10s10-4.477 10-10S17.523 2 12 2S2 6.477 2 12m0 0v10\"/></g>",
                24,
                24,
            )),
            Icon::Pencil => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"m14.363 5.652l1.48-1.48a2 2 0 0 1 2.829 0l1.414 1.414a2 2 0 0 1 0 2.828l-1.48 1.48m-4.243-4.242l-9.616 9.615a2 2 0 0 0-.578 1.238l-.242 2.74a1 1 0 0 0 1.084 1.085l2.74-.242a2 2 0 0 0 1.24-.578l9.615-9.616m-4.243-4.242l4.243 4.242\"/>",
                24,
                24,
            )),
            Icon::Key => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M10 12a4 4 0 1 1-8 0a4 4 0 0 1 8 0m0 0h12v3m-4-3v3\"/>",
                24,
                24,
            )),
            Icon::Lock => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M16 12h1.4a.6.6 0 0 1 .6.6v6.8a.6.6 0 0 1-.6.6H6.6a.6.6 0 0 1-.6-.6v-6.8a.6.6 0 0 1 .6-.6H8m8 0V8c0-1.333-.8-4-4-4S8 6.667 8 8v4m8 0H8\"/>",
                24,
                24,
            )),
            Icon::Brain => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\"><path d=\"M7 14a3 3 0 1 0 1 5.83\"/><path d=\"M4.264 15.605a4 4 0 0 1-.874-6.636m.03-.081A2.5 2.5 0 0 1 7 5.5m.238.065A2.5 2.5 0 1 1 12 4.5V20m-4 0a2 2 0 1 0 4 0m0-13a3 3 0 0 0 3 3m2 4a3 3 0 1 1-1 5.83\"/><path d=\"M19.736 15.605a4 4 0 0 0 .874-6.636m-.03-.081A2.5 2.5 0 0 0 17 5.5m-5-1a2.5 2.5 0 1 1 4.762 1.065M16 20a2 2 0 1 1-4 0\"/></g>",
                24,
                24,
            )),
            Icon::Cat => None,
            Icon::Dog => None,
            Icon::Palette => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.5\"><path d=\"M20.51 9.54a1.9 1.9 0 0 1-1 1.09A7 7 0 0 0 15.37 17q.002.707.14 1.4a2.16 2.16 0 0 1-.31 1.65a1.8 1.8 0 0 1-1.21.8q-.804.15-1.62.15a9 9 0 0 1-9-9.28A9.05 9.05 0 0 1 11.85 3h.51a9 9 0 0 1 8.06 5a2 2 0 0 1 .09 1.52z\"/><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m8 16.01l.01-.011M6 12.01l.01-.011M8 8.01l.01-.011M12 6.01l.01-.011M16 8.01l.01-.011\"/></g>",
                24,
                24,
            )),
            Icon::Wrench => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"m10.05 10.607l-7.07 7.07a2 2 0 0 0 0 2.83v0a2 2 0 0 0 2.828 0l7.07-7.072m-2.828-2.828c-.844-2.153-.679-4.978 1.06-6.718s4.95-2.121 6.718-1.06l-3.04 3.04l-.283 3.111l3.111-.282l3.04-3.041c1.062 1.768.68 4.978-1.06 6.717c-1.74 1.74-4.564 1.905-6.717 1.061\"/>",
                24,
                24,
            )),
            Icon::Trophy => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\"><path d=\"M6.745 4h10.568s-.88 13.257-5.284 13.257c-2.15 0-3.461-3.164-4.239-6.4C6.976 7.468 6.745 4 6.745 4\"/><path d=\"M17.313 4s.921-.983 1.687-1c1.5-.034 1.777 1 1.777 1c.294.61.529 2.194-.88 3.657s-2.987 2.743-3.629 3.2M6.745 4S5.785 3.006 5 3c-1.5-.012-1.777 1-1.777 1c-.294.61-.529 2.194.88 3.657a30 30 0 0 0 3.687 3.2M8.507 20c0-1.829 3.522-2.743 3.522-2.743s3.523.914 3.523 2.743z\"/></g>",
                24,
                24,
            )),
            Icon::Rocket => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\"><path d=\"M16.061 10.404L14 17h-4l-2.061-6.596a6 6 0 0 1 .998-5.484l2.59-3.315a.6.6 0 0 1 .946 0l2.59 3.315a6 6 0 0 1 .998 5.484M10 20c0 2 2 3 2 3s2-1 2-3m-5.5-7.5C5 15 7 19 7 19l3-2m5.931-4.5c3.5 2.5 1.5 6.5 1.5 6.5l-3-2\"/><path d=\"M12 11a2 2 0 1 1 0-4a2 2 0 0 1 0 4\"/></g>",
                24,
                24,
            )),
            Icon::Wine => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\"><path d=\"M8 22h4m4 0h-4m0 0v-7m-5.422-4.952C7.783 12.682 12 15 12 15s4.217-2.318 5.422-4.952c1.3-2.845 0-8.048 0-8.048H6.578s-1.3 5.203 0 8.048\"/><path d=\"m12.5 2l-2 4h3l-2 4\"/></g>",
                24,
                24,
            )),
            Icon::Pizza => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.5\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m14 9.01l.01-.011M8 8.01l.01-.011M8 14.01l.01-.011\"/><path d=\"M6 19L2.236 3.004a.6.6 0 0 1 .754-.713L19 7\"/><path stroke-linecap=\"round\" d=\"M22.198 8.425a1.75 1.75 0 0 0-3.396-.85c-.391 1.568-1.9 4.05-4.227 6.375c-2.3 2.301-5.148 4.194-7.968 4.845a1.75 1.75 0 1 0 .787 3.41c3.68-.849 7.082-3.206 9.656-5.78c2.549-2.549 4.54-5.568 5.148-8Z\"/></g>",
                24,
                24,
            )),
            Icon::Bank => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M3 9.5L12 4l9 5.5M5 20h14M10 9h4m-8 8v-5m4 5v-5m4 5v-5m4 5v-5\"/>",
                24,
                24,
            )),
            Icon::Medal => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M14.272 10.445L18 2m-8.684 8.632L5 2m7.762 8.048L8.835 2m5.525 0l-1.04 2.5M6 16a6 6 0 1 0 12 0a6 6 0 0 0-12 0\"/>",
                24,
                24,
            )),
            Icon::Truck => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-width=\"1.5\"><path stroke-linejoin=\"round\" stroke-miterlimit=\"1.5\" d=\"M7 19a2 2 0 1 0 0-4a2 2 0 0 0 0 4m10 0a2 2 0 1 0 0-4a2 2 0 0 0 0 4\"/><path d=\"M14 17V6.6a.6.6 0 0 0-.6-.6H2.6a.6.6 0 0 0-.6.6v9.8a.6.6 0 0 0 .6.6h2.05M14 17H9.05M14 9h5.61a.6.6 0 0 1 .548.356l1.79 4.028a.6.6 0 0 1 .052.243V16.4a.6.6 0 0 1-.6.6h-1.9M14 17h1\"/></g>",
                24,
                24,
            )),
            Icon::Bag => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"m19.26 9.696l1.385 9A2 2 0 0 1 18.67 21H5.33a2 2 0 0 1-1.977-2.304l1.385-9A2 2 0 0 1 6.716 8h10.568a2 2 0 0 1 1.977 1.696M14 5a2 2 0 1 0-4 0\"/>",
                24,
                24,
            )),
            Icon::Movie => None,
            Icon::Bookmark => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M5 21V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16l-5.918-3.805a2 2 0 0 0-2.164 0z\"/>",
                24,
                24,
            )),
            Icon::Folder => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M2 11V4.6a.6.6 0 0 1 .6-.6h6.178a.6.6 0 0 1 .39.144l3.164 2.712a.6.6 0 0 0 .39.144H21.4a.6.6 0 0 1 .6.6V11M2 11v8.4a.6.6 0 0 0 .6.6h18.8a.6.6 0 0 0 .6-.6V11M2 11h20\"/>",
                24,
                24,
            )),
            Icon::User => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M5 20v-1a7 7 0 0 1 7-7v0a7 7 0 0 1 7 7v1m-7-8a4 4 0 1 0 0-8a4 4 0 0 0 0 8\"/>",
                24,
                24,
            )),
            Icon::Pram => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M11.5 3a8.5 8.5 0 0 0-7.212 13m14.425 0A8.46 8.46 0 0 0 20 11.5v-2h2.5M8 21a2 2 0 1 1 0-4a2 2 0 0 1 0 4m7 0a2 2 0 1 1 0-4a2 2 0 0 1 0 4M11.5 3v9m-8 0h16\"/>",
                24,
                24,
            )),
            Icon::Paint => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"m7 13.161l5.464-5.464a1 1 0 0 1 1.415 0l2.12 2.12a1 1 0 0 1 0 1.415l-1.928 1.929m-7.071 0l-2.172 2.172a1 1 0 0 0-.218.327l-1.028 2.496c-.508 1.233.725 2.466 1.958 1.959l2.497-1.028q.185-.077.326-.218l5.708-5.708m-7.071 0h7.071m-.193-9.707l2.121 2.121m4.243 4.243l-2.121-2.121m-2.122-2.122l1.414-1.414a1 1 0 0 1 1.415 0l.707.707a1 1 0 0 1 0 1.414L18.12 7.697m-2.122-2.122l2.122 2.122\"/>",
                24,
                24,
            )),
            Icon::Tree => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M12 22v-8m0-4v4m0 0l4-2m1-5A5 5 0 0 0 7 7m5 11H7.5a5.5 5.5 0 1 1 0-11H9m3 11h4.5A5.5 5.5 0 0 0 17 7.022\"/>",
                24,
                24,
            )),
            Icon::Ship => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\" d=\"M3 10c2.483 0 4.345-3 4.345-3s1.862 3 4.345 3s4.965-3 4.965-3s2.483 3 4.345 3M3 17c2.483 0 4.345-3 4.345-3s1.862 3 4.345 3s4.965-3 4.965-3s2.483 3 4.345 3\"/>",
                24,
                24,
            )),
            Icon::Train => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-width=\"1.5\"><path stroke-linejoin=\"round\" d=\"M9.609 7h4.782A2.61 2.61 0 0 1 17 9.609a.39.39 0 0 1-.391.391H7.39A.39.39 0 0 1 7 9.609A2.61 2.61 0 0 1 9.609 7\"/><path stroke-linejoin=\"round\" d=\"M9 3h6a6 6 0 0 1 6 6v4a6 6 0 0 1-6 6H9a6 6 0 0 1-6-6V9a6 6 0 0 1 6-6m7 12.01l.01-.011M8 15.01l.01-.011\"/><path d=\"m10.5 19l-2 2.5m5-2.5l2 2.5m1-2.5l2 2.5M7.5 19l-2 2.5\"/></g>",
                24,
                24,
            )),
            Icon::Bed => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\"><path d=\"M21 4v16a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2\"/><path d=\"M3 8h8V6m10 2h-8V6\"/></g>",
                24,
                24,
            )),
            Icon::Cake => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"1.5\"><path d=\"M4 16.5V20a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-3.5M3 14v-1a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v1m-9-6v3m0-3c1.262 0 2-.968 2-2.625S12 2 12 2s-2 1.718-2 3.375S10.738 8 12 8\"/><path d=\"M9 14a3 3 0 1 1-6 0m12 0a3 3 0 1 1-6 0m12 0a3 3 0 1 1-6 0\"/></g>",
                24,
                24,
            )),
            Icon::Drink => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.5\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M10 4h4v2.568c0 .258.17.487.412.579C22.938 10.37 20.908 22 15 22H9c-5.907 0-7.937-11.63.588-14.853a.63.63 0 0 0 .412-.58z\"/><path d=\"M6 10h12\"/><path stroke-linecap=\"round\" d=\"M9 2h6\"/><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M11.667 13L10 16h4l-1.667 3\"/></g>",
                24,
                24,
            )),
        }
    }

    fn solar(self) -> Option<(&'static str, u32, u32)> {
        match self {
            Icon::Coffee => Some((
                "<g fill=\"currentColor\"><path fill-rule=\"evenodd\" d=\"M3.28441 11.2661C3.15113 9.26687 3.08449 8.26726 3.67729 7.63363C4.27009 7 5.27191 7 7.27555 7H12.7245C14.7281 7 15.7299 7 16.3227 7.63363C16.4852 7.80726 16.5981 8.00837 16.6737 8.25H17C19.5261 8.25 21.75 10.0619 21.75 12.5C21.75 14.9381 19.5261 16.75 17 16.75H16.3499C16.3383 16.9226 16.3269 17.0891 16.3155 17.25H3.68451C3.66551 16.9826 3.64663 16.6993 3.62662 16.3991L3.28441 11.2661ZM16.45 15.25H17C18.8921 15.25 20.25 13.9278 20.25 12.5C20.25 11.0722 18.8921 9.75 17 9.75H16.8007C16.788 10.1801 16.7547 10.6802 16.7156 11.2661L16.45 15.25Z\" clip-rule=\"evenodd\"/><path d=\"M3.81902 18.75H16.181C16.0372 19.9266 15.8026 20.6671 15.2429 21.1907C14.3779 22 13.0475 22 10.3867 22H9.61333C6.95253 22 5.62212 22 4.75712 21.1907C4.19745 20.6671 3.96285 19.9266 3.81902 18.75Z\"/><path fill-rule=\"evenodd\" d=\"M6.97721 1.32673C7.31443 1.56726 7.39281 2.03562 7.15227 2.37284L6.7662 2.91409C7.39202 3.38836 7.53073 4.27761 7.07175 4.92108L6.66113 5.49675C6.42059 5.83396 5.95223 5.91234 5.61501 5.67181C5.2778 5.43127 5.19942 4.96291 5.43996 4.62569L5.82603 4.08444C5.2002 3.61018 5.0615 2.72092 5.52048 2.07745L5.9311 1.50179C6.17163 1.16457 6.63999 1.08619 6.97721 1.32673ZM10.9772 1.32673C11.3144 1.56726 11.3928 2.03562 11.1523 2.37284L10.7662 2.91409C11.392 3.38836 11.5307 4.27761 11.0717 4.92108L10.6611 5.49675C10.4206 5.83396 9.95223 5.91234 9.61501 5.67181C9.2778 5.43127 9.19942 4.96291 9.43996 4.62569L9.82603 4.08444C9.2002 3.61018 9.0615 2.72092 9.52048 2.07745L9.9311 1.50179C10.1716 1.16457 10.64 1.08619 10.9772 1.32673ZM14.9772 1.32673C15.3144 1.56726 15.3928 2.03562 15.1523 2.37284L14.7662 2.91409C15.392 3.38836 15.5307 4.27761 15.0717 4.92108L14.6611 5.49675C14.4206 5.83396 13.9522 5.91234 13.615 5.67181C13.2778 5.43127 13.1994 4.96291 13.44 4.62569L13.826 4.08444C13.2002 3.61018 13.0615 2.72092 13.5205 2.07745L13.9311 1.50179C14.1716 1.16457 14.64 1.08619 14.9772 1.32673Z\" clip-rule=\"evenodd\"/></g>",
                24,
                24,
            )),
            Icon::Book => Some((
                "<g fill=\"currentColor\"><path fill-rule=\"evenodd\" d=\"M6.27103 2.11151C5.46135 2.21816 5.03258 2.41324 4.72718 2.71244C4.42179 3.01165 4.22268 3.43172 4.11382 4.225C4.00176 5.04159 4 6.12387 4 7.67568V16.2442C4.38867 15.9781 4.82674 15.7756 5.29899 15.6517C5.82716 15.513 6.44305 15.5132 7.34563 15.5135L20 15.5135V7.67568C20 6.12387 19.9982 5.04159 19.8862 4.22499C19.7773 3.43172 19.5782 3.01165 19.2728 2.71244C18.9674 2.41324 18.5387 2.21816 17.729 2.11151C16.8955 2.00172 15.7908 2 14.2069 2H9.7931C8.2092 2 7.10452 2.00172 6.27103 2.11151ZM6.75862 6.59459C6.75862 6.1468 7.12914 5.78378 7.58621 5.78378H16.4138C16.8709 5.78378 17.2414 6.1468 17.2414 6.59459C17.2414 7.04239 16.8709 7.40541 16.4138 7.40541H7.58621C7.12914 7.40541 6.75862 7.04239 6.75862 6.59459ZM7.58621 9.56757C7.12914 9.56757 6.75862 9.93058 6.75862 10.3784C6.75862 10.8262 7.12914 11.1892 7.58621 11.1892H13.1034C13.5605 11.1892 13.931 10.8262 13.931 10.3784C13.931 9.93058 13.5605 9.56757 13.1034 9.56757H7.58621Z\" clip-rule=\"evenodd\"/><path d=\"M7.47341 17.1351H8.68965H13.1034H19.9991C19.9956 18.2657 19.9776 19.1088 19.8862 19.775C19.7773 20.5683 19.5782 20.9884 19.2728 21.2876C18.9674 21.5868 18.5387 21.7818 17.729 21.8885C16.8955 21.9983 15.7908 22 14.2069 22H9.7931C8.2092 22 7.10452 21.9983 6.27103 21.8885C5.46135 21.7818 5.03258 21.5868 4.72718 21.2876C4.42179 20.9884 4.22268 20.5683 4.11382 19.775C4.07259 19.4746 4.0463 19.1382 4.02952 18.7558C4.30088 18.0044 4.93365 17.4264 5.72738 17.218C6.01657 17.1421 6.39395 17.1351 7.47341 17.1351Z\"/></g>",
                24,
                24,
            )),
            Icon::Camera => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M9.77778 21H14.2222C17.3433 21 18.9038 21 20.0248 20.2646C20.51 19.9462 20.9267 19.5371 21.251 19.0607C22 17.9601 22 16.4279 22 13.3636C22 10.2994 22 8.76721 21.251 7.6666C20.9267 7.19014 20.51 6.78104 20.0248 6.46268C19.3044 5.99013 18.4027 5.82123 17.022 5.76086C16.3631 5.76086 15.7959 5.27068 15.6667 4.63636C15.4728 3.68489 14.6219 3 13.6337 3H10.3663C9.37805 3 8.52715 3.68489 8.33333 4.63636C8.20412 5.27068 7.63685 5.76086 6.978 5.76086C5.59733 5.82123 4.69555 5.99013 3.97524 6.46268C3.48995 6.78104 3.07328 7.19014 2.74902 7.6666C2 8.76721 2 10.2994 2 13.3636C2 16.4279 2 17.9601 2.74902 19.0607C3.07328 19.5371 3.48995 19.9462 3.97524 20.2646C5.09624 21 6.65675 21 9.77778 21ZM12 9.27273C9.69881 9.27273 7.83333 11.1043 7.83333 13.3636C7.83333 15.623 9.69881 17.4545 12 17.4545C14.3012 17.4545 16.1667 15.623 16.1667 13.3636C16.1667 11.1043 14.3012 9.27273 12 9.27273ZM12 10.9091C10.6193 10.9091 9.5 12.008 9.5 13.3636C9.5 14.7192 10.6193 15.8182 12 15.8182C13.3807 15.8182 14.5 14.7192 14.5 13.3636C14.5 12.008 13.3807 10.9091 12 10.9091ZM16.7222 10.0909C16.7222 9.63904 17.0953 9.27273 17.5556 9.27273H18.6667C19.1269 9.27273 19.5 9.63904 19.5 10.0909C19.5 10.5428 19.1269 10.9091 18.6667 10.9091H17.5556C17.0953 10.9091 16.7222 10.5428 16.7222 10.0909Z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Home => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M2.5192 7.82274C2 8.77128 2 9.91549 2 12.2039V13.725C2 17.6258 2 19.5763 3.17157 20.7881C4.34315 22 6.22876 22 10 22H14C17.7712 22 19.6569 22 20.8284 20.7881C22 19.5763 22 17.6258 22 13.725V12.2039C22 9.91549 22 8.77128 21.4808 7.82274C20.9616 6.87421 20.0131 6.28551 18.116 5.10812L16.116 3.86687C14.1106 2.62229 13.1079 2 12 2C10.8921 2 9.88939 2.62229 7.88403 3.86687L5.88403 5.10813C3.98695 6.28551 3.0384 6.87421 2.5192 7.82274ZM9 17.25C8.58579 17.25 8.25 17.5858 8.25 18C8.25 18.4142 8.58579 18.75 9 18.75H15C15.4142 18.75 15.75 18.4142 15.75 18C15.75 17.5858 15.4142 17.25 15 17.25H9Z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Work => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M12.052 1.25H11.948C11.0495 1.24997 10.3003 1.24995 9.70552 1.32991C9.07773 1.41432 8.51093 1.59999 8.05546 2.05546C7.59999 2.51093 7.41432 3.07773 7.32991 3.70552C7.24995 4.3003 7.24997 5.04951 7.25 5.94799V6.02572C5.22882 6.09185 4.01511 6.32803 3.17157 7.17157C2 8.34315 2 10.2288 2 14C2 17.7712 2 19.6569 3.17157 20.8284C4.34315 22 6.22876 22 10 22H14C17.7712 22 19.6569 22 20.8284 20.8284C22 19.6569 22 17.7712 22 14C22 10.2288 22 8.34315 20.8284 7.17157C19.9849 6.32803 18.7712 6.09185 16.75 6.02572V5.94801C16.75 5.04954 16.7501 4.3003 16.6701 3.70552C16.5857 3.07773 16.4 2.51093 15.9445 2.05546C15.4891 1.59999 14.9223 1.41432 14.2945 1.32991C13.6997 1.24995 12.9505 1.24997 12.052 1.25ZM15.25 6.00189V6C15.25 5.03599 15.2484 4.38843 15.1835 3.9054C15.1214 3.44393 15.0142 3.24644 14.8839 3.11612C14.7536 2.9858 14.5561 2.87858 14.0946 2.81654C13.6116 2.7516 12.964 2.75 12 2.75C11.036 2.75 10.3884 2.7516 9.90539 2.81654C9.44393 2.87858 9.24644 2.9858 9.11612 3.11612C8.9858 3.24644 8.87858 3.44393 8.81654 3.9054C8.7516 4.38843 8.75 5.03599 8.75 6V6.00189C9.14203 6 9.55807 6 10 6H14C14.4419 6 14.858 6 15.25 6.00189ZM17 9C17 9.55229 16.5523 10 16 10C15.4477 10 15 9.55229 15 9C15 8.44772 15.4477 8 16 8C16.5523 8 17 8.44772 17 9ZM8 10C8.55228 10 9 9.55229 9 9C9 8.44772 8.55228 8 8 8C7.44772 8 7 8.44772 7 9C7 9.55229 7.44772 10 8 10Z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Music => Some((
                "<path fill=\"currentColor\" d=\"M10.0909 11.9629L19.3636 8.63087V14.1707C18.8126 13.8538 18.1574 13.67 17.4545 13.67C15.4964 13.67 13.9091 15.096 13.9091 16.855C13.9091 18.614 15.4964 20.04 17.4545 20.04C19.4126 20.04 21 18.614 21 16.855C21 16.855 21 16.8551 21 16.855L21 7.49236C21 6.37238 21 5.4331 20.9123 4.68472C20.8999 4.57895 20.8852 4.4738 20.869 4.37569C20.7845 3.86441 20.6352 3.38745 20.347 2.98917C20.2028 2.79002 20.024 2.61055 19.8012 2.45628C19.7594 2.42736 19.716 2.39932 19.6711 2.3722L19.6621 2.36679C18.8906 1.90553 18.0233 1.93852 17.1298 2.14305C16.2657 2.34086 15.1944 2.74368 13.8808 3.23763L11.5963 4.09656C10.9806 4.32806 10.4589 4.52419 10.0494 4.72734C9.61376 4.94348 9.23849 5.1984 8.95707 5.57828C8.67564 5.95817 8.55876 6.36756 8.50501 6.81203C8.4545 7.22978 8.45452 7.7378 8.45455 8.33743V16.1307C7.90347 15.8138 7.24835 15.63 6.54545 15.63C4.58735 15.63 3 17.056 3 18.815C3 20.574 4.58735 22 6.54545 22C8.50355 22 10.0909 20.574 10.0909 18.815C10.0909 18.815 10.0909 18.8151 10.0909 18.815L10.0909 11.9629Z\"/>",
                24,
                24,
            )),
            Icon::Heart => Some((
                "<path fill=\"currentColor\" d=\"M2 9.1371C2 14 6.01943 16.5914 8.96173 18.9109C10 19.7294 11 20.5 12 20.5C13 20.5 14 19.7294 15.0383 18.9109C17.9806 16.5914 22 14 22 9.1371C22 4.27416 16.4998 0.825464 12 5.50063C7.50016 0.825464 2 4.27416 2 9.1371Z\"/>",
                24,
                24,
            )),
            Icon::Star => Some((
                "<path fill=\"currentColor\" d=\"M9.15316 5.40838C10.4198 3.13613 11.0531 2 12 2C12.9469 2 13.5802 3.13612 14.8468 5.40837L15.1745 5.99623C15.5345 6.64193 15.7144 6.96479 15.9951 7.17781C16.2757 7.39083 16.6251 7.4699 17.3241 7.62805L17.9605 7.77203C20.4201 8.32856 21.65 8.60682 21.9426 9.54773C22.2352 10.4886 21.3968 11.4691 19.7199 13.4299L19.2861 13.9372C18.8096 14.4944 18.5713 14.773 18.4641 15.1177C18.357 15.4624 18.393 15.8341 18.465 16.5776L18.5306 17.2544C18.7841 19.8706 18.9109 21.1787 18.1449 21.7602C17.3788 22.3417 16.2273 21.8115 13.9243 20.7512L13.3285 20.4768C12.6741 20.1755 12.3469 20.0248 12 20.0248C11.6531 20.0248 11.3259 20.1755 10.6715 20.4768L10.0757 20.7512C7.77268 21.8115 6.62118 22.3417 5.85515 21.7602C5.08912 21.1787 5.21588 19.8706 5.4694 17.2544L5.53498 16.5776C5.60703 15.8341 5.64305 15.4624 5.53586 15.1177C5.42868 14.773 5.19043 14.4944 4.71392 13.9372L4.2801 13.4299C2.60325 11.4691 1.76482 10.4886 2.05742 9.54773C2.35002 8.60682 3.57986 8.32856 6.03954 7.77203L6.67589 7.62805C7.37485 7.4699 7.72433 7.39083 8.00494 7.17781C8.28555 6.96479 8.46553 6.64194 8.82547 5.99623L9.15316 5.40838Z\"/>",
                24,
                24,
            )),
            Icon::Plane => Some((
                "<path fill=\"currentColor\" d=\"M18.6357 15.6701L20.3521 10.5208C21.8516 6.02242 22.6013 3.77322 21.414 2.58595C20.2268 1.39869 17.9776 2.14842 13.4792 3.64788L8.32987 5.36432C4.69923 6.57453 2.88392 7.17964 2.36806 8.06698C1.87731 8.91112 1.87731 9.95369 2.36806 10.7978C2.88392 11.6852 4.69923 12.2903 8.32987 13.5005C8.77981 13.6505 9.28601 13.5434 9.62294 13.2096L15.1286 7.75495C15.4383 7.44808 15.9382 7.45041 16.245 7.76015C16.5519 8.06989 16.5496 8.56975 16.2398 8.87662L10.8231 14.2432C10.4518 14.6111 10.3342 15.1742 10.4995 15.6701C11.7097 19.3007 12.3148 21.1161 13.2022 21.6319C14.0463 22.1227 15.0889 22.1227 15.933 21.6319C16.8204 21.1161 17.4255 19.3008 18.6357 15.6701Z\"/>",
                24,
                24,
            )),
            Icon::Food => Some((
                "<g fill=\"currentColor\"><path d=\"M7 5C4.23858 5 2 7.23858 2 10C2 12.0503 3.2341 13.8124 5 14.584V17.25H19L19 14.584C20.7659 13.8124 22 12.0503 22 10C22 7.23858 19.7614 5 17 5C16.7495 5 16.5033 5.01842 16.2626 5.05399C15.6604 3.27806 13.9794 2 12 2C10.0206 2 8.33961 3.27806 7.73736 5.05399C7.49673 5.01842 7.25052 5 7 5Z\"/><path d=\"M18.9983 18.75H5.00169C5.01188 20.1469 5.08343 20.9119 5.58579 21.4142C6.17157 22 7.11438 22 9 22H15C16.8856 22 17.8284 22 18.4142 21.4142C18.9166 20.9119 18.9881 20.1469 18.9983 18.75Z\"/></g>",
                24,
                24,
            )),
            Icon::Idea => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M11.5 2C15.6421 2 19 5.43634 19 9.6748C18.9999 11.947 18.0335 13.9894 16.501 15.3936C15.9903 15.8614 15.6119 16.2084 15.3438 16.46C15.2098 16.5857 15.1069 16.6834 15.0312 16.7588C14.9935 16.7964 14.9649 16.8265 14.9434 16.8496C14.9328 16.861 14.9247 16.8703 14.9189 16.877L14.9141 16.8828L14.9111 16.8867C14.6746 17.1854 14.6233 17.2622 14.5928 17.332C14.5623 17.4018 14.5401 17.4925 14.4805 17.8721C14.457 18.0219 14.4541 18.2787 14.4541 18.9766V19.0068C14.4541 19.4159 14.4544 19.769 14.4287 20.0586C14.4016 20.3646 14.3417 20.6675 14.1807 20.9531C14.0011 21.2713 13.7426 21.536 13.4316 21.7197C13.1524 21.8846 12.8558 21.9459 12.5566 21.9736C12.2737 21.9998 11.9289 22 11.5293 22H11.4707C11.0711 22 10.7263 21.9998 10.4434 21.9736C10.1442 21.9459 9.84759 21.8846 9.56836 21.7197C9.25741 21.536 8.99886 21.2713 8.81934 20.9531C8.65829 20.6675 8.59841 20.3646 8.57129 20.0586C8.54564 19.769 8.54588 19.4159 8.5459 19.0068V18.9766C8.5459 18.2787 8.54304 18.0219 8.51953 17.8721C8.45986 17.4925 8.43771 17.4018 8.40723 17.332C8.37674 17.2622 8.32538 17.1854 8.08887 16.8867L8.08594 16.8828L8.08105 16.877C8.07526 16.8703 8.06721 16.861 8.05664 16.8496C8.03515 16.8265 8.00646 16.7964 7.96875 16.7588C7.89312 16.6834 7.79023 16.5857 7.65625 16.46C7.38812 16.2084 7.00967 15.8614 6.49902 15.3936C4.96655 13.9894 4.00011 11.947 4 9.6748C4 5.43634 7.35786 2 11.5 2ZM9.91406 19.6748C9.91704 19.7716 9.92196 19.8564 9.92871 19.9326C9.94726 20.1419 9.97856 20.2179 10 20.2559C10.0598 20.3619 10.1454 20.4505 10.249 20.5117C10.2862 20.5337 10.3608 20.565 10.5654 20.584C10.7794 20.6038 11.0618 20.6045 11.499 20.6045C11.9364 20.6045 12.2195 20.6038 12.4336 20.584C12.6372 20.5651 12.7118 20.5336 12.749 20.5117C12.8527 20.4505 12.9392 20.3619 12.999 20.2559C13.0205 20.2178 13.0518 20.1417 13.0703 19.9326C13.0771 19.8564 13.081 19.7716 13.084 19.6748H9.91406ZM13.4893 13.3506C13.1307 13.1432 12.6713 13.2665 12.4639 13.625C12.2465 14.0003 11.8424 14.25 11.3809 14.25C10.9193 14.25 10.5151 14.0004 10.2979 13.625C10.0905 13.2666 9.63192 13.1435 9.27344 13.3506C8.91489 13.558 8.79162 14.0174 8.99902 14.376C9.35228 14.9866 9.93553 15.4497 10.6309 15.6465V17C10.6309 17.4142 10.9667 17.7499 11.3809 17.75C11.7951 17.75 12.1309 17.4142 12.1309 17V15.6465C12.8262 15.4497 13.4094 14.9866 13.7627 14.376C13.9701 14.0175 13.8478 13.558 13.4893 13.3506Z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Code => Some((
                "<g fill=\"currentColor\"><path d=\"M14.1809 4.2755C14.581 4.3827 14.8185 4.79396 14.7113 5.19406L10.7377 20.0238C10.6304 20.4239 10.2192 20.6613 9.81909 20.5541C9.41899 20.4469 9.18156 20.0356 9.28876 19.6355L13.2624 4.80583C13.3696 4.40573 13.7808 4.16829 14.1809 4.2755Z\"/><path d=\"M16.4425 7.32781C16.7196 7.01993 17.1938 6.99497 17.5017 7.27206L19.2392 8.8358C19.9756 9.49847 20.5864 10.0482 21.0058 10.5467C21.4468 11.071 21.7603 11.6342 21.7603 12.3295C21.7603 13.0248 21.4468 13.5881 21.0058 14.1123C20.5864 14.6109 19.9756 15.1606 19.2392 15.8233L17.5017 17.387C17.1938 17.6641 16.7196 17.6391 16.4425 17.3313C16.1654 17.0234 16.1904 16.5492 16.4983 16.2721L18.1947 14.7452C18.9826 14.0362 19.5138 13.5558 19.8579 13.1467C20.1882 12.7541 20.2603 12.525 20.2603 12.3295C20.2603 12.1341 20.1882 11.9049 19.8579 11.5123C19.5138 11.1033 18.9826 10.6229 18.1947 9.91383L16.4983 8.387C16.1904 8.10991 16.1654 7.63569 16.4425 7.32781Z\"/><path d=\"M7.50178 8.387C7.80966 8.10991 7.83462 7.63569 7.55752 7.32781C7.28043 7.01993 6.80621 6.99497 6.49833 7.27206L4.76084 8.8358C4.0245 9.49847 3.41369 10.0482 2.99428 10.5467C2.55325 11.071 2.23975 11.6342 2.23975 12.3295C2.23975 13.0248 2.55325 13.5881 2.99428 14.1123C3.41369 14.6109 4.02449 15.1606 4.76082 15.8232L6.49833 17.387C6.80621 17.6641 7.28043 17.6391 7.55752 17.3313C7.83462 17.0234 7.80966 16.5492 7.50178 16.2721L5.80531 14.7452C5.01743 14.0362 4.48623 13.5558 4.14213 13.1467C3.81188 12.7541 3.73975 12.525 3.73975 12.3295C3.73975 12.1341 3.81188 11.9049 4.14213 11.5123C4.48623 11.1033 5.01743 10.6229 5.80531 9.91383L7.50178 8.387Z\"/></g>",
                24,
                24,
            )),
            Icon::Money => Some((
                "<g fill=\"currentColor\"><path d=\"M11.25 7.84748C10.3141 8.10339 9.75 8.82154 9.75 9.5C9.75 10.1785 10.3141 10.8966 11.25 11.1525V7.84748Z\"/><path d=\"M12.75 12.8475V16.1525C13.6859 15.8966 14.25 15.1785 14.25 14.5C14.25 13.8215 13.6859 13.1034 12.75 12.8475Z\"/><path fill-rule=\"evenodd\" d=\"M22 12C22 17.5228 17.5228 22 12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C17.5228 2 22 6.47715 22 12ZM12 5.25C12.4142 5.25 12.75 5.58579 12.75 6V6.31673C14.3804 6.60867 15.75 7.83361 15.75 9.5C15.75 9.91421 15.4142 10.25 15 10.25C14.5858 10.25 14.25 9.91421 14.25 9.5C14.25 8.82154 13.6859 8.10339 12.75 7.84748V11.3167C14.3804 11.6087 15.75 12.8336 15.75 14.5C15.75 16.1664 14.3804 17.3913 12.75 17.6833V18C12.75 18.4142 12.4142 18.75 12 18.75C11.5858 18.75 11.25 18.4142 11.25 18V17.6833C9.61957 17.3913 8.25 16.1664 8.25 14.5C8.25 14.0858 8.58579 13.75 9 13.75C9.41421 13.75 9.75 14.0858 9.75 14.5C9.75 15.1785 10.3141 15.8966 11.25 16.1525V12.6833C9.61957 12.3913 8.25 11.1664 8.25 9.5C8.25 7.83361 9.61957 6.60867 11.25 6.31673V6C11.25 5.58579 11.5858 5.25 12 5.25Z\" clip-rule=\"evenodd\"/></g>",
                24,
                24,
            )),
            Icon::Gift => Some((
                "<g fill=\"currentColor\"><path d=\"M11.2498 2C7.03145 2.00411 4.84888 2.07958 3.46423 3.46423C2.07958 4.84888 2.00411 7.03145 2 11.2498H6.91352C6.56255 10.8114 6.30031 10.2943 6.15731 9.72228C5.61906 7.56926 7.56926 5.61906 9.72228 6.15731C10.2943 6.30031 10.8114 6.56255 11.2498 6.91352V2Z\"/><path d=\"M2 12.7498C2.00411 16.9681 2.07958 19.1506 3.46423 20.5353C4.84888 21.9199 7.03145 21.9954 11.2498 21.9995V14.1234C10.4701 15.6807 8.8598 16.7498 6.99976 16.7498C6.58555 16.7498 6.24976 16.414 6.24976 15.9998C6.24976 15.5856 6.58555 15.2498 6.99976 15.2498C8.53655 15.2498 9.82422 14.1831 10.1628 12.7498H2Z\"/><path d=\"M12.7498 21.9995C16.9681 21.9954 19.1506 21.9199 20.5353 20.5353C21.9199 19.1506 21.9954 16.9681 21.9995 12.7498H13.8367C14.1753 14.1831 15.463 15.2498 16.9998 15.2498C17.414 15.2498 17.7498 15.5856 17.7498 15.9998C17.7498 16.414 17.414 16.7498 16.9998 16.7498C15.1397 16.7498 13.5294 15.6807 12.7498 14.1234V21.9995Z\"/><path d=\"M21.9995 11.2498C21.9954 7.03145 21.9199 4.84888 20.5353 3.46423C19.1506 2.07958 16.9681 2.00411 12.7498 2V6.91352C13.1882 6.56255 13.7053 6.30031 14.2772 6.15731C16.4303 5.61906 18.3805 7.56926 17.8422 9.72228C17.6992 10.2943 17.437 10.8114 17.086 11.2498H21.9995Z\"/><path d=\"M9.35847 7.61252C10.47 7.8904 11.2498 8.88911 11.2498 10.0348V11.2498H10.0348C8.88911 11.2498 7.8904 10.47 7.61252 9.35847C7.34891 8.30403 8.30403 7.34891 9.35847 7.61252Z\"/><path d=\"M12.7498 10.0348V11.2498H13.9647C15.1104 11.2498 16.1091 10.47 16.387 9.35847C16.6506 8.30403 15.6955 7.34891 14.6411 7.61252C13.5295 7.8904 12.7498 8.88911 12.7498 10.0348Z\"/></g>",
                24,
                24,
            )),
            Icon::Leaf => Some((
                "<g fill=\"currentColor\"><path d=\"M11.25 2.08258C11.0066 2.13684 10.7675 2.21782 10.5371 2.32554C6.55332 4.18758 4 9.39452 4 13.8567C4 18.0967 7.18341 21.5798 11.25 21.9647V2.08258Z\"/><path d=\"M12.75 21.9647C16.8166 21.5798 20 18.0967 20 13.8567C20 13.4507 19.9789 13.0385 19.9374 12.6232L12.75 19.8106V21.9647Z\"/><path d=\"M18.2597 7.17964C17.8707 6.45482 17.4222 5.76815 16.92 5.14068L12.75 9.31065V12.6893L18.2597 7.17964Z\"/><path d=\"M15.9084 4.03088C15.1732 3.32565 14.3538 2.74195 13.4629 2.32554C13.2325 2.21782 12.9934 2.13684 12.75 2.08258V7.18933L15.9084 4.03088Z\"/><path d=\"M18.9364 8.62421L12.75 14.8106V17.6893L19.5 10.9393L19.6319 10.8074C19.458 10.0697 19.2246 9.33633 18.9364 8.62421Z\"/></g>",
                24,
                24,
            )),
            Icon::Gear => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M14.2788 2.15224C13.9085 2 13.439 2 12.5 2C11.561 2 11.0915 2 10.7212 2.15224C10.2274 2.35523 9.83509 2.74458 9.63056 3.23463C9.53719 3.45834 9.50065 3.7185 9.48635 4.09799C9.46534 4.65568 9.17716 5.17189 8.69017 5.45093C8.20318 5.72996 7.60864 5.71954 7.11149 5.45876C6.77318 5.2813 6.52789 5.18262 6.28599 5.15102C5.75609 5.08178 5.22018 5.22429 4.79616 5.5472C4.47814 5.78938 4.24339 6.1929 3.7739 6.99993C3.30441 7.80697 3.06967 8.21048 3.01735 8.60491C2.94758 9.1308 3.09118 9.66266 3.41655 10.0835C3.56506 10.2756 3.77377 10.437 4.0977 10.639C4.57391 10.936 4.88032 11.4419 4.88029 12C4.88026 12.5581 4.57386 13.0639 4.0977 13.3608C3.77372 13.5629 3.56497 13.7244 3.41645 13.9165C3.09108 14.3373 2.94749 14.8691 3.01725 15.395C3.06957 15.7894 3.30432 16.193 3.7738 17C4.24329 17.807 4.47804 18.2106 4.79606 18.4527C5.22008 18.7756 5.75599 18.9181 6.28589 18.8489C6.52778 18.8173 6.77305 18.7186 7.11133 18.5412C7.60852 18.2804 8.2031 18.27 8.69012 18.549C9.17714 18.8281 9.46533 19.3443 9.48635 19.9021C9.50065 20.2815 9.53719 20.5417 9.63056 20.7654C9.83509 21.2554 10.2274 21.6448 10.7212 21.8478C11.0915 22 11.561 22 12.5 22C13.439 22 13.9085 22 14.2788 21.8478C14.7726 21.6448 15.1649 21.2554 15.3694 20.7654C15.4628 20.5417 15.4994 20.2815 15.5137 19.902C15.5347 19.3443 15.8228 18.8281 16.3098 18.549C16.7968 18.2699 17.3914 18.2804 17.8886 18.5412C18.2269 18.7186 18.4721 18.8172 18.714 18.8488C19.2439 18.9181 19.7798 18.7756 20.2038 18.4527C20.5219 18.2105 20.7566 17.807 21.2261 16.9999C21.6956 16.1929 21.9303 15.7894 21.9827 15.395C22.0524 14.8691 21.9088 14.3372 21.5835 13.9164C21.4349 13.7243 21.2262 13.5628 20.9022 13.3608C20.4261 13.0639 20.1197 12.558 20.1197 11.9999C20.1197 11.4418 20.4261 10.9361 20.9022 10.6392C21.2263 10.4371 21.435 10.2757 21.5836 10.0835C21.9089 9.66273 22.0525 9.13087 21.9828 8.60497C21.9304 8.21055 21.6957 7.80703 21.2262 7C20.7567 6.19297 20.522 5.78945 20.2039 5.54727C19.7799 5.22436 19.244 5.08185 18.7141 5.15109C18.4722 5.18269 18.2269 5.28136 17.8887 5.4588C17.3915 5.71959 16.7969 5.73002 16.3099 5.45096C15.8229 5.17191 15.5347 4.65566 15.5136 4.09794C15.4993 3.71848 15.4628 3.45833 15.3694 3.23463C15.1649 2.74458 14.7726 2.35523 14.2788 2.15224ZM12.5 15C14.1695 15 15.5228 13.6569 15.5228 12C15.5228 10.3431 14.1695 9 12.5 9C10.8305 9 9.47716 10.3431 9.47716 12C9.47716 13.6569 10.8305 15 12.5 15Z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Flag => Some((
                "<path fill=\"currentColor\" d=\"M5.75 1C6.16421 1 6.5 1.33579 6.5 1.75V3.6L8.22067 3.25587C9.8712 2.92576 11.5821 3.08284 13.1449 3.70797L13.3486 3.78943C14.9097 4.41389 16.628 4.53051 18.2592 4.1227C19.0165 3.93339 19.75 4.50613 19.75 5.28669V12.6537C19.75 13.298 19.3115 13.8596 18.6864 14.0159L18.472 14.0695C16.7024 14.5119 14.8385 14.3854 13.1449 13.708C11.5821 13.0828 9.8712 12.9258 8.22067 13.2559L6.5 13.6V21.75C6.5 22.1642 6.16421 22.5 5.75 22.5C5.33579 22.5 5 22.1642 5 21.75V1.75C5 1.33579 5.33579 1 5.75 1Z\"/>",
                24,
                24,
            )),
            Icon::Pin => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M12 2C7.58172 2 4 6.00258 4 10.5C4 14.9622 6.55332 19.8124 10.5371 21.6744C11.4657 22.1085 12.5343 22.1085 13.4629 21.6744C17.4467 19.8124 20 14.9622 20 10.5C20 6.00258 16.4183 2 12 2ZM12 12C13.1046 12 14 11.1046 14 10C14 8.89543 13.1046 8 12 8C10.8954 8 10 8.89543 10 10C10 11.1046 10.8954 12 12 12Z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Bug => Some((
                "<g fill=\"currentColor\"><path d=\"M17.416 2.62412C17.7607 2.39435 17.8538 1.9287 17.624 1.58405C17.3943 1.23941 16.9286 1.14628 16.584 1.37604L13.6687 3.31955C13.1527 3.11343 12.5897 3.00006 12.0001 3.00006C11.4105 3.00006 10.8474 3.11345 10.3314 3.31962L7.41603 1.37604C7.07138 1.14628 6.60573 1.23941 6.37596 1.58405C6.1462 1.9287 6.23933 2.39435 6.58397 2.62412L8.9437 4.19727C8.24831 4.84109 7.75664 5.70181 7.57617 6.6719C8.01128 6.55973 8.46749 6.50006 8.93763 6.50006H15.0626C15.5328 6.50006 15.989 6.55973 16.4241 6.6719C16.2436 5.70176 15.7519 4.841 15.0564 4.19717L17.416 2.62412Z\"/><path d=\"M1.25 14.0001C1.25 13.5859 1.58579 13.2501 2 13.2501H5V11.9376C5 11.1019 5.26034 10.327 5.70435 9.68959L3.22141 8.69624C2.83684 8.54238 2.6498 8.10589 2.80366 7.72131C2.95752 7.33673 3.39401 7.1497 3.77859 7.30356L6.91514 8.55841C7.50624 8.20388 8.19807 8.00006 8.9375 8.00006H15.0625C15.8019 8.00006 16.4938 8.20388 17.0849 8.55841L20.2214 7.30356C20.606 7.1497 21.0425 7.33673 21.1963 7.72131C21.3502 8.10589 21.1632 8.54238 20.7786 8.69624L18.2957 9.68959C18.7397 10.327 19 11.1019 19 11.9376V13.2501H22C22.4142 13.2501 22.75 13.5859 22.75 14.0001C22.75 14.4143 22.4142 14.7501 22 14.7501H19V15.0001C19 16.1808 18.7077 17.2932 18.1915 18.2689L20.7786 19.3039C21.1632 19.4578 21.3502 19.8943 21.1963 20.2789C21.0425 20.6634 20.606 20.8505 20.2214 20.6966L17.3288 19.5394C16.1974 20.8664 14.5789 21.7655 12.75 21.9604V15.0001C12.75 14.5858 12.4142 14.2501 12 14.2501C11.5858 14.2501 11.25 14.5858 11.25 15.0001V21.9604C9.42109 21.7655 7.80265 20.8664 6.67115 19.5394L3.77859 20.6966C3.39401 20.8505 2.95752 20.6634 2.80366 20.2789C2.6498 19.8943 2.83684 19.4578 3.22141 19.3039L5.80852 18.2689C5.29231 17.2932 5 16.1808 5 15.0001V14.7501H2C1.58579 14.7501 1.25 14.4143 1.25 14.0001Z\"/></g>",
                24,
                24,
            )),
            Icon::Game => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M10.6669 6.13443L10.165 5.77922C9.44862 5.27225 8.59264 5 7.71504 5H7.10257C6.69838 5 6.29009 5.02549 5.90915 5.16059C3.52645 6.00566 1.88749 9.09504 2.00604 16.1026C2.02992 17.5145 2.3603 19.075 3.63423 19.6842C4.03121 19.8741 4.49667 20 5.02671 20C5.66273 20 6.1678 19.8187 6.55763 19.5632C6.96641 19.2953 7.32633 18.9471 7.68612 18.599C8.13071 18.1688 8.57511 17.7389 9.11125 17.4609C9.69519 17.1581 10.3434 17 11.0011 17H12.9989C13.6566 17 14.3048 17.1581 14.8888 17.4609C15.4249 17.7389 15.8693 18.1688 16.3139 18.599C16.6737 18.9471 17.0336 19.2953 17.4424 19.5632C17.8322 19.8187 18.3373 20 18.9733 20C19.5033 20 19.9688 19.8741 20.3658 19.6842C21.6397 19.075 21.9701 17.5145 21.994 16.1026C22.1125 9.09503 20.4735 6.00566 18.0908 5.16059C17.7099 5.02549 17.3016 5 16.8974 5H16.2849C15.4074 5 14.5514 5.27225 13.8351 5.77922L13.3332 6.13441C12.9434 6.41029 12.4776 6.55844 12 6.55844C11.5225 6.55844 11.0567 6.41029 10.6669 6.13443ZM16.75 9C17.1642 9 17.5 9.33579 17.5 9.75C17.5 10.1642 17.1642 10.5 16.75 10.5C16.3358 10.5 16 10.1642 16 9.75C16 9.33579 16.3358 9 16.75 9ZM7.5 9.25C7.91421 9.25 8.25 9.58579 8.25 10V10.75H9C9.41421 10.75 9.75 11.0858 9.75 11.5C9.75 11.9142 9.41421 12.25 9 12.25H8.25V13C8.25 13.4142 7.91421 13.75 7.5 13.75C7.08579 13.75 6.75 13.4142 6.75 13V12.25H6C5.58579 12.25 5.25 11.9142 5.25 11.5C5.25 11.0858 5.58579 10.75 6 10.75H6.75V10C6.75 9.58579 7.08579 9.25 7.5 9.25ZM19 11.25C19 11.6642 18.6642 12 18.25 12C17.8358 12 17.5 11.6642 17.5 11.25C17.5 10.8358 17.8358 10.5 18.25 10.5C18.6642 10.5 19 10.8358 19 11.25ZM15.25 12C15.6642 12 16 11.6642 16 11.25C16 10.8358 15.6642 10.5 15.25 10.5C14.8358 10.5 14.5 10.8358 14.5 11.25C14.5 11.6642 14.8358 12 15.25 12ZM17.5 12.75C17.5 12.3358 17.1642 12 16.75 12C16.3358 12 16 12.3358 16 12.75C16 13.1642 16.3358 13.5 16.75 13.5C17.1642 13.5 17.5 13.1642 17.5 12.75Z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Cart => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M8.41799 3.25089C8.69867 2.65917 9.30155 2.25 10 2.25H14C14.6984 2.25 15.3013 2.65917 15.582 3.25089C16.2655 3.25586 16.7983 3.28724 17.2738 3.47309C17.842 3.69516 18.3362 4.07266 18.6999 4.56242C19.0668 5.0565 19.2391 5.68979 19.4762 6.56144L20.2181 9.28272L20.4985 10.124C20.5065 10.1339 20.5144 10.1438 20.5222 10.1539C21.4231 11.3076 20.9941 13.0235 20.1362 16.4553C19.5905 18.638 19.3176 19.7293 18.5039 20.3647C17.6901 21.0001 16.5652 21.0001 14.3153 21.0001H9.68462C7.43476 21.0001 6.30983 21.0001 5.49605 20.3647C4.68227 19.7293 4.40943 18.638 3.86376 16.4553C3.00581 13.0235 2.57684 11.3076 3.47767 10.1539C3.48555 10.1438 3.4935 10.1338 3.50152 10.1239L3.7819 9.28271L4.52384 6.56145C4.76092 5.6898 4.93316 5.0565 5.30009 4.56242C5.66381 4.07266 6.15802 3.69516 6.72621 3.4731C7.20175 3.28724 7.73447 3.25586 8.41799 3.25089ZM8.41951 4.75231C7.75763 4.759 7.49204 4.78427 7.27224 4.87018C6.96629 4.98976 6.70018 5.19303 6.50433 5.45674C6.32822 5.69388 6.22488 6.0252 5.93398 7.09206L5.36442 9.18091C6.38451 9.00012 7.77753 9.00012 9.68462 9.00012H14.3153C16.2224 9.00012 17.6155 9.00012 18.6356 9.18092L18.066 7.09206C17.7751 6.0252 17.6718 5.69388 17.4957 5.45674C17.2998 5.19303 17.0337 4.98976 16.7278 4.87018C16.508 4.78427 16.2424 4.759 15.5805 4.75231C15.2992 5.3423 14.6972 5.75 14 5.75H10C9.30281 5.75 8.70084 5.3423 8.41951 4.75231Z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Bell => Some((
                "<g fill=\"currentColor\"><path d=\"M8.35179 20.2418C9.19288 21.311 10.5142 22 12 22C13.4858 22 14.8071 21.311 15.6482 20.2418C13.2264 20.57 10.7736 20.57 8.35179 20.2418Z\"/><path d=\"M18.7491 9V9.7041C18.7491 10.5491 18.9903 11.3752 19.4422 12.0782L20.5496 13.8012C21.5612 15.3749 20.789 17.5139 19.0296 18.0116C14.4273 19.3134 9.57274 19.3134 4.97036 18.0116C3.21105 17.5139 2.43882 15.3749 3.45036 13.8012L4.5578 12.0782C5.00972 11.3752 5.25087 10.5491 5.25087 9.7041V9C5.25087 5.13401 8.27256 2 12 2C15.7274 2 18.7491 5.13401 18.7491 9Z\"/></g>",
                24,
                24,
            )),
            Icon::Calendar => Some((
                "<g fill=\"currentColor\"><path d=\"M7.75 2.5C7.75 2.08579 7.41421 1.75 7 1.75C6.58579 1.75 6.25 2.08579 6.25 2.5V4.07926C4.81067 4.19451 3.86577 4.47737 3.17157 5.17157C2.47737 5.86577 2.19451 6.81067 2.07926 8.25H21.9207C21.8055 6.81067 21.5226 5.86577 20.8284 5.17157C20.1342 4.47737 19.1893 4.19451 17.75 4.07926V2.5C17.75 2.08579 17.4142 1.75 17 1.75C16.5858 1.75 16.25 2.08579 16.25 2.5V4.0129C15.5847 4 14.839 4 14 4H10C9.16097 4 8.41527 4 7.75 4.0129V2.5Z\"/><path fill-rule=\"evenodd\" d=\"M2 12C2 11.161 2 10.4153 2.0129 9.75H21.9871C22 10.4153 22 11.161 22 12V14C22 17.7712 22 19.6569 20.8284 20.8284C19.6569 22 17.7712 22 14 22H10C6.22876 22 4.34315 22 3.17157 20.8284C2 19.6569 2 17.7712 2 14V12ZM17 14C17.5523 14 18 13.5523 18 13C18 12.4477 17.5523 12 17 12C16.4477 12 16 12.4477 16 13C16 13.5523 16.4477 14 17 14ZM17 18C17.5523 18 18 17.5523 18 17C18 16.4477 17.5523 16 17 16C16.4477 16 16 16.4477 16 17C16 17.5523 16.4477 18 17 18ZM13 13C13 13.5523 12.5523 14 12 14C11.4477 14 11 13.5523 11 13C11 12.4477 11.4477 12 12 12C12.5523 12 13 12.4477 13 13ZM13 17C13 17.5523 12.5523 18 12 18C11.4477 18 11 17.5523 11 17C11 16.4477 11.4477 16 12 16C12.5523 16 13 16.4477 13 17ZM7 14C7.55228 14 8 13.5523 8 13C8 12.4477 7.55228 12 7 12C6.44772 12 6 12.4477 6 13C6 13.5523 6.44772 14 7 14ZM7 18C7.55228 18 8 17.5523 8 17C8 16.4477 7.55228 16 7 16C6.44772 16 6 16.4477 6 17C6 17.5523 6.44772 18 7 18Z\" clip-rule=\"evenodd\"/></g>",
                24,
                24,
            )),
            Icon::Envelope => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M3.17157 5.17157C2 6.34315 2 8.22876 2 12C2 15.7712 2 17.6569 3.17157 18.8284C4.34315 20 6.22876 20 10 20H14C17.7712 20 19.6569 20 20.8284 18.8284C22 17.6569 22 15.7712 22 12C22 8.22876 22 6.34315 20.8284 5.17157C19.6569 4 17.7712 4 14 4H10C6.22876 4 4.34315 4 3.17157 5.17157ZM18.5762 7.51986C18.8413 7.83807 18.7983 8.31099 18.4801 8.57617L16.2837 10.4066C15.3973 11.1452 14.6789 11.7439 14.0448 12.1517C13.3843 12.5765 12.7411 12.8449 12 12.8449C11.2589 12.8449 10.6157 12.5765 9.95518 12.1517C9.32112 11.7439 8.60271 11.1452 7.71636 10.4066L5.51986 8.57617C5.20165 8.31099 5.15866 7.83807 5.42383 7.51986C5.68901 7.20165 6.16193 7.15866 6.48014 7.42383L8.63903 9.22291C9.57199 10.0004 10.2197 10.5384 10.7666 10.8901C11.2959 11.2306 11.6549 11.3449 12 11.3449C12.3451 11.3449 12.7041 11.2306 13.2334 10.8901C13.7803 10.5384 14.428 10.0004 15.361 9.22291L17.5199 7.42383C17.8381 7.15866 18.311 7.20165 18.5762 7.51986Z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Phone => Some((
                "<path fill=\"currentColor\" d=\"M16.5562 12.9062L16.1007 13.359C16.1007 13.359 15.0181 14.4355 12.0631 11.4972C9.10812 8.55901 10.1907 7.48257 10.1907 7.48257L10.4775 7.19738C11.1841 6.49484 11.2507 5.36691 10.6342 4.54348L9.37326 2.85908C8.61028 1.83992 7.13596 1.70529 6.26145 2.57483L4.69185 4.13552C4.25823 4.56668 3.96765 5.12559 4.00289 5.74561C4.09304 7.33182 4.81071 10.7447 8.81536 14.7266C13.0621 18.9492 17.0468 19.117 18.6763 18.9651C19.1917 18.9171 19.6399 18.6546 20.0011 18.2954L21.4217 16.883C22.3806 15.9295 22.1102 14.2949 20.8833 13.628L18.9728 12.5894C18.1672 12.1515 17.1858 12.2801 16.5562 12.9062Z\"/>",
                24,
                24,
            )),
            Icon::Moon => Some((
                "<path fill=\"currentColor\" d=\"M12 22C17.5228 22 22 17.5228 22 12C22 11.5373 21.3065 11.4608 21.0672 11.8568C19.9289 13.7406 17.8615 15 15.5 15C11.9101 15 9 12.0899 9 8.5C9 6.13845 10.2594 4.07105 12.1432 2.93276C12.5392 2.69347 12.4627 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z\"/>",
                24,
                24,
            )),
            Icon::Sun => Some((
                "<g fill=\"currentColor\"><path d=\"M18 12C18 15.3137 15.3137 18 12 18C8.68629 18 6 15.3137 6 12C6 8.68629 8.68629 6 12 6C15.3137 6 18 8.68629 18 12Z\"/><path fill-rule=\"evenodd\" d=\"M12 1.25C12.4142 1.25 12.75 1.58579 12.75 2V3C12.75 3.41421 12.4142 3.75 12 3.75C11.5858 3.75 11.25 3.41421 11.25 3V2C11.25 1.58579 11.5858 1.25 12 1.25ZM4.39861 4.39861C4.6915 4.10572 5.16638 4.10572 5.45927 4.39861L5.85211 4.79145C6.145 5.08434 6.145 5.55921 5.85211 5.85211C5.55921 6.145 5.08434 6.145 4.79145 5.85211L4.39861 5.45927C4.10572 5.16638 4.10572 4.6915 4.39861 4.39861ZM19.6011 4.39887C19.894 4.69176 19.894 5.16664 19.6011 5.45953L19.2083 5.85237C18.9154 6.14526 18.4405 6.14526 18.1476 5.85237C17.8547 5.55947 17.8547 5.0846 18.1476 4.79171L18.5405 4.39887C18.8334 4.10598 19.3082 4.10598 19.6011 4.39887ZM1.25 12C1.25 11.5858 1.58579 11.25 2 11.25H3C3.41421 11.25 3.75 11.5858 3.75 12C3.75 12.4142 3.41421 12.75 3 12.75H2C1.58579 12.75 1.25 12.4142 1.25 12ZM20.25 12C20.25 11.5858 20.5858 11.25 21 11.25H22C22.4142 11.25 22.75 11.5858 22.75 12C22.75 12.4142 22.4142 12.75 22 12.75H21C20.5858 12.75 20.25 12.4142 20.25 12ZM18.1476 18.1476C18.4405 17.8547 18.9154 17.8547 19.2083 18.1476L19.6011 18.5405C19.894 18.8334 19.894 19.3082 19.6011 19.6011C19.3082 19.894 18.8334 19.894 18.5405 19.6011L18.1476 19.2083C17.8547 18.9154 17.8547 18.4405 18.1476 18.1476ZM5.85211 18.1479C6.145 18.4408 6.145 18.9157 5.85211 19.2086L5.45927 19.6014C5.16638 19.8943 4.6915 19.8943 4.39861 19.6014C4.10572 19.3085 4.10572 18.8336 4.39861 18.5407L4.79145 18.1479C5.08434 17.855 5.55921 17.855 5.85211 18.1479ZM12 20.25C12.4142 20.25 12.75 20.5858 12.75 21V22C12.75 22.4142 12.4142 22.75 12 22.75C11.5858 22.75 11.25 22.4142 11.25 22V21C11.25 20.5858 11.5858 20.25 12 20.25Z\" clip-rule=\"evenodd\"/></g>",
                24,
                24,
            )),
            Icon::Cloud => Some((
                "<path fill=\"currentColor\" d=\"M16.2857 20C19.4416 20 22 17.4717 22 14.3529C22 11.8811 20.393 9.78024 18.1551 9.01498C17.8371 6.19371 15.4159 4 12.4762 4C9.32028 4 6.7619 6.52827 6.7619 9.64706C6.7619 10.3369 6.88706 10.9978 7.11616 11.6089C6.8475 11.5567 6.56983 11.5294 6.28571 11.5294C3.91878 11.5294 2 13.4256 2 15.7647C2 18.1038 3.91878 20 6.28571 20H16.2857Z\"/>",
                24,
                24,
            )),
            Icon::Film => Some((
                "<g fill=\"currentColor\"><path d=\"M10.0957 2.00445C6.62194 2.03072 4.71683 2.2121 3.46447 3.46447C2.6068 4.32213 2.25143 5.48593 2.10418 7.25002H6.59861L10.0957 2.00445Z\"/><path d=\"M2.02644 8.75002C2 9.68875 2 10.7633 2 12C2 16.714 2 19.0711 3.46447 20.5355C4.92893 22 7.28595 22 12 22C16.714 22 19.0711 22 20.5355 20.5355C22 19.0711 22 16.714 22 12C22 10.7633 22 9.68875 21.9736 8.75002H2.02644Z\"/><path d=\"M21.8958 7.25002C21.7486 5.48593 21.3932 4.32213 20.5355 3.46447C19.9382 2.86714 19.1924 2.51345 18.1987 2.30403L14.9014 7.25002H21.8958Z\"/><path d=\"M16.5401 2.08783C15.3293 2 13.8452 2 12 2C11.967 2 11.9342 2 11.9014 2L8.40139 7.25002H13.0986L16.5401 2.08783Z\"/></g>",
                24,
                24,
            )),
            Icon::Pencil => Some((
                "<g fill=\"currentColor\"><path d=\"M11.4001 18.1612L11.4001 18.1612L18.796 10.7653C17.7894 10.3464 16.5972 9.6582 15.4697 8.53068C14.342 7.40298 13.6537 6.21058 13.2348 5.2039L5.83882 12.5999L5.83879 12.5999C5.26166 13.1771 4.97307 13.4657 4.7249 13.7838C4.43213 14.1592 4.18114 14.5653 3.97634 14.995C3.80273 15.3593 3.67368 15.7465 3.41556 16.5208L2.05445 20.6042C1.92743 20.9852 2.0266 21.4053 2.31063 21.6894C2.59466 21.9734 3.01478 22.0726 3.39584 21.9456L7.47918 20.5844C8.25351 20.3263 8.6407 20.1973 9.00498 20.0237C9.43469 19.8189 9.84082 19.5679 10.2162 19.2751C10.5343 19.0269 10.823 18.7383 11.4001 18.1612Z\"/><path d=\"M20.8482 8.71306C22.3839 7.17735 22.3839 4.68748 20.8482 3.15178C19.3125 1.61607 16.8226 1.61607 15.2869 3.15178L14.3999 4.03882C14.4121 4.0755 14.4246 4.11268 14.4377 4.15035C14.7628 5.0875 15.3763 6.31601 16.5303 7.47002C17.6843 8.62403 18.9128 9.23749 19.85 9.56262C19.8875 9.57563 19.9245 9.58817 19.961 9.60026L20.8482 8.71306Z\"/></g>",
                24,
                24,
            )),
            Icon::Key => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M22 8.29344C22 11.7692 19.1708 14.5869 15.6807 14.5869C15.0439 14.5869 13.5939 14.4405 12.8885 13.8551L12.0067 14.7333C11.4883 15.2496 11.6283 15.4016 11.8589 15.652C11.9551 15.7565 12.0672 15.8781 12.1537 16.0505C12.1537 16.0505 12.8885 17.075 12.1537 18.0995C11.7128 18.6849 10.4783 19.5045 9.06754 18.0995L8.77362 18.3922C8.77362 18.3922 9.65538 19.4167 8.92058 20.4412C8.4797 21.0267 7.30403 21.6121 6.27531 20.5876L5.2466 21.6121C4.54119 22.3146 3.67905 21.9048 3.33616 21.6121L2.45441 20.7339C1.63143 19.9143 2.1115 19.0264 2.45441 18.6849L10.0963 11.0743C10.0963 11.0743 9.3615 9.90338 9.3615 8.29344C9.3615 4.81767 12.1907 2 15.6807 2C19.1708 2 22 4.81767 22 8.29344ZM15.681 10.4889C16.8984 10.4889 17.8853 9.50601 17.8853 8.29353C17.8853 7.08105 16.8984 6.09814 15.681 6.09814C14.4635 6.09814 13.4766 7.08105 13.4766 8.29353C13.4766 9.50601 14.4635 10.4889 15.681 10.4889Z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Lock => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M5.25 10.0546V8C5.25 4.27208 8.27208 1.25 12 1.25C15.7279 1.25 18.75 4.27208 18.75 8V10.0546C19.8648 10.1379 20.5907 10.348 21.1213 10.8787C22 11.7574 22 13.1716 22 16C22 18.8284 22 20.2426 21.1213 21.1213C20.2426 22 18.8284 22 16 22H8C5.17157 22 3.75736 22 2.87868 21.1213C2 20.2426 2 18.8284 2 16C2 13.1716 2 11.7574 2.87868 10.8787C3.40931 10.348 4.13525 10.1379 5.25 10.0546ZM6.75 8C6.75 5.10051 9.10051 2.75 12 2.75C14.8995 2.75 17.25 5.10051 17.25 8V10.0036C16.867 10 16.4515 10 16 10H8C7.54849 10 7.13301 10 6.75 10.0036V8Z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Brain => Some((
                "<g fill=\"currentColor\" fill-rule=\"evenodd\" clip-rule=\"evenodd\"><path d=\"M8.99932 2.00048C10.4323 1.97209 12.0013 3.2091 12.0013 4.60204V18.7407C12.0013 19.1371 11.8495 22.0301 8.24932 21.9995C3.74874 21.961 0.372906 16.1832 2.82354 12.1382C2.07338 10.7127 1.40111 7.4605 4.71221 5.8579C4.67391 3.65294 6.96702 2.04091 8.99932 2.00048ZM6.20049 15.5151C6.07837 15.1267 5.65359 14.9138 5.25225 15.0405C4.85093 15.1674 4.6235 15.5857 4.74542 15.9741C5.29382 17.7181 7.19108 18.6067 8.96319 17.9497C9.35776 17.8032 9.56242 17.3742 9.42022 16.9927C9.27781 16.6114 8.84216 16.4216 8.44757 16.5679C7.49365 16.9213 6.49608 16.4536 6.20049 15.5151ZM7.09112 8.45653C6.96899 8.0681 6.54424 7.85515 6.14288 7.98192C5.74179 8.10899 5.51514 8.52722 5.63702 8.91552C6.1855 10.6595 8.08267 11.5482 9.85479 10.8911C10.2492 10.7445 10.453 10.3155 10.3108 9.93407C10.1684 9.55287 9.73275 9.36297 9.33819 9.50927C8.38434 9.86246 7.38662 9.39502 7.09112 8.45653Z\"/><path d=\"M15.002 2.00047C17.0343 2.04074 19.3283 3.65282 19.29 5.85789C22.6009 7.46053 21.9279 10.7127 21.1777 12.1382C23.6287 16.1832 20.2536 21.961 15.7529 21.9995C12.1525 22.0302 12.001 19.1371 12.001 18.7407V4.60203C12.001 3.20922 13.5691 1.9723 15.002 2.00047ZM18.75 15.0405C18.3486 14.9137 17.9239 15.1267 17.8018 15.5151C17.5062 16.4536 16.5086 16.9212 15.5547 16.5679C15.1601 16.4215 14.7245 16.6114 14.582 16.9927C14.4398 17.3742 14.6445 17.8032 15.0391 17.9497C16.811 18.6065 18.7074 17.7179 19.2559 15.9741C19.3778 15.5857 19.1512 15.1675 18.75 15.0405ZM17.8584 7.98192C17.4571 7.8553 17.0332 8.06817 16.9111 8.45653C16.6156 9.39519 15.6171 9.86271 14.6631 9.50926C14.2685 9.36309 13.8328 9.55279 13.6904 9.93407C13.5482 10.3156 13.7529 10.7446 14.1475 10.8911C15.9196 11.5482 17.8168 10.6595 18.3652 8.91551C18.4871 8.52709 18.2597 8.10884 17.8584 7.98192Z\"/></g>",
                24,
                24,
            )),
            Icon::Cat => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M11.7501 6.40636C10.2698 6.40636 10.1222 6.5625 9.3561 6.5625C8.71769 6.5625 6.80245 5 5.84485 5C4.88724 5 3.77004 5.5625 3.77004 7.1875V9.0625C3.77197 9.55469 3.95081 11.0634 4.65075 10.6602C3.82323 11.6382 3.73963 12.7786 3.751 13.8826C3.52812 13.947 3.30072 14.0196 3.08003 14.095C2.39614 14.3289 1.67085 14.6271 1.3432 14.8387C0.995241 15.0634 0.895339 15.5277 1.12006 15.8756C1.34478 16.2236 1.80903 16.3235 2.15698 16.0988C2.3132 15.9979 2.87823 15.7493 3.56532 15.5144C3.64124 15.4884 3.71731 15.4631 3.79298 15.4386C3.83925 15.8724 3.95408 16.2684 4.12478 16.6292L4.1012 16.6416C3.69148 16.8581 3.3113 17.1067 3.06889 17.2652C3.02694 17.2926 2.98912 17.3173 2.95599 17.3387C2.60803 17.5634 2.50813 18.0277 2.73285 18.3756C2.95757 18.7236 3.42182 18.8235 3.76978 18.5988C3.8109 18.5722 3.85472 18.5436 3.90097 18.5134C4.1463 18.3533 4.45999 18.1485 4.80199 17.9678C4.88218 17.9254 4.95935 17.887 5.03317 17.8524C6.76347 19.4748 9.86991 20 11.7501 20C13.6302 20 16.7367 19.4748 18.467 17.8524C18.5408 17.887 18.6179 17.9254 18.6981 17.9678C19.0401 18.1485 19.3538 18.3533 19.5991 18.5134C19.6454 18.5436 19.6892 18.5722 19.7303 18.5988C20.0783 18.8235 20.5425 18.7236 20.7673 18.3756C20.992 18.0277 20.8921 17.5634 20.5441 17.3387C20.511 17.3173 20.4732 17.2926 20.4312 17.2652C20.1888 17.1067 19.8086 16.8581 19.3989 16.6416L19.3754 16.6292C19.5461 16.2683 19.6609 15.8724 19.7072 15.4385C19.783 15.463 19.8592 15.4883 19.9352 15.5144C20.6223 15.7493 21.1874 15.9979 21.3436 16.0988C21.6915 16.3235 22.1558 16.2236 22.3805 15.8756C22.6052 15.5277 22.5053 15.0634 22.1574 14.8387C21.8297 14.6271 21.1044 14.3289 20.4205 14.095C20.1997 14.0195 19.9722 13.947 19.7492 13.8825C19.7605 12.7785 19.6769 11.6382 18.8494 10.6602C19.5494 11.0634 19.7282 9.55469 19.7302 9.0625V7.18761C19.7302 5.56261 18.6129 5.00011 17.6553 5.00011C16.6977 5.00011 14.7825 6.5625 14.1441 6.5625C13.378 6.5625 13.2305 6.40636 11.7501 6.40636ZM11.0745 15.6004C11.2771 15.5314 11.5162 15.5 11.7501 15.5C11.984 15.5 12.2231 15.5314 12.4257 15.6004C12.5251 15.6342 12.6467 15.6876 12.7537 15.7738C12.8612 15.8603 13.0001 16.0206 13.0001 16.25C13.0001 16.4794 12.8612 16.6397 12.7537 16.7262C12.6467 16.8124 12.5251 16.8658 12.4257 16.8996C12.2231 16.9686 11.984 17 11.7501 17C11.5162 17 11.2771 16.9686 11.0745 16.8996C10.9751 16.8658 10.8535 16.8124 10.7464 16.7262C10.6389 16.6397 10.5001 16.4794 10.5001 16.25C10.5001 16.0206 10.6389 15.8603 10.7464 15.7738C10.8535 15.6876 10.9751 15.6342 11.0745 15.6004ZM13.9201 12.5005C14.0566 12.2721 14.326 12 14.7301 12C15.1342 12 15.4036 12.2721 15.54 12.5005C15.6823 12.7387 15.7501 13.0274 15.7501 13.3125C15.7501 13.5976 15.6823 13.8863 15.54 14.1245C15.4036 14.3529 15.1342 14.625 14.7301 14.625C14.326 14.625 14.0566 14.3529 13.9201 14.1245C13.7778 13.8863 13.7101 13.5976 13.7101 13.3125C13.7101 13.0274 13.7778 12.7387 13.9201 12.5005ZM7.96016 12.5005C8.09658 12.2721 8.36599 12 8.7701 12C9.17421 12 9.44362 12.2721 9.58004 12.5005C9.72234 12.7387 9.79011 13.0274 9.79011 13.3125C9.79011 13.5976 9.72234 13.8863 9.58004 14.1245C9.44362 14.3529 9.17421 14.625 8.7701 14.625C8.36599 14.625 8.09658 14.3529 7.96016 14.1245C7.81786 13.8863 7.75009 13.5976 7.75009 13.3125C7.75009 13.0274 7.81786 12.7387 7.96016 12.5005Z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Palette => Some((
                "<g fill=\"currentColor\"><path fill-rule=\"evenodd\" d=\"M10 6V18C10 19.4001 10 20.1002 9.72752 20.635C9.48783 21.1054 9.10538 21.4878 8.63498 21.7275C8.1002 22 7.40013 22 6 22C4.59987 22 3.8998 22 3.36502 21.7275C2.89462 21.4878 2.51217 21.1054 2.27248 20.635C2 20.1002 2 19.4001 2 18V6C2 4.59987 2 3.8998 2.27248 3.36502C2.51217 2.89462 2.89462 2.51217 3.36502 2.27248C3.8998 2 4.59987 2 6 2C7.40013 2 8.1002 2 8.63498 2.27248C9.10538 2.51217 9.48783 2.89462 9.72752 3.36502C10 3.8998 10 4.59987 10 6ZM7 19.75C7.41421 19.75 7.75 19.4142 7.75 19C7.75 18.5858 7.41421 18.25 7 18.25H5C4.58579 18.25 4.25 18.5858 4.25 19C4.25 19.4142 4.58579 19.75 5 19.75H7Z\" clip-rule=\"evenodd\"/><path d=\"M19.0599 10.6144L13.2219 16.704C12.492 17.4653 12.1271 17.8459 11.8135 17.7199C11.5 17.5939 11.5 17.0666 11.5 16.0119L11.5 7.7738C11.5012 7.11381 11.7633 6.48107 12.2291 6.01357L13.2839 4.95882L13.7141 4.62987C14.7183 3.86212 15.2204 3.47825 15.7673 3.3603C16.2175 3.26322 16.6857 3.29236 17.1204 3.4445C17.6484 3.62934 18.099 4.0725 19.0003 4.95883C19.9999 5.95839 20.4997 6.45818 20.685 7.03056C20.843 7.51871 20.847 8.04366 20.6964 8.53417C20.5199 9.10931 20.0332 9.61101 19.0599 10.6144Z\"/><path d=\"M12.7897 22H17.8994C19.2995 22 19.9996 22 20.5344 21.7275C21.0048 21.4878 21.3872 21.1054 21.6269 20.635C21.8994 20.1002 21.8994 19.4001 21.8994 18C21.8994 16.5999 21.8994 15.8998 21.6269 15.365C21.3872 14.8946 21.0048 14.5122 20.5344 14.2725C19.9996 14 19.2995 14 17.8994 14H17.6797L11.878 19.798C11.636 20.0399 11.5 20.3391 11.5 20.6813C11.5 21.3936 12.0774 22 12.7897 22Z\"/></g>",
                24,
                24,
            )),
            Icon::Trophy => Some((
                "<g fill=\"currentColor\"><path d=\"M21.9999 8.16234L21.9999 8.23487C21.9999 9.09561 21.9999 9.52598 21.7927 9.8781C21.5855 10.2302 21.2093 10.4392 20.4569 10.8572L19.6636 11.298C20.2102 9.44984 20.3926 7.46414 20.4601 5.76597C20.4629 5.69316 20.4662 5.61945 20.4695 5.54497L20.4718 5.49279C21.1231 5.71896 21.4887 5.88758 21.7168 6.20408C22 6.59692 22 7.11873 21.9999 8.16234Z\"/><path d=\"M2 8.16234L2 8.23487C2.00003 9.09561 2.00004 9.52598 2.20723 9.8781C2.41442 10.2302 2.79063 10.4392 3.54305 10.8572L4.33681 11.2982C3.79007 9.45001 3.60767 7.46422 3.54025 5.76597C3.53736 5.69316 3.5341 5.61945 3.53081 5.54497L3.5285 5.49266C2.87701 5.7189 2.51126 5.88752 2.2831 6.20408C1.99996 6.59692 1.99997 7.11873 2 8.16234Z\"/><path fill-rule=\"evenodd\" d=\"M16.3771 2.34674C15.2531 2.15709 13.7837 2 12.0002 2C10.2166 2 8.74724 2.15709 7.62318 2.34674C6.48445 2.53887 5.91508 2.63494 5.43937 3.22083C4.96365 3.80673 4.98879 4.43998 5.03907 5.70647C5.21169 10.0544 6.14996 15.4851 11.25 15.9657V19.5H9.8198C9.34312 19.5 8.93271 19.8365 8.83922 20.3039L8.65 21.25H6C5.58579 21.25 5.25 21.5858 5.25 22C5.25 22.4142 5.58579 22.75 6 22.75H18C18.4142 22.75 18.75 22.4142 18.75 22C18.75 21.5858 18.4142 21.25 18 21.25H15.35L15.1608 20.3039C15.0673 19.8365 14.6569 19.5 14.1802 19.5H12.75V15.9657C17.8503 15.4853 18.7886 10.0545 18.9612 5.70647C19.0115 4.43998 19.0367 3.80673 18.5609 3.22083C18.0852 2.63494 17.5159 2.53887 16.3771 2.34674ZM12.787 5.80711C13.0673 5.9232 13.25 6.19668 13.25 6.50002V10.5C13.25 10.9142 12.9142 11.25 12.5 11.25C12.0858 11.25 11.75 10.9142 11.75 10.5V8.31068L11.5303 8.53035C11.2374 8.82325 10.7626 8.82325 10.4697 8.53035C10.1768 8.23746 10.1768 7.76258 10.4697 7.46969L11.9697 5.96969C12.1842 5.75519 12.5068 5.69103 12.787 5.80711Z\" clip-rule=\"evenodd\"/></g>",
                24,
                24,
            )),
            Icon::Rocket => Some((
                "<g fill=\"currentColor\"><path d=\"M9.03429 5.96305L6.49114 8.49856C6.02369 8.9646 5.59488 9.3921 5.25624 9.77856C5.03877 10.0267 4.82145 10.2984 4.63737 10.5985L4.61259 10.5738C4.56555 10.5269 4.54201 10.5034 4.51839 10.4805C4.07636 10.0516 3.55641 9.71062 2.98636 9.47575C2.9559 9.4632 2.92498 9.45095 2.86314 9.42645L2.48449 9.27641C1.97153 9.07315 1.83482 8.41279 2.22514 8.02365C3.34535 6.90684 4.69032 5.56594 5.33941 5.29662C5.91185 5.05911 6.53023 4.98008 7.12664 5.06822C7.67311 5.14898 8.19006 5.42968 9.03429 5.96305Z\"/><path d=\"M13.3767 19.3132C13.5816 19.5212 13.7177 19.6681 13.8408 19.8251C14.0031 20.0322 14.1483 20.2523 14.2748 20.4829C14.4172 20.7426 14.5278 21.02 14.749 21.5748C14.929 22.0265 15.5272 22.1459 15.8746 21.7995L15.9586 21.7157C17.0788 20.5988 18.4237 19.2579 18.6938 18.6108C18.9321 18.04 19.0113 17.4235 18.9229 16.8289C18.8419 16.2841 18.5605 15.7688 18.0256 14.9273L15.474 17.4713C14.9959 17.9479 14.5576 18.385 14.1612 18.7273C13.9236 18.9325 13.6637 19.1376 13.3767 19.3132Z\"/><path fill-rule=\"evenodd\" d=\"M14.4467 16.3769L20.2935 10.5476C21.1356 9.70811 21.5566 9.28836 21.7783 8.75458C22.0001 8.22081 22.0001 7.62719 22.0001 6.43996V5.87277C22.0001 4.04713 22.0001 3.13431 21.4312 2.56715C20.8624 2 19.9468 2 18.1157 2H17.5468C16.356 2 15.7606 2 15.2252 2.2211C14.6898 2.4422 14.2688 2.86195 13.4268 3.70146L7.57991 9.53078C6.59599 10.5117 5.98591 11.12 5.74966 11.7075C5.67502 11.8931 5.6377 12.0767 5.6377 12.2692C5.6377 13.0713 6.2851 13.7168 7.57991 15.0077L7.75393 15.1812L9.79245 13.1123C10.0832 12.8172 10.558 12.8137 10.8531 13.1044C11.1481 13.3951 11.1516 13.87 10.8609 14.1651L8.8162 16.2403L8.95326 16.3769C10.2481 17.6679 10.8955 18.3133 11.7 18.3133C11.8777 18.3133 12.0478 18.2818 12.2189 18.2188C12.8222 17.9966 13.438 17.3826 14.4467 16.3769ZM17.1935 9.5312C16.435 10.2874 15.2053 10.2874 14.4468 9.5312C13.6883 8.775 13.6883 7.54895 14.4468 6.79274C15.2053 6.03653 16.435 6.03653 17.1935 6.79274C17.952 7.54895 17.952 8.775 17.1935 9.5312Z\" clip-rule=\"evenodd\"/></g>",
                24,
                24,
            )),
            Icon::Wine => Some((
                "<g fill=\"currentColor\"><path d=\"M5 4.89474C5 3.8483 5.8483 3 6.89474 3H17.1053C18.1517 3 19 3.8483 19 4.89474V8C19 8.08368 18.9985 8.16701 18.9956 8.24997C18.9032 8.25046 18.8094 8.26813 18.7185 8.30484L18.7148 8.30632L18.6981 8.31297C18.683 8.31899 18.6598 8.3281 18.6295 8.33989C18.5688 8.36346 18.4792 8.39769 18.3666 8.43924C18.1409 8.52248 17.8245 8.6344 17.4626 8.74874C16.722 8.98276 15.8541 9.20628 15.1885 9.24402C14.1043 9.3055 13.3288 8.88551 12.3672 8.3459L12.3243 8.32176C11.3911 7.79786 10.2738 7.17056 8.72697 7.25827C7.86456 7.30717 6.84781 7.58009 6.08585 7.82084C5.69641 7.94389 5.35704 8.06394 5.11471 8.15334C5.0747 8.16809 5.03728 8.18204 5.00266 8.19504C5.00089 8.13024 5 8.06523 5 8V4.89474Z\"/><path d=\"M5.21268 9.7192C5.91966 12.519 8.31356 14.6475 11.25 14.9603V20.2499H8C7.58579 20.2499 7.25 20.5857 7.25 20.9999C7.25 21.4141 7.58579 21.7499 8 21.7499H16C16.4142 21.7499 16.75 21.4141 16.75 20.9999C16.75 20.5857 16.4142 20.2499 16 20.2499H12.75V14.9603C15.6229 14.6543 17.9765 12.6103 18.7391 9.90002C18.514 9.98118 18.2308 10.0791 17.9145 10.179C17.1526 10.4198 16.1358 10.6927 15.2734 10.7416C13.7266 10.8293 12.6093 10.202 11.6761 9.67812L11.6332 9.65399C10.6716 9.11438 9.89609 8.69439 8.81189 8.75586C8.1463 8.7936 7.27844 9.01712 6.53778 9.25115C6.17591 9.36548 5.85947 9.47741 5.63383 9.56064C5.52118 9.60219 5.43164 9.63642 5.3709 9.66C5.34055 9.67178 5.31743 9.68089 5.30225 9.68691L5.28556 9.69356L5.28186 9.69505C5.25896 9.7043 5.23588 9.71234 5.21268 9.7192Z\"/></g>",
                24,
                24,
            )),
            Icon::Medal => Some((
                "<g fill=\"currentColor\"><path fill-rule=\"evenodd\" d=\"M13.436 5.78311C12.5407 5.29495 11.4588 5.29495 10.5636 5.78311L5.76937 8.39728C4.80539 8.92292 4.20557 9.93319 4.20557 11.0312V15.9688C4.20557 17.0668 4.80539 18.0771 5.76937 18.6027L10.5636 21.2169C11.4588 21.705 12.5407 21.705 13.436 21.2169L18.2302 18.6027C19.1942 18.0771 19.794 17.0668 19.794 15.9688V11.0312C19.794 9.93319 19.1942 8.92292 18.2302 8.39728L13.436 5.78311ZM12 10.5C11.7159 10.5 11.5259 10.8408 11.1459 11.5225L11.0476 11.6989C10.9397 11.8926 10.8857 11.9894 10.8015 12.0533C10.7173 12.1172 10.6125 12.141 10.4028 12.1884L10.2119 12.2316C9.47396 12.3986 9.10501 12.482 9.01723 12.7643C8.92945 13.0466 9.18097 13.3407 9.68403 13.929L9.81418 14.0812C9.95713 14.2483 10.0286 14.3319 10.0608 14.4353C10.0929 14.5387 10.0821 14.6502 10.0605 14.8733L10.0408 15.0763C9.96476 15.8612 9.92674 16.2536 10.1565 16.4281C10.3864 16.6025 10.7318 16.4435 11.4227 16.1254L11.6014 16.0431C11.7978 15.9527 11.8959 15.9075 12 15.9075C12.1041 15.9075 12.2022 15.9527 12.3986 16.0431L12.5773 16.1254C13.2682 16.4435 13.6136 16.6025 13.8435 16.4281C14.0733 16.2536 14.0352 15.8612 13.9592 15.0763L13.9395 14.8733C13.9179 14.6502 13.9071 14.5387 13.9392 14.4353C13.9714 14.3319 14.0429 14.2483 14.1858 14.0812L14.316 13.929C14.819 13.3407 15.0706 13.0466 14.9828 12.7643C14.895 12.482 14.526 12.3986 13.7881 12.2316L13.5972 12.1884C13.3875 12.141 13.2827 12.1172 13.1985 12.0533C13.1143 11.9894 13.0603 11.8926 12.9524 11.6989L12.8541 11.5225C12.4741 10.8408 12.2841 10.5 12 10.5Z\" clip-rule=\"evenodd\"/><path d=\"M11 2H13C14.8856 2 15.8284 2 16.4142 2.58579C17 3.17157 17 4.11438 17 6V6.01797L14.1541 4.46616C12.8112 3.73394 11.1884 3.73393 9.84551 4.46616L7 6.01775V6C7 4.11438 7 3.17157 7.58579 2.58579C8.17157 2 9.11438 2 11 2Z\"/></g>",
                24,
                24,
            )),
            Icon::Truck => Some((
                "<g fill=\"currentColor\"><path d=\"M9.56443 8.73049L10.0789 10.5926C10.5639 12.3481 10.8064 13.2259 11.5194 13.6252C12.2323 14.0244 13.1374 13.7892 14.9474 13.3188L16.8673 12.8199C18.6774 12.3495 19.5824 12.1143 19.9941 11.4227C20.4057 10.7312 20.1632 9.85344 19.6782 8.09788L19.1638 6.2358C18.6788 4.48023 18.4363 3.60244 17.7233 3.20319C17.0103 2.80394 16.1052 3.03915 14.2952 3.50955L12.3753 4.00849C10.5652 4.47889 9.66021 4.71409 9.24856 5.40562C8.83692 6.09714 9.07943 6.97493 9.56443 8.73049Z\"/><path d=\"M2.27749 5.24694C2.38823 4.84781 2.80157 4.61402 3.2007 4.72476L4.9044 5.19744C5.82129 5.45183 6.5469 6.15866 6.80003 7.07489L8.95106 14.8609L9.10935 15.4075C9.74249 15.6438 10.2863 16.0866 10.6314 16.6747L10.9414 16.579L19.8115 14.2739C20.2124 14.1697 20.6219 14.4102 20.7261 14.8111C20.8303 15.212 20.5897 15.6214 20.1888 15.7256L11.3515 18.0223L11.0228 18.1238C11.0161 19.3947 10.1392 20.5555 8.81236 20.9003C7.22189 21.3136 5.58709 20.3982 5.16092 18.8556C4.73476 17.313 5.67861 15.7274 7.26908 15.3141C7.3479 15.2936 7.42682 15.2764 7.5057 15.2623L5.35419 7.47433C5.24592 7.08242 4.92897 6.76092 4.50338 6.64284L2.79968 6.17016C2.40054 6.05942 2.16675 5.64608 2.27749 5.24694Z\"/></g>",
                24,
                24,
            )),
            Icon::Bag => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M8.25013 6.01489C8.25003 6.00994 8.24998 6.00498 8.24998 6V5C8.24998 2.92893 9.92892 1.25 12 1.25C14.0711 1.25 15.75 2.92893 15.75 5V6C15.75 6.00498 15.7499 6.00994 15.7498 6.01489C17.0371 6.05353 17.8248 6.1924 18.4261 6.69147C19.2593 7.38295 19.4787 8.55339 19.9177 10.8943L20.6677 14.8943C21.2849 18.186 21.5934 19.8318 20.6937 20.9159C19.794 22 18.1195 22 14.7704 22H9.22954C5.88048 22 4.20595 22 3.30624 20.9159C2.40652 19.8318 2.71512 18.186 3.33231 14.8943L4.08231 10.8943C4.52122 8.55339 4.74068 7.38295 5.57386 6.69147C6.17521 6.1924 6.96287 6.05353 8.25013 6.01489ZM9.74998 5C9.74998 3.75736 10.7573 2.75 12 2.75C13.2426 2.75 14.25 3.75736 14.25 5V6C14.25 5.99999 14.25 6.00001 14.25 6C14.1747 5.99998 14.0982 6 14.0204 6H9.97954C9.90176 6 9.82525 6 9.74998 6.00002C9.74998 6.00002 9.74998 6.00003 9.74998 6.00002V5Z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Movie => Some((
                "<g fill=\"currentColor\"><path d=\"M10.0957 2.00445C6.62194 2.03072 4.71683 2.2121 3.46447 3.46447C2.6068 4.32213 2.25143 5.48593 2.10418 7.25002H6.59861L10.0957 2.00445Z\"/><path d=\"M2.02644 8.75002C2 9.68875 2 10.7633 2 12C2 16.714 2 19.0711 3.46447 20.5355C4.92893 22 7.28595 22 12 22C16.714 22 19.0711 22 20.5355 20.5355C22 19.0711 22 16.714 22 12C22 10.7633 22 9.68875 21.9736 8.75002H2.02644Z\"/><path d=\"M21.8958 7.25002C21.7486 5.48593 21.3932 4.32213 20.5355 3.46447C19.9382 2.86714 19.1924 2.51345 18.1987 2.30403L14.9014 7.25002H21.8958Z\"/><path d=\"M16.5401 2.08783C15.3293 2 13.8452 2 12 2C11.967 2 11.9342 2 11.9014 2L8.40139 7.25002H13.0986L16.5401 2.08783Z\"/></g>",
                24,
                24,
            )),
            Icon::Bookmark => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M21 11.0975V16.0909C21 19.1875 21 20.7358 20.2659 21.4123C19.9158 21.735 19.4739 21.9377 19.0031 21.9915C18.016 22.1045 16.8633 21.0849 14.5578 19.0458C13.5388 18.1445 13.0292 17.6938 12.4397 17.5751C12.1494 17.5166 11.8506 17.5166 11.5603 17.5751C10.9708 17.6938 10.4612 18.1445 9.44216 19.0458C7.13673 21.0849 5.98402 22.1045 4.99692 21.9915C4.52615 21.9377 4.08421 21.735 3.73411 21.4123C3 20.7358 3 19.1875 3 16.0909V11.0975C3 6.80891 3 4.6646 4.31802 3.3323C5.63604 2 7.75736 2 12 2C16.2426 2 18.364 2 19.682 3.3323C21 4.6646 21 6.80891 21 11.0975ZM8.25 6C8.25 5.58579 8.58579 5.25 9 5.25H15C15.4142 5.25 15.75 5.58579 15.75 6C15.75 6.41421 15.4142 6.75 15 6.75H9C8.58579 6.75 8.25 6.41421 8.25 6Z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Folder => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M2.06935 5.25839C2 5.62595 2 6.06722 2 6.94975V14C2 17.7712 2 19.6569 3.17157 20.8284C4.34315 22 6.22876 22 10 22H14C17.7712 22 19.6569 22 20.8284 20.8284C22 19.6569 22 17.7712 22 14V11.7979C22 9.16554 22 7.84935 21.2305 6.99383C21.1598 6.91514 21.0849 6.84024 21.0062 6.76946C20.1506 6 18.8345 6 16.2021 6H15.8284C14.6747 6 14.0979 6 13.5604 5.84678C13.2651 5.7626 12.9804 5.64471 12.7121 5.49543C12.2237 5.22367 11.8158 4.81578 11 4L10.4497 3.44975C10.1763 3.17633 10.0396 3.03961 9.89594 2.92051C9.27652 2.40704 8.51665 2.09229 7.71557 2.01738C7.52976 2 7.33642 2 6.94975 2C6.06722 2 5.62595 2 5.25839 2.06935C3.64031 2.37464 2.37464 3.64031 2.06935 5.25839ZM12.25 10C12.25 9.58579 12.5858 9.25 13 9.25H18C18.4142 9.25 18.75 9.58579 18.75 10C18.75 10.4142 18.4142 10.75 18 10.75H13C12.5858 10.75 12.25 10.4142 12.25 10Z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::User => Some((
                "<g fill=\"currentColor\"><circle cx=\"12\" cy=\"6\" r=\"4\"/><path d=\"M20 17.5C20 19.9853 20 22 12 22C4 22 4 19.9853 4 17.5C4 15.0147 7.58172 13 12 13C16.4183 13 20 15.0147 20 17.5Z\"/></g>",
                24,
                24,
            )),
            Icon::Paint => Some((
                "<g fill=\"currentColor\"><path d=\"M6 4.5C6 3.56538 6 3.09808 6.20096 2.75C6.33261 2.52197 6.52197 2.33261 6.75 2.20096C7.09808 2 7.56538 2 8.5 2H15.5C16.4346 2 16.9019 2 17.25 2.20096C17.478 2.33261 17.6674 2.52197 17.799 2.75C18 3.09808 18 3.56538 18 4.5C18 5.43462 18 5.90192 17.799 6.25C17.6674 6.47803 17.478 6.66739 17.25 6.79904C16.9019 7 16.4346 7 15.5 7H8.5C7.56538 7 7.09808 7 6.75 6.79904C6.52197 6.66739 6.33261 6.47803 6.20096 6.25C6 5.90192 6 5.43462 6 4.5Z\"/><path d=\"M5.00214 3.93909C4.84746 4.07647 4.75 4.27687 4.75 4.50002C4.75 4.72318 4.84746 4.92357 5.00214 5.06095C4.99998 4.89619 4.99999 4.72205 5 4.54025V4.4598C4.99999 4.278 4.99998 4.10385 5.00214 3.93909Z\"/><path d=\"M10 16V20C10 20.9428 10 21.4142 10.2929 21.7071C10.5858 22 11.0572 22 12 22C12.9428 22 13.4142 22 13.7071 21.7071C14 21.4142 14 20.9428 14 20V16C14 15.0572 14 14.5858 13.7071 14.2929C13.4142 14 12.9428 14 12 14C11.0572 14 10.5858 14 10.2929 14.2929C10 14.5858 10 15.0572 10 16Z\"/><path d=\"M18.9944 5.25H19.0453C19.4999 5.25 19.8051 5.25037 20.0416 5.26579C20.2718 5.2808 20.3843 5.30776 20.4583 5.3369C20.781 5.46395 21.0364 5.71937 21.1635 6.04208C21.1926 6.11609 21.2196 6.22858 21.2346 6.45878C21.25 6.6953 21.2504 7.00044 21.2504 7.4551C21.2504 8.29243 21.2398 8.52185 21.179 8.69392C21.0747 8.98918 20.8634 9.23455 20.5869 9.38148C20.4257 9.4671 20.2004 9.5116 19.3724 9.63581L15.249 10.2543C14.4763 10.3702 13.8277 10.4675 13.3152 10.6116C12.7721 10.7643 12.2916 10.9923 11.9166 11.4278C11.5334 11.8727 11.3753 12.4055 11.3071 13.0062C11.5113 12.9999 11.727 13 11.9458 13H12.0546C12.3196 13 12.5799 12.9999 12.8208 13.011C12.8708 12.6902 12.9482 12.5286 13.0532 12.4067C13.1626 12.2796 13.3307 12.1654 13.7212 12.0556C14.1321 11.94 14.6865 11.8555 15.5182 11.7307L19.714 11.1014C20.3648 11.0044 20.8716 10.9288 21.2907 10.7061C21.8991 10.3829 22.3639 9.84304 22.5934 9.19346C22.7514 8.74599 22.751 8.23353 22.7504 7.57559L22.7504 7.43098C22.7504 7.00661 22.7504 6.65233 22.7314 6.3612C22.7116 6.05823 22.6691 5.77171 22.5592 5.49258C22.2797 4.78261 21.7177 4.22069 21.0078 3.94117C20.7286 3.83128 20.4421 3.78872 20.1392 3.76897C19.848 3.74999 19.4937 3.74999 19.0694 3.75H18.9944C19.0002 3.96867 19.0002 4.20681 19.0002 4.45976V4.54024C19.0002 4.7932 19.0002 5.03133 18.9944 5.25Z\"/></g>",
                24,
                24,
            )),
            Icon::Train => Some((
                "<g fill=\"currentColor\"><path fill-rule=\"evenodd\" d=\"M6.37562 19.5723C5.89805 19.4016 5.50576 19.1627 5.17157 18.8285C4.19724 17.8542 4.03321 16.386 4.00559 13.7501H19.9944C19.9668 16.386 19.8028 17.8542 18.8284 18.8285C18.4943 19.1626 18.1021 19.4015 17.6247 19.5722L18.671 21.6649C18.8562 22.0353 18.7061 22.4858 18.3356 22.6711C17.9651 22.8563 17.5146 22.7062 17.3293 22.3357L16.1054 19.8878C15.057 20.0001 13.726 20.0001 12 20.0001C10.2742 20.0001 8.94323 20.0001 7.89491 19.8878L6.67098 22.3357C6.48574 22.7062 6.03524 22.8563 5.66475 22.6711C5.29427 22.4858 5.1441 22.0353 5.32934 21.6649L6.37562 19.5723ZM14.75 16.0001C14.75 15.5859 15.0858 15.2501 15.5 15.2501H17C17.4142 15.2501 17.75 15.5859 17.75 16.0001C17.75 16.4143 17.4142 16.7501 17 16.7501H15.5C15.0858 16.7501 14.75 16.4143 14.75 16.0001ZM7 15.2501C6.58579 15.2501 6.25 15.5859 6.25 16.0001C6.25 16.4143 6.58579 16.7501 7 16.7501H8.5C8.91421 16.7501 9.25 16.4143 9.25 16.0001C9.25 15.5859 8.91421 15.2501 8.5 15.2501H7Z\" clip-rule=\"evenodd\"/><path d=\"M20 12.2501V10.0001C20 6.22886 20 4.34324 18.8284 3.17167C17.9495 2.29279 16.6688 2.07322 14.4917 2.01837C14.2216 2.01156 14 2.22991 14 2.5001C14 3.60467 13.1046 4.5001 12 4.5001C10.8954 4.5001 10 3.60467 10 2.5001C10 2.22991 9.77845 2.01156 9.50835 2.01837C7.33118 2.07322 6.05046 2.29279 5.17157 3.17167C4 4.34324 4 6.22886 4 10.0001V12.2501H20Z\"/></g>",
                24,
                24,
            )),
            Icon::Bed => Some((
                "<g fill=\"currentColor\"><path d=\"M11.25 10.5V7.25H9.50003C8.53602 7.25 7.88846 7.2516 7.40542 7.31654C6.94396 7.37858 6.74646 7.4858 6.61615 7.61612C6.48583 7.74644 6.37861 7.94393 6.31656 8.40539C6.25162 8.88843 6.25003 9.53599 6.25003 10.5L11.25 10.5Z\"/><path d=\"M17.75 10.5C17.75 9.53599 17.7484 8.88843 17.6835 8.40539C17.6215 7.94393 17.5142 7.74644 17.3839 7.61612C17.2536 7.4858 17.0561 7.37858 16.5946 7.31654C16.1116 7.2516 15.464 7.25 14.5 7.25H12.75V10.5L17.75 10.5Z\"/><path fill-rule=\"evenodd\" d=\"M13 4H11C7.22879 4 5.34317 4 4.1716 5.17157C3.14913 6.19404 3.019 7.76038 3.00244 10.6494V12.2665C2.6221 12.4854 2.322 12.8248 2.15224 13.2346C2 13.6022 2 14.0681 2 15C2 15.9319 2 16.3978 2.15224 16.7654C2.35523 17.2554 2.74458 17.6448 3.23463 17.8478C3.48702 17.9523 3.78581 17.9851 4.25 17.9953V20C4.25 20.4142 4.58579 20.75 5 20.75C5.41421 20.75 5.75 20.4142 5.75 20V18H18.25V20C18.25 20.4142 18.5858 20.75 19 20.75C19.4142 20.75 19.75 20.4142 19.75 20V17.9953C20.2142 17.9851 20.513 17.9523 20.7654 17.8478C21.2554 17.6448 21.6448 17.2554 21.8478 16.7654C22 16.3978 22 15.9319 22 15C22 14.0681 22 13.6022 21.8478 13.2346C21.678 12.8248 21.3779 12.4854 20.9976 12.2666V10.6494C20.9811 7.76038 20.8509 6.19404 19.8285 5.17157C18.6569 4 16.7713 4 13 4ZM19.25 12.0001V10.448C19.2501 9.54955 19.2501 8.80028 19.1701 8.20552C19.0857 7.57773 18.9 7.01093 18.4446 6.55546C17.9891 6.09999 17.4223 5.91432 16.7945 5.82991C16.1997 5.74995 15.4505 5.74997 14.552 5.75H9.44803C8.54955 5.74997 7.80033 5.74995 7.20555 5.82991C6.57776 5.91432 6.01095 6.09999 5.55549 6.55546C5.10002 7.01093 4.91434 7.57773 4.82994 8.20552C4.74997 8.8003 4.75 9.54952 4.75003 10.448V12.0001C4.82946 12 4.91269 12 5 12H19C19.0873 12 19.1706 12 19.25 12.0001Z\" clip-rule=\"evenodd\"/></g>",
                24,
                24,
            )),
            _ => None,
        }
    }

    fn mynaui(self) -> Option<(&'static str, u32, u32)> {
        match self {
            Icon::Coffee => Some((
                "<path fill=\"currentColor\" d=\"M10.624 3.416a.75.75 0 1 0-1.248-.832l-2 3a.75.75 0 0 0 1.248.832zm3 0a.75.75 0 1 0-1.248-.832l-2 3a.75.75 0 1 0 1.248.832zm3 0a.75.75 0 1 0-1.248-.832l-2 3a.75.75 0 1 0 1.248.832zM4.923 8.25c-.924 0-1.673.749-1.673 1.673V17A4.75 4.75 0 0 0 8 21.75h6A4.75 4.75 0 0 0 18.75 17v-1.25h.75a3.25 3.25 0 0 0 0-6.5h-.91c-.244-.556-.765-1-1.513-1z\"/>",
                24,
                24,
            )),
            Icon::Book => Some((
                "<path fill=\"currentColor\" d=\"M8.17 2.25h9.23c.667 0 1.336.109 1.803.593c.46.478.547 1.14.547 1.757v11.8c0 .543-.072 1.077-.35 1.509a1.65 1.65 0 0 1-.65.583v.908c0 .666-.108 1.335-.591 1.802c-.478.462-1.14.548-1.757.548H5.75a1.5 1.5 0 0 1-1.5-1.5V6.017c-.003-.498-.006-1.12.13-1.687c.167-.692.552-1.363 1.371-1.78c.338-.172.694-.24 1.074-.27c.365-.03.81-.03 1.345-.03m-2.42 18h10.652c.547 0 .683-.096.714-.126c.025-.024.134-.155.134-.724v-.65h-10a1.5 1.5 0 0 0-1.5 1.5\"/>",
                24,
                24,
            )),
            Icon::Camera => Some((
                "<path fill=\"currentColor\" d=\"M6.153 2.249c.212-.004.515-.009.794.036c.933.152 1.54.743 1.969 1.158l.122.118c.463.441.792.689 1.345.689h8.98c.688 0 1.308.335 1.738.86c.427.523.649 1.204.649 1.89v12c0 .686-.222 1.367-.649 1.89c-.43.525-1.05.86-1.737.86H4.636c-.687 0-1.307-.335-1.737-.86c-.427-.523-.649-1.204-.649-1.89V5.997c0-.455 0-.834.022-1.146c.022-.324.071-.63.197-.926A2.75 2.75 0 0 1 3.925 2.47c.296-.126.602-.175.926-.197c.312-.022.691-.022 1.146-.022h.025zM15.25 3a.75.75 0 0 1 .75-.75h3a.75.75 0 0 1 0 1.5h-3a.75.75 0 0 1-.75-.75M13.5 9.75a3.25 3.25 0 1 0 0 6.5a3.25 3.25 0 0 0 0-6.5\"/>",
                24,
                24,
            )),
            Icon::Home => Some((
                "<path fill=\"currentColor\" d=\"m12.857 4.06l5.866 4.817c.33.27.527.686.527 1.13v8.803c0 .814-.638 1.44-1.383 1.44H15.25V15.5a2.75 2.75 0 0 0-2.75-2.75h-1a2.75 2.75 0 0 0-2.75 2.75v4.75H6.133c-.745 0-1.383-.626-1.383-1.44v-8.802c0-.445.197-.86.527-1.13l5.866-4.819a1.34 1.34 0 0 1 1.714 0m5.01 17.69c1.61 0 2.883-1.335 2.883-2.94v-8.802a2.96 2.96 0 0 0-1.075-2.29L13.81 2.9a2.84 2.84 0 0 0-3.618 0L4.325 7.718a2.96 2.96 0 0 0-1.075 2.29v8.802c0 1.605 1.273 2.94 2.883 2.94z\"/>",
                24,
                24,
            )),
            Icon::Work => Some((
                "<path fill=\"currentColor\" d=\"M8.3 7.35h-.915c-1.888 0-2.761.433-3.173.91c-.408.473-.534 1.188-.424 2.203a14.5 14.5 0 0 0 5.639 2.064a2.75 2.75 0 0 1 5.148.006c1.966-.274 3.89-.96 5.634-2.04c.116-1.03-.01-1.755-.421-2.233c-.412-.477-1.285-.91-3.173-.91zm-6 3.315c-.13-1.158-.054-2.422.777-3.385c.834-.968 2.254-1.43 4.308-1.43h.173c0-.378 0-.798.053-1.192c.055-.41.174-.853.454-1.25c.589-.83 1.655-1.158 3.197-1.158h1.476c1.542 0 2.608.329 3.197 1.159c.28.396.4.838.454 1.25c.053.393.053.813.053 1.191h.173c2.054 0 3.474.462 4.308 1.43c.831.963.907 2.227.776 3.385l-.693 7.204c-.102.933-.344 1.973-1.212 2.746c-.856.763-2.174 1.135-4.102 1.135H8.308c-1.929 0-3.246-.372-4.102-1.135c-.868-.773-1.11-1.812-1.212-2.746l-.002-.01zM14.943 5.85c0-.396-.002-.714-.04-.993c-.037-.278-.103-.456-.191-.58c-.15-.212-.56-.527-1.973-.527h-1.476c-1.412 0-1.823.315-1.973.526c-.088.125-.154.303-.191.581c-.038.28-.04.597-.04.993zM12 12.25a1.25 1.25 0 1 0 0 2.5a1.25 1.25 0 0 0 0-2.5\"/>",
                24,
                24,
            )),
            Icon::Music => Some((
                "<path fill=\"currentColor\" d=\"M19.978 4.251c.141-.013.272.1.272.255v9.07a3.5 3.5 0 0 0-2.062-.665c-1.977 0-3.563 1.621-3.563 3.6s1.586 3.6 3.563 3.6c1.976 0 3.562-1.622 3.562-3.6V4.506c0-1.029-.88-1.843-1.91-1.749l-10.375.956a1.753 1.753 0 0 0-1.59 1.748v9.254a3.5 3.5 0 0 0-2.062-.664c-1.977 0-3.563 1.621-3.563 3.6c0 1.978 1.586 3.599 3.563 3.599c1.976 0 3.562-1.62 3.562-3.6V5.462c0-.135.102-.243.228-.254z\"/>",
                24,
                24,
            )),
            Icon::Heart => Some((
                "<path fill=\"currentColor\" d=\"M11.566 21.112L12 20.5za.75.75 0 0 0 .867 0L12 20.5l.434.612l.008-.006l.021-.015l.08-.058q.104-.075.295-.219a38.5 38.5 0 0 0 4.197-3.674c1.148-1.168 2.315-2.533 3.199-3.981c.88-1.44 1.516-3.024 1.516-4.612c0-1.885-.585-3.358-1.62-4.358c-1.03-.994-2.42-1.439-3.88-1.439c-1.725 0-3.248.833-4.25 2.117C10.998 3.583 9.474 2.75 7.75 2.75c-3.08 0-5.5 2.639-5.5 5.797c0 1.588.637 3.171 1.516 4.612c.884 1.448 2.051 2.813 3.199 3.982a38.5 38.5 0 0 0 4.492 3.892l.08.058l.021.015z\"/>",
                24,
                24,
            )),
            Icon::Star => Some((
                "<path fill=\"currentColor\" d=\"M13.51 3.139c-.652-1.185-2.368-1.185-3.021 0a28 28 0 0 0-2.114 4.894a.35.35 0 0 1-.33.223a30 30 0 0 0-4.375.436c-1.337.233-1.926 1.837-.91 2.83q.192.188.388.374a32 32 0 0 0 3.103 2.587a.274.274 0 0 1 .11.31a27.6 27.6 0 0 0-1.172 5.065c-.19 1.424 1.318 2.298 2.495 1.694a29.3 29.3 0 0 0 4.085-2.537a.4.4 0 0 1 .462 0a29 29 0 0 0 4.085 2.537c1.177.604 2.685-.27 2.495-1.694a27.6 27.6 0 0 0-1.171-5.065a.274.274 0 0 1 .11-.31a32 32 0 0 0 3.49-2.96c1.016-.994.427-2.598-.91-2.831a30 30 0 0 0-4.376-.436a.35.35 0 0 1-.329-.223a27.7 27.7 0 0 0-2.114-4.894\"/>",
                24,
                24,
            )),
            Icon::Plane => Some((
                "<path fill=\"currentColor\" d=\"M20.935 3.065a2.783 2.783 0 0 0-3.935 0l-3.112 3.112l-8.75-1.758c-.554-.111-1.076.083-1.462.333a3.1 3.1 0 0 0-.979 1.027c-.23.393-.406.905-.321 1.431c.093.586.484 1.044 1.091 1.28l5.834 2.274L6.866 13.2c-.83-.067-1.541-.103-2.199.059c-.797.196-1.442.655-2.197 1.41a.75.75 0 0 0 .144 1.174L6.08 17.92l2.08 3.465a.75.75 0 0 0 1.173.144c.755-.755 1.214-1.4 1.41-2.197c.162-.658.126-1.368.059-2.199l2.435-2.435l2.274 5.835c.236.607.695.998 1.28 1.09c.527.085 1.038-.09 1.432-.321a3.1 3.1 0 0 0 1.026-.979c.25-.386.444-.908.333-1.461l-1.758-8.75L20.935 7a2.78 2.78 0 0 0 0-3.935\"/>",
                24,
                24,
            )),
            Icon::Food => Some((
                "<path fill=\"currentColor\" d=\"M3.956 10.25c-.94 0-1.813.778-1.696 1.83a9.7 9.7 0 0 0 2.812 5.816a9.8 9.8 0 0 0 1.984 1.524c.126.073.177.184.176.268v.306a1.753 1.753 0 0 0 1.755 1.756h6.03c.965 0 1.755-.78 1.755-1.75v-.449l.002-.017l.003-.007a.1.1 0 0 1 .047-.038a9.8 9.8 0 0 0 2.104-1.593a9.7 9.7 0 0 0 2.812-5.815c.117-1.053-.756-1.831-1.696-1.831zM6 4.25a.75.75 0 0 1 .75.75v2a.75.75 0 0 1-1.5 0V5A.75.75 0 0 1 6 4.25m12 0a.75.75 0 0 1 .75.75v2a.75.75 0 0 1-1.5 0V5a.75.75 0 0 1 .75-.75m-6-2a.75.75 0 0 1 .75.75v4a.75.75 0 0 1-1.5 0V3a.75.75 0 0 1 .75-.75\"/>",
                24,
                24,
            )),
            Icon::Idea => Some((
                "<path fill=\"currentColor\" d=\"M10.386 2.25A2.75 2.75 0 0 0 7.8 4.065l-2.513 7.702A.75.75 0 0 0 6 12.75h5.25v7.5H9a.75.75 0 0 0 0 1.5h6a.75.75 0 0 0 0-1.5h-2.25v-7.5H18a.75.75 0 0 0 .713-.983L16.2 4.065l-.011-.03a2.75 2.75 0 0 0-2.575-1.785z\"/>",
                24,
                24,
            )),
            Icon::Code => Some((
                "<path fill=\"currentColor\" d=\"M9.367 2.25h5.266c1.092 0 1.958 0 2.655.057c.714.058 1.317.18 1.869.46a4.75 4.75 0 0 1 2.075 2.077c.281.55.403 1.154.461 1.868c.057.697.057 1.563.057 2.655v5.266c0 1.092 0 1.958-.057 2.655c-.058.714-.18 1.317-.46 1.869a4.75 4.75 0 0 1-2.076 2.075c-.552.281-1.155.403-1.869.461c-.697.057-1.563.057-2.655.057H9.367c-1.092 0-1.958 0-2.655-.057c-.714-.058-1.317-.18-1.868-.46a4.75 4.75 0 0 1-2.076-2.076c-.281-.552-.403-1.155-.461-1.869c-.057-.697-.057-1.563-.057-2.655V9.367c0-1.092 0-1.958.057-2.655c.058-.714.18-1.317.46-1.868a4.75 4.75 0 0 1 2.077-2.076c.55-.281 1.154-.403 1.868-.461c.697-.057 1.563-.057 2.655-.057M8.53 8.47a.75.75 0 0 0-1.06 1.06L9.94 12l-2.47 2.47a.75.75 0 1 0 1.06 1.06l3-3a.75.75 0 0 0 0-1.06zM13 14.25a.75.75 0 0 0 0 1.5h3a.75.75 0 0 0 0-1.5z\"/>",
                24,
                24,
            )),
            Icon::Money => Some((
                "<path fill=\"currentColor\" d=\"M12 2.25a.75.75 0 0 1 .75.75v1.25H17a.75.75 0 0 1 0 1.5h-4.25v5.5h1.75a4.25 4.25 0 0 1 0 8.5h-1.75V21a.75.75 0 0 1-1.5 0v-1.25H6a.75.75 0 0 1 0-1.5h5.25v-5.5H9.5a4.25 4.25 0 0 1 0-8.5h1.75V3a.75.75 0 0 1 .75-.75m-.75 3.5H9.5a2.75 2.75 0 0 0 0 5.5h1.75zm1.5 7v5.5h1.75a2.75 2.75 0 1 0 0-5.5z\"/>",
                24,
                24,
            )),
            Icon::Gift => Some((
                "<path fill=\"currentColor\" d=\"M6.559 4.984c.013.637.457 1.266 1.391 1.266h3.007a6 6 0 0 0-.439-.855C9.964 4.498 9.152 3.75 7.95 3.75c-.555 0-.89.183-1.085.39a1.18 1.18 0 0 0-.306.844m6.484 1.266h3.007c.541 0 .882-.181 1.09-.396c.215-.223.332-.531.332-.854s-.117-.63-.333-.854c-.207-.215-.548-.396-1.089-.396c-1.202 0-2.014.748-2.568 1.645c-.182.293-.327.59-.44.855m7.707 6.5V19A2.75 2.75 0 0 1 18 21.75H6A2.75 2.75 0 0 1 3.25 19v-6.25H3a.75.75 0 0 1-.75-.75V9A2.75 2.75 0 0 1 5 6.25h.37a2.85 2.85 0 0 1-.311-1.234a2.68 2.68 0 0 1 .716-1.906c.513-.543 1.26-.86 2.175-.86c1.948 0 3.161 1.252 3.844 2.355q.11.18.206.356q.096-.176.206-.356c.683-1.103 1.896-2.355 3.844-2.355c.907 0 1.651.319 2.168.854c.509.527.754 1.219.754 1.896c0 .426-.097.857-.295 1.25H19A2.75 2.75 0 0 1 21.75 9v3a.75.75 0 0 1-.75.75zm-1.5 0h-6.5v7.5H18c.69 0 1.25-.56 1.25-1.25zm-8 7.5v-7.5h-6.5V19c0 .69.56 1.25 1.25 1.25z\"/>",
                24,
                24,
            )),
            Icon::Leaf => Some((
                "<g fill=\"currentColor\"><path d=\"M5.037 17.903c.126.238.514.783 1.06 1.06L4.53 20.53a.75.75 0 1 1-1.06-1.06z\"/><path d=\"M6.097 18.964c1.828.997 3.611 1.435 5.275 1.326c1.826-.12 3.447-.897 4.758-2.208c2.599-2.599 3.943-7.24 3.555-13.067a.75.75 0 0 0-.699-.699C13.16 3.928 8.517 5.272 5.92 7.87c-1.31 1.311-2.088 2.933-2.208 4.76c-.11 1.662.329 3.446 1.326 5.273L9.57 13.37a.75.75 0 1 1 1.06 1.06z\"/></g>",
                24,
                24,
            )),
            Icon::Gear => Some((
                "<path fill=\"currentColor\" d=\"M9.391 3.646a1.75 1.75 0 0 1 1.714-1.396h1.79a1.75 1.75 0 0 1 1.714 1.396a8.7 8.7 0 0 1 1.453.602a1.75 1.75 0 0 1 2.2.225l1.265 1.266a1.75 1.75 0 0 1 .225 2.199q.365.693.602 1.453a1.75 1.75 0 0 1 1.396 1.714v1.79a1.75 1.75 0 0 1-1.396 1.714q-.237.76-.602 1.453a1.75 1.75 0 0 1-.225 2.2l-1.266 1.265a1.75 1.75 0 0 1-2.199.225a8.7 8.7 0 0 1-1.453.602a1.75 1.75 0 0 1-1.714 1.396h-1.79a1.75 1.75 0 0 1-1.714-1.396a8.7 8.7 0 0 1-1.453-.602a1.75 1.75 0 0 1-2.2-.225l-1.265-1.266a1.75 1.75 0 0 1-.225-2.199a8.7 8.7 0 0 1-.602-1.453a1.75 1.75 0 0 1-1.396-1.714v-1.79a1.75 1.75 0 0 1 1.396-1.714a8.7 8.7 0 0 1 .602-1.453a1.75 1.75 0 0 1 .225-2.2l1.266-1.265a1.75 1.75 0 0 1 2.199-.225a8.7 8.7 0 0 1 1.453-.602M8.75 12a3.25 3.25 0 1 0 6.5 0a3.25 3.25 0 0 0-6.5 0\"/>",
                24,
                24,
            )),
            Icon::Flag => Some((
                "<path fill=\"currentColor\" d=\"M19.538 3.723c-1.3 1.016-2.469 1.246-3.594 1.124c-1.18-.127-2.342-.64-3.638-1.218l-.053-.024c-1.235-.552-2.6-1.162-4.036-1.317c-1.511-.163-3.07.176-4.679 1.434a.75.75 0 0 0-.288.591V21a.75.75 0 0 0 1.5 0v-4.936c1.186-.835 2.264-1.023 3.306-.91c1.18.126 2.342.639 3.638 1.218l.053.023c1.235.553 2.6 1.162 4.036 1.317c1.511.163 3.07-.176 4.679-1.434a.75.75 0 0 0 .288-.591V4.313a.75.75 0 0 0-1.212-.59\"/>",
                24,
                24,
            )),
            Icon::Pin => Some((
                "<g fill=\"currentColor\"><path d=\"M12 8.75a1.25 1.25 0 1 0 0 2.5a1.25 1.25 0 0 0 0-2.5\"/><path d=\"M18.227 3.9A8.68 8.68 0 0 0 12 1.25c-2.34 0-4.579.956-6.227 2.65c-3.03 3.117-3.012 6.85-1.612 10.199c1.386 3.312 4.143 6.335 6.794 8.304a1.75 1.75 0 0 0 2.09 0c2.65-1.969 5.408-4.992 6.794-8.304c1.4-3.348 1.418-7.082-1.612-10.199M12 12.75a2.75 2.75 0 1 1 0-5.5a2.75 2.75 0 0 1 0 5.5\"/></g>",
                24,
                24,
            )),
            Icon::Bug => Some((
                "<path fill=\"currentColor\" d=\"M16.074 2.254a.75.75 0 0 1 .672.82c-.115 1.162-.787 1.885-1.499 2.403c.862.618 1.446 1.457 1.83 2.306q.184-.062.392-.17c.328-.176.654-.42.95-.678a9 9 0 0 0 .977-1.006l.012-.015a.75.75 0 1 1 1.182.924l-.009.01l-.018.024l-.067.081a10 10 0 0 1-1.09 1.112a6.8 6.8 0 0 1-1.23.87c-.195.105-.408.2-.631.275c.23 1.075.224 2.167.212 3.165H20a.75.75 0 0 1 0 1.5h-2.243c.012.998.018 2.09-.212 3.164c.223.076.435.171.632.276c.46.247.88.566 1.23.87a10 10 0 0 1 1.09 1.112l.066.08l.018.024c-.001-.001.002.003 0 0l.008.01a.75.75 0 1 1-1.179.928l-.014-.018l-.05-.061a9 9 0 0 0-.926-.944a5.3 5.3 0 0 0-.951-.678a3 3 0 0 0-.393-.172c-.331.735-.811 1.463-1.497 2.048c-.883.753-2.063 1.236-3.579 1.236s-2.696-.483-3.58-1.236c-.685-.585-1.166-1.313-1.497-2.048a3 3 0 0 0-.392.172a5.3 5.3 0 0 0-.95.677a9 9 0 0 0-.977 1.006l-.012.015a.75.75 0 1 1-1.182-.924l.009-.01l.018-.024l.067-.081a10 10 0 0 1 1.09-1.112a6.8 6.8 0 0 1 1.23-.87a4 4 0 0 1 .63-.275c-.23-1.075-.224-2.167-.21-3.165H4a.75.75 0 0 1 0-1.5h2.24c-.02-.986-.04-2.092.171-3.164a4 4 0 0 1-.633-.278a6.6 6.6 0 0 1-1.212-.872a10 10 0 0 1-1.134-1.196l-.019-.023c-.002-.003.001.002 0 0l-.008-.01a.75.75 0 0 1 1.189-.914c.002.003.01.01.014.018l.05.061a8.6 8.6 0 0 0 .904.942c.291.259.61.5.932.675q.199.106.375.166a5.45 5.45 0 0 1 1.914-2.27c-.72-.527-1.397-1.258-1.528-2.426a.75.75 0 0 1 1.49-.168c.098.875.715 1.284 1.748 1.906l3.044-.038c1.03-.614 1.632-1.005 1.717-1.858a.75.75 0 0 1 .82-.672\"/>",
                24,
                24,
            )),
            Icon::Game => Some((
                "<path fill=\"currentColor\" d=\"M12 3.032a.75.75 0 0 1 .75.75v3.715c.428-.109.867-.296 1.371-.51q.219-.094.455-.192c.952-.395 2.122-.801 3.441-.374c1.37.444 2.474 1.472 3.203 3.157c.718 1.66 1.081 3.97 1.024 7.062c-.023 1.255-.34 3.057-1.996 3.78c-1.789.782-3.198-.024-4.296-.774q-.253-.174-.483-.337c-.376-.265-.72-.507-1.089-.714c-.499-.28-.957-.44-1.431-.44H11.05c-.475 0-.935.16-1.435.44c-.37.207-.716.45-1.094.716q-.229.162-.48.334c-1.097.748-2.506 1.555-4.29.776c-1.656-.724-1.973-2.526-1.996-3.78c-.057-3.093.306-5.403 1.024-7.063c.73-1.685 1.833-2.713 3.203-3.157c1.32-.427 2.489-.02 3.441.374q.237.098.455.192c.504.214.943.401 1.371.51V3.782a.75.75 0 0 1 .75-.75M8.25 11.5a.75.75 0 0 0-1.5 0v.75H6a.75.75 0 0 0 0 1.5h.75v.75a.75.75 0 0 0 1.5 0v-.75H9a.75.75 0 0 0 0-1.5h-.75zm6.624.75a.75.75 0 0 0 0 1.5h3a.75.75 0 0 0 0-1.5z\"/>",
                24,
                24,
            )),
            Icon::Cart => Some((
                "<path fill=\"currentColor\" d=\"M2.787 2.28a.75.75 0 0 1 .932.507l.55 1.863h14.655c1.84 0 3.245 1.717 2.715 3.51l-1.655 5.6c-.352 1.193-1.471 1.99-2.715 1.99H8.113c-1.244 0-2.362-.797-2.715-1.99L2.281 3.212a.75.75 0 0 1 .506-.931M6.25 19.5a2.25 2.25 0 1 1 4.5 0a2.25 2.25 0 0 1-4.5 0m8 0a2.25 2.25 0 1 1 4.5 0a2.25 2.25 0 0 1-4.5 0\"/>",
                24,
                24,
            )),
            Icon::Bell => Some((
                "<path fill=\"currentColor\" d=\"M15.737 17.75c-.07.813-.27 1.654-.696 2.36c-.592.98-1.588 1.64-3.042 1.64s-2.449-.66-3.04-1.64c-.427-.706-.627-1.547-.697-2.36H5.366c-.596 0-1.129-.148-1.526-.497c-.403-.356-.566-.831-.588-1.28c-.04-.846.405-1.742.976-2.309c.68-.676.985-1.602 1.138-2.749c.076-.571.111-1.169.146-1.796l.004-.066c.034-.596.069-1.22.144-1.822c.156-1.241.5-2.536 1.508-3.5C8.182 2.758 9.73 2.25 11.999 2.25s3.818.509 4.832 1.48c1.008.965 1.352 2.26 1.508 3.501c.075.602.11 1.226.144 1.822l.003.066c.036.627.07 1.225.147 1.796c.153 1.147.458 2.073 1.138 2.75c.588.584 1.028 1.485.975 2.334c-.028.448-.2.916-.603 1.263c-.396.342-.923.488-1.51.488z\"/>",
                24,
                24,
            )),
            Icon::Calendar => Some((
                "<path fill=\"currentColor\" d=\"M7.5 2.25a.75.75 0 0 1 .75.75v.253q.515-.004 1.119-.003h5.262q.604 0 1.119.003V3a.75.75 0 0 1 1.5 0v.301q.018 0 .035.003c.71.054 1.309.169 1.856.432a4.65 4.65 0 0 1 2.083 1.97c.287.532.41 1.113.469 1.793c.057.662.057 1.482.057 2.51v4.981c0 1.029 0 1.85-.057 2.511c-.06.68-.182 1.261-.469 1.792a4.65 4.65 0 0 1-2.083 1.971c-.547.263-1.146.378-1.856.432c-.696.054-1.56.054-2.654.054H9.37c-1.094 0-1.958 0-2.654-.054c-.71-.055-1.309-.169-1.856-.432a4.65 4.65 0 0 1-2.083-1.97c-.287-.532-.41-1.113-.469-1.793c-.057-.662-.057-1.482-.057-2.51V10.01c0-1.029 0-1.85.057-2.511c.06-.68.182-1.261.469-1.792A4.65 4.65 0 0 1 4.86 3.736c.547-.263 1.146-.378 1.856-.432L6.75 3.3V3a.75.75 0 0 1 .75-.75m-.75 2.556c-.577.05-.946.14-1.24.282a3.15 3.15 0 0 0-1.414 1.33c-.114.212-.196.466-.25.832h16.309c-.055-.366-.137-.62-.251-.831a3.15 3.15 0 0 0-1.413-1.331c-.295-.142-.664-.232-1.241-.282V5a.75.75 0 0 1-1.5 0v-.247q-.511-.004-1.15-.003H9.4q-.639 0-1.15.003V5a.75.75 0 0 1-1.5 0z\"/>",
                24,
                24,
            )),
            Icon::Envelope => Some((
                "<path fill=\"currentColor\" d=\"M7.125 3.75h9.75c.813 0 1.468 0 2 .043c.546.045 1.026.14 1.47.366a3.75 3.75 0 0 1 1.64 1.639c.226.444.32.924.365 1.47q.01.12.016.247a.75.75 0 0 1 .014.336c.013.41.013.879.013 1.417v5.464c0 .813 0 1.469-.043 2c-.045.546-.14 1.026-.366 1.47a3.75 3.75 0 0 1-1.639 1.64c-.444.226-.924.32-1.47.365c-.532.043-1.187.043-2 .043h-9.75c-.813 0-1.468 0-2-.043c-.546-.045-1.026-.14-1.47-.366a3.75 3.75 0 0 1-1.639-1.639c-.226-.444-.32-.924-.365-1.47c-.044-.531-.044-1.187-.044-2V9.268c0-.538 0-1.007.013-1.417a.75.75 0 0 1 .014-.336q.007-.128.017-.246c.044-.547.139-1.027.365-1.471a3.75 3.75 0 0 1 1.639-1.64c.444-.226.924-.32 1.47-.365c.532-.043 1.187-.043 2-.043M20.85 7.341c-.038-.423-.105-.672-.202-.862a2.25 2.25 0 0 0-.983-.984c-.198-.1-.459-.17-.913-.207c-.462-.037-1.057-.038-1.909-.038H7.157c-.852 0-1.446 0-1.91.038c-.453.037-.714.107-.911.207a2.25 2.25 0 0 0-.984.984c-.096.19-.164.439-.202.862l6.604 4.403c1.01.674 1.363.895 1.722.981a2.25 2.25 0 0 0 1.048 0c.36-.086.711-.307 1.723-.981z\"/>",
                24,
                24,
            )),
            Icon::Phone => Some((
                "<path fill=\"currentColor\" d=\"M9.004 3.416C8.432 2.606 7.64 2.241 6.8 2.25c-.797.008-1.573.349-2.221.803A6.2 6.2 0 0 0 2.92 4.79c-.41.649-.706 1.416-.666 2.165c.193 3.603 2.22 7.453 5.067 10.302c2.845 2.846 6.644 4.824 10.48 4.446c.752-.074 1.463-.457 2.044-.945a5.8 5.8 0 0 0 1.443-1.84c.34-.692.543-1.49.431-2.267c-.116-.81-.569-1.534-1.402-2.014a16 16 0 0 1-.512-.31c-.15-.093-.31-.194-.504-.31a7.5 7.5 0 0 0-1.249-.618c-.447-.163-.958-.27-1.49-.197c-.551.076-1.063.336-1.506.802c-.341.36-.843.472-1.549.268c-.718-.208-1.526-.724-2.228-1.422c-.702-.696-1.233-1.51-1.46-2.245c-.224-.728-.125-1.263.225-1.632c.473-.498.725-1.052.778-1.638c.052-.57-.09-1.106-.293-1.574c-.304-.699-.82-1.394-1.224-1.936a22 22 0 0 1-.3-.41\"/>",
                24,
                24,
            )),
            Icon::Moon => Some((
                "<path fill=\"currentColor\" d=\"M11.712 3.45a.75.75 0 0 0-.668-1.197c-5.414.494-8.436 4.752-8.764 9.105c-.328 4.361 2.037 8.975 7.451 10.166c5.686 1.25 11.472-2.837 12.016-8.646a.75.75 0 0 0-1.189-.676c-2.837 2.069-6.08 1.316-8.136-.724c-2.054-2.039-2.8-5.239-.71-8.028\"/>",
                24,
                24,
            )),
            Icon::Sun => Some((
                "<path fill=\"currentColor\" d=\"M12 2.25a.75.75 0 0 1 .75.75v2a.75.75 0 1 1-1.5 0V3a.75.75 0 0 1 .75-.75m0 16.004a.75.75 0 0 1 .75.75v2a.75.75 0 1 1-1.5 0v-2a.75.75 0 0 1 .75-.75M2.25 12a.75.75 0 0 1 .75-.75h2a.75.75 0 0 1 0 1.5H3a.75.75 0 0 1-.75-.75m16 0a.75.75 0 0 1 .75-.75h2a.75.75 0 1 1 0 1.5h-2a.75.75 0 0 1-.75-.75m1.28-7.53a.75.75 0 0 1 0 1.06l-2 2a.75.75 0 1 1-1.06-1.06l2-2a.75.75 0 0 1 1.06 0m-15.06 0a.75.75 0 0 1 1.06 0l2 2a.75.75 0 0 1-1.06 1.06l-2-2a.75.75 0 0 1 0-1.06m3.06 12a.75.75 0 0 1 0 1.06l-2 2a.75.75 0 0 1-1.06-1.06l2-2a.75.75 0 0 1 1.06 0m8.94 0a.75.75 0 0 1 1.06 0l2 2a.75.75 0 1 1-1.06 1.06l-2-2a.75.75 0 0 1 0-1.06M12 7.25a4.75 4.75 0 1 0 0 9.5a4.75 4.75 0 0 0 0-9.5\"/>",
                24,
                24,
            )),
            Icon::Cloud => Some((
                "<path fill=\"currentColor\" d=\"M12.103 4.552c2 .614 3.66 2.175 4.493 4.836c2.05.177 3.997 1.285 5.063 2.803c.59.842.932 1.844.82 2.9c-.113 1.066-.677 2.087-1.713 2.975c-1.116.957-2.676 1.184-3.894 1.184H8.026a6.14 6.14 0 0 1-4.72-2.211c-2-2.263-2.424-4.666-1.773-6.796c.64-2.09 2.28-3.812 4.216-4.86c1.935-1.046 4.267-1.472 6.354-.831\"/>",
                24,
                24,
            )),
            Icon::Film => Some((
                "<path fill=\"currentColor\" d=\"M11.943 2.25h.114c2.073 0 3.705 0 4.98.171c1.31.176 2.354.545 3.175 1.367c.822.821 1.19 1.866 1.367 3.174l.033.271a.75.75 0 0 1 .033.334c.105 1.175.105 2.617.105 4.376v.114c0 1.744 0 3.177-.102 4.347a.8.8 0 0 1-.027.28q-.02.18-.042.354c-.176 1.308-.545 2.353-1.367 3.174c-.821.822-1.866 1.19-3.174 1.367c-1.276.171-2.908.171-4.981.171h-.114c-2.073 0-3.705 0-4.98-.171c-1.31-.176-2.354-.545-3.175-1.367c-.822-.821-1.19-1.866-1.367-3.174c-.171-1.276-.171-2.908-.171-4.981v-.114c0-2.073 0-3.705.171-4.98c.176-1.31.545-2.354 1.367-3.175c.821-.822 1.866-1.19 3.174-1.367c1.276-.171 2.908-.171 4.981-.171m8.25 13.473c.047-.823.055-1.797.057-2.973h-3.1v2.973zm-3.042 1.5v2.821c.956-.163 1.551-.443 2-.892c.439-.438.716-1.015.88-1.929zm-10.5 2.785v-2.785H3.968c.165.914.442 1.49.88 1.929c.418.417.961.689 1.803.856m-2.844-4.285h2.844V12.75h-2.9c0 1.176.01 2.15.056 2.973M3.75 11.25h2.9V8.223H3.81c-.05.834-.058 1.825-.06 3.027m.228-4.527h2.673v-2.73c-.842.166-1.385.438-1.803.855c-.429.43-.704.992-.87 1.875m13.173-2.767v2.767h2.87c-.165-.883-.44-1.445-.87-1.875c-.449-.449-1.044-.73-2-.892m3.039 4.267h-3.04v3.027h3.1c-.002-1.202-.01-2.193-.06-3.027\"/>",
                24,
                24,
            )),
            Icon::Pencil => Some((
                "<path fill=\"currentColor\" d=\"M14.678 3.272a3.483 3.483 0 0 1 4.928-.001l1.127 1.127a3.483 3.483 0 0 1 0 4.925L9.33 20.729a3.48 3.48 0 0 1-2.463 1.021H3a.75.75 0 0 1-.75-.75v-3.844a3.48 3.48 0 0 1 1.019-2.461zm3.867 1.06a1.983 1.983 0 0 0-2.806 0l-.896.897l3.931 3.931l.898-.898a1.983 1.983 0 0 0 0-2.804z\"/>",
                24,
                24,
            )),
            Icon::Key => Some((
                "<path fill=\"currentColor\" d=\"M7.5 11.25a5.25 5.25 0 1 0 4.205 2.106l3.444-3.444l1.002 1.002c.344.344.788.59 1.317.493c.437-.08.763-.378.947-.552l.197-.188c.354-.34.577-.554.927-.612c.268-.045.602-.124.908-.315a1.73 1.73 0 0 0 .732-.934a1.7 1.7 0 0 0-.035-1.206c-.149-.368-.411-.698-.719-1.005l-.98-.98L21.53 3.53a.75.75 0 0 0-1.06-1.06l-9.826 9.825A5.23 5.23 0 0 0 7.5 11.25\"/>",
                24,
                24,
            )),
            Icon::Lock => Some((
                "<path fill=\"currentColor\" d=\"M9.572 4.904c.51-.703 1.28-1.154 2.428-1.154s1.919.45 2.428 1.154c.532.736.822 1.813.822 3.096v1.25h-6.5V8c0-1.283.29-2.36.822-3.096M16.75 9.25V8c0-1.478-.33-2.901-1.107-3.975c-.8-1.107-2.03-1.775-3.643-1.775s-2.842.668-3.643 1.775C7.58 5.099 7.25 6.522 7.25 8v1.25h-.58c-.535 0-.98 0-1.345.03c-.38.031-.736.098-1.073.27a2.75 2.75 0 0 0-1.202 1.202c-.172.337-.24.694-.27 1.074c-.03.364-.03.81-.03 1.344v4.66c0 .535 0 .98.03 1.345c.03.38.098.737.27 1.074a2.75 2.75 0 0 0 1.202 1.202c.337.172.693.239 1.073.27c.365.03.81.03 1.345.03h10.66c.535 0 .98 0 1.345-.03c.38-.031.736-.098 1.073-.27a2.75 2.75 0 0 0 1.202-1.202c.172-.337.24-.694.27-1.074c.03-.364.03-.81.03-1.344V13.17c0-.534 0-.98-.03-1.344c-.03-.38-.098-.737-.27-1.074a2.75 2.75 0 0 0-1.2-1.202c-.338-.172-.694-.239-1.074-.27c-.365-.03-.81-.03-1.345-.03z\"/>",
                24,
                24,
            )),
            Icon::Wrench => Some((
                "<path fill=\"currentColor\" d=\"M7.598 2.343a6.433 6.433 0 0 1 7.419 7.5c-.125.682.008 1.28.359 1.63l5.566 5.567a2.76 2.76 0 0 1-3.902 3.902l-5.566-5.566c-.35-.35-.949-.484-1.63-.36a6.434 6.434 0 0 1-7.5-7.418a1.495 1.495 0 0 1 1.154-1.226c.54-.123 1.133.04 1.56.467l2.545 2.544a.197.197 0 0 0 .272 0l1.508-1.508a.197.197 0 0 0 0-.272L6.84 5.058a1.69 1.69 0 0 1-.467-1.56c.127-.56.562-1.04 1.226-1.155\"/>",
                24,
                24,
            )),
            Icon::Rocket => Some((
                "<path fill=\"currentColor\" d=\"M10.83 7.11c2.238-2.523 5.72-3.61 8.92-3.61a.75.75 0 0 1 .75.75c0 3.2-1.087 6.682-3.61 8.92c-.061 1.016-.375 2.033-.824 2.926c-.5.994-1.195 1.887-1.973 2.478c-.761.578-1.745.963-2.717.601c-.92-.343-1.54-1.25-1.9-2.538l-2.113-2.114c-1.288-.359-2.195-.979-2.538-1.899c-.362-.972.023-1.956.601-2.717c.591-.778 1.484-1.473 2.478-1.973c.893-.449 1.91-.763 2.925-.823M9.5 8.892a7 7 0 0 0-.922.383c-.836.421-1.533.982-1.957 1.54c-.437.576-.493 1.01-.39 1.286c.083.226.352.557 1.094.836c.169-.379.412-.903.717-1.504c.388-.763.882-1.66 1.458-2.54m1.564 7.784c.278.742.61 1.01.836 1.095c.275.102.71.046 1.286-.39c.558-.425 1.119-1.122 1.54-1.958q.227-.451.383-.922a27 27 0 0 1-2.54 1.458c-.602.305-1.126.548-1.505.717m-6.029-.672a2.144 2.144 0 0 1 2.848.088l.009.01c.799.79.786 2.054.103 2.865c-.295.352-.698.606-1.077.792c-.387.19-.804.333-1.175.44a11 11 0 0 1-1.358.295l-.024.003l-.008.001H4.35a.75.75 0 0 1-.843-.842v-.003l.001-.008l.004-.024a7 7 0 0 1 .066-.389c.047-.251.121-.596.228-.971a7.3 7.3 0 0 1 .439-1.178c.185-.38.439-.783.79-1.08M15.687 8.22a.75.75 0 0 0-1.06 0l-.707.707a.75.75 0 0 0 1.06 1.06l.707-.707a.75.75 0 0 0 0-1.06\"/>",
                24,
                24,
            )),
            Icon::Wine => Some((
                "<path fill=\"currentColor\" d=\"M6.75 2.25c-.69 0-1.25.56-1.25 1.25v5.75A5.75 5.75 0 0 0 11.25 15v5.25H8.4a.75.75 0 0 0 0 1.5h7.2a.75.75 0 0 0 0-1.5h-2.85V15a5.75 5.75 0 0 0 5.75-5.75V3.5c0-.69-.56-1.25-1.25-1.25z\"/>",
                24,
                24,
            )),
            Icon::Pizza => Some((
                "<path fill=\"currentColor\" d=\"m16.895 3.514l4.778 15.928c.41 1.367-.864 2.641-2.231 2.231L3.514 16.895c-.921-.277-1.496-1.258-1.175-2.222A19.5 19.5 0 0 1 14.673 2.34c.964-.322 1.945.254 2.222 1.175m-1.747.248A18 18 0 0 0 3.762 15.148c-.037.11.02.261.183.31l1.802.54A16.73 16.73 0 0 1 16 5.749l-.541-1.803c-.049-.163-.2-.22-.31-.183m.382 6.708a.75.75 0 1 0-1.06 1.06l.353.354a.75.75 0 0 0 1.06-1.06zm-4 4a.75.75 0 1 0-1.06 1.06l.353.354a.75.75 0 0 0 1.06-1.06zm5 1a.75.75 0 1 0-1.06 1.06l.353.354a.75.75 0 0 0 1.06-1.06z\"/>",
                24,
                24,
            )),
            Icon::Bank => Some((
                "<path fill=\"currentColor\" d=\"M12.784 2.436a1.74 1.74 0 0 0-1.568 0L3.452 6.344c-1.64.825-1.074 3.327.784 3.327H6.25v7.579H4.5a2.25 2.25 0 0 0 0 4.5h15a2.25 2.25 0 0 0 0-4.5h-1.75V9.67h2.014c1.858 0 2.423-2.501.784-3.326zM7.75 17.25V9.67h3.5v7.58zm5 0V9.67h3.5v7.58z\"/>",
                24,
                24,
            )),
            Icon::Medal => Some((
                "<path fill=\"currentColor\" d=\"M13.435 2.075a3.33 3.33 0 0 0-2.87 0c-.394.189-.755.497-1.26.928l-.079.066a2.56 2.56 0 0 1-1.58.655l-.102.008c-.662.053-1.135.09-1.547.236a3.33 3.33 0 0 0-2.03 2.029c-.145.412-.182.885-.235 1.547l-.008.102a2.56 2.56 0 0 1-.655 1.58l-.066.078c-.431.506-.74.867-.928 1.261a3.33 3.33 0 0 0 0 2.87c.189.394.497.755.928 1.26l.066.079c.41.48.604.939.655 1.58l.008.102c.053.662.09 1.135.236 1.547a3.33 3.33 0 0 0 2.029 2.03c.412.145.885.182 1.547.235l.102.008c.629.05 1.09.238 1.58.655l.078.066c.506.431.867.74 1.261.928a3.33 3.33 0 0 0 2.87 0c.394-.189.755-.497 1.26-.928l.079-.066c.48-.41.939-.604 1.58-.655l.102-.008c.662-.053 1.135-.09 1.547-.236a3.33 3.33 0 0 0 2.03-2.029c.145-.412.182-.885.235-1.547l.008-.102c.05-.629.238-1.09.655-1.58l.066-.079c.431-.505.74-.866.928-1.26a3.33 3.33 0 0 0 0-2.87c-.189-.394-.497-.755-.928-1.26l-.066-.079a2.56 2.56 0 0 1-.655-1.58l-.008-.102c-.053-.662-.09-1.135-.236-1.547a3.33 3.33 0 0 0-2.029-2.03c-.412-.145-.885-.182-1.547-.235l-.102-.008a2.56 2.56 0 0 1-1.58-.655l-.079-.066c-.505-.431-.866-.74-1.26-.928\"/>",
                24,
                24,
            )),
            Icon::Truck => Some((
                "<path fill=\"currentColor\" d=\"M14.748 17.205H9.872c-.331 1.448-1.602 2.545-3.148 2.545c-1.545 0-2.816-1.097-3.148-2.546H3a.75.75 0 0 1-.75-.75V6c0-.966.784-1.75 1.75-1.75h9.793c.967 0 1.75.784 1.75 1.75v1.432h1.51c.742 0 1.452.299 1.97.83l2.514 2.578a.75.75 0 0 1 .213.524v5.09a.75.75 0 0 1-.705.75c-.331 1.448-1.603 2.546-3.148 2.546s-2.817-1.097-3.149-2.546m-8.024-2.546c-.94 0-1.733.786-1.733 1.795c0 1.01.794 1.796 1.733 1.796s1.733-.786 1.733-1.796c0-1.009-.793-1.795-1.733-1.795m11.173 0c-.94 0-1.733.786-1.733 1.795c0 1.01.793 1.796 1.733 1.796c.939 0 1.732-.786 1.732-1.796c0-1.009-.793-1.795-1.732-1.795\"/>",
                24,
                24,
            )),
            Icon::Bag => Some((
                "<path fill=\"currentColor\" d=\"M15.815 5v1.25h.412c1.451 0 2.68 1.101 2.786 2.553l.73 10c.117 1.609-1.182 2.947-2.786 2.947H7.043c-1.604 0-2.903-1.338-2.786-2.947l.73-10C5.093 7.35 6.322 6.25 7.773 6.25h.412V5c0-1.534 1.266-2.75 2.794-2.75h2.043c1.527 0 2.793 1.216 2.793 2.75m-6.13 0v1.25h4.63V5c0-.675-.564-1.25-1.293-1.25h-2.043c-.73 0-1.294.575-1.294 1.25\"/>",
                24,
                24,
            )),
            Icon::Bookmark => Some((
                "<path fill=\"currentColor\" d=\"M6.75 2.25c-.979 0-1.5.926-1.5 1.692v16.01c0 1.352 1.469 2.308 2.686 1.518l3.945-2.561a.21.21 0 0 1 .238 0l3.945 2.561c1.217.79 2.686-.165 2.686-1.518V3.942c0-.766-.521-1.692-1.5-1.692z\"/>",
                24,
                24,
            )),
            Icon::Folder => Some((
                "<path fill=\"currentColor\" d=\"M5 3.25A2.75 2.75 0 0 0 2.25 6v12A2.75 2.75 0 0 0 5 20.75h14A2.75 2.75 0 0 0 21.75 18V9A2.75 2.75 0 0 0 19 6.25h-7.34a1.25 1.25 0 0 1-.826-.312L8.562 3.936a2.75 2.75 0 0 0-1.817-.686z\"/>",
                24,
                24,
            )),
            Icon::User => Some((
                "<path fill=\"currentColor\" d=\"M12 12.75c3.942 0 7.987 2.563 8.249 7.712a.75.75 0 0 1-.71.787c-2.08.106-11.713.171-15.077 0a.75.75 0 0 1-.711-.787C4.013 15.314 8.058 12.75 12 12.75m0-9a3.75 3.75 0 1 0 0 7.5a3.75 3.75 0 0 0 0-7.5\"/>",
                24,
                24,
            )),
            Icon::Pram => Some((
                "<path fill=\"currentColor\" d=\"m12.161 2.251l.089-.001q.048 0 .095.006C17.57 2.438 21.75 6.731 21.75 12c0 5.385-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12S6.615 2.25 12 2.25zM9.46 4.15a3 3 0 0 0 4.791 3.337a.75.75 0 1 0-1-1.118a1.5 1.5 0 1 1-1.074-2.616a8.3 8.3 0 0 0-2.717.397M9 9.25c-.486 0-.916.195-1.247.488a.75.75 0 1 0 .994 1.124A.38.38 0 0 1 9 10.75a.38.38 0 0 1 .253.112a.75.75 0 1 0 .994-1.124A1.88 1.88 0 0 0 9 9.25m6 0c-.486 0-.916.195-1.247.488a.75.75 0 1 0 .994 1.124a.38.38 0 0 1 .253-.112a.38.38 0 0 1 .253.112a.75.75 0 1 0 .994-1.124A1.88 1.88 0 0 0 15 9.25m-5.553 5.148a.75.75 0 1 0-.894 1.204A5.77 5.77 0 0 0 12 16.75a5.77 5.77 0 0 0 3.447-1.148a.75.75 0 1 0-.894-1.204A4.27 4.27 0 0 1 12 15.25a4.27 4.27 0 0 1-2.553-.852\"/>",
                24,
                24,
            )),
            Icon::Paint => Some((
                "<path fill=\"currentColor\" d=\"M5.5 2.25A1.75 1.75 0 0 0 3.75 4v3c0 .966.784 1.75 1.75 1.75h10A1.75 1.75 0 0 0 17.25 7v-.75h.25c.483 0 .815.09 1.003.213c.14.092.247.225.247.537v2c0 .69-.56 1.25-1.25 1.25h-7A1.75 1.75 0 0 0 8.75 12v8c0 .966.784 1.75 1.75 1.75h1a.75.75 0 0 0 0-1.5h-1a.25.25 0 0 1-.25-.25v-8a.25.25 0 0 1 .25-.25h7A2.75 2.75 0 0 0 20.25 9V7c0-.792-.34-1.41-.925-1.792c-.536-.35-1.204-.458-1.825-.458h-.25V4a1.75 1.75 0 0 0-1.75-1.75z\"/>",
                24,
                24,
            )),
            Icon::Tree => Some((
                "<path fill=\"currentColor\" d=\"M12.605 2.556a.75.75 0 0 0-1.21 0l-5.5 7.5A.75.75 0 0 0 6.5 11.25h1.569l-3.686 5.323A.75.75 0 0 0 5 17.75h6.25V21a.75.75 0 0 0 1.5 0v-3.25H19a.75.75 0 0 0 .617-1.177L15.93 11.25h1.57a.75.75 0 0 0 .605-1.194z\"/>",
                24,
                24,
            )),
            Icon::Ship => Some((
                "<path fill=\"currentColor\" d=\"M14.55 4.997V4.8a2.55 2.55 0 0 0-5.1 0v.197H7.7a2.75 2.75 0 0 0-2.75 2.75v3.288a2.75 2.75 0 0 0-1.486 2.947l.598 3.344a.75.75 0 0 0 1.476-.264l-.597-3.344a1.25 1.25 0 0 1 .829-1.403l5.829-1.977a1.25 1.25 0 0 1 .803 0l5.828 1.977c.585.198.938.795.83 1.404l-.598 3.343a.75.75 0 1 0 1.476.264l.598-3.344a2.75 2.75 0 0 0-1.486-2.947V7.747a2.75 2.75 0 0 0-2.75-2.75zM9.868 21.251c-.892-.312-2.004-.298-2.854.056a5.6 5.6 0 0 1-2.53.43c-.761-.058-1.52-.306-2.019-.813a.75.75 0 1 1 1.07-1.051c.148.15.5.325 1.063.367a4.1 4.1 0 0 0 1.839-.317c1.214-.506 2.709-.514 3.927-.087a5.08 5.08 0 0 0 3.272 0c1.218-.427 2.713-.42 3.927.087a4.1 4.1 0 0 0 1.84.318c.562-.043.914-.217 1.062-.368a.75.75 0 1 1 1.07 1.051c-.498.507-1.258.755-2.02.812a5.6 5.6 0 0 1-2.529-.429c-.85-.354-1.962-.368-2.854-.056a6.58 6.58 0 0 1-4.264 0\"/>",
                24,
                24,
            )),
            Icon::Train => Some((
                "<path fill=\"currentColor\" d=\"M4.25 6A3.75 3.75 0 0 1 8 2.25h8A3.75 3.75 0 0 1 19.75 6v9c0 1.63-1.04 3.017-2.493 3.534l1.367 2.05a.75.75 0 1 1-1.248.832l-1.777-2.666H8.4l-1.777 2.666a.75.75 0 1 1-1.248-.832l1.367-2.05A3.75 3.75 0 0 1 4.25 15zm14 .75h-5.5v3.5h5.5zm-12.5 3.5h5.5v-3.5h-5.5zm3 3.75a.75.75 0 0 0-1.5 0v1a.75.75 0 0 0 1.5 0zm7.25-.75a.75.75 0 0 0-.75.75v1a.75.75 0 0 0 1.5 0v-1a.75.75 0 0 0-.75-.75\"/>",
                24,
                24,
            )),
            Icon::Cake => Some((
                "<path fill=\"currentColor\" d=\"M7 3.25a.75.75 0 0 1 .75.75v.5a.75.75 0 0 1-1.5 0V4A.75.75 0 0 1 7 3.25m5 0a.75.75 0 0 1 .75.75v.5a.75.75 0 0 1-1.5 0V4a.75.75 0 0 1 .75-.75m5 0a.75.75 0 0 1 .75.75v.5a.75.75 0 0 1-1.5 0V4a.75.75 0 0 1 .75-.75m-10 3a.75.75 0 0 1 .75.75v3.25h3.5V7a.75.75 0 0 1 1.5 0v3.25h3.5V7a.75.75 0 0 1 1.5 0v3.25H18a2.77 2.77 0 0 1 2.75 2.73v7.27H22a.75.75 0 0 1 0 1.5H2a.75.75 0 0 1 0-1.5h1.25V13a3 3 0 0 1 .007-.191A2.77 2.77 0 0 1 6 10.25h.25V7A.75.75 0 0 1 7 6.25m-1 5.5c-.56 0-1.052.396-1.203.917c.568.549 1.167.675 1.665.522c.453-.14.989-.559 1.434-1.439zm10.12 0c.408.758.922 1.209 1.397 1.394c.525.205 1.126.135 1.699-.428A1.275 1.275 0 0 0 18 11.75zm-2.222 0h-3.78c.546 1.016 1.276 1.464 1.86 1.498c.558.032 1.32-.307 1.92-1.498\"/>",
                24,
                24,
            )),
            Icon::Drink => Some((
                "<path fill=\"currentColor\" d=\"M5.012 2.25c-1.463 0-2.307 1.69-1.372 2.846l7.61 9.42v5.734H7.33a.75.75 0 0 0 0 1.5h9.341a.75.75 0 0 0 0-1.5h-3.92v-5.735l7.61-9.419c.934-1.157.09-2.846-1.373-2.846zm1.893 4.5L4.807 4.153a.25.25 0 0 1 .205-.403h13.976c.231 0 .33.25.206.403L17.096 6.75z\"/>",
                24,
                24,
            )),
            _ => None,
        }
    }

    fn majesticons(self) -> Option<(&'static str, u32, u32)> {
        match self {
            Icon::Coffee => Some((
                "<g fill=\"none\"><path fill=\"currentColor\" d=\"M4 5h12v7a4 4 0 0 1-4 4H8a4 4 0 0 1-4-4z\"/><path stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M16 5H4v7a4 4 0 0 0 4 4h4a4 4 0 0 0 4-4zm0 0h2v0a2 2 0 0 1 2 2v4M4 19h14\"/></g>",
                24,
                24,
            )),
            Icon::Book => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M3 5a3 3 0 0 1 3-3h12a3 3 0 0 1 3 3v14a3 3 0 0 1-3 3H6a3 3 0 0 1-3-3zm5 0v7l2.293-2.293a1 1 0 0 1 1.414 0L14 12V5a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Camera => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M7.574 4.336A3 3 0 0 1 10.07 3h3.86a3 3 0 0 1 2.496 1.336l.812 1.219A1 1 0 0 0 18.07 6H19a3 3 0 0 1 3 3v9a3 3 0 0 1-3 3H5a3 3 0 0 1-3-3V9a3 3 0 0 1 3-3h.93a1 1 0 0 0 .832-.445l.812-1.22zM10 13a2 2 0 1 1 4 0a2 2 0 0 1-4 0m2-4a4 4 0 1 0 0 8a4 4 0 0 0 0-8\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Home => Some((
                "<path fill=\"currentColor\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M20 19v-8.5a1 1 0 0 0-.4-.8l-7-5.25a1 1 0 0 0-1.2 0l-7 5.25a1 1 0 0 0-.4.8V19a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1v-3a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v3a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1\"/>",
                24,
                24,
            )),
            Icon::Work => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M8 6a3 3 0 0 1 3-3h2a3 3 0 0 1 3 3v1h3a3 3 0 0 1 3 3v2h-7v-1a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v1H2v-2a3 3 0 0 1 3-3h3zm-6 8v4a3 3 0 0 0 3 3h14a3 3 0 0 0 3-3v-4h-7v1a1 1 0 0 1-1 1h-4a1 1 0 0 1-1-1v-1zm8-7h4V6a1 1 0 0 0-1-1h-2a1 1 0 0 0-1 1zm3 7h-2v-2h2z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Music => Some((
                "<g fill=\"none\"><path fill=\"currentColor\" d=\"M13 4h4v4h-4v9c0 1-.6 3-3 3s-3-2-3-3s.6-3 3-3s3 2 3 3z\"/><path stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M13 17V8m0 9c0 1-.6 3-3 3s-3-2-3-3s.6-3 3-3s3 2 3 3m0-9V4h4v4z\"/></g>",
                24,
                24,
            )),
            Icon::Heart => Some((
                "<path fill=\"currentColor\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M17 4c-3.2 0-5 2.667-5 4c0-1.333-1.8-4-5-4S3 6.667 3 8c0 7 9 12 9 12s9-5 9-12c0-1.333-.8-4-4-4\"/>",
                24,
                24,
            )),
            Icon::Star => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\"><path d=\"M9 8c-1.667.667-5.4 2.7-7 5.5m9.5-2.5C9.167 12.333 4 16.4 2 22m10.5-7.5c-1.167 1.167-3.8 4.1-5 6.5\"/><path fill=\"currentColor\" d=\"m14.674 6.45l.673-3.285l2.225 2.51l3.027-.294l-1.768 3.062l1.743 2.639l-3.286-.673l-2.51 2.225l.19-3.156l-3.062-1.768z\"/></g>",
                24,
                24,
            )),
            Icon::Plane => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M16.48 14h4.02a2.5 2.5 0 1 0 0-5H6.618a1 1 0 0 1-.894-.553l-.448-.894A1 1 0 0 0 4.382 7H2.517a1 1 0 0 0-.92 1.394l2.143 5a1 1 0 0 0 .92.606h3.863a1 1 0 0 1 .928 1.371L8.55 17.63A1 1 0 0 0 9.477 19h2.042a1 1 0 0 0 .781-.375l3.4-4.25a1 1 0 0 1 .78-.375M9.5 8h4.75L12.3 5.4a1 1 0 0 0-.8-.4H9.618a1 1 0 0 0-.894 1.447z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Idea => Some((
                "<g fill=\"none\"><path fill=\"currentColor\" d=\"M12 7a5 5 0 0 0-2 9.584V19h4v-2.416A5.001 5.001 0 0 0 12 7\"/><path stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M21 12h1m-3.5-6.5l1-1M12 3V2M5.5 5.5l-1-1M3 12H2m8 10h4m3-10a5 5 0 1 0-7 4.584V19h4v-2.416A5 5 0 0 0 17 12\"/></g>",
                24,
                24,
            )),
            Icon::Code => Some((
                "<path fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"m8 7l-5 5l5 5m8 0l5-5l-5-5\"/>",
                24,
                24,
            )),
            Icon::Money => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M1 12C1 5.925 5.925 1 12 1s11 4.925 11 11s-4.925 11-11 11S1 18.075 1 12m12-6a1 1 0 1 0-2 0v1a3 3 0 0 0 0 6h2a1 1 0 1 1 0 2H9a1 1 0 1 0 0 2h2v1a1 1 0 1 0 2 0v-1a3 3 0 1 0 0-6h-2a1 1 0 1 1 0-2h4a1 1 0 1 0 0-2h-2z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Leaf => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M20.156 6.473a2.64 2.64 0 0 0-2.63-2.629C15.049 3.817 9.909 4.193 6.5 7.6c-2.796 2.796-2.915 7.3-.157 10.057s7.26 2.639 10.057-.157c3.407-3.408 3.783-8.548 3.756-11.027m-4.62 1.991a1 1 0 0 1 0 1.415l-.978.977l.587.195a1 1 0 0 1-.633 1.898l-1.535-.512l-1.247 1.247l.586.196a1 1 0 0 1-.632 1.897l-1.173-.39a3 3 0 0 1-.344-.14l-2.41 2.41l-3.121 3.121a1 1 0 0 1-1.414-1.414l3.121-3.121l2.41-2.41a3 3 0 0 1-.14-.344l-.39-1.173a1 1 0 0 1 1.897-.632l.196.586l1.247-1.247l-.512-1.535a1 1 0 0 1 1.898-.633l.195.587l.977-.978a1 1 0 0 1 1.414 0z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Gear => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M9.024 2.783A1 1 0 0 1 10 2h4a1 1 0 0 1 .976.783l.44 1.981q.6.285 1.14.66l1.938-.61a1 1 0 0 1 1.166.454l2 3.464a1 1 0 0 1-.19 1.237l-1.497 1.373a8 8 0 0 1 0 1.316l1.497 1.373a1 1 0 0 1 .19 1.237l-2 3.464a1 1 0 0 1-1.166.454l-1.937-.61q-.54.375-1.14.66l-.44 1.98A1 1 0 0 1 14 22h-4a1 1 0 0 1-.976-.783l-.44-1.981q-.6-.285-1.14-.66l-1.938.61a1 1 0 0 1-1.166-.454l-2-3.464a1 1 0 0 1 .19-1.237l1.497-1.373a8 8 0 0 1 0-1.316L2.53 9.97a1 1 0 0 1-.19-1.237l2-3.464a1 1 0 0 1 1.166-.454l1.937.61q.54-.375 1.14-.66l.44-1.98zM12 15a3 3 0 1 0 0-6a3 3 0 0 0 0 6\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Flag => Some((
                "<g fill=\"none\"><path fill=\"currentColor\" d=\"M19 5H5v9h14z\"/><path stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M5 20v-6m0-9h14v9H5m0-9v9m0-9V4\"/></g>",
                24,
                24,
            )),
            Icon::Pin => Some((
                "<g fill=\"none\"><path fill=\"currentColor\" d=\"M12.956 18.956L9 15l-3.956-3.956a1 1 0 0 1 .314-1.626l5.261-2.255a1 1 0 0 0 .535-.548l1.283-3.207a1 1 0 0 1 1.635-.336l6.856 6.856a1 1 0 0 1-.336 1.635l-3.207 1.283a1 1 0 0 0-.548.535l-2.255 5.261a1 1 0 0 1-1.626.314\"/><path stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"m4 20l5-5m0 0l3.956 3.956a1 1 0 0 0 1.626-.314l2.255-5.261a1 1 0 0 1 .548-.535l3.207-1.283a1 1 0 0 0 .336-1.635l-6.856-6.856a1 1 0 0 0-1.635.336l-1.283 3.207a1 1 0 0 1-.535.548L5.358 9.418a1 1 0 0 0-.314 1.626z\"/></g>",
                24,
                24,
            )),
            Icon::Bug => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M7.293 3.293a1 1 0 0 1 1.414 0l1.876 1.876A6.4 6.4 0 0 1 12 5q.772.002 1.445.14l1.848-1.847a1 1 0 1 1 1.414 1.414L15.45 5.965A5.5 5.5 0 0 1 17.249 8H18c.173 0 .456-.06.666-.212c.159-.114.334-.314.334-.788a1 1 0 1 1 2 0c0 1.126-.491 1.926-1.166 2.412A3.23 3.23 0 0 1 18 10h-.086c.06.36.086.7.086 1v1h2a1 1 0 1 1 0 2h-2v1c0 .3-.026.64-.086 1H18c.493 0 1.211.14 1.834.588C20.51 17.075 21 17.875 21 19a1 1 0 1 1-2 0c0-.474-.175-.674-.334-.788A1.24 1.24 0 0 0 18 18h-.751a5.5 5.5 0 0 1-1.552 1.857C14.766 20.563 13.543 21 12 21s-2.765-.437-3.697-1.143c-.7-.53-1.2-1.188-1.552-1.857H6c-.173 0-.456.06-.666.212c-.159.114-.334.314-.334.788a1 1 0 1 1-2 0c0-1.126.492-1.926 1.166-2.412A3.23 3.23 0 0 1 6 16h.086c-.06-.36-.086-.7-.086-1v-1H4a1 1 0 1 1 0-2h2v-1q0-.523.065-1H6c-.493 0-1.211-.14-1.834-.588C3.492 8.926 3 8.126 3 7a1 1 0 0 1 2 0c0 .474.175.674.334.788c.21.152.493.212.666.212h.696A5.34 5.34 0 0 1 8.58 5.994L7.293 4.707a1 1 0 0 1 0-1.414M12 9a1 1 0 1 0 0 2h.001a1 1 0 1 0 0-2zm-3 4a1 1 0 0 1 1-1h.001a1 1 0 1 1 0 2H10a1 1 0 0 1-1-1m5-1a1 1 0 1 0 0 2h.001a1 1 0 1 0 0-2z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Cart => Some((
                "<g fill=\"none\"><path fill=\"currentColor\" d=\"M18 15H7L5.5 6H21z\"/><path stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M3 3h2l.5 3m0 0L7 15h11l3-9z\"/><circle cx=\"8\" cy=\"20\" r=\"1\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\"/><circle cx=\"17\" cy=\"20\" r=\"1\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\"/></g>",
                24,
                24,
            )),
            Icon::Car => Some((
                "<g fill=\"none\"><path stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"m3 11l2.48-5.788A2 2 0 0 1 7.32 4h9.362a2 2 0 0 1 1.838 1.212L21 11M3 11h18M3 11v7m18-7v7m-3 0v.5a1.5 1.5 0 0 0 1.5 1.5v0a1.5 1.5 0 0 0 1.5-1.5V18m-3 0H6m12 0h3M6 18v.5A1.5 1.5 0 0 1 4.5 20v0A1.5 1.5 0 0 1 3 18.5V18m3 0H3\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M3 11h18v7H3zm3 3a1 1 0 0 1 1-1h.001a1 1 0 1 1 0 2H7a1 1 0 0 1-1-1m11-1a1 1 0 1 0 0 2h.001a1 1 0 1 0 0-2z\" clip-rule=\"evenodd\"/></g>",
                24,
                24,
            )),
            Icon::Bell => Some((
                "<g fill=\"none\"><path fill=\"currentColor\" d=\"M6 11c0-4.8 4-6 6-6c4.8 0 6 4 6 6v4l2 2H4l2-2z\"/><path stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M12 5c-2 0-6 1.2-6 6v4l-2 2h16l-2-2v-4c0-2-1.2-6-6-6m0 0V3M9 18c0 1 .6 3 3 3s3-2 3-3\"/></g>",
                24,
                24,
            )),
            Icon::Calendar => Some((
                "<g fill=\"none\"><path fill=\"currentColor\" d=\"M4 7v2h16V7a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2\"/><path stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M16 5h2a2 2 0 0 1 2 2v2H4V7a2 2 0 0 1 2-2h2m8 0V3m0 2H8m0-2v2M4 9.5V19a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9.5\"/></g>",
                24,
                24,
            )),
            Icon::Envelope => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M5 20a3 3 0 0 1-3-3V7a3 3 0 0 1 3-3h14a3 3 0 0 1 3 3v10a3 3 0 0 1-3 3zM7.625 8.22a1 1 0 1 0-1.25 1.56l3.75 3.001a3 3 0 0 0 3.75 0l3.75-3a1 1 0 1 0-1.25-1.562l-3.75 3a1 1 0 0 1-1.25 0z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Phone => Some((
                "<g fill=\"none\"><path fill=\"currentColor\" d=\"M20 16v4c-2.758 0-5.07-.495-7-1.325c-3.841-1.652-6.176-4.63-7.5-7.675C4.4 8.472 4 5.898 4 4h4l1 4l-3.5 3c1.324 3.045 3.659 6.023 7.5 7.675L16 15z\"/><path stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M13 18.675c1.93.83 4.242 1.325 7 1.325v-4l-4-1zm0 0C9.159 17.023 6.824 14.045 5.5 11m0 0C4.4 8.472 4 5.898 4 4h4l1 4z\"/></g>",
                24,
                24,
            )),
            Icon::Moon => Some((
                "<path fill=\"currentColor\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M9.353 3C5.849 4.408 3 7.463 3 11.47A9.53 9.53 0 0 0 12.53 21c4.007 0 7.062-2.849 8.47-6.353C8.17 17.065 8.14 8.14 9.353 3\"/>",
                24,
                24,
            )),
            Icon::Cloud => Some((
                "<path fill=\"currentColor\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M2 13.5C2 17.9 5.667 19 7.5 19h10c1.5 0 4.5-.9 4.5-4.5S19 10 17.5 10c0-1.5-1.5-5-5-5c-2.8 0-4.5 2-5 3C5.667 8 2 9.1 2 13.5\"/>",
                24,
                24,
            )),
            Icon::Film => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M5 5a3 3 0 0 0-3 3v8a3 3 0 0 0 3 3h10a3 3 0 0 0 3-3v-1.586l2.293 2.293A1 1 0 0 0 22 16V8a1 1 0 0 0-1.707-.707L18 9.586V8a3 3 0 0 0-3-3z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Pencil => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M15.586 3a2 2 0 0 1 2.828 0L21 5.586a2 2 0 0 1 0 2.828L19.414 10L14 4.586zm-3 3l-9 9A2 2 0 0 0 3 16.414V19a2 2 0 0 0 2 2h2.586A2 2 0 0 0 9 20.414l9-9z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Key => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M8 9a7 7 0 1 1 5.562 6.852L12 17.414a2 2 0 0 1-1.414.586H10a2 2 0 0 1-2 2a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-2.586A2 2 0 0 1 2.586 16l5.562-5.562A7 7 0 0 1 8 9m7-3a1 1 0 1 0 0 2a1 1 0 0 1 1 1a1 1 0 1 0 2 0a3 3 0 0 0-3-3\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Lock => Some((
                "<g fill=\"none\"><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M3 12a3 3 0 0 1 3-3h12a3 3 0 0 1 3 3v7a3 3 0 0 1-3 3H6a3 3 0 0 1-3-3zm10 2a1 1 0 1 0-2 0v3a1 1 0 1 0 2 0z\" clip-rule=\"evenodd\"/><path stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M8 10V7a4 4 0 0 1 4-4v0a4 4 0 0 1 4 4v3\"/></g>",
                24,
                24,
            )),
            Icon::Trophy => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\"><circle cx=\"12\" cy=\"9\" r=\"7\"/><path fill=\"currentColor\" d=\"M7 20.234V14c.667.667 2.6 2 5 2s4.333-1.333 5-2v6.234a1 1 0 0 1-1.514.857l-2.972-1.782a1 1 0 0 0-1.028 0L8.514 21.09A1 1 0 0 1 7 20.234\"/></g>",
                24,
                24,
            )),
            Icon::Rocket => Some((
                "<g fill=\"none\"><path stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M4.95 16.264s-1.703 2.54-.707 3.535c.995.996 3.535-.707 3.535-.707\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M20.506 3.536a1 1 0 0 1 .268.928l-.317 1.402a9 9 0 0 1-2.414 4.375l-4.644 4.644c1.027 1.272 1.36 2.48 1.1 3.632c-.271 1.2-1.16 2.086-1.712 2.637l-.06.06a1 1 0 0 1-1.564-.193L9.17 17.696a1 1 0 0 0-.15-.192l-2.57-2.568l-.76-.456l3.459-3.843l.007.005L13.8 6a9 9 0 0 1 4.376-2.414l1.402-.318a1 1 0 0 1 .928.269zM8.322 10.062c-.969-.565-1.9-.722-2.797-.52c-1.2.272-2.086 1.16-2.637 1.713l-.06.059a1 1 0 0 0 .193 1.564l1.796 1.078z\" clip-rule=\"evenodd\"/></g>",
                24,
                24,
            )),
            Icon::Medal => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\"><circle cx=\"12\" cy=\"9\" r=\"7\"/><path fill=\"currentColor\" d=\"M7 20.234V14c.667.667 2.6 2 5 2s4.333-1.333 5-2v6.234a1 1 0 0 1-1.514.857l-2.972-1.782a1 1 0 0 0-1.028 0L8.514 21.09A1 1 0 0 1 7 20.234\"/></g>",
                24,
                24,
            )),
            Icon::Bookmark => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M7 2a3 3 0 0 0-3 3v15.138a1.5 1.5 0 0 0 2.244 1.303l5.26-3.006a1 1 0 0 1 .992 0l5.26 3.006A1.5 1.5 0 0 0 20 20.138V5a3 3 0 0 0-3-3z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Folder => Some((
                "<path fill=\"currentColor\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M3 6a2 2 0 0 1 2-2h3.93a2 2 0 0 1 1.664.89l.812 1.22A2 2 0 0 0 13.07 7H19a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z\"/>",
                24,
                24,
            )),
            Icon::User => Some((
                "<g fill=\"none\" stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\"><circle cx=\"12\" cy=\"8\" r=\"5\" fill=\"currentColor\"/><path d=\"M20 21a8 8 0 1 0-16 0\"/><path fill=\"currentColor\" d=\"M12 13a8 8 0 0 0-8 8h16a8 8 0 0 0-8-8\"/></g>",
                24,
                24,
            )),
            Icon::Ship => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M9 3a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2h3a1 1 0 0 1 .981 1.192l-.437 2.238l-1.327-.295l-5-1.111a1 1 0 0 0-.434 0l-5 1.11l-1.327.296l-.437-2.238A1 1 0 0 1 6 5h3zm-6.092 7.996l-.125.028a1 1 0 0 0-.677 1.423l2 4a1 1 0 0 0 1.035.543L12 16.01l6.859.98a1 1 0 0 0 1.035-.543l2-4a1 1 0 0 0-.677-1.423l-.125-.028a1 1 0 0 1-.309-.02l-4-.889L12 9.024l-4.783 1.063l-4 .89a1 1 0 0 1-.309.019m6.36 7.609a3.63 3.63 0 0 1 5.465 0l.035.04a1.57 1.57 0 0 0 2.053.273a3.57 3.57 0 0 1 3.305-.344l1.245.497a1 1 0 0 1-.742 1.857l-1.245-.498a1.57 1.57 0 0 0-1.454.152a3.57 3.57 0 0 1-4.667-.62l-.035-.04a1.63 1.63 0 0 0-2.456 0l-.035.04a3.57 3.57 0 0 1-4.667.62a1.57 1.57 0 0 0-1.454-.152l-1.245.498a1 1 0 1 1-.742-1.857l1.245-.497a3.57 3.57 0 0 1 3.305.344a1.57 1.57 0 0 0 2.053-.273l.035-.04z\" clip-rule=\"evenodd\"/>",
                24,
                24,
            )),
            Icon::Cake => Some((
                "<g fill=\"none\"><path stroke=\"currentColor\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-width=\"2\" d=\"M12 3h.01M7 3h.01M17 3h.01\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M7 5a1 1 0 0 1 1 1v2h3V6a1 1 0 1 1 2 0v2h3V6a1 1 0 1 1 2 0v2.1a5.005 5.005 0 0 1 3.937 4.102c-.4.155-.75.383-1.047.63c-.532.44-.966.994-1.302 1.46c-.265.367-.714.708-1.588.708s-1.324-.342-1.588-.71A2.36 2.36 0 0 1 16 13a1 1 0 1 0-2 0c0 .34-.11.872-.412 1.29c-.264.368-.714.71-1.588.71s-1.324-.342-1.588-.71A2.36 2.36 0 0 1 10 13a1 1 0 1 0-2 0c0 .34-.11.872-.412 1.29c-.264.368-.714.71-1.588.71s-1.323-.341-1.588-.709c-.336-.465-.77-1.019-1.302-1.46a3.8 3.8 0 0 0-1.047-.629A5.005 5.005 0 0 1 6 8.1V6a1 1 0 0 1 1-1m-5 9.52V19a3 3 0 0 0 3 3h14a3 3 0 0 0 3-3v-4.48c-.27.256-.532.583-.79.94c-.635.883-1.685 1.54-3.21 1.54c-1.367 0-2.353-.529-3-1.273c-.647.744-1.633 1.273-3 1.273s-2.353-.529-3-1.273C8.353 16.47 7.367 17 6 17c-1.525 0-2.575-.657-3.21-1.54a7 7 0 0 0-.79-.94\" clip-rule=\"evenodd\"/></g>",
                24,
                24,
            )),
            _ => None,
        }
    }

    fn pixelarticons(self) -> Option<(&'static str, u32, u32)> {
        match self {
            Icon::Coffee => Some((
                "<path fill=\"currentColor\" d=\"M4 4h16v2H4zm0 2h2v8H4zm2 8h10v2H6zm14-8h2v4h-2zm-2 4h2v2h-2zm-2-4h2v8h-2zM2 18h18v2H2z\"/>",
                24,
                24,
            )),
            Icon::Book => Some((
                "<path fill=\"currentColor\" d=\"M2 3h9v2H2zM0 19h11v2H0zM13 3h9v2h-9zm0 16h11v2H13zM11 5h2v18h-2zM0 5h2v14H0zm22 0h2v14h-2zm-7 2h5v2h-5zm0 4h5v2h-5zm0 4h2v2h-2z\"/>",
                24,
                24,
            )),
            Icon::Camera => Some((
                "<path fill=\"currentColor\" d=\"M4 5h4v2H4zm4-2h8v2H8zm8 2h4v2h-4zM2 7h2v12H2zm2 12h16v2H4zM20 7h2v12h-2zM10 8h4v2h-4zm0 6h4v2h-4zm-2-4h2v4H8zm6 0h2v4h-2z\"/>",
                24,
                24,
            )),
            Icon::Home => Some((
                "<path fill=\"currentColor\" d=\"M4 20h16v2H4zm16-10h2v10h-2zM2 10h2v10H2zm2-2h2v2H4zm2-2h2v2H6zm2-2h2v2H8zm2-2h4v2h-4zm4 2h2v2h-2zm2 2h2v2h-2zm2 2h2v2h-2zM8 14h2v6H8zm2-2h4v2h-4zm4 2h2v6h-2z\"/>",
                24,
                24,
            )),
            Icon::Work => Some((
                "<path fill=\"currentColor\" d=\"M2 8h2v12H2zm18 0h2v12h-2zM4 6h16v2H4zm0 14h16v2H4zM8 4h2v2H8zm2-2h4v2h-4zm4 2h2v2h-2z\"/>",
                24,
                24,
            )),
            Icon::Music => Some((
                "<path fill=\"currentColor\" d=\"M4 12h4v2H4zm-2 2h2v4H2zm2 4h4v2H4zM8 6h2v12H8zm10 0h2v12h-2zm-6 8h2v4h-2zm2-2h4v2h-4zm0 6h4v2h-4zM10 4h8v2h-8z\"/>",
                24,
                24,
            )),
            Icon::Heart => Some((
                "<path fill=\"currentColor\" d=\"M13 22h-2v-2h2zm-2-2H9v-2h2zm4 0h-2v-2h2zm-6-2H7v-2h2zm8 0h-2v-2h2zM7 16H5v-2h2zm12 0h-2v-2h2zM5 14H3v-2h2zm16 0h-2v-2h2zM3 12H1V6h2zm20 0h-2V6h2zM13 8h-2V6h2zM5 6H3V4h2zm6 0H9V4h2zm4 0h-2V4h2zm6 0h-2V4h2zM9 4H5V2h4zm10 0h-4V2h4z\"/>",
                24,
                24,
            )),
            Icon::Star => Some((
                "<path fill=\"currentColor\" d=\"M5 20h3v2H3v-6h2zm16 2h-5v-2h3v-4h2zm-11-2H8v-2h2zm6 0h-2v-2h2zm-2-2h-4v-2h4zm-7-2H5v-3h2zm12 0h-2v-3h2zM5 13H3v-2h2zm16 0h-2v-2h2zM9 9H3v2H1V7h8zm14 2h-2V9h-6V7h8zM11 7H9V3h2zm4 0h-2V3h2zm-2-4h-2V1h2z\"/>",
                24,
                24,
            )),
            Icon::Idea => Some((
                "<path fill=\"currentColor\" d=\"M9 4h6v2H9zM7 6h2v2H7zm8 0h2v2h-2zm4-2h2v2h-2zm2-2h2v2h-2zM0 10h3v2H0zm21 0h3v2h-3zM3 4h2v2H3zM1 2h2v2H1zm6 12h2v2H7zm8 0h2v2h-2zM5 8h2v6H5zm12 0h2v6h-2zm-8 8h6v2H9zm0 4h6v2H9zm0-2h2v2H9zm4 0h2v2h-2zM11 0h2v3h-2z\"/>",
                24,
                24,
            )),
            Icon::Code => Some((
                "<path fill=\"currentColor\" d=\"M4 2h16v2H4zm0 18h16v2H4zM2 4h2v16H2zm18 0h2v16h-2zM6 16h2v2H6zm2-2h2v2H8zm-2-2h2v2H6z\"/>",
                24,
                24,
            )),
            Icon::Money => Some((
                "<path fill=\"currentColor\" d=\"M17 18h-4v4h-2v-4H7v-2h10zm2-2h-2v-3h2zm-2-3H7v-2h10zM7 11H5V8h2zm6-5h4v2H7V6h4V2h2z\"/>",
                24,
                24,
            )),
            Icon::Gift => Some((
                "<path fill=\"currentColor\" d=\"M4 6h16v2H4zM2 8h2v4H2zm2 4h16v2H4zm16-4h2v4h-2zM6 4h2v2H6zm2-2h3v2H8zm3 2h2v2h-2zm2-2h3v2h-3zm3 2h2v2h-2zM4 14h2v6H4zm2 6h12v2H6zm12-6h2v6h-2zm-7-6h2v4h-2zm0 6h2v6h-2z\"/>",
                24,
                24,
            )),
            Icon::Leaf => Some((
                "<path fill=\"currentColor\" d=\"M1 18h2v4H1zm2-2h2v2H3zm2-2h6v2H5zm6-2h2v2h-2zm-6 6h4v2H5zm4 2h4v2H9zm4-2h4v2h-4zm4-2h2v2h-2zm2-8h2v8h-2zm0-4h2v4h-2zm-2-2h2v2h-2zm-4 2h4v2h-4zM7 6h6v2H7zM5 8h2v2H5zm-2 2h2v4H3z\"/>",
                24,
                24,
            )),
            Icon::Gear => Some((
                "<g fill=\"currentColor\"><path d=\"M4 14h2v6H4zm6 0h2v6h-2zm-4-2h4v2H6zm0 8h4v2H6zm-4-4h2v2H2zm20-8h-4V6h4z\"/><path d=\"M10 16h12v2H10zm4-8H2V6h12zm6-4v2h-2V4zm0 6V8h-2v2zm-6-8h4v2h-4zm0 10h4v-2h-4zm-2-8h2v2h-2zm0 6h2V8h-2z\"/></g>",
                24,
                24,
            )),
            Icon::Flag => Some((
                "<g fill=\"currentColor\"><path d=\"M4 2h2v20H4z\"/><path d=\"M4 4h16v2H4zm12 2h2v2h-2zm-2 2h2v2h-2zm2 2h2v2h-2zM4 12h16v2H4z\"/></g>",
                24,
                24,
            )),
            Icon::Pin => Some((
                "<path fill=\"currentColor\" d=\"M7 2h10v2H7zM5 4h2v2H5zm14 0h-2v2h2zM7 17h2v2H7zm2 2h2v2H9zm6-2h2v2h-2zm-2 2h2v2h-2zm-2 2h2v2h-2zm-6-7h2v3H5zm12 0h2v3h-2zM3 6h2v8H3zm18 0h-2v8h2zM10 6h4v2h-4zM8 8h2v4H8zm2 4h4v2h-4zm4-4h2v4h-2z\"/>",
                24,
                24,
            )),
            Icon::Bug => Some((
                "<g fill=\"currentColor\"><path d=\"M2 5h2v4H2zm20 0h-2v4h2zM4 9h2v2H4zm16 0h-2v2h2zM2 13h4v2H2zm20 0h-4v2h4zM4 17h2v2H4zm16 0h-2v2h2zM2 19h2v2H2zm20 0h-2v2h2zM6 11h12v2H6z\"/><path d=\"M6 7h2v12H6zm10 0h2v12h-2zM8 19h8v2H8zM8 5h8v2H8z\"/><path d=\"M11 15h2v6h-2zM8 1h2v6H8zm6 0h2v6h-2z\"/></g>",
                24,
                24,
            )),
            Icon::Game => Some((
                "<g fill=\"currentColor\"><path d=\"M4 4h16v2H4zm0 14h16v2H4zM2 6h2v12H2zm18 0h2v12h-2zM8 9h2v6H8z\"/><path d=\"M6 11h6v2H6zm8-2h2v2h-2zm2 4h2v2h-2z\"/></g>",
                24,
                24,
            )),
            Icon::Cart => Some((
                "<path fill=\"currentColor\" d=\"M2 2h2v2H2zm2 6h2v4H4zm2 4h2v4H6zm2 4h10v2H8zm10-4h2v4h-2zm2-4h2v4h-2zM4 6h18v2H4zm0-4h2v4H4zm2 17h3v3H6zm11 0h3v3h-3z\"/>",
                24,
                24,
            )),
            Icon::Car => Some((
                "<path fill=\"currentColor\" d=\"M4 13h6v2H4zm10 0h6v2h-6zM4 17h6v2H4zm10 0h6v2h-6zM2 15h4v2H2zm6 0h8v2H8zm10 0h4v2h-4zm4-4h2v4h-2zm-6-4h2v2h-2zM4 5h12v2H4zm-4 6h2v4H0zm12-2h10v2H12zM2 7h2v4H2zm8 0h2v2h-2z\"/>",
                24,
                24,
            )),
            Icon::Bell => Some((
                "<g fill=\"currentColor\"><path d=\"M9 2h6v2H9zM7 4h2v2H7zm8 0h2v2h-2zM5 6h2v7H5zm12 0h2v7h-2zM3 13h2v4H3zm16 0h2v4h-2z\"/><path d=\"M3 15h18v2H3zm5 3h2v2H8zm6 0h2v2h-2zm-4 2h4v2h-4z\"/></g>",
                24,
                24,
            )),
            Icon::Calendar => Some((
                "<path fill=\"currentColor\" d=\"M5 4h14v2H5zm0 16h14v2H5zM3 10h2v10H3zm0-4h2v2H3zm16 0h2v2h-2zm0 4h2v10h-2zM3 8h18v2H3zm12-6h2v2h-2zM7 2h2v2H7z\"/>",
                24,
                24,
            )),
            Icon::Envelope => Some((
                "<path fill=\"currentColor\" d=\"M6 8h2v2H6zm2 2h2v2H8zm10-2h-2v2h2zm-2 2h-2v2h2zm-6 2h4v2h-4zM2 6h2v12H2zm18 0h2v12h-2zM4 4h16v2H4zm0 14h16v2H4z\"/>",
                24,
                24,
            )),
            Icon::Phone => Some((
                "<path fill=\"currentColor\" d=\"M4 1h5v2H4zm5 2h2v4H9zM7 7h2v4H7zm-3 5h2v2H4zM2 3h2v9H2zm7 8h2v2H9zm2 2h2v2h-2zm2 2h4v2h-4zm4-2h4v2h-4zm4 2h2v5h-2zM6 14h2v2H6zm2 2h2v2H8zm2 2h2v2h-2zm2 2h9v2h-9z\"/>",
                24,
                24,
            )),
            Icon::Moon => Some((
                "<path fill=\"currentColor\" d=\"M18 22H8v-2h10zM8 20H6v-2h2zm12 0h-2v-2h2zM6 18H4v-2h2zm16 0h-2v-4h-2v-2h2v-2h2zM4 16H2V6h2zm14 0h-6v-2h6zm-6-2h-2v-2h2zm-2-2H8V6h2zM6 6H4V4h2zm8-2h-2v2h-2V4H6V2h8z\"/>",
                24,
                24,
            )),
            Icon::Sun => Some((
                "<path fill=\"currentColor\" d=\"M13 22h-2v-3h2zm-6-3H5v-2h2zm12 0h-2v-2h2zm-4-2H9v-2h6zm-6-2H7V9h2zm8 0h-2V9h2zM5 13H2v-2h3zm17 0h-3v-2h3zm-7-4H9V7h6zM7 7H5V5h2zm12 0h-2V5h2zm-6-2h-2V2h2z\"/>",
                24,
                24,
            )),
            Icon::Cloud => Some((
                "<g fill=\"currentColor\"><path d=\"M22 10h-4v2h4zm2 2h-2v6h2zm-2 6H2v2h20zM2 12H0v6h2zm2-2H2v2h2zm4-2H4v2h4zm8-4h-6v2h6zm-6 2H8v2h2zm0 4H8v2h2zm8-4h-2v2h2z\"/><path d=\"M20 8h-2v4h2zm-2 4h-2v2h2z\"/></g>",
                24,
                24,
            )),
            Icon::Film => Some((
                "<path fill=\"currentColor\" d=\"M4 3h16v2H4zm0 6h16v2H4zM2 5h2v14H2zm18 0h2v14h-2zM4 19h16v2H4zM18 7h-2v2h2zm-8 0H8v2h2zm6-2h-2v2h2zM8 5H6v2h2z\"/>",
                24,
                24,
            )),
            Icon::Pencil => Some((
                "<path fill=\"currentColor\" d=\"M4 16h2v2h2v2h2v2H2v-8h2zm8 4h-2v-2h2zm2-2h-2v-2h2zm-4-2H8v-2h2zm6 0h-2v-2h2zM6 14H4v-2h2zm6 0h-2v-2h2zm6 0h-2v-2h2zM8 12H6v-2h2zm6 0h-2v-2h2zm6 0h-2v-2h2zm-10-2H8V8h2zm8 0h-2V8h2zm4 0h-2V8h2zM12 8h-2V6h2zm4 0h-2V6h2zm4 0h-2V6h2zm-6-2h-2V4h2zm4 0h-2V4h2zm-2-2h-2V2h2z\"/>",
                24,
                24,
            )),
            Icon::Key => Some((
                "<path fill=\"currentColor\" d=\"M11 18H3v-2h8zm12-3h-2v3h-4v-2h2v-3h2v-2H11V8h2v1h10zM3 16H1V8h2zm14 0h-2v-1h-2v1h-2v-3h6zm-8-2H5v-4h4zm2-6H3V6h8z\"/>",
                24,
                24,
            )),
            Icon::Lock => Some((
                "<path fill=\"currentColor\" d=\"M5 8h14v2H5zm0 12h14v2H5zM3 10h2v10H3zm16 0h2v10h-2zM7 4h2v4H7zm2-2h6v2H9zm6 2h2v4h-2z\"/>",
                24,
                24,
            )),
            Icon::Dog => Some((
                "<path fill=\"currentColor\" d=\"M14 22h-4v-2h4zm2-2h-2v-2h-4v2H8v-2H5v-2h6v-2h2v2h6v2h-3zM3 10h2v6H3v-4H1V6h2zm20 2h-2v4h-2v-6h2V6h2zm-12 0H9V8h2zm4 0h-2V8h2zm-8-2H5V6h2zm12 0h-2V6h2zM5 6H3V4h2zm14-2h-2v2h-2V4H9v2H7V4H5V2h14zm2 2h-2V4h2z\"/>",
                24,
                24,
            )),
            Icon::Wrench => Some((
                "<path fill=\"currentColor\" d=\"M9 22H7v-2h2zm12-6h2v6h-6v-6h2v-6h2zm-2 2v2h2v-2zM7 20H5v-8h2zm4 0H9v-8h2zm-6-8H3v-2h2zm8 0h-2v-2h2zM3 10H1V4h2zm12 0h-2V4h2zm4 0h-2V4h2zm4 0h-2V4h2zM7 6h2V2h4v2h-2v4H5V4H3V2h4zm14-2h-2V2h2z\"/>",
                24,
                24,
            )),
            Icon::Trophy => Some((
                "<path fill=\"currentColor\" d=\"M16 17h-3v2h2v2H9v-2h2v-2H8v-2h8zm2-12h4v6h-2V7h-2v4h2v2h-2v2h-2V5H8v10H6v-2H4v-2h2V7H4v4H2V5h4V3h12z\"/>",
                24,
                24,
            )),
            Icon::Wine => Some((
                "<path fill=\"currentColor\" d=\"M9 1h6v2H9zm0 2h2v4H9zm4 0h2v4h-2zM7 7h2v2H7zm8 0h2v2h-2zm2 2h2v12h-2zM5 9h2v12H5zm2 12h10v2H7z\"/>",
                24,
                24,
            )),
            Icon::Truck => Some((
                "<g fill=\"currentColor\"><path d=\"M2 4h12v2H2zM0 16h4v2H0zm10 0h4v2h-4zm12-4h2v6h-2zm-8-6h2v12h-2zM0 6h2v10H0zm20 4h2v2h-2z\"/><path d=\"M14 8h6v2h-6zM4 14h6v2H4zm10 0h6v2h-6zM4 16h2v2H4zm10 0h2v2h-2zM4 18h6v2H4zm10 0h6v2h-6zm-6-2h2v2H8zm10 0h4v2h-4z\"/></g>",
                24,
                24,
            )),
            Icon::Bag => Some((
                "<g fill=\"currentColor\"><path d=\"M3 6h18v2H3zm2 14h14v2H5zM3 8h2v12H3zm16 0h2v12h-2z\"/><path d=\"M7 4h2v6H7zm2-2h6v2H9zm6 2h2v6h-2z\"/></g>",
                24,
                24,
            )),
            Icon::Movie => Some((
                "<path fill=\"currentColor\" d=\"M4 3h16v2H4zm0 6h16v2H4zM2 5h2v14H2zm18 0h2v14h-2zM4 19h16v2H4zM18 7h-2v2h2zm-8 0H8v2h2zm6-2h-2v2h2zM8 5H6v2h2z\"/>",
                24,
                24,
            )),
            Icon::Bookmark => Some((
                "<path fill=\"currentColor\" d=\"M6 2h12v2H6zM4 4h2v18H4zm14 0h2v18h-2zm-2 16h2v2h-2zm-2-2h2v2h-2zm-8 2h2v2H6zm2-2h2v2H8zm2-2h4v2h-4z\"/>",
                24,
                24,
            )),
            Icon::Folder => Some((
                "<path fill=\"currentColor\" d=\"M4 4h6v2H4zm0 14h16v2H4zM20 8h2v10h-2zM2 6h2v12H2zm8 0h10v2H10z\"/>",
                24,
                24,
            )),
            Icon::User => Some((
                "<path fill=\"currentColor\" d=\"M9 2h6v2H9zm0 8h6v2H9zm6-6h2v6h-2zM7 4h2v6H7zM4 18h2v4H4zm14 0h2v4h-2zM8 14h8v2H8zm-2 2h2v2H6zm10 0h2v2h-2z\"/>",
                24,
                24,
            )),
            Icon::Paint => Some((
                "<g fill=\"currentColor\"><path d=\"M7 2h10v2H7zM5 4h2v10H5zm12-2h2v12h-2z\"/><path d=\"M13 2h2v6h-2zM9 2h2v4H9zm-4 8h14v2H5zm2 4h10v2H7zm2 2h2v4H9zm4 0h2v4h-2zm-4 4h6v2H9z\"/></g>",
                24,
                24,
            )),
            Icon::Tree => Some((
                "<g fill=\"currentColor\"><path d=\"M6 4h2v2H6zm2-2h8v2H8zm10 4h2v4h-2zm2 4h2v6h-2zm-2 6h2v2h-2zM4 16h2v2H4zm-2-6h2v6H2zm4 8h12v2H6zM4 6h2v4H4z\"/><path d=\"M11 18h2v4h-2zm5-14h2v2h-2z\"/></g>",
                24,
                24,
            )),
            Icon::Ship => Some((
                "<g fill=\"currentColor\"><path d=\"M14 8h2v2h-2zm4 8h2v2h-2zM8 4h2v4H8z\"/><path d=\"M6 6h8v2H6zm-4 4h20v2H2zm18 2h2v4h-2zM2 12h2v6H2zm4-4h2v2H6z\"/><path d=\"M0 16h4v2H0zm4 2h4v2H4zm4-2h4v2H8zm4 2h4v2h-4zm4-2h4v2h-4zm4 2h4v2h-4z\"/></g>",
                24,
                24,
            )),
            Icon::Bed => Some((
                "<g fill=\"currentColor\"><path d=\"M2 4h2v16H2zm18 6h2v10h-2z\"/><path d=\"M2 16h20v2H2zm2-8h16v2H4zm2 2h2v6H6z\"/></g>",
                24,
                24,
            )),
            Icon::Cake => Some((
                "<path fill=\"currentColor\" d=\"M1 20h22v2H1zm2-8h2v8H3zm2-2h14v2H5zm14 2h2v8h-2zm-8-5h2v3h-2zM7 7h2v3H7zm8 0h2v3h-2zM7 3h2v2H7zm4 0h2v2h-2zm4 0h2v2h-2zM5 14h2v2H5zm2 2h4v2H7zm4-2h6v2h-6zm6 2h2v2h-2z\"/>",
                24,
                24,
            )),
            _ => None,
        }
    }

    fn duoicons(self) -> Option<(&'static str, u32, u32)> {
        match self {
            Icon::Book => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M6 2a2 2 0 0 0-2 2v1a1 1 0 1 0 0 2v2a1 1 0 1 0 0 2v2a1 1 0 1 0 0 2v2a1 1 0 1 0 0 2v1a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M8.5 6A1.5 1.5 0 0 0 7 7.5v1A1.5 1.5 0 0 0 8.5 10h7A1.5 1.5 0 0 0 17 8.5v-1A1.5 1.5 0 0 0 15.5 6z\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Camera => Some((
                "<path fill=\"currentColor\" d=\"M14.793 3c.346 0 .682.12.95.34l.11.1L17.415 5H20a2 2 0 0 1 1.995 1.85L22 7v12a2 2 0 0 1-1.85 1.995L20 21H4a2 2 0 0 1-1.995-1.85L2 19V7a2 2 0 0 1 1.85-1.995L4 5h2.586l1.56-1.56c.245-.246.568-.399.913-.433L9.207 3z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" d=\"M12 7.5c-3.849 0-6.255 4.167-4.33 7.5A5 5 0 0 0 12 17.5c3.849 0 6.255-4.167 4.33-7.5A5 5 0 0 0 12 7.5\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Work => Some((
                "<path fill=\"currentColor\" d=\"M14 3a3 3 0 0 1 3 3h3a2 2 0 0 1 2 2v11a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3a3 3 0 0 1 3-3z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" d=\"M14 5h-4a1 1 0 0 0-.993.883L9 6h6a1 1 0 0 0-.883-.993zm5 5H5a1 1 0 0 0-.117 1.993L5 12h6v1a1 1 0 0 0 1.993.117L13 13v-1h6a1 1 0 0 0 .117-1.993z\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Idea => Some((
                "<path fill=\"currentColor\" d=\"M13 20a1 1 0 0 1 .117 1.993L13 22h-2a1 1 0 0 1-.117-1.993L11 20zm.707-13.707a1 1 0 0 0-1.32-.083l-.094.083L10.3 8.286a1.01 1.01 0 0 0-.084 1.333l.084.095L11.586 11l-1.293 1.293a1 1 0 0 0 1.32 1.497l.094-.083l1.993-1.993c.36-.36.396-.931.084-1.333l-.084-.095L12.414 9l1.293-1.293a1 1 0 0 0 0-1.414\" class=\"duoicon-primary-layer\"/><path fill=\"currentColor\" d=\"M12 2c4.41 0 8 3.543 8 7.933c0 3.006-1.522 5.196-2.78 6.494l-.284.283l-.27.252l-.252.22l-.33.27l-.328.244c-.241.17-.403.419-.55.678l-.205.364c-.238.41-.517.762-1.108.762h-3.786c-.59 0-.87-.351-1.108-.762l-.118-.208c-.172-.312-.348-.63-.637-.834l-.232-.171l-.199-.155l-.227-.188l-.252-.22l-.27-.252l-.285-.283C5.522 15.129 4 12.939 4 9.933C4 5.543 7.59 2 12 2\" class=\"duoicon-secondary-layer\" opacity=\".55\"/>",
                24,
                24,
            )),
            Icon::Money => Some((
                "<path fill=\"currentColor\" d=\"M21 16.143V17.5c0 .814-.381 1.51-.91 2.057c-.523.542-1.233.984-2.032 1.334C16.456 21.591 14.314 22 12 22s-4.456-.408-6.058-1.109c-.799-.35-1.509-.792-2.032-1.334c-.485-.5-.845-1.128-.902-1.856L3 17.5v-1.357q.697.398 1.494.695c2.03.751 4.685 1.17 7.506 1.17s5.476-.419 7.506-1.17q.598-.222 1.139-.503zM12 3c2.314 0 4.456.408 6.058 1.109c.799.35 1.509.792 2.032 1.334c.485.5.845 1.128.902 1.856L21 7.5v.748a8.3 8.3 0 0 1-2.188 1.214c-1.755.65-4.164 1.047-6.812 1.047c-2.647 0-5.056-.397-6.812-1.047a8.3 8.3 0 0 1-1.905-1.006L3 8.248V7.5c0-.814.381-1.51.91-2.057c.523-.542 1.233-.984 2.032-1.334C7.544 3.409 9.686 3 12 3\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" d=\"M3 10.643q.697.398 1.494.695c2.03.751 4.685 1.17 7.506 1.17s5.476-.419 7.506-1.17A10 10 0 0 0 21 10.643v3.105a8.3 8.3 0 0 1-2.188 1.214c-1.755.65-4.164 1.047-6.812 1.047c-2.647 0-5.056-.397-6.812-1.047A8.3 8.3 0 0 1 3 13.748z\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Gear => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M9.965 2.809a1.51 1.51 0 0 0-1.401-.203a10 10 0 0 0-2.982 1.725a1.51 1.51 0 0 0-.524 1.313c.075.753-.058 1.48-.42 2.106c-.361.627-.925 1.106-1.615 1.417c-.458.203-.786.62-.875 1.113a10 10 0 0 0 0 3.44c.093.537.46.926.875 1.114c.69.31 1.254.79 1.616 1.416c.361.627.494 1.353.419 2.106c-.045.452.107.964.524 1.313a10 10 0 0 0 2.982 1.725c.471.169.996.093 1.4-.203c.615-.442 1.312-.691 2.036-.691s1.42.249 2.035.691c.37.266.89.39 1.401.203a10 10 0 0 0 2.982-1.725c.417-.349.57-.86.524-1.313c-.075-.753.057-1.48.42-2.106c.361-.627.925-1.105 1.615-1.416c.414-.188.782-.577.875-1.114a10.1 10.1 0 0 0 0-3.44a1.51 1.51 0 0 0-.875-1.113c-.69-.311-1.254-.79-1.616-1.417c-.362-.626-.494-1.353-.419-2.106a1.51 1.51 0 0 0-.524-1.313a10 10 0 0 0-2.982-1.725a1.51 1.51 0 0 0-1.4.203C13.42 3.25 12.723 3.5 12 3.5s-1.42-.249-2.035-.691\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M9 12c0-2.309 2.5-3.753 4.5-2.598A3 3 0 0 1 15 12c0 2.309-2.5 3.753-4.5 2.598A3 3 0 0 1 9 12\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Pin => Some((
                "<path fill=\"currentColor\" d=\"M6.72 16.64a1 1 0 1 1 .56 1.92c-.5.146-.86.3-1.091.44c.238.143.614.303 1.136.452C8.48 19.782 10.133 20 12 20s3.52-.218 4.675-.548c.523-.149.898-.309 1.136-.452c-.23-.14-.59-.294-1.09-.44a1 1 0 0 1 .559-1.92c.668.195 1.28.445 1.75.766c.435.299.97.82.97 1.594c0 .783-.548 1.308-.99 1.607c-.478.322-1.103.573-1.786.768C15.846 21.77 14 22 12 22s-3.846-.23-5.224-.625c-.683-.195-1.308-.446-1.786-.768c-.442-.3-.99-.824-.99-1.607c0-.774.535-1.295.97-1.594c.47-.321 1.082-.571 1.75-.766M12 7.5c-1.54 0-2.502 1.667-1.732 3c.357.619 1.017 1 1.732 1c1.54 0 2.502-1.667 1.732-3A2 2 0 0 0 12 7.5\" class=\"duoicon-primary-layer\"/><path fill=\"currentColor\" d=\"M12 2a7.5 7.5 0 0 1 7.5 7.5c0 2.568-1.4 4.656-2.85 6.14a16.4 16.4 0 0 1-1.853 1.615c-.594.446-1.952 1.282-1.952 1.282a1.71 1.71 0 0 1-1.69 0a21 21 0 0 1-1.952-1.282A16.4 16.4 0 0 1 7.35 15.64C5.9 14.156 4.5 12.068 4.5 9.5A7.5 7.5 0 0 1 12 2\" class=\"duoicon-secondary-layer\" opacity=\".55\"/>",
                24,
                24,
            )),
            Icon::Bug => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M7.67 5.5a5 5 0 0 1 8.66 0L17.2 7H6.8z\" class=\"duoicon-primary-layer\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M3.553 6.106a1 1 0 0 1 1.341.447c.147.293.5.674.973.99C6.353 7.867 6.781 8 7 8h10c.219 0 .647-.133 1.133-.457c.474-.316.826-.697.973-.99a1 1 0 1 1 1.788.894c-.353.707-1 1.326-1.652 1.76a5.5 5.5 0 0 1-.966.516c.297.731.503 1.496.616 2.277H21a1 1 0 1 1 0 2h-2.012a10 10 0 0 1-.74 3.327c.572.33.963.86 1.209 1.35c.349.725.534 1.518.543 2.323a1 1 0 1 1-2 0c0-.374-.101-.966-.332-1.428c-.13-.26-.26-.409-.385-.49c-1.056 1.486-2.539 2.54-4.283 2.835V13a1 1 0 1 0-2 0v8.917c-1.744-.295-3.227-1.35-4.283-2.834c-.126.08-.255.23-.385.49c-.21.447-.323.933-.332 1.427a1 1 0 1 1-2 0a5.5 5.5 0 0 1 .543-2.322c.246-.492.637-1.02 1.209-1.35A10 10 0 0 1 5.012 14H3a1 1 0 1 1 0-2h2.108c.113-.781.32-1.546.616-2.277a5.5 5.5 0 0 1-.966-.516c-.651-.434-1.3-1.053-1.652-1.76a1 1 0 0 1 .447-1.341\" class=\"duoicon-secondary-layer\" opacity=\".55\"/>",
                24,
                24,
            )),
            Icon::Car => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M5.553 5.658A3 3 0 0 1 8.236 4h7.528a3 3 0 0 1 2.683 1.658l1.386 2.771q.366-.15.72-.324a1 1 0 0 1 .894 1.79q-.36.16-.725.312l.961 1.923c.209.417.317.877.317 1.343V16a3 3 0 0 1-1 2.236V19.5a1.5 1.5 0 0 1-3 0V19H6v.5a1.5 1.5 0 0 1-3 0v-1.264c-.614-.55-1-1.348-1-2.236v-2.528c0-.466.109-.925.317-1.341l.953-1.908q-.362-.152-.715-.327a1.01 1.01 0 0 1-.45-1.343a1 1 0 0 1 1.342-.448q.355.175.72.324z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M7.342 6.553A1 1 0 0 1 8.236 6h7.528c.379 0 .725.214.894.553l1.27 2.538C16.38 9.555 14.294 10 12 10s-4.38-.445-5.927-.91zM16.5 16a1.5 1.5 0 1 0 0-3a1.5 1.5 0 0 0 0 3M9 14.5a1.5 1.5 0 1 1-3 0a1.5 1.5 0 0 1 3 0\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Bell => Some((
                "<path fill=\"currentColor\" d=\"M9.042 19.003h5.916c-.385 2.277-3.09 3.283-4.87 1.811a3 3 0 0 1-1.046-1.811\" class=\"duoicon-primary-layer\"/><path fill=\"currentColor\" d=\"M12 2.003a7.5 7.5 0 0 1 7.5 7.5v4l1.418 3.16A.95.95 0 0 1 20.052 18h-16.1a.95.95 0 0 1-.867-1.338l1.415-3.16V9.49l.005-.25A7.5 7.5 0 0 1 12 2.004z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/>",
                24,
                24,
            )),
            Icon::Calendar => Some((
                "<path fill=\"currentColor\" d=\"M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-7z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" d=\"M16 3a1 1 0 0 1 1 1v1h2a2 2 0 0 1 2 2v3H3V7a2 2 0 0 1 2-2h2V4a1 1 0 1 1 2 0v1h6V4a1 1 0 0 1 1-1\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Phone => Some((
                "<path fill=\"currentColor\" d=\"M17 2a2 2 0 0 1 1.995 1.85L19 4v16a2 2 0 0 1-1.85 1.995L17 22H7a2 2 0 0 1-1.995-1.85L5 20V4a2 2 0 0 1 1.85-1.995L7 2z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" d=\"M12.5 16h-1a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h1a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Moon => Some((
                "<path fill=\"currentColor\" d=\"M20.15 18.125L5.875 3.85a9.9 9.9 0 0 1 2.437-1.825A10.3 10.3 0 0 1 11.25 1q-.45 2.475.275 4.837a9.9 9.9 0 0 0 2.5 4.138a9.9 9.9 0 0 0 4.138 2.5q2.362.725 4.837.275a9.6 9.6 0 0 1-1.012 2.938a10.2 10.2 0 0 1-1.838 2.437\" class=\"duoicon-primary-layer\"/><path fill=\"currentColor\" d=\"m19.375 23.05l-2.7-2.7a10 10 0 0 1-1.737.487Q14.05 21 13.1 21a9.8 9.8 0 0 1-3.938-.8a10.3 10.3 0 0 1-3.199-2.162a10.3 10.3 0 0 1-2.163-3.2A9.8 9.8 0 0 1 3 10.9q0-.95.163-1.838q.162-.887.487-1.737L.975 4.65L2.4 3.225l18.4 18.4z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/>",
                24,
                24,
            )),
            Icon::Sun => Some((
                "<path fill=\"currentColor\" d=\"M12 18.5a1.5 1.5 0 0 1 1.493 1.356L13.5 20v1a1.5 1.5 0 0 1-2.993.144L10.5 21v-1a1.5 1.5 0 0 1 1.5-1.5m0-17a1.5 1.5 0 0 1 1.493 1.356L13.5 3v1a1.5 1.5 0 0 1-2.993.144L10.5 4V3A1.5 1.5 0 0 1 12 1.5m5.303 3.075a1.5 1.5 0 0 1 2.225 2.008l-.103.114l-.707.707a1.5 1.5 0 0 1-2.225-2.008l.103-.114zm-12.728 0a1.5 1.5 0 0 1 2.008-.103l.114.103l.707.707a1.5 1.5 0 0 1-2.008 2.225l-.114-.103l-.707-.707a1.5 1.5 0 0 1 0-2.122M21 10.5a1.5 1.5 0 0 1 .144 2.993L21 13.5h-1a1.5 1.5 0 0 1-.144-2.993L20 10.5zm-17 0a1.5 1.5 0 0 1 .144 2.993L4 13.5H3a1.5 1.5 0 0 1-.144-2.993L3 10.5z\" class=\"duoicon-primary-layer\"/><path fill=\"currentColor\" d=\"M12 6c4.619 0 7.506 5 5.196 9A6 6 0 0 1 12 18c-4.619 0-7.506-5-5.196-9A6 6 0 0 1 12 6\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" d=\"M5.282 16.596a1.5 1.5 0 0 1 2.225 2.008l-.103.114l-.707.707a1.5 1.5 0 0 1-2.225-2.008l.103-.114zm11.314 0a1.5 1.5 0 0 1 2.008-.103l.114.103l.707.707a1.5 1.5 0 0 1-2.008 2.225l-.114-.103l-.707-.707a1.5 1.5 0 0 1 0-2.122\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Cloud => Some((
                "<path fill=\"currentColor\" d=\"M14.5 19a1.5 1.5 0 1 1 0 3a1.5 1.5 0 0 1 0-3m-4-6a1.5 1.5 0 1 1 0 3a1.5 1.5 0 0 1 0-3m4 2a1.5 1.5 0 1 1 0 3a1.5 1.5 0 0 1 0-3\" class=\"duoicon-primary-layer\"/><path fill=\"currentColor\" d=\"M11.5 2a6.5 6.5 0 0 1 6.086 4.212c4.455 1.223 5.916 6.811 2.629 10.058a6 6 0 0 1-2.439 1.462C18.637 15.443 16.945 13 14.5 13a1.52 1.52 0 0 1-1.199-.599c-1.615-2.157-4.959-1.757-6.019.72a3.5 3.5 0 0 0 .007 2.772c.167.388.167.828 0 1.216q-.184.422-.253.892H7c-3.849.003-6.257-4.163-4.335-7.497A5 5 0 0 1 5 8.417A6.5 6.5 0 0 1 11.5 2\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" d=\"M10.5 17a1.5 1.5 0 1 1 0 3a1.5 1.5 0 0 1 0-3\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Film => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M2 5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M13 8h3l1-3h-3zM8 8h3l1-3H9zM4 8h2l1-3H4zm16-3h-1l-1 3h2z\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Palette => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M10 3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M13.636 4a2 2 0 0 1 2.701-.117l.127.117L20 7.536a2 2 0 0 1 .204 2.589L13 17.357V4.636zM7.5 15a1.5 1.5 0 1 0 0 3a1.5 1.5 0 0 0 0-3\" class=\"duoicon-primary-layer\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M19.66 12.111c.731.256 1.27.924 1.334 1.727L21 19a2 2 0 0 1-1.85 1.995L19 21h-6v-2.23z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/>",
                24,
                24,
            )),
            Icon::Trophy => Some((
                "<path fill=\"currentColor\" d=\"M12 2c6.158 0 10.007 6.667 6.928 12A8 8 0 0 1 17 16.245v4.61a1.1 1.1 0 0 1-1.486 1.03L12 20.569l-3.514 1.318A1.1 1.1 0 0 1 7 20.856v-4.61C2.192 12.398 3.352 4.788 9.089 2.548A8 8 0 0 1 12 2\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" d=\"M12 6c3.079 0 5.004 3.333 3.464 6A4 4 0 0 1 12 14c-3.079 0-5.004-3.333-3.464-6A4 4 0 0 1 12 6\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Rocket => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"m18.165 2.765l.255.032c.674.093 1.566.218 2.071.724c.414.413.573 1.085.668 1.685l.056.386c.126.91.159 2.102-.056 3.426c-.424 2.613-1.815 5.731-5.308 8.145c-.019.188-.02.378-.016.568l.01.284c.016.437.032.873-.09 1.298c-.19.66-.867 1.095-1.5 1.407l-.31.147l-.4.176c-.748.318-1.758.644-2.391.01c-.38-.379-.536-.935-.663-1.488l-.047-.207a8 8 0 0 0-.2-.774q-.075-.22-.162-.445a3 3 0 0 1-.203.225c-.345.345-.86.586-1.284.755c-.463.183-.987.343-1.472.475l-.249.066l-.477.119l-.432.1l-.517.11l-.323.063a1.01 1.01 0 0 1-1.177-1.177l.086-.431l.154-.698l.124-.51l.094-.36c.132-.484.292-1.008.476-1.47c.168-.425.409-.94.754-1.285l.08-.077l-.064-.026a8 8 0 0 0-.519-.177l-.277-.085c-.694-.21-1.436-.436-1.897-.898c-.56-.559-.371-1.41-.101-2.118l.11-.274l.177-.4l.147-.31c.312-.632.747-1.309 1.407-1.499c.35-.1.714-.106 1.08-.096l.22.007c.286.01.571.021.85-.006c2.414-3.494 5.532-4.885 8.145-5.309a11.8 11.8 0 0 1 3.171-.088\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M15.536 8.466c-1.088-1.089-2.948-.591-3.346.896a2 2 0 0 0 .517 1.932c1.088 1.089 2.948.591 3.346-.896a2 2 0 0 0-.517-1.932M8.353 15.44a1 1 0 0 0-1.1-.06l-.11.074l-.093.083l-.125.158c-.26.376-.408.896-.523 1.382l-.108.468l-.051.213l.191-.046l.418-.096c.578-.135 1.219-.31 1.613-.665a1 1 0 0 0 .088-1.314l-.082-.094l-.024-.023z\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Bank => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"m12.67 2.217l8.5 4.75A1.5 1.5 0 0 1 22 8.31v1.44c0 .69-.56 1.25-1.25 1.25H20v8h1a1 1 0 1 1 0 2H3a1 1 0 1 1 0-2h1v-8h-.75C2.56 11 2 10.44 2 9.75V8.31c0-.522.27-1.002.706-1.274l8.623-4.819c.422-.211.92-.211 1.342 0z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M12 6a1 1 0 1 0 0 2a1 1 0 0 0 0-2m5 5H7v8h2v-6h2v6h2v-6h2v6h2z\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Medal => Some((
                "<path fill=\"currentColor\" d=\"M12 2c6.158 0 10.007 6.667 6.928 12A8 8 0 0 1 17 16.245v4.61a1.1 1.1 0 0 1-1.486 1.03L12 20.569l-3.514 1.318A1.1 1.1 0 0 1 7 20.856v-4.61C2.192 12.398 3.352 4.788 9.089 2.548A8 8 0 0 1 12 2\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" d=\"M12 6c3.079 0 5.004 3.333 3.464 6A4 4 0 0 1 12 14c-3.079 0-5.004-3.333-3.464-6A4 4 0 0 1 12 6\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Bag => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M10.464 3.282a2 2 0 0 1 2.964-.12l.108.12L17.468 8h2.985a1.49 1.49 0 0 1 1.484 1.655l-.092.766l-.1.74l-.082.554l-.095.595l-.108.625l-.122.648l-.136.661q-.108.5-.232.998a21 21 0 0 1-.832 2.584l-.221.54l-.214.488l-.202.434l-.094.194l-.249.49c-.32.61-.924.97-1.563 1.022l-.16.006H6.555a1.93 1.93 0 0 1-1.71-1.008l-.232-.45l-.18-.37l-.095-.205l-.2-.449a21.5 21.5 0 0 1-1.108-3.276a35 35 0 0 1-.156-.654l-.142-.648l-.127-.634l-.112-.613l-.1-.587l-.087-.554l-.074-.513l-.09-.683l-.066-.556l-.017-.153a1.49 1.49 0 0 1 1.348-1.64L3.543 8h2.989z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M12 4.562L9.135 8h5.73zm3.164 7.452a1 1 0 0 0-1.125.708l-.025.114l-.5 3a1 1 0 0 0 1.947.442l.025-.114l.5-3a1 1 0 0 0-.822-1.15m-5.203.708a1 1 0 0 0-1.96.326l.013.116l.5 3l.025.114a1 1 0 0 0 1.96-.326l-.013-.116l-.5-3z\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Movie => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M2 5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M13 8h3l1-3h-3zM8 8h3l1-3H9zM4 8h2l1-3H4zm16-3h-1l-1 3h2z\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Bookmark => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M17 2a3 3 0 0 1 3 3v16a1 1 0 0 1-1.625.78l-1.875-1.5l-1.875 1.5a1 1 0 0 1-1.332-.073L12 20.414l-1.293 1.293a1 1 0 0 1-1.332.074L7.5 20.28l-1.875 1.5A1 1 0 0 1 4 21V5a3 3 0 0 1 3-3z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M15 8H9a1 1 0 0 0-.117 1.993L9 10h6a1 1 0 0 0 .117-1.993zm-3 4H9a1 1 0 1 0 0 2h3a1 1 0 1 0 0-2\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Folder => Some((
                "<path fill=\"currentColor\" d=\"M19.82 6a2 2 0 0 1 1.972 2.329l-1.666 10A2 2 0 0 1 18.153 20H5.847a2 2 0 0 1-1.973-1.671l-1.666-10A2 2 0 0 1 4.18 6z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" d=\"M18 3a1 1 0 1 1 0 2H6a1 1 0 1 1 0-2z\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::User => Some((
                "<path fill=\"currentColor\" d=\"M12 13c2.396 0 4.575.694 6.178 1.671c.8.49 1.484 1.065 1.978 1.69c.486.616.844 1.352.844 2.139c0 .845-.411 1.511-1.003 1.986c-.56.45-1.299.748-2.084.956c-1.578.417-3.684.558-5.913.558s-4.335-.14-5.913-.558c-.785-.208-1.524-.506-2.084-.956C3.41 20.01 3 19.345 3 18.5c0-.787.358-1.523.844-2.139c.494-.625 1.177-1.2 1.978-1.69C7.425 13.694 9.605 13 12 13\" class=\"duoicon-primary-layer\"/><path fill=\"currentColor\" d=\"M12 2c3.849 0 6.255 4.167 4.33 7.5A5 5 0 0 1 12 12c-3.849 0-6.255-4.167-4.33-7.5A5 5 0 0 1 12 2\" class=\"duoicon-secondary-layer\" opacity=\".55\"/>",
                24,
                24,
            )),
            Icon::Pram => Some((
                "<path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M7.746 2.609c.764-.296 1.566.093 1.877.773L12.643 10H18V8.5A2.5 2.5 0 0 1 20.5 6h.5a1 1 0 1 1 0 2h-.5a.5.5 0 0 0-.5.5V11a9 9 0 0 1-2.489 6.213c1.76.778 2.018 3.17.464 4.305s-3.755.163-3.961-1.751A3 3 0 0 1 14 19.5v-.015a9 9 0 0 1-6 0v.015c0 1.925-2.084 3.127-3.75 2.164c-1.667-.962-1.666-3.368 0-4.33q.117-.067.239-.121C.063 12.574 1.769 4.927 7.746 2.609\" class=\"duoicon-secondary-layer\" opacity=\".55\"/><path fill=\"currentColor\" fill-rule=\"evenodd\" d=\"M8.012 4.669A7 7 0 0 0 4.072 10h6.372L8.012 4.67z\" class=\"duoicon-primary-layer\"/>",
                24,
                24,
            )),
            Icon::Paint => Some((
                "<path fill=\"currentColor\" d=\"M22 15v5a2 2 0 0 1-2 2h-1v-3a1 1 0 1 0-2 0v3h-4v-3a1 1 0 1 0-2 0v3H7v-3a1 1 0 1 0-2 0v3H4a2 2 0 0 1-2-2v-5z\" class=\"duoicon-primary-layer\"/><path fill=\"currentColor\" d=\"M13 2a2 2 0 0 1 2 2v4a1 1 0 0 0 1 1h4a2 2 0 0 1 2 2v2H2v-2a2 2 0 0 1 2-2h4a1 1 0 0 0 1-1V4a2 2 0 0 1 2-2z\" class=\"duoicon-secondary-layer\" opacity=\".55\"/>",
                24,
                24,
            )),
            Icon::Cake => Some((
                "<path fill=\"currentColor\" d=\"M17.707 15.707a.414.414 0 0 1 .586 0a2.41 2.41 0 0 0 2.707.491V20a1 1 0 1 1 0 2H3a1 1 0 1 1 0-2v-3.802c.89.405 1.975.241 2.707-.49a.414.414 0 0 1 .586 0a2.414 2.414 0 0 0 3.414 0a.414.414 0 0 1 .586 0a2.414 2.414 0 0 0 3.414 0a.414.414 0 0 1 .586 0a2.414 2.414 0 0 0 3.414 0zM16.5 2c-.319.638-.028 1.05.225 1.41c.144.203.275.39.275.59a1 1 0 1 1-2 0c0-.552.5-1.5 1.5-2m-8 0c-.319.638-.028 1.05.225 1.41c.144.203.275.39.275.59a1 1 0 1 1-2 0c0-.552.5-1.5 1.5-2m4 0c-.319.638-.028 1.05.225 1.41c.144.203.275.39.275.59a1 1 0 1 1-2 0c0-.552.5-1.5 1.5-2\" class=\"duoicon-primary-layer\"/><path fill=\"currentColor\" d=\"M16 6a1 1 0 0 1 1 1v2h1a3 3 0 0 1 3 3v1.586l-.707.707a.414.414 0 0 1-.586 0a2.414 2.414 0 0 0-3.414 0a.414.414 0 0 1-.586 0a2.414 2.414 0 0 0-3.414 0a.414.414 0 0 1-.586 0a2.414 2.414 0 0 0-3.414 0a.414.414 0 0 1-.586 0a2.414 2.414 0 0 0-3.414 0a.414.414 0 0 1-.586 0L3 13.586V12a3 3 0 0 1 3-3h1V7a1 1 0 1 1 2 0v2h2V7a1 1 0 1 1 2 0v2h2V7a1 1 0 0 1 1-1\" class=\"duoicon-secondary-layer\" opacity=\".55\"/>",
                24,
                24,
            )),
            _ => None,
        }
    }

    /// An SVG of the icon in `color`.
    pub fn svg(self, set: IconSet, color: Color) -> String {
        let (body, w, h) = self.body(set);
        let fill = hex(color);
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\">{}</svg>",
            body.replace("currentColor", &fill)
        )
    }

    /// A cached renderer handle for the icon in `set` and `color` — the
    /// same handle every frame, so the renderer keeps its rasterisation.
    pub fn handle(self, set: IconSet, color: Color) -> svg::Handle {
        static CACHE: OnceLock<Mutex<HashMap<(Icon, IconSet, [u8; 3]), svg::Handle>>> =
            OnceLock::new();
        let key = (self, set, rgb(color));
        let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
        let mut cache = cache.lock().expect("icon cache");
        cache
            .entry(key)
            .or_insert_with(|| svg::Handle::from_memory(self.svg(set, color).into_bytes()))
            .clone()
    }
}

fn rgb(c: Color) -> [u8; 3] {
    [
        (c.r * 255.0).round() as u8,
        (c.g * 255.0).round() as u8,
        (c.b * 255.0).round() as u8,
    ]
}

fn hex(c: Color) -> String {
    let [r, g, b] = rgb(c);
    format!("#{r:02x}{g:02x}{b:02x}")
}

/// The icon a tag wears: an explicit assignment (by full path, else by its
/// leaf), or the coffee cup for coffee words.
pub fn for_tag(tag: &str, assigned: &HashMap<String, Icon>) -> Option<Icon> {
    let leaf = tag.rsplit('/').next().unwrap_or(tag);
    assigned
        .get(tag)
        .or_else(|| assigned.get(leaf))
        .copied()
        .or_else(|| crate::coffee::is_coffee_tag(tag).then_some(Icon::Coffee))
}

/// Config form: `tag=key` per entry.
pub fn parse_assignments(entries: &[String]) -> HashMap<String, Icon> {
    entries
        .iter()
        .filter_map(|e| {
            let (tag, key) = e.split_once('=')?;
            Some((tag.to_owned(), Icon::from_key(key)?))
        })
        .collect()
}

pub fn serialise_assignments(map: &HashMap<String, Icon>) -> Vec<String> {
    let mut v: Vec<String> = map
        .iter()
        .map(|(t, i)| format!("{t}={}", i.key()))
        .collect();
    v.sort();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_icon_draws_in_every_set_with_a_unique_key() {
        let mut keys = std::collections::HashSet::new();
        for icon in Icon::ALL {
            for set in IconSet::ALL {
                assert!(!icon.body(set).0.is_empty(), "{icon:?} {set:?}");
            }
            assert!(keys.insert(icon.key()), "duplicate key {}", icon.key());
            assert_eq!(Icon::from_key(icon.key()), Some(icon));
        }
        assert!(
            Icon::Star
                .svg(IconSet::Iconoir, Color::WHITE)
                .contains("#ffffff")
        );
        assert_eq!(IconSet::from_key("iconoir"), IconSet::Iconoir);
        assert_eq!(IconSet::from_key("nope"), IconSet::Boxicons);
        assert_eq!(IconSet::from_key("solar"), IconSet::Solar);
        assert_eq!(IconSet::ALL.len(), 7);
    }

    #[test]
    fn assignments_round_trip_and_resolve() {
        let entries = vec!["travels=plane".to_owned(), "work/incab=bug".to_owned()];
        let map = parse_assignments(&entries);
        assert_eq!(for_tag("travels", &map), Some(Icon::Plane));
        assert_eq!(for_tag("travels/japan", &map), None);
        assert_eq!(for_tag("work/incab", &map), Some(Icon::Bug));
        assert_eq!(for_tag("espresso", &map), Some(Icon::Coffee));
        assert_eq!(serialise_assignments(&map), entries);
    }
}
