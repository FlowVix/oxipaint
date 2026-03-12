use std::cell::Cell;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use ahash::{AHashMap, AHashSet, AHasher};
use glam::{U8Vec4, u8vec4};
use godot::global::{godot_error, godot_print, godot_warn};
use godot::obj::Gd;
use indexmap::IndexMap;
use uuid::Uuid;
use walkdir::WalkDir;
use wasmtime::{Caller, Engine, Instance, Linker, Module, Store, TypedFunc};
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::p1::WasiP1Ctx;

use crate::DIRS;
use crate::app::AppBase;
use crate::utils::temp_id;

pub struct ExtExportedFuncs {
    pub __init: TypedFunc<(), ()>,
}
impl ExtExportedFuncs {
    pub fn get(instance: &Instance, store: &mut Store<StoreData>) -> Self {
        Self {
            __init: instance.get_typed_func(store, "__init").unwrap(),
        }
    }
}

macro_rules! ext_state {
    ($($field:ident : $typ:ty,)*) => {
        #[derive(Debug, Default)]
        pub struct ExtState {
            $(
                pub $field: $typ,
            )*
        }
        pub struct ExtStateRef<'a> {
            $(
                pub $field: &'a mut $typ,
            )*
        }
        impl<'a> ExtStateRef<'a> {
            pub fn deref(&self) -> ExtState {
                ExtState {
                    $(
                        $field: *self.$field,
                    )*
                }
            }
            pub fn set(&mut self, state: &ExtState) {
                $(
                    *self.$field = state.$field.clone();
                )*
            }
        }
    };
}
ext_state! {
    main_color: U8Vec4,
    secondary_color: U8Vec4,
    main_color_selected: bool,
}

pub struct ExtData {
    pub files: AHashMap<String, PathBuf>,
    pub module: Module,
    pub store: Store<StoreData>,
    pub instance: Instance,
    funcs: ExtExportedFuncs,
}
impl ExtData {
    /// doing this for safety so that we never forget to send and modify values
    pub fn manage_state_and_call(&mut self, mut state: ExtStateRef, cb: impl FnOnce(&ExtExportedFuncs, &mut Store<StoreData>)) {
        self.store.data_mut().state = state.deref();
        self.store.data_mut().registrations.clear();
        cb(&self.funcs, &mut self.store);
        state.set(&self.store.data().state);
    }
}

pub enum ExtRegister {
    Tool(String, String),
}
pub struct ExtManager {
    pub engine: Engine,
    pub extensions: IndexMap<String, ExtData>,
}
pub struct StoreData {
    pub ext_name: String,
    pub wasi: WasiP1Ctx,
    pub state: ExtState,
    pub registrations: AHashMap<u32, ExtRegister>,
}

