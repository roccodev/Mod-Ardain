use std::{fmt::Debug, lazy::SyncOnceCell, sync::atomic::Ordering};

use crate::{
    ffi::{
        ui::{UIObjectAcc, UIStr},
        FfiConfig, Offset, Register, RegisterValue,
    },
    get_platform_data,
    input::{PadButton, PadData},
    ui::Point,
    PlatformData, StaticPtr,
};
use skyline::{hooks::InlineCtx, libc::c_void};

use super::ui::UiOffsets;

static KEY_ITEM_MAX_QTY_ORIG: SyncOnceCell<StaticPtr> = SyncOnceCell::new();

#[derive(Debug, Clone, Copy)]
pub struct Offsets {
    return_title: Option<Offset>,
    input_register: Register,
    bdat_item_id: Option<Register>,
    bdat_item_type: Option<Register>,
    chain_attack_rate_branch: Option<Register>,
    title_root_register: Option<Register>,
    pub draw_square_2d: Option<Offset>,
    draw_compare_z: Option<Offset>,
    pub ui_offsets: Option<UiOffsets>,
}

impl Offsets {
    pub fn read_all(config: &FfiConfig) -> Self {
        let ui_offsets = match UiOffsets::load(config) {
            Ok(o) => Some(o),
            Err(e) => {
                println!("[XC2MM] Couldn't load UI offsets, missing fn {:?}", e);
                None
            }
        };
        Self {
            return_title: config.get_function("return-title"),
            input_register: config
                .get_register("input-pad-data")
                .expect("input offset required"),
            bdat_item_id: config.get_register("bdat-item-cond-id"),
            bdat_item_type: config.get_register("bdat-item-cond-type"),
            chain_attack_rate_branch: config.get_register("chain-attack-rate-branch"),
            title_root_register: config.get_register("title-root"),
            draw_square_2d: config.get_function("draw-square-2d"),
            draw_compare_z: config.get_function("draw-compare-z"),
            ui_offsets,
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
    if let Some(hook) = config.get_hook("chain-attack-enemy-atk-rate") {
        hook.patch_inline(platform, chain_attack_rate_fix);
    }
    if let Some(hook) = config.get_hook("title-screen-load") {
        hook.patch_inline(platform, title_screen_load);
    }
}

/// Hook into the checkEmergencyEscape function, just after the input data is loaded into a
/// register.
unsafe extern "C" fn on_frame(inline_ctx: &mut InlineCtx) {
    let platform = crate::get_platform_data();
    let inputs = PadData::from(platform.ffi_offsets.input_register.get(inline_ctx));
    let can_input = platform.no_input_frames.fetch_add(1, Ordering::Relaxed) >= 10;

    let mut had_input = if can_input {
        if inputs.contains(PadButton::L + PadButton::LeftStickClick) {
            // Toggle UI visibility
            platform
                .ui_visible
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |b| Some(!b))
                .ok();
            true
        } else if inputs.contains(PadButton::L + PadButton::R + PadButton::A + PadButton::Plus) {
            // Return to title
            match platform.ffi_offsets.return_title {
                Some(return_title) => {
                    // 0xff_ff_ff_ff is always used in the executable (it's the save
                    // slot)
                    std::mem::transmute::<_, extern "C" fn(u32)>(return_title.as_fn(platform))(
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

    if platform.ui_visible.load(Ordering::Relaxed) {
        if let Some(renderer) = crate::ui::get_renderer() {
            if crate::ui::overlay::render(
                platform,
                renderer,
                if can_input {
                    // TODO: Disable in-game inputs
                    inputs
                } else {
                    PadData::default()
                },
            ) {
                had_input = true;
            }
        }
    }

    if had_input {
        platform.no_input_frames.store(0, Ordering::Relaxed);
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

unsafe extern "C" fn chain_attack_rate_fix(inline_ctx: &mut InlineCtx) {
    // When "Enemy Attack Power" is > 1.0 and the number of "Cancel Attacks" is > 0,
    // the chain attack base damage rate glitches out to 100%, regardless of any
    // other bonuses.
    //
    // We tweak the branch so it unconditionally skips this check.
    get_platform_data()
        .ffi_offsets
        .chain_attack_rate_branch
        .expect("branch register required for chain attack fix")
        .set(inline_ctx, RegisterValue::RW(1));
}

unsafe extern "C" fn title_screen_load(inline_ctx: &mut InlineCtx) {
    let platform = get_platform_data();
    if platform.ffi_offsets.ui_offsets.is_some() {
        if let Some(root) = platform.ffi_offsets.title_root_register {
            let root_obj = UIObjectAcc::from_ptr(platform, root.get(inline_ctx) as *const _);
            if let Some(root_obj) = root_obj {
                let dup_id = root_obj.duplicate_child(c_str!("TXT_copyright"));
                let dup = UIObjectAcc::new_from_id(platform, dup_id);

                let text = UIStr::new(platform, crate::VERSION_STRING.as_ptr() as *const _, false);

                // It's important that we set the position first, as setting
                // the text shifts the object to keep horizontal alignment.
                dup.set_pos(Point::new(920, 60));
                dup.set_text(&text);
            }
        }
    }
}
