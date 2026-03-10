use std::cell::RefCell;
use std::mem;

use echo::{App, Builder, tree};
use glam::{U8Vec4, Vec2, u8vec4, vec2};
use godot::classes::base_button::ActionMode;
use godot::classes::control::{LayoutPreset, MouseFilter, SizeFlags};
use godot::classes::scroll_container::ScrollMode;
use godot::classes::sub_viewport::UpdateMode;
use godot::classes::viewport::DefaultCanvasItemTextureFilter;
use godot::classes::{
    Button, Camera2D, ColorRect, Container, Control, GridContainer, HBoxContainer, HSeparator, Input, InputEvent, InputEventKey, InputEventMouseButton, Label, LineEdit, MarginContainer, MenuBar,
    OptionButton, Panel, PanelContainer, PopupMenu, ScrollContainer, Shader, ShaderMaterial, Sprite2D, StyleBox, SubViewport, SubViewportContainer, Texture, Texture2D, TextureRect, Theme,
    VBoxContainer, VSeparator,
};
use godot::global::{HorizontalAlignment, Key, MouseButton};
use godot::prelude::*;
use image::{GenericImageView, ImageBuffer, ImageReader, Rgba};
use indexmap::IndexSet;
use rfd::FileDialog;
use uuid::Uuid;

use crate::color_panel::slider::{ColorSliderType, color_slider};
use crate::popups::{MenuPopup, MenuPopupItem, menu_popup};
use crate::utils::{
    FrameSettings, GlamToGodot, control, frame, hbox, hsl_to_rgb, hsv_to_rgb, memo_res, okhsl_to_rgb, okhsv_to_rgb, panel, rgb_to_hsl, rgb_to_hsv, rgb_to_okhsl, rgb_to_okhsv, shortcut, vbox,
};

pub mod slider;

