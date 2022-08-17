use std::io::Cursor;

use skyline::hooks::Region;

static VERSION: &str = concat!("Mod Ardain v. ", env!("CARGO_PKG_VERSION"), " / XC3 v.\0");

#[skyline::main(name = "mod_ardain_xc3")]
pub fn main() {
    println!("[Mod-Ardain] Loading...");

    // offsets.bin is populated by build.rs
    let config: bool /*FfiConfig*/ = {
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

    println!("[Mod-Ardain] Loaded config: {:#?}", config);

    let text_ptr = unsafe { skyline::hooks::getRegionAddress(Region::Text) } as *const u8;

    println!("[Mod-Ardain] Installing hooks");
    unsafe {
        //ffi::hooks::install_all(STATE.get().unwrap(), &config);
    }

    println!("[Mod-Ardain] Loaded!");
}
