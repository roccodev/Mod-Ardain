use std::{ffi::CString, lazy::SyncOnceCell, sync::atomic::Ordering};

use crate::{
    ffi::{FfiConfig, Offset, Register, RegisterValue},
    get_platform_data,
    input::{PadButton, PadData},
    ui::text::Text,
    PlatformData, StaticPtr,
};
use skyline::{hooks::InlineCtx, libc::c_void};

static KEY_ITEM_MAX_QTY_ORIG: SyncOnceCell<StaticPtr> = SyncOnceCell::new();

#[derive(Debug, Clone, Copy)]
pub(crate) struct Offsets {
    return_title: Option<Offset>,
    input_register: Register,
    bdat_item_id: Option<Register>,
    bdat_item_type: Option<Register>,
}

impl Offsets {
    pub fn read_all(config: &FfiConfig) -> Self {
        Self {
            return_title: config.get_function("return-title"),
            input_register: config
                .get_register("input-pad-data")
                .expect("input offset required"),
            bdat_item_id: config.get_register("bdat-item-cond-id"),
            bdat_item_type: config.get_register("bdat-item-cond-type"),
        }
    }
}

pub(crate) unsafe fn install_all(platform: &PlatformData, config: &FfiConfig) {
    if let Some(hook) = config.get_hook("input") {
        hook.patch_inline(platform, on_frame);
    }
    if let Some(hook) = config.get_hook("blade-create-save") {
        hook.patch(platform, blade_create_disable_save as *const c_void);
    }
    if let Some(hook) = config.get_hook("bdat-item-condition") {
        hook.patch_inline(platform, bdat_item_condition);
    }
    if let Some(hook) = config.get_hook("key-item-max-quantity") {
        KEY_ITEM_MAX_QTY_ORIG
            .set(StaticPtr::copy_of(
                hook.patch(platform, key_item_max_quantity as *const c_void),
            ))
            .unwrap();
    }
}

/// Hook into the checkEmergencyEscape function, just after the input data is loaded into a
/// register.
unsafe extern "C" fn on_frame(inline_ctx: &mut InlineCtx) {
    let platform = crate::get_platform_data();
    let inputs = PadData::from(platform.ffi_offsets.input_register.get(inline_ctx));

    let had_input = if platform.no_input_frames.fetch_add(1, Ordering::Relaxed) >= 10 {
        if inputs.contains(&(PadButton::L + PadButton::LeftStickClick)) {
            // Toggle UI visibility
            platform
                .ui_visible
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |b| Some(!b))
                .ok();
            true
        } else if inputs.contains(&(PadButton::L + PadButton::R + PadButton::A + PadButton::Plus)) {
            // Return to title
            match platform.ffi_offsets.return_title {
                Some(return_title) => {
                    // 0xff_ff_ff_ff is always used in the executable (it's the save
                    // slot)
                    std::mem::transmute::<_, extern "C" fn(u32)>(return_title.as_fn(&platform))(
                        0xff_ff_ff_ff,
                    );
                    true
                }
                None => false,
            }
        } else {
            false
        }
    } else {
        false
    };

    if had_input {
        platform.no_input_frames.store(0, Ordering::Relaxed);
    }

    // Render UI
    if platform.ui_visible.load(Ordering::Relaxed) {
        let text = CString::new("XC2 Mod Menu").unwrap();
        platform.text_renderer.draw_text(
            platform,
            0,
            0,
            Text::new(&text).color(0.0, 0.0, 1.0, 1.0),
        );
    }
}

unsafe extern "C" fn blade_create_disable_save(_save_slot: i64) -> i64 {
    1
}

/// Part 1 of allowing more than one "Flutterheart Grass" (the item needed to
/// craft the Love Source) to be obtained at once.
///
/// The game checks a list of conditions to spawn the grass "NPC" and allow
/// dialogue. One of these conditions is an Item condition with ID 300, which
/// makes sure that you have no Flutterheart Grass already in your inventory.
///
/// See: <https://xenoblade.github.io/xb2/bdat/common/FLD_ConditionList.html#2848>
unsafe extern "C" fn bdat_item_condition(inline_ctx: &mut InlineCtx) {
    let platform = get_platform_data();

    if let Some(id) = platform.ffi_offsets.bdat_item_id.map(|i| i.get(inline_ctx)) {
        if id == 300 {
            // We set the condition type to something unknown.
            // This triggers the default case in the switch branch, making the
            // condition evaluate to true.
            platform
                .ffi_offsets
                .bdat_item_type
                .expect("found BDAT id field, type is required")
                .set(inline_ctx, RegisterValue::RW(30));
        }
    }
}

unsafe extern "C" fn key_item_max_quantity(ptr: u64, id: u32) -> u64 {
    if id == 25447 {
        // Flutterheart Grass
        99
    } else {
        let orig: extern "C" fn(u64, u32) -> u64 =
            std::mem::transmute(KEY_ITEM_MAX_QTY_ORIG.get().unwrap().inner() as *const ());
        (orig)(ptr, id)
    }
}
