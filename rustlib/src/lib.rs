#![allow(clippy::too_many_arguments)]

mod app;
mod color_panel;
mod extensions;
mod popups;
mod top_panel;
mod utils;

use std::sync::LazyLock;

use directories::ProjectDirs;
use godot::prelude::*;

struct AppExtension;
#[gdextension]
unsafe impl ExtensionLibrary for AppExtension {}

const APP_NAME: &str = "oxipaint";
pub static DIRS: LazyLock<ProjectDirs> = LazyLock::new(|| ProjectDirs::from("", "", APP_NAME).unwrap());
