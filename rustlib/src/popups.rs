use std::cell::LazyCell;
use std::hash::Hash;
use std::sync::LazyLock;

use echo::{Builder, tree};
use godot::builtin::varray;
use godot::classes::{InputEventKey, Node, PopupMenu, Shortcut, Texture2D};
use godot::global::{Key, KeyModifierMask, godot_print};
use godot::meta::ToGodot;
use godot::obj::{Gd, Inherits, NewAlloc, NewGd, OnReady};

use crate::app::State;
use crate::utils::memo_res;

pub struct MenuPopup {
    pub items: Box<dyn Fn() -> Vec<MenuPopupItem>>,
}

pub enum MenuPopupItem {
    Simple {
        text: String,
        icon: String,
        key: Option<Gd<Shortcut>>,
        cb: fn(&mut State),
    },
    Nested {
        text: String,
        icon: String,
        menu: Box<MenuPopup>,
    },
    Separator,
}

#[tree(Node())]
pub fn menu_popup(name: &str, id: &str, menu: &MenuPopup, state: &mut State) {
    PopupMenu..{
        INIT(name = name, hide_on_checkable_item_selection = false);
        {
            let mut items = (menu.items)();
            let mut node = __builder.node();
            let init = __builder.init();
        }
        for (idx, item) in items.iter().enumerate() {
            KEY(idx);

            if let MenuPopupItem::Simple { text, icon, key, .. } = item {
                if init {
                    {
                        node.add_icon_item(&memo_res::<Texture2D>(&format!("res://resources/icons/{icon}")), text);
                        if let Some(key) = key {
                            node.set_item_shortcut(idx as i32, key);
                        }
                    }
                }
            } else if let MenuPopupItem::Nested { text, icon, menu } = item {
                menu_popup(&format!("submenu{}", idx), &format!("submenu{}", id), menu, state)..{};
                if init {
                    {
                        node.add_submenu_item(text, &format!("submenu{}", idx));
                        node.set_item_icon(idx as i32, &memo_res::<Texture2D>(&format!("res://resources/icons/{icon}")));
                    }
                }
            } else {
                if init {
                    {
                        node.add_separator();
                    }
                }
            }
        }
        ON(index_pressed = |args| {
            let idx = args[0].to::<i32>() as usize;
            if let MenuPopupItem::Simple { cb, .. } = &mut items[idx] {
                cb(state);
            }
        });
    };
}
