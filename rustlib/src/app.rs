use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;

use echo::{App, Builder, tree};
use glam::{U8Vec4, Vec2, ivec2, u8vec4, vec2};
use godot::classes::base_button::ActionMode;
use godot::classes::box_container::AlignmentMode;
use godot::classes::control::{LayoutPreset, MouseFilter, SizeFlags};
use godot::classes::scroll_container::ScrollMode;
use godot::classes::sub_viewport::UpdateMode;
use godot::classes::viewport::DefaultCanvasItemTextureFilter;
use godot::classes::window::WindowInitialPosition;
use godot::classes::{
    Button, Camera2D, ColorRect, Container, Control, HBoxContainer, HSeparator, Input, Label, LineEdit, MarginContainer, MenuBar, OptionButton, Panel, PanelContainer, PopupMenu, ScrollContainer,
    Shader, ShaderMaterial, SpinBox, Sprite2D, StyleBox, SubViewport, SubViewportContainer, Texture, Texture2D, TextureRect, Theme, VBoxContainer, VSeparator, Window,
};
use godot::global::{HorizontalAlignment, Key};
use godot::prelude::*;
use indexmap::IndexMap;
use parking_lot::Mutex;
use uuid::Uuid;

use crate::color_panel::slider::{ColorSliderType, color_slider};
use crate::color_panel::{ColorMode, ColorState, color_picker_widget};
use crate::extensions::{ExtManager, ExtState, ExtStateRef};
use crate::popups::{MenuPopup, MenuPopupItem, menu_popup};
use crate::project::{ProjectInfo, ProjectState};
use crate::top_panel::{button_bar, menubar};
use crate::utils::{FrameSettings, GlamToGodot, GodotToGlam, control, frame, hbox, memo_res, panel, shortcut, spinbox, subwindow, temp_id, vbox};
use crate::{CLIPBOARD, DIRS};

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
        self.app.init(App::new(node.clone(), |b, s| app(b.upcast(), s, &mut |b, ()| b).cast()));
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

pub struct State {
    pub ext_manager: ExtManager,
    pub color: ColorState,
    pub projects: ProjectState,
    pub new_file_window: OnReady<Gd<Window>>,
    pub new_file_width: u32,
    pub new_file_height: u32,
}
impl State {
    pub fn new() -> Self {
        _ = fs::create_dir_all(DIRS.data_dir());
        let mut out = Self {
            ext_manager: ExtManager::init(),
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
            projects: ProjectState {
                projects: IndexMap::new(),
                current_project: None,
            },
            new_file_window: OnReady::manual(),
            new_file_width: 800,
            new_file_height: 600,
        };
        for (_, data) in &mut out.ext_manager.extensions {
            data.manage_state_and_call(
                ExtStateRef {
                    main_color: &mut out.color.main_color,
                    secondary_color: &mut out.color.secondary_color,
                    main_color_selected: &mut out.color.main_color_selected,
                },
                |f, s| f.__init.call(s, ()).unwrap(),
            )
        }
        out
    }

    pub fn open_new_file_dialog(&mut self) {
        if let Ok(img) = CLIPBOARD.lock().get_image() {
            self.new_file_width = img.width as u32;
            self.new_file_height = img.height as u32;
        } else {
            self.new_file_width = 800;
            self.new_file_height = 600;
        }
        self.new_file_window.show();
    }
}

#[tree(Node())]
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
                INIT(custom_minimum_size = vec2(0.0, 60.0).to_godot());
                vbox(4, SizeFlags::SHRINK_CENTER, SizeFlags::EXPAND_FILL)..{
                    menubar(state)..{};
                    button_bar(state)..{};
                };
                for (id, proj) in &state.projects.projects {
                    KEY(id);
                    Button..{
                        INIT(custom_minimum_size = vec2(60.0, 60.0).to_godot(), action_mode = ActionMode::PRESS);
                        UPDATE(
                            theme_type_variation = if state.projects.current_project == Some(*id) {
                                "ProjPreviewButton"
                            } else {
                                "ProjPreviewButtonFaded"
                            },
                        );
                        ON(pressed = |_| {
                            state.projects.current_project = Some(*id);
                        });
                    };
                }
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

    subwindow(ivec2(200, 150), "New Project", &mut state.new_file_window)..{
        let (mut window) = ARGS;
        vbox(4, SizeFlags::EXPAND_FILL, SizeFlags::EXPAND_FILL)..{
            INIT(alignment = AlignmentMode::CENTER);
            for (label, value) in [("Width:", &mut state.new_file_width), ("Height:", &mut state.new_file_height)] {
                KEY(label);
                hbox(4, SizeFlags::EXPAND_FILL, SizeFlags::SHRINK_BEGIN)..{
                    INIT(alignment = AlignmentMode::CENTER);
                    Label..{
                        INIT(text = label, custom_minimum_size = vec2(60.0, 0.0).to_godot(), horizontal_alignment = HorizontalAlignment::RIGHT);
                    };
                    spinbox()..{
                        INIT(custom_minimum_size = vec2(100.0, 0.0).to_godot(), suffix = "px", max_value = 16384, min_value = 1);
                        {
                            __builder.node().set_value_no_signal(*value as f64);
                        }
                        ON(value_changed = |args| {
                            *value = args[0].to::<f32>() as u32;
                        });
                    };
                };
            }
            Button..{
                INIT(text = "Create", size_flags_horizontal = SizeFlags::SHRINK_CENTER);
                ON(pressed = |_| {
                    let id = temp_id();
                    state.projects.projects.insert(id, ProjectInfo {});
                    state.projects.current_project = Some(id);
                    window.hide();
                });
            };
        };
    };
}
