// SPDX-License-Identifier: GPL-3.0-only

//! Folder icons a tag can wear instead of its `#`, in two styles the user
//! picks between: Boxicons Solid (MIT, github.com/box-icons/boxicons) and
//! Iconoir outline (MIT, github.com/iconoir-icons/iconoir). Bundled as SVG
//! path data from Iconify; drawn in the theme's tag colour in the sidebar,
//! the picker and over the tag's hash in a rendered note.

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
}

impl IconSet {
    pub const ALL: [IconSet; 2] = [IconSet::Boxicons, IconSet::Iconoir];

    pub fn key(self) -> &'static str {
        match self {
            IconSet::Boxicons => "boxicons",
            IconSet::Iconoir => "iconoir",
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
        }
    }

    pub fn blurb(self) -> &'static str {
        match self {
            IconSet::Boxicons => "solid shapes, bold at small sizes",
            IconSet::Iconoir => "thin outlines, lighter on the eye",
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

    /// The SVG body (24-grid, `currentColor`) in `set`; an icon the set
    /// lacks falls back to the other style rather than vanishing.
    fn body(self, set: IconSet) -> (&'static str, u32, u32) {
        match set {
            IconSet::Boxicons => self.boxicons(),
            IconSet::Iconoir => self.iconoir().unwrap_or_else(|| self.boxicons()),
        }
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
    fn every_icon_draws_in_both_sets_with_a_unique_key() {
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