fn build_linker(engine: &Engine) -> Linker<StoreData> {
    let mut linker = Linker::new(engine);
    wasmtime_wasi::p1::add_to_linker_sync(&mut linker, |s: &mut StoreData| &mut s.wasi).unwrap();

    linker
        .func_wrap("ext", "__print", |mut caller: Caller<'_, StoreData>, ptr: u32, len: u32| {
            let memory = caller.get_export("memory").and_then(|e| e.into_memory()).expect("guest exported memory");

            godot_print!("[{}] {}", &caller.data().ext_name, str::from_utf8(&memory.data(&caller)[ptr as usize..(ptr + len) as usize]).unwrap());
        })
        .unwrap();
    linker
        .func_wrap("ext", "__warn", |mut caller: Caller<'_, StoreData>, ptr: u32, len: u32| {
            let memory = caller.get_export("memory").and_then(|e| e.into_memory()).expect("guest exported memory");

            godot_warn!("[{}] {}", &caller.data().ext_name, str::from_utf8(&memory.data(&caller)[ptr as usize..(ptr + len) as usize]).unwrap());
        })
        .unwrap();
    linker
        .func_wrap("ext", "__error", |mut caller: Caller<'_, StoreData>, ptr: u32, len: u32| {
            let memory = caller.get_export("memory").and_then(|e| e.into_memory()).expect("guest exported memory");

            godot_error!("[{}] {}", &caller.data().ext_name, str::from_utf8(&memory.data(&caller)[ptr as usize..(ptr + len) as usize]).unwrap());
        })
        .unwrap();
    linker
        .func_wrap("ext", "__get_main_color", |caller: Caller<'_, StoreData>| -> u32 {
            u32::from_le_bytes(caller.data().state.main_color.to_array())
        })
        .unwrap();
    linker
        .func_wrap("ext", "__get_secondary_color", |caller: Caller<'_, StoreData>| -> u32 {
            u32::from_le_bytes(caller.data().state.secondary_color.to_array())
        })
        .unwrap();
    linker
        .func_wrap("ext", "__get_main_color_selected", |caller: Caller<'_, StoreData>| -> u32 {
            caller.data().state.main_color_selected as u32
        })
        .unwrap();
    linker
        .func_wrap("ext", "__set_main_color", |mut caller: Caller<'_, StoreData>, to: u32| {
            let [r, g, b, a] = to.to_le_bytes();
            caller.data_mut().state.main_color = u8vec4(r, g, b, a);
        })
        .unwrap();
    linker
        .func_wrap("ext", "__set_secondary_color", |mut caller: Caller<'_, StoreData>, to: u32| {
            let [r, g, b, a] = to.to_le_bytes();
            caller.data_mut().state.secondary_color = u8vec4(r, g, b, a);
        })
        .unwrap();
    linker
        .func_wrap("ext", "__set_main_color_selected", |mut caller: Caller<'_, StoreData>, to: u32| {
            caller.data_mut().state.main_color_selected = to > 0;
        })
        .unwrap();
    linker
        .func_wrap(
            "ext",
            "__register_tool",
            |mut caller: Caller<'_, StoreData>, name_ptr: u32, name_len: u32, icon_ptr: u32, icon_len: u32| -> u32 {
                let memory = caller.get_export("memory").and_then(|e| e.into_memory()).expect("guest exported memory");

                let name = str::from_utf8(&memory.data(&caller)[name_ptr as usize..(name_ptr + name_len) as usize]).unwrap().to_string();
                let icon = str::from_utf8(&memory.data(&caller)[icon_ptr as usize..(icon_ptr + icon_len) as usize]).unwrap().to_string();

                let id = temp_id();

                caller.data_mut().registrations.insert(id, ExtRegister::Tool(name, icon));

                id
            },
        )
        .unwrap();

    linker
}

fn build_extension(name: &str, path: &Path, engine: &Engine, linker: &mut Linker<StoreData>) -> ExtData {
    godot_print!("Loading extension [{}]", name);

    let mut files = AHashMap::new();
    for entry in WalkDir::new(path).min_depth(1).max_depth(10).follow_links(false).into_iter().filter_map(|v| v.ok()) {
        let p = entry.path();
        if p.is_dir() {
            continue;
        }
        files.insert(p.strip_prefix(path).unwrap().to_string_lossy().to_string(), p.to_path_buf());
    }

    let module = Module::from_file(engine, files.get("main.wasm").unwrap()).unwrap();

    let wasi = WasiCtx::builder().build_p1();
    let mut store = Store::new(
        engine,
        StoreData {
            ext_name: name.to_string(),
            wasi,
            state: ExtState::default(),
            registrations: AHashMap::new(),
            // state: state.clone(),
        },
    );
    let instance = linker.instantiate(&mut store, &module).unwrap();
    let funcs = ExtExportedFuncs::get(&instance, &mut store);

    ExtData {
        files,
        module,
        store,
        instance,
        funcs,
    }
}

impl ExtManager {
    pub fn init() -> Self {
        let engine = Engine::default();

        let mut linker = build_linker(&engine);

        let mut extensions = IndexMap::new();

        if let Ok(r) = fs::read_dir(DIRS.data_dir()) {
            for d in r {
                let Ok(d) = d else { continue };
                let path = d.path();
                if !path.is_dir() {
                    continue;
                }
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                let ext = build_extension(&name, &path, &engine, &mut linker);
                extensions.insert(name, ext);
            }
        }

        Self { engine, extensions }
    }
}
