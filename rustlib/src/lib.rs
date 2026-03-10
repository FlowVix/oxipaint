#![allow(clippy::too_many_arguments)]

mod app;
mod color_panel;
mod popups;
mod top_panel;
mod utils;

use godot::prelude::*;

struct AppExtension;
#[gdextension]
unsafe impl ExtensionLibrary for AppExtension {}

const APP_NAME: &str = "oxipaint";