fn palette_image_dialog() -> FileDialog {
    FileDialog::new()
        .add_filter("bmp", &["bmp"])
        .add_filter("dds", &["dds"])
        .add_filter("exr", &["exr"])
        .add_filter("ff", &["ff"])
        .add_filter("gif", &["gif"])
        .add_filter("hdr", &["hdr"])
        .add_filter("ico", &["ico"])
        .add_filter("jpeg", &["jpeg"])
        .add_filter("png", &["png"])
        .add_filter("pnm", &["pnm"])
        .add_filter("qoi", &["qoi"])
        .add_filter("tga", &["tga"])
        .add_filter("tiff", &["tiff"])
        .add_filter("webp", &["webp"])
        .set_can_create_directories(true)
        .set_directory("/")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorMode {
    Hsv = 0,
    Hsl,
    OkHsv,
    OkHsl,
}

pub struct ColorState {
    pub mode: ColorMode,
    pub main_color: U8Vec4,
    pub secondary_color: U8Vec4,
    pub main_color_selected: bool,

    pub r_slider: u32,
    pub g_slider: u32,
    pub b_slider: u32,
    pub a_slider: u32,
    pub h_slider: u32,
    pub s_slider: u32,
    pub vl_slider: u32,
    pub change_hex_edit: Option<String>,

    pub svl_pin_pos: Vec2,
    pub h_pin_pos: f32,

    pub dragging_svl: bool,
    pub dragging_h: bool,
    pub bind_square_rect: Rect2,
    pub bind_bar_rect: Rect2,

    pub palette: Vec<(Uuid, U8Vec4)>,
}
impl ColorState {
    pub fn selected_color(&mut self) -> &mut U8Vec4 {
        if self.main_color_selected { &mut self.main_color } else { &mut self.secondary_color }
    }

    pub fn update_color_info(&mut self, update_rgba: bool, update_hsvl: bool, update_hex: bool, update_square_bar: bool) {
        let color = *self.selected_color();
        if update_rgba {
            self.r_slider = color.x as u32;
            self.g_slider = color.y as u32;
            self.b_slider = color.z as u32;
            self.a_slider = color.w as u32;
        }
        if update_hsvl {
            let conv = match self.mode {
                ColorMode::Hsv => rgb_to_hsv(color.x as f32 / 255.0, color.y as f32 / 255.0, color.z as f32 / 255.0),
                ColorMode::Hsl => rgb_to_hsl(color.x as f32 / 255.0, color.y as f32 / 255.0, color.z as f32 / 255.0),
                ColorMode::OkHsv => rgb_to_okhsv(color.x as f32 / 255.0, color.y as f32 / 255.0, color.z as f32 / 255.0),
                ColorMode::OkHsl => rgb_to_okhsl(color.x as f32 / 255.0, color.y as f32 / 255.0, color.z as f32 / 255.0),
            };
            self.h_slider = (conv.x * 360.0).round() as u32;
            self.s_slider = (conv.y * 100.0).round() as u32;
            self.vl_slider = (conv.z * 100.0).round() as u32;
        }
        if update_hex {
            let color = Color::from_rgba8(color.x, color.y, color.z, color.w);
            self.change_hex_edit = Some(color.to_html_without_alpha().to_string().to_uppercase());
        }
        if update_square_bar {
            let conv = match self.mode {
                ColorMode::Hsv => rgb_to_hsv(color.x as f32 / 255.0, color.y as f32 / 255.0, color.z as f32 / 255.0),
                ColorMode::Hsl => rgb_to_hsl(color.x as f32 / 255.0, color.y as f32 / 255.0, color.z as f32 / 255.0),
                ColorMode::OkHsv => rgb_to_okhsv(color.x as f32 / 255.0, color.y as f32 / 255.0, color.z as f32 / 255.0),
                ColorMode::OkHsl => rgb_to_okhsl(color.x as f32 / 255.0, color.y as f32 / 255.0, color.z as f32 / 255.0),
            };
            self.svl_pin_pos = vec2((conv.y * 240.0).round(), (conv.z * 240.0).round());
            self.h_pin_pos = (conv.x * 240.0).round();
        }
    }

    pub fn square_bar_changed(&mut self) {
        let h = self.h_pin_pos / 240.0;
        let s = self.svl_pin_pos.x / 240.0;
        let vl = self.svl_pin_pos.y / 240.0;
        let conv = match self.mode {
            ColorMode::Hsv => hsv_to_rgb(h, s, vl),
            ColorMode::Hsl => hsl_to_rgb(h, s, vl),
            ColorMode::OkHsv => okhsv_to_rgb(h, s, vl),
            ColorMode::OkHsl => okhsl_to_rgb(h, s, vl),
        };
        *self.selected_color() = u8vec4((conv.x * 255.0).round() as u8, (conv.y * 255.0).round() as u8, (conv.z * 255.0).round() as u8, self.a_slider as u8);
        self.update_color_info(true, true, true, false);
    }

    pub fn get_value(&mut self, slider: ColorSliderType) -> &mut u32 {
        match slider {
            ColorSliderType::R => &mut self.r_slider,
            ColorSliderType::G => &mut self.g_slider,
            ColorSliderType::B => &mut self.b_slider,
            ColorSliderType::HsvH | ColorSliderType::HslH | ColorSliderType::OkHsvH | ColorSliderType::OkHslH => &mut self.h_slider,
            ColorSliderType::HsvS | ColorSliderType::HslS | ColorSliderType::OkHsvS | ColorSliderType::OkHslS => &mut self.s_slider,
            ColorSliderType::HsvV | ColorSliderType::HslL | ColorSliderType::OkHsvV | ColorSliderType::OkHslL => &mut self.vl_slider,
            ColorSliderType::A => &mut self.a_slider,
        }
    }
}

#[tree(Node)]
pub fn color_picker_widget(state: &mut ColorState) {
    frame(FrameSettings {
        panel: true,
        padding: [4; 4],
        arrange: Some((false, 4)),
        ..Default::default()
    })..{
        hbox(4, SizeFlags::EXPAND_FILL, SizeFlags::EXPAND_FILL)..{
            vbox(4, SizeFlags::EXPAND_FILL, SizeFlags::EXPAND_FILL)..{
                Control..{
                    INIT(custom_minimum_size = vec2(54.0, 54.0).to_godot(), z_index = 1);

                    {
                        #[tree(Node)]
                        fn color_button(pos: Vec2, color: U8Vec4, main: bool, state: &mut ColorState) {
                            Button..{
                                INIT(
                                    toggle_mode = true,
                                    action_mode = ActionMode::PRESS,
                                    custom_minimum_size = vec2(36.0, 36.0).to_godot(),
                                    position = pos.to_godot(),
                                    theme_type_variation = "ColorPreviewButton",
                                );
                                UPDATE(button_pressed = state.main_color_selected == main);
                                ON(pressed = |_| {
                                    state.main_color_selected = main;
                                    state.update_color_info(true, true, true, true);
                                });
                                TextureRect..{
                                    INIT(texture = memo_res::<Texture2D>("res://resources/textures/behind-color.png"), position = vec2(2.0, 2.0).to_godot());
                                };
                                ColorRect..{
                                    INIT(
                                        texture = memo_res::<Texture2D>("res://resources/textures/behind-color.png"),
                                        size = vec2(32.0, 32.0).to_godot(),
                                        position = vec2(2.0, 2.0).to_godot(),
                                        anchors_preset = LayoutPreset::FULL_RECT,
                                        mouse_filter = MouseFilter::IGNORE,
                                    );
                                    UPDATE(color = Color::from_rgba8(color.x, color.y, color.z, color.w));
                                };
                            };
                        }
                    }

                    color_button(vec2(18.0, 18.0), state.secondary_color, false, state)..{};
                    color_button(vec2(0.0, 0.0), state.main_color, true, state)..{};
                };
                Button..{
                    INIT(
                        icon = memo_res::<Texture2D>("res://resources/icons/arrow-left-right.svg"),
                        size_flags_horizontal = SizeFlags::SHRINK_CENTER,
                        theme_type_variation = "BlankButton",
                        tooltip_text = "Swap primary and secondary colors",
                    );
                    ON(pressed = |_| {
                        mem::swap(&mut state.main_color, &mut state.secondary_color);
                        state.update_color_info(true, true, true, true);
                    });
                };
                control(SizeFlags::SHRINK_BEGIN, SizeFlags::EXPAND_FILL)..{};
                Button..{
                    INIT(
                        icon = memo_res::<Texture2D>("res://resources/icons/plus.svg"),
                        size_flags_horizontal = SizeFlags::SHRINK_CENTER,
                        theme_type_variation = "BlankButton",
                        tooltip_text = "Add color to palette",
                    );
                    ON(pressed = |_| {
                        let col = *state.selected_color();
                        state.palette.push((Uuid::new_v4(), col));
                    });
                };
                Button..{
                    INIT(
                        icon = memo_res::<Texture2D>("res://resources/icons/folder-open.svg"),
                        size_flags_horizontal = SizeFlags::SHRINK_CENTER,
                        theme_type_variation = "BlankButton",
                        tooltip_text = "Load palette",
                    );
                    ON(pressed = |_| {
                        let Some(path) = palette_image_dialog().pick_file() else {
                            return;
                        };
                        let Ok(img) = ImageReader::open(path) else {
                            return;
                        };
                        let Ok(img) = img.with_guessed_format() else {
                            return;
                        };
                        let Ok(img) = img.decode() else {
                            return;
                        };
                        let mut colors = IndexSet::new();
                        for y in 0..img.height() {
                            for x in 0..img.width() {
                                colors.insert(img.get_pixel(x, y));
                            }
                        }
                        state.palette.clear();
                        for i in colors {
                            state.palette.push((Uuid::new_v4(), u8vec4(i.0[0], i.0[1], i.0[2], i.0[3])));
                        }
                    });
                };
                Button..{
                    INIT(
                        icon = memo_res::<Texture2D>("res://resources/icons/save.svg"),
                        size_flags_horizontal = SizeFlags::SHRINK_CENTER,
                        theme_type_variation = "BlankButton",
                        tooltip_text = "Save palette",
                    );
                    ON(pressed = |_| {
                        let mut colors = IndexSet::new();
                        for v in &state.palette {
                            colors.insert(v.1.to_array());
                        }
                        let img = ImageBuffer::from_fn(colors.len() as u32, 1, |x, _| Rgba(colors[x as usize]));
                        let Some(path) = palette_image_dialog().save_file() else {
                            return;
                        };
                        _ = img.save(path);
                    });
                };
            };
            vbox(4, SizeFlags::EXPAND_FILL, SizeFlags::EXPAND_FILL)..{
                hbox(4, SizeFlags::EXPAND_FILL, SizeFlags::EXPAND_FILL)..{
                    ColorRect..{
                        INIT(
                            custom_minimum_size = vec2(240.0, 240.0).to_godot(),
                            material = {
                                let mut mat = ShaderMaterial::new_gd();
                                mat.set_shader(&memo_res::<Shader>("res://resources/shaders/SVLSquare.gdshader"));
                                mat
                            },
                        );
                        Panel..{
                            INIT(
                                custom_minimum_size = vec2(240.0, 240.0).to_godot(),
                                mouse_filter = MouseFilter::IGNORE,
                                theme(stylebox, panel) = &memo_res::<StyleBox>("res://resources/misc/RoundingStyleBox.tres"),
                            );
                        };
                        {
                            __builder.node().set_instance_shader_parameter("mode", &(state.mode as i32).to_variant());
                            let color = state.selected_color();
                            __builder.node().set_instance_shader_parameter("current", &(color.as_vec4() / 255.0).to_godot().to_variant());
                            state.bind_square_rect = __builder.node().get_global_rect();
                        }
                        Sprite2D..{
                            INIT(texture = memo_res::<Texture2D>("res://resources/icons/svl-square-pin.svg"), z_index = 2);
                            UPDATE(position = state.svl_pin_pos.to_godot());
                            ColorRect..{
                                INIT(
                                    mouse_filter = MouseFilter::IGNORE,
                                    size = vec2(4.0, 4.0).to_godot(),
                                    position = vec2(-2.0, -2.0).to_godot(),
                                    show_behind_parent = true,
                                );
                                UPDATE(
                                    color = {
                                        let c = *state.selected_color();
                                        Color::from_rgba8(c.x, c.y, c.z, c.w)
                                    },
                                );
                            };
                        };
                        ON(gui_input = |args| {
                            match_class! { args[0].to::<Gd<InputEvent>>(),
                                event @ InputEventMouseButton => {
                                    if event.is_pressed() && event.get_button_index() == MouseButton::LEFT {
                                        state.dragging_svl = true;
                                    }
                                },
                                _ => {}
                            }
                        });
                    };
                    ColorRect..{
                        INIT(
                            custom_minimum_size = vec2(32.0, 240.0).to_godot(),
                            material = {
                                let mut mat = ShaderMaterial::new_gd();
                                mat.set_shader(&memo_res::<Shader>("res://resources/shaders/HBar.gdshader"));
                                mat
                            },
                        );
                        Panel..{
                            INIT(
                                custom_minimum_size = vec2(32.0, 240.0).to_godot(),
                                mouse_filter = MouseFilter::IGNORE,
                                theme(stylebox, panel) = &memo_res::<StyleBox>("res://resources/misc/RoundingStyleBox.tres"),
                            );
                        };
                        {
                            __builder.node().set_instance_shader_parameter("mode", &(state.mode as i32).to_variant());
                            let color = state.selected_color();
                            __builder.node().set_instance_shader_parameter("current", &(color.as_vec4() / 255.0).to_godot().to_variant());
                            state.bind_bar_rect = __builder.node().get_global_rect();
                        }
                        Sprite2D..{
                            INIT(texture = memo_res::<Texture2D>("res://resources/icons/h-bar-pin.svg"), z_index = 2);
                            UPDATE(position = vec2(16.0, state.h_pin_pos).to_godot());
                            ColorRect..{
                                INIT(
                                    mouse_filter = MouseFilter::IGNORE,
                                    size = vec2(34.0, 6.0).to_godot(),
                                    position = vec2(-17.0, -3.0).to_godot(),
                                    show_behind_parent = true,
                                );
                                UPDATE(
                                    color = {
                                        let c = *state.selected_color();
                                        Color::from_rgba8(c.x, c.y, c.z, c.w)
                                    },
                                );
                            };
                        };
                        ON(gui_input = |args| {
                            match_class! { args[0].to::<Gd<InputEvent>>(),
                                event @ InputEventMouseButton => {
                                    if event.is_pressed() && event.get_button_index() == MouseButton::LEFT {
                                        state.dragging_h = true;
                                    }
                                },
                                _ => {}
                            }
                        });
                    };
                };
                ScrollContainer..{
                    INIT(
                        follow_focus = true,
                        custom_minimum_size = vec2(276.0, 67.0).to_godot(),
                        theme(stylebox, panel) = &memo_res::<StyleBox>("res://resources/misc/ColorScrollPanelStyleBox.tres"),
                        vertical_scroll_mode = ScrollMode::SHOW_ALWAYS,
                    );
                    GridContainer..{
                        INIT(columns = 16, theme(constant, h_separation) = 1, theme(constant, v_separation) = 1);

                        for (idx, (id, color)) in state.palette.clone().into_iter().enumerate() {
                            KEY(id);

                            Button..{
                                INIT(
                                    custom_minimum_size = vec2(16.0, 16.0).to_godot(),
                                    theme_type_variation = "ColorPaletteButton",
                                    self_modulate = Color::from_rgba8(color.x, color.y, color.z, color.w),
                                    action_mode = ActionMode::PRESS,
                                );
                                TextureRect..{
                                    INIT(texture = memo_res::<Texture2D>("res://resources/textures/palette-transparent-bg.png"), show_behind_parent = true);
                                };
                                ON(
                                    pressed = |_| {
                                        *state.selected_color() = color;
                                        state.update_color_info(true, true, true, true);
                                    },
                                    gui_input = |args| {
                                        match_class! { args[0].to::<Gd<InputEvent>>(),
                                            event @ InputEventKey => {
                                                if event.is_pressed() {
                                                    if event.is_shift_pressed() {
                                                        if event.get_keycode() == Key::LEFT && idx > 0 {
                                                            state.palette.swap(idx, idx - 1);
                                                        } else if event.get_keycode() == Key::RIGHT && idx < state.palette.len() - 1 {
                                                            state.palette.swap(idx, idx + 1);
                                                        }
                                                    } else {
                                                        if event.get_keycode() == Key::DELETE {
                                                            state.palette.retain(|v| v.0 != id);
                                                        }
                                                    }
                                                }
                                            },
                                            _ => {}
                                        }
                                    },
                                );
                            };
                        }
                    };
                };
            };
        };
        VSeparator..{};
        vbox(4, SizeFlags::EXPAND_FILL, SizeFlags::EXPAND_FILL)..{
            hbox(4, SizeFlags::SHRINK_CENTER, SizeFlags::SHRINK_BEGIN)..{
                Label..{
                    INIT(text = "Mode:");
                };
                OptionButton..{
                    {
                        if __builder.init() {
                            let mut node = __builder.node();
                            node.add_item("HSV");
                            node.add_item("HSL");
                            node.add_item("OkHSV");
                            node.add_item("OkHSL");
                            for i in 0..node.get_item_count() {
                                node.get_popup().unwrap().set_item_as_radio_checkable(i, false);
                            }
                        }
                        __builder.node().select(state.mode as i32);
                    }
                    ON(item_selected = |args| {
                        state.mode = match args[0].to::<i32>() {
                            0 => ColorMode::Hsv,
                            1 => ColorMode::Hsl,
                            2 => ColorMode::OkHsv,
                            3 => ColorMode::OkHsl,
                            _ => unreachable!(),
                        };
                        state.update_color_info(false, true, false, true);
                    });
                };
            };
            hbox(4, SizeFlags::SHRINK_CENTER, SizeFlags::SHRINK_BEGIN)..{
                Label..{
                    INIT(text = "Hex:");
                };
                LineEdit..{
                    INIT(
                        alignment = HorizontalAlignment::CENTER,
                        max_length = 7,
                        context_menu_enabled = false,
                        emoji_menu_enabled = false,
                        middle_mouse_paste_enabled = false,
                        select_all_on_focus = true,
                        custom_minimum_size = vec2(72.0, 0.0).to_godot(),
                    );
                    {
                        if let Some(to) = state.change_hex_edit.take() {
                            __builder.node().set_text(&to);
                        }
                    }
                    ON(text_changed = |args| {
                        let to = args[0].to::<String>();
                        if let Some(v) = Color::from_html(&to) {
                            *state.selected_color() = u8vec4(v.r8(), v.g8(), v.b8(), v.a8());
                            state.update_color_info(true, true, false, true);
                        }
                    });
                };
            };
            HSeparator..{};
            color_slider("R", ColorSliderType::R, 255, state)..{};
            color_slider("G", ColorSliderType::G, 255, state)..{};
            color_slider("B", ColorSliderType::B, 255, state)..{};
            color_slider("A", ColorSliderType::A, 255, state)..{};
            HSeparator..{};
            color_slider(
                "H",
                match state.mode {
                    ColorMode::Hsv => ColorSliderType::HsvH,
                    ColorMode::Hsl => ColorSliderType::HslH,
                    ColorMode::OkHsv => ColorSliderType::OkHsvH,
                    ColorMode::OkHsl => ColorSliderType::OkHslH,
                },
                360,
                state,
            )..{};
            color_slider(
                "S",
                match state.mode {
                    ColorMode::Hsv => ColorSliderType::HsvS,
                    ColorMode::Hsl => ColorSliderType::HslS,
                    ColorMode::OkHsv => ColorSliderType::OkHsvS,
                    ColorMode::OkHsl => ColorSliderType::OkHslS,
                },
                100,
                state,
            )..{};
            color_slider(
                if matches!(state.mode, ColorMode::Hsv | ColorMode::OkHsv) { "V" } else { "L" },
                match state.mode {
                    ColorMode::Hsv => ColorSliderType::HsvV,
                    ColorMode::Hsl => ColorSliderType::HslL,
                    ColorMode::OkHsv => ColorSliderType::OkHsvV,
                    ColorMode::OkHsl => ColorSliderType::OkHslL,
                },
                100,
                state,
            )..{};
        };
    };
}
