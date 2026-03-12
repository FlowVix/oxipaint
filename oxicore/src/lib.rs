mod imports;

use std::sync::{LazyLock, Mutex};

use glam::{U8Vec4, u8vec4};

use crate::imports::{get_main_color, register_tool, set_main_color, set_main_color_selected};

#[unsafe(no_mangle)]
pub fn __init() {
    print!("Hello from wasm, main color is {:?}, but ill make it red", get_main_color());
    set_main_color(u8vec4(255, 0, 0, 255));
    register_tool("Foo", "icons/pencil.svg");
}
