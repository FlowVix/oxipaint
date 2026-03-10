use std::cell::RefCell;

use echo::{App, Builder, tree};
use glam::{U8Vec4, Vec2, u8vec4, vec2};
use godot::classes::base_button::ActionMode;
use godot::classes::control::{LayoutPreset, MouseFilter, SizeFlags};
use godot::classes::scroll_container::ScrollMode;
use godot::classes::sub_viewport::UpdateMode;
use godot::classes::viewport::DefaultCanvasItemTextureFilter;
use godot::classes::{
    Button, Camera2D, ColorRect, Container, Control, HBoxContainer, HSeparator, Input, Label, LineEdit, MarginContainer, MenuBar, OptionButton, Panel, PanelContainer, PopupMenu, ScrollContainer,
    Shader, ShaderMaterial, Sprite2D, StyleBox, SubViewport, SubViewportContainer, Texture, Texture2D, TextureRect, Theme, VBoxContainer, VSeparator,
};
use godot::global::{HorizontalAlignment, Key};
use godot::prelude::*;
use uuid::Uuid;

use crate::color_panel::slider::{ColorSliderType, color_slider};
use crate::color_panel::{ColorMode, ColorState, color_picker_widget};
use crate::popups::{MenuPopup, MenuPopupItem, menu_popup};
use crate::top_panel::{button_bar, menubar};
use crate::utils::{FrameSettings, GlamToGodot, GodotToGlam, control, frame, hbox, memo_res, panel, shortcut, vbox};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AppBase {
    base: Base<Node>,

    #[init(val = OnReady::manual())]
    app: OnReady<App<AppBase, State>>,
    #[init(val = State::new())]
    state: State,
}

#[godot_api]
impl INode for AppBase {
    fn ready(&mut self) {
        let mut node = self.to_gd();
        self.app.init(App::new(node.clone(), |b, s| app(b, s, &mut |b| b)));
        node.clone().connect(
            "__echo_rerun",
            &Callable::from_fn("a", move |_| {
                let slef = &mut *node.bind_mut();
                slef.app.run(&mut slef.state);
            }),
        );
    }

    fn physics_process(&mut self, delta: f32) {}

    fn process(&mut self, delta: f32) {
        if Input::singleton().is_action_just_released("Left Mouse") {
            self.state.color.dragging_svl = false;
            self.state.color.dragging_h = false;
        }
        // if Input::singleton().is_action_just_released("Middle Mouse") {
        //     self.dragging_view = None;
        // }

        let mouse = self.base().get_viewport().unwrap().get_mouse_position();
        if self.state.color.dragging_svl {
            let pos = (mouse - self.state.color.bind_square_rect.position).clamp(Vector2::new(0.0, 0.0), self.state.color.bind_square_rect.size);
            self.state.color.svl_pin_pos = pos.to_glam();
            self.state.color.square_bar_changed();
        }
        if self.state.color.dragging_h {
            let pos = (mouse.y - self.state.color.bind_bar_rect.position.y).clamp(0.0, self.state.color.bind_bar_rect.size.y);
            self.state.color.h_pin_pos = pos;
            self.state.color.square_bar_changed();
        }
        // if let Some((prev_mouse, prev_camera)) = self.dragging_view {
        //     let zoom = self.camera.get_zoom().x;
        //     self.camera.set_position(prev_camera - (mouse - prev_mouse) / zoom);
        //     self.post_camera_change();
        // }

        self.app.run(&mut self.state);
    }
}

pub struct Project {
    pub id: Uuid,
}
pub struct State {
    color: ColorState,
}
impl State {
    pub fn new() -> Self {
        Self {
            color: ColorState {
                mode: ColorMode::Hsv,
                main_color: u8vec4(0, 0, 0, 255),
                secondary_color: u8vec4(255, 255, 255, 255),
                main_color_selected: true,
                r_slider: 0,
                g_slider: 0,
                b_slider: 0,
                a_slider: 0,
                h_slider: 0,
                s_slider: 0,
                vl_slider: 0,
                change_hex_edit: Some("000000".to_string()),
                dragging_svl: false,
                dragging_h: false,
                bind_square_rect: Rect2::default(),
                bind_bar_rect: Rect2::default(),
                svl_pin_pos: vec2(0.0, 0.0),
                h_pin_pos: 0.0,
                palette: include_str!("color_panel/aap-64.hex")
                    .lines()
                    .map(|v| Color::from_html(v.trim()).unwrap())
                    .map(|v| (Uuid::new_v4(), u8vec4(v.r8(), v.g8(), v.b8(), 255)))
                    .collect(),
            },
        }
    }
}

#[tree(AppBase)]
fn app(state: &mut State) {
    SubViewportContainer..{
        INIT(stretch = true, size = vec2(1280.0, 720.0).to_godot(), anchors_preset = LayoutPreset::FULL_RECT);
        SubViewport..{
            INIT(update_mode = UpdateMode::ALWAYS, canvas_item_default_texture_filter = DefaultCanvasItemTextureFilter::NEAREST);
            Camera2D..{};
            Sprite2D..{
                INIT(texture = load::<Texture2D>("res://icon.svg"));
            };
        };
    };

    HBoxContainer..{
        {
            if __builder.init() {
                state.color.update_color_info(true, true, true, true);
            }
        }
        INIT(
            size = vec2(1280.0, 720.0).to_godot(),
            anchors_preset = LayoutPreset::FULL_RECT,
            theme = load::<Theme>("res://resources/themes/GlobalTheme.tres"),
        );
        frame(FrameSettings {
            h: SizeFlags::EXPAND_FILL,
            v: SizeFlags::EXPAND_FILL,
            padding: [4; 4],
            arrange: Some((true, 4)),
            ..Default::default()
        })..{
            frame(FrameSettings {
                h: SizeFlags::EXPAND_FILL,
                padding: [4; 4],
                arrange: Some((false, 4)),
                panel: true,
                ..Default::default()
            })..{
                vbox(4, SizeFlags::SHRINK_BEGIN, SizeFlags::EXPAND_FILL)..{
                    menubar(state)..{};

                    button_bar(state)..{};
                };
            };
            hbox(4, SizeFlags::EXPAND_FILL, SizeFlags::EXPAND_FILL)..{
                vbox(4, SizeFlags::SHRINK_BEGIN, SizeFlags::EXPAND_FILL)..{
                    frame(FrameSettings {
                        panel: true,
                        padding: [4; 4],
                        ..Default::default()
                    })..{};
                    control(SizeFlags::SHRINK_BEGIN, SizeFlags::EXPAND_FILL)..{};
                    color_picker_widget(&mut state.color)..{};
                };
            };
        };
    };
}
