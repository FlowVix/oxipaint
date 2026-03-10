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

use crate::app::State;
use crate::color_panel::slider::{ColorSliderType, color_slider};
use crate::popups::{MenuPopup, MenuPopupItem, menu_popup};
use crate::utils::{FrameSettings, GlamToGodot, control, frame, hbox, memo_res, panel, shortcut, vbox};

#[tree(Node)]
pub fn menubar(state: &mut State) {
    MenuBar..{
        INIT(theme_type_variation = "TopMenu", anchors_preset = LayoutPreset::CENTER, h_size_flags = SizeFlags::SHRINK_CENTER);
        menu_popup(
            "File",
            "file popup menu",
            &MenuPopup {
                items: Box::new(move || {
                    vec![
                        MenuPopupItem::Simple {
                            text: "New".into(),
                            icon: "file-plus.svg".into(),
                            key: Some(shortcut(Key::N, true, false, false)),
                            cb: |_| {},
                        },
                        MenuPopupItem::Simple {
                            text: "Open".into(),
                            icon: "folder-open.svg".into(),
                            key: Some(shortcut(Key::O, true, false, false)),
                            cb: |_| {
                                godot_print!("gaga");
                            },
                        },
                        MenuPopupItem::Nested {
                            text: "Open Recent".into(),
                            icon: "folder-clock.svg".into(),
                            menu: Box::new(MenuPopup {
                                items: Box::new(|| {
                                    vec![MenuPopupItem::Simple {
                                        text: "Foo".into(),
                                        icon: "file-plus.svg".into(),
                                        key: None,
                                        cb: |_| {},
                                    }]
                                }),
                            }),
                        },
                        MenuPopupItem::Separator,
                        MenuPopupItem::Simple {
                            text: "Save".into(),
                            icon: "save.svg".into(),
                            key: Some(shortcut(Key::S, true, false, false)),
                            cb: |_| {},
                        },
                        MenuPopupItem::Simple {
                            text: "Save As".into(),
                            icon: "blank.png".into(),
                            key: Some(shortcut(Key::S, true, true, false)),
                            cb: |_| {},
                        },
                        MenuPopupItem::Simple {
                            text: "Save All".into(),
                            icon: "save-all.svg".into(),
                            key: Some(shortcut(Key::N, true, false, true)),
                            cb: |_| {},
                        },
                    ]
                }),
            },
            state,
        )..{};
        menu_popup("Edit", "Edit popup menu", &MenuPopup { items: Box::new(|| vec![]) }, state)..{};
        menu_popup("View", "View popup menu", &MenuPopup { items: Box::new(|| vec![]) }, state)..{};
        menu_popup("Image", "Image popup menu", &MenuPopup { items: Box::new(|| vec![]) }, state)..{};
        menu_popup("Layers", "Layers popup menu", &MenuPopup { items: Box::new(|| vec![]) }, state)..{};
        menu_popup("Adjustments", "Adjustments popup menu", &MenuPopup { items: Box::new(|| vec![]) }, state)..{};
        menu_popup("Effects", "Effects popup menu", &MenuPopup { items: Box::new(|| vec![]) }, state)..{};
    };
}

#[tree(Node)]
pub fn button_bar(state: &mut State) {
    hbox(4, SizeFlags::SHRINK_CENTER, SizeFlags::SHRINK_CENTER)..{
        {
            #[tree(Node)]
            fn add_button(icon: &str, tooltip: &str, cb: impl FnOnce()) {
                Button..{
                    INIT(
                        icon = memo_res::<Texture2D>(&format!("res://resources/icons/{icon}")),
                        theme_type_variation = "BlankButton",
                        tooltip_text = tooltip,
                    );
                    ON(pressed = |_| {
                        cb();
                    });
                };
            }
        }
        add_button(
            "file-plus.svg",
            "New",
            Box::new(|| {
                godot_print!("bfjkvdhjfg");
            }),
        )..{};
        add_button(
            "folder-open.svg",
            "Open",
            Box::new(|| {
                godot_print!("bfjkvdhjfg");
            }),
        )..{};
        add_button(
            "save.svg",
            "Save",
            Box::new(|| {
                godot_print!("bfjkvdhjfg");
            }),
        )..{};
        VSeparator..{};
        add_button(
            "scissors.svg",
            "Cut",
            Box::new(|| {
                godot_print!("bfjkvdhjfg");
            }),
        )..{};
        add_button(
            "copy.svg",
            "Copy",
            Box::new(|| {
                godot_print!("bfjkvdhjfg");
            }),
        )..{};
        add_button(
            "clipboard-paste.svg",
            "Paste",
            Box::new(|| {
                godot_print!("bfjkvdhjfg");
            }),
        )..{};
        add_button(
            "crop.svg",
            "Crop",
            Box::new(|| {
                godot_print!("bfjkvdhjfg");
            }),
        )..{};
        VSeparator..{};
        add_button(
            "undo.svg",
            "Undo",
            Box::new(|| {
                godot_print!("bfjkvdhjfg");
            }),
        )..{};
        add_button(
            "redo.svg",
            "Redo",
            Box::new(|| {
                godot_print!("bfjkvdhjfg");
            }),
        )..{};
        VSeparator..{};
        add_button(
            "grid-3x3.svg",
            "Grid",
            Box::new(|| {
                godot_print!("bfjkvdhjfg");
            }),
        )..{};
        add_button(
            "ruler.svg",
            "Ruler",
            Box::new(|| {
                godot_print!("bfjkvdhjfg");
            }),
        )..{};
    };
}
