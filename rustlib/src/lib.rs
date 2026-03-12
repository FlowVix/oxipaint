#![allow(clippy::too_many_arguments)]

mod app;
mod color_panel;
mod extensions;
mod popups;
mod project;
mod top_panel;
mod utils;

use std::sync::LazyLock;

use arboard::Clipboard;
use directories::ProjectDirs;
use godot::prelude::*;
use parking_lot::Mutex;

struct AppExtension;
#[gdextension]
unsafe impl ExtensionLibrary for AppExtension {}

const APP_NAME: &str = "oxipaint";
pub static DIRS: LazyLock<ProjectDirs> = LazyLock::new(|| ProjectDirs::from("", "", APP_NAME).unwrap());
pub static CLIPBOARD: LazyLock<Mutex<Clipboard>> = LazyLock::new(|| Mutex::new(Clipboard::new().unwrap()));
