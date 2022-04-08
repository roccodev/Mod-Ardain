#![feature(proc_macro_hygiene)]
#![feature(once_cell)]

use std::{
    io::Cursor,
    lazy::SyncOnceCell,
    sync::atomic::{AtomicBool, AtomicU32},
};

use skyline::hooks::Region;

use crate::{ffi::FfiConfig, ui::text::TextRenderer};

mod config;
pub(crate) mod ffi;
pub mod input;
pub mod ui;

static STATE: SyncOnceCell<PlatformData> = SyncOnceCell::new();

#[derive(Debug)]
pub(crate) struct PlatformData {
    pub text_renderer: TextRenderer,
    pub text_ptr: StaticPtr,
    pub ui_visible: AtomicBool,
    pub no_input_frames: AtomicU32,
    pub ffi_offsets: ffi::hooks::Offsets,
}

/// A pointer to read-only memory.
#[derive(Clone, Copy, Debug)]
struct StaticPtr(*const u8);

// These are immutable pointers to a section of the program's code.
// They are thread-safe, and they're also already unsafe to dereference.
unsafe impl Send for StaticPtr {}
unsafe impl Sync for StaticPtr {}

impl StaticPtr {
    /// Copies a pointer and marks it as Send + Sync.
    /// While there are no unsafe operations, this function is unsafe
    /// as a warning.
    ///
    /// # Safety
    /// This function (and the resulting pointer) is safe if `ptr` points to
    /// valid, read-only memory.
    pub unsafe fn copy_of<T>(ptr: *const T) -> Self {
        Self(ptr.clone().cast())
    }

    pub fn offset<T>(&self, addr: isize) -> *const T {
        // Safety: behaviour is defined if the resulting pointer is in the
        // section. The pointer is unsafe to use regardless.
        unsafe { self.0.offset(addr).cast() }
    }

    pub fn inner<T>(&self) -> *const T {
        self.offset(0)
    }
}

#[skyline::main(name = "xc2_mod_menu")]
pub fn main() {
    println!("[XC2MM] Loading...");

    // offsets.bin is populated by build.rs
    let config: FfiConfig = {
        let reader = include_bytes!(concat!(env!("OUT_DIR"), "/offsets.bin"));
        let reader = Cursor::new(reader);
        match ciborium::de::from_reader(reader) {
            Ok(cfg) => cfg,
            Err(e) => {
                println!("Couldn't parse offset config: {:?}", e);
                return;
            }
        }
    };

    println!("[XC2MM] Loaded config: {:#?}", config);

    let text_renderer = TextRenderer::new(&config);
    let text_ptr = unsafe { skyline::hooks::getRegionAddress(Region::Text) } as *const u8;

    let state = PlatformData {
        text_ptr: StaticPtr(text_ptr),
        text_renderer,
        ui_visible: AtomicBool::new(false),
        no_input_frames: AtomicU32::new(0),
        ffi_offsets: ffi::hooks::Offsets::read_all(&config),
    };
    STATE.set(state).unwrap();

    println!("[XC2MM] Installing hooks");
    unsafe {
        ffi::hooks::install_all(&STATE.get().unwrap(), &config);
    }

    println!("[XC2MM] Loaded!");
}

pub(crate) fn get_platform_data() -> &'static PlatformData {
    STATE.get().expect("not yet initialized")
}
