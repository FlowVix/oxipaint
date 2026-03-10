use glam::{U8Vec4, u8vec4};

#[link(wasm_import_module = "ext")]
unsafe extern "C" {
    pub fn __print(ptr: *const u8, len: usize);
    pub fn __warn(ptr: *const u8, len: usize);
    pub fn __error(ptr: *const u8, len: usize);
    fn __get_main_color() -> u32;
    fn __get_secondary_color() -> u32;
    fn __get_main_color_selected() -> u32;
    fn __set_main_color(to: u32);
    fn __set_secondary_color(to: u32);
    fn __set_main_color_selected(to: u32);
}
#[macro_export]
macro_rules! print {
    ($($t:tt)*) => {
        {
            let s = &format!($($t)*);
            unsafe { $crate::imports::__print(s.as_ptr(), s.len()) };
        }
    };
}
#[macro_export]
macro_rules! warn {
    ($($t:tt)*) => {
        {
            let s = &format!($($t)*);
            unsafe { $crate::imports::__warn(s.as_ptr(), s.len()) };
        }
    };
}
#[macro_export]
macro_rules! error {
    ($($t:tt)*) => {
        {
            let s = &format!($($t)*);
            unsafe { $crate::imports::__error(s.as_ptr(), s.len()) };
        }
    };
}

pub fn get_main_color() -> U8Vec4 {
    let [r, g, b, a] = unsafe { __get_main_color() }.to_le_bytes();
    u8vec4(r, g, b, a)
}
pub fn get_secondary_color() -> U8Vec4 {
    let [r, g, b, a] = unsafe { __get_secondary_color() }.to_le_bytes();
    u8vec4(r, g, b, a)
}
pub fn get_main_color_selected() -> bool {
    unsafe { __get_main_color_selected() > 0 }
}

pub fn set_main_color(to: U8Vec4) {
    unsafe { __set_main_color(u32::from_le_bytes(to.to_array())) };
}
pub fn set_secondary_color(to: U8Vec4) {
    unsafe { __set_secondary_color(u32::from_le_bytes(to.to_array())) };
}
pub fn set_main_color_selected(to: bool) {
    unsafe {
        __set_main_color_selected(to as u32);
    }
}
