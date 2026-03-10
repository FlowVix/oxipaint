use std::cell::RefCell;
use std::hash::Hash;
use std::io;
use std::mem::ManuallyDrop;
use std::path::PathBuf;

use ahash::AHashMap;
use echo::{Builder, tree};
use glam::*;
use godot::classes::control::SizeFlags;
use godot::classes::{Control, HBoxContainer, InputEventKey, MarginContainer, PanelContainer, Shortcut, VBoxContainer};
use godot::global::Key;
use godot::prelude::*;

pub struct MemoData {
    map: AHashMap<String, Variant>,
}

thread_local! {
    pub static MEMO_DATA: RefCell<ManuallyDrop<MemoData>> = RefCell::new(ManuallyDrop::new(MemoData { map: AHashMap::new() }));
}

pub fn memo_variant<T: ToGodot + FromGodot + Clone>(name: impl ToString, init: impl FnOnce() -> T) -> T {
    MEMO_DATA.with_borrow_mut(|v| T::from_variant(v.map.entry(name.to_string()).or_insert_with(|| init().to_variant())))
}
pub fn memo_res<T: Inherits<Resource>>(path: impl ToString) -> Gd<T> {
    let s = path.to_string();
    memo_variant(&s, || load::<T>(&s))
}

pub trait GlamToGodot {
    type T;

    fn to_godot(self) -> Self::T;
}
pub trait GodotToGlam {
    type T;

    fn to_glam(self) -> Self::T;
}
impl GlamToGodot for Vec2 {
    type T = Vector2;

    #[inline(always)]
    fn to_godot(self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}
impl GlamToGodot for Vec3 {
    type T = Vector3;

    #[inline(always)]
    fn to_godot(self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}
impl GlamToGodot for Vec4 {
    type T = Vector4;

    #[inline(always)]
    fn to_godot(self) -> Vector4 {
        Vector4::new(self.x, self.y, self.z, self.w)
    }
}
impl GlamToGodot for IVec2 {
    type T = Vector2i;

    #[inline(always)]
    fn to_godot(self) -> Vector2i {
        Vector2i::new(self.x, self.y)
    }
}
impl GlamToGodot for IVec3 {
    type T = Vector3i;

    #[inline(always)]
    fn to_godot(self) -> Vector3i {
        Vector3i::new(self.x, self.y, self.z)
    }
}
impl GlamToGodot for IVec4 {
    type T = Vector4i;

    #[inline(always)]
    fn to_godot(self) -> Vector4i {
        Vector4i::new(self.x, self.y, self.z, self.w)
    }
}
impl GodotToGlam for Vector2 {
    type T = Vec2;

    #[inline(always)]
    fn to_glam(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}
impl GodotToGlam for Vector3 {
    type T = Vec3;

    #[inline(always)]
    fn to_glam(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}
impl GodotToGlam for Vector4 {
    type T = Vec4;

    #[inline(always)]
    fn to_glam(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.w)
    }
}
impl GodotToGlam for Vector2i {
    type T = IVec2;

    #[inline(always)]
    fn to_glam(self) -> IVec2 {
        IVec2::new(self.x, self.y)
    }
}
impl GodotToGlam for Vector3i {
    type T = IVec3;

    #[inline(always)]
    fn to_glam(self) -> IVec3 {
        IVec3::new(self.x, self.y, self.z)
    }
}
impl GodotToGlam for Vector4i {
    type T = IVec4;

