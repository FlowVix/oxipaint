use echo::tree;
use glam::{U8Vec4, ivec2, u8vec4, vec2};
use godot::classes::control::{LayoutPreset, SizeFlags};
use godot::classes::{BoxContainer, ColorRect, HBoxContainer, HSlider, Label, Node, Shader, ShaderMaterial, SpinBox, StyleBoxEmpty, StyleBoxTexture, SubViewport, Texture2D};
use godot::global::HorizontalAlignment;
use godot::obj::NewGd;
use godot::prelude::*;

use crate::color_panel::{ColorMode, ColorState};
use crate::utils::{GlamToGodot, hsl_to_rgb, hsv_to_rgb, margin, memo_res, okhsl_to_rgb, okhsv_to_rgb, rgb_to_hsl, rgb_to_hsv, rgb_to_okhsl, rgb_to_okhsv, spinbox};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorSliderType {
    R = 0,
    G,
    B,
    HsvH,
    HsvS,
    HsvV,
    HslH,
    HslS,
    HslL,
    OkHsvH,
    OkHsvS,
    OkHsvV,
    OkHslH,
    OkHslS,
    OkHslL,
    A,
}

#[tree(Node())]
pub fn color_slider(letter: &str, typ: ColorSliderType, max: u32, state: &mut ColorState) {
    HBoxContainer..{
        {
            let mut slider_stylebox = if __builder.init() {
                let mut out = StyleBoxTexture::new_gd();
                out.set_content_margin(Side::TOP, 16.0);
                out.set_local_to_scene(true);
                Some(out)
            } else {
                None
            };
        }
        Label..{
            INIT(horizontal_alignment = HorizontalAlignment::CENTER, custom_minimum_size = vec2(16.0, 0.0).to_godot());
            UPDATE(text = letter);
        };
        {
            let changed = |state: &mut ColorState, to: u32| {
                *state.get_value(typ) = to;
                if matches!(typ, ColorSliderType::R | ColorSliderType::G | ColorSliderType::B | ColorSliderType::A) {
                    *state.selected_color() = u8vec4(state.r_slider as u8, state.g_slider as u8, state.b_slider as u8, state.a_slider as u8);
                    state.update_color_info(false, true, true, true);
                } else {
                    let h = state.h_slider as f32 / 360.0;
                    let s = state.s_slider as f32 / 100.0;
                    let vl = state.vl_slider as f32 / 100.0;
                    let conv = match state.mode {
                        ColorMode::Hsv => hsv_to_rgb(h, s, vl),
                        ColorMode::Hsl => hsl_to_rgb(h, s, vl),
                        ColorMode::OkHsv => okhsv_to_rgb(h, s, vl),
                        ColorMode::OkHsl => okhsl_to_rgb(h, s, vl),
                    };

                    *state.selected_color() = u8vec4((conv.x * 255.0).round() as u8, (conv.y * 255.0).round() as u8, (conv.z * 255.0).round() as u8, state.a_slider as u8);
                    state.update_color_info(true, false, true, true);
                }
            };
        }
        margin(0, 1, 0, 0, SizeFlags::SHRINK_BEGIN, SizeFlags::EXPAND_FILL)..{
            HSlider..{
                INIT(
                    custom_minimum_size = vec2(100.0, 0.0).to_godot(),
                    size_flags_vertical = SizeFlags::SHRINK_CENTER,
                    theme(constant, center_grabber) = 1,
                    theme(constant, grabber_offset) = 12,
                    theme(icon, grabber) = &memo_res::<Texture2D>("res://resources/icons/color-slider-grabber.svg"),
                    theme(icon, grabber_highlight) = &memo_res::<Texture2D>("res://resources/icons/color-slider-grabber.svg"),
                    theme(stylebox, slider) = slider_stylebox.as_ref().unwrap(),
                    theme(stylebox, grabber_area) = &StyleBoxEmpty::new_gd(),
                    theme(stylebox, grabber_area_highlight) = &StyleBoxEmpty::new_gd(),
                    max_value = max,
                );
                {
                    // godot_print!("WTF {}", state.get_value(typ));
                    __builder.node().set_value_no_signal(*state.get_value(typ) as f64);
                }
                ON(value_changed = |args| {
                    let to = args[0].to::<f32>() as u32;
                    changed(state, to);
                });
            };
        };
        spinbox()..{
            INIT(max_value = max);
            {
                __builder.node().set_value_no_signal(*state.get_value(typ) as f64);
            }

            ON(value_changed = |args| {
                changed(state, args[0].to::<f32>() as u32);
            });
        };
        SubViewport..{
            INIT(size = ivec2(100, 16).to_godot());
            {
                if __builder.init() {
                    slider_stylebox.as_mut().unwrap().set_texture(&__builder.node().get_texture().unwrap());
                }
            }
            ColorRect..{
                INIT(
                    anchors_preset = LayoutPreset::FULL_RECT,
                    material = {
                        let mut mat = ShaderMaterial::new_gd();
                        mat.set_shader(&memo_res::<Shader>("res://resources/shaders/ColorSliderInput.gdshader"));
                        mat
                    },
                );
                {
                    __builder.node().set_instance_shader_parameter("type", &(typ as i32).to_variant());
                    __builder
                        .node()
                        .set_instance_shader_parameter("current", &(state.selected_color().as_vec4() / 255.0).to_godot().to_variant());
                }
            };
        };
    };
}