    #[inline(always)]
    fn to_glam(self) -> IVec4 {
        IVec4::new(self.x, self.y, self.z, self.w)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FrameSettings {
    pub margins: [i32; 4],
    pub padding: [i32; 4],
    pub arrange: Option<(bool, i32)>,
    pub panel: bool,
    pub h: SizeFlags,
    pub v: SizeFlags,
}
impl FrameSettings {
    pub fn new() -> Self {
        Self {
            margins: [0; 4],
            padding: [0; 4],
            arrange: None,
            panel: false,
            h: SizeFlags::SHRINK_BEGIN,
            v: SizeFlags::SHRINK_BEGIN,
        }
    }
}
impl Default for FrameSettings {
    fn default() -> Self {
        Self::new()
    }
}

#[tree(Node)]
pub fn margin(l: i32, u: i32, r: i32, d: i32, h: SizeFlags, v: SizeFlags) {
    MarginContainer..{
        INIT(
            theme(constant, margin_left) = l,
            theme(constant, margin_top) = u,
            theme(constant, margin_right) = r,
            theme(constant, margin_bottom) = d,
            size_flags_horizontal = h,
            size_flags_vertical = v,
        );
        BODY;
    };
}
#[tree(Node)]
pub fn hbox(sep: i32, h: SizeFlags, v: SizeFlags) {
    HBoxContainer..{
        INIT(theme(constant, separation) = sep, size_flags_horizontal = h, size_flags_vertical = v);
        BODY;
    };
}
#[tree(Node)]
pub fn vbox(sep: i32, h: SizeFlags, v: SizeFlags) {
    VBoxContainer..{
        INIT(theme(constant, separation) = sep, size_flags_horizontal = h, size_flags_vertical = v);
        BODY;
    };
}
#[tree(Node)]
pub fn panel(theme_override: &str, h: SizeFlags, v: SizeFlags) {
    PanelContainer..{
        INIT(theme_type_variation = theme_override, size_flags_horizontal = h, size_flags_vertical = v);
        BODY;
    };
}
#[tree(Node)]
pub fn control(h: SizeFlags, v: SizeFlags) {
    Control..{
        INIT(size_flags_horizontal = h, size_flags_vertical = v);
        BODY;
    };
}
#[tree(Node)]
pub fn frame(settings: FrameSettings) {
    margin(settings.margins[0], settings.margins[1], settings.margins[2], settings.margins[3], settings.h, settings.v)..{
        panel(if settings.panel { "ViewPanel" } else { "EmptyPanel" }, SizeFlags::EXPAND_FILL, SizeFlags::EXPAND_FILL)..{
            margin(
                settings.padding[0],
                settings.padding[1],
                settings.padding[2],
                settings.padding[3],
                SizeFlags::EXPAND_FILL,
                SizeFlags::EXPAND_FILL,
            )..{
                if let Some((v, sep)) = settings.arrange {
                    if v {
                        vbox(sep, SizeFlags::EXPAND_FILL, SizeFlags::EXPAND_FILL)..{
                            BODY;
                        };
                    } else {
                        hbox(sep, SizeFlags::EXPAND_FILL, SizeFlags::EXPAND_FILL)..{
                            BODY;
                        };
                    }
                } else {
                    margin(0, 0, 0, 0, SizeFlags::EXPAND_FILL, SizeFlags::EXPAND_FILL)..{
                        BODY;
                    };
                }
            };
        };
    };
}

pub fn shortcut(key: Key, ctrl: bool, shift: bool, alt: bool) -> Gd<Shortcut> {
    let mut key_event = InputEventKey::new_gd();
    key_event.set_keycode(key);
    key_event.set_ctrl_pressed(ctrl);
    key_event.set_shift_pressed(shift);
    key_event.set_alt_pressed(alt);
    key_event.set_command_or_control_autoremap(true);
    let mut shortcut = Shortcut::new_gd();
    shortcut.set_events(&varray![&key_event]);
    shortcut
}

use palette::{FromColor, Hsl, Hsv, Okhsl, Okhsv, Srgb};

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Vec3 {
    let hsv = Hsv::new_srgb(h * 360.0, s, v);
    let rgb = Srgb::from_color(hsv.into_format());
    vec3(rgb.red, rgb.green, rgb.blue)
}
pub fn rgb_to_hsv(r: f32, g: f32, b: f32) -> Vec3 {
    let rgb = Srgb::new(r, g, b);
    let hsv = Hsv::from_color(rgb.into_format());
    vec3(hsv.hue.into_positive_degrees() / 360.0, hsv.saturation, hsv.value)
}

pub fn hsl_to_rgb(h: f32, s: f32, v: f32) -> Vec3 {
    let hsv = Hsl::new_srgb(h * 360.0, s, v);
    let rgb = Srgb::from_color(hsv.into_format());
    vec3(rgb.red, rgb.green, rgb.blue)
}
pub fn rgb_to_hsl(r: f32, g: f32, b: f32) -> Vec3 {
    let rgb = Srgb::new(r, g, b);
    let hsv = Hsl::from_color(rgb.into_format());
    vec3(hsv.hue.into_positive_degrees() / 360.0, hsv.saturation, hsv.lightness)
}

pub fn okhsv_to_rgb(h: f32, s: f32, v: f32) -> Vec3 {
    let hsv = Okhsv::new(h * 360.0, s, v);
    let rgb = Srgb::from_color(hsv.into_format());
    vec3(rgb.red, rgb.green, rgb.blue)
}
pub fn rgb_to_okhsv(r: f32, g: f32, b: f32) -> Vec3 {
    let rgb = Srgb::new(r, g, b);
    let hsv = Okhsv::from_color(rgb.into_format());
    vec3(hsv.hue.into_positive_degrees() / 360.0, hsv.saturation, hsv.value)
}

pub fn okhsl_to_rgb(h: f32, s: f32, v: f32) -> Vec3 {
    let hsv = Okhsl::new(h * 360.0, s, v);
    let rgb = Srgb::from_color(hsv.into_format());
    vec3(rgb.red, rgb.green, rgb.blue)
}
pub fn rgb_to_okhsl(r: f32, g: f32, b: f32) -> Vec3 {
    let rgb = Srgb::new(r, g, b);
    let hsv = Okhsl::from_color(rgb.into_format());
    vec3(hsv.hue.into_positive_degrees() / 360.0, hsv.saturation, hsv.lightness)
}
