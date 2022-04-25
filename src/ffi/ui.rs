use skyline::libc::{c_char, c_void};

use super::{owned::FfiPointer, FfiConfig, Offset};
use crate::ui::Point;
use crate::PlatformData;

// Sizes as of XC2 2.1.0
const UI_STR_SIZE: usize = 20;
const UI_OBJ_ACC_SIZE: usize = 20;

#[derive(Clone, Copy, Debug)]
pub struct UiOffsets {
    ui_str_constructor: Offset,
    ui_str_destructor: Offset,

    ui_acc_constructor: Offset,
    ui_acc_destructor: Offset,
    ui_acc_set_pos: Offset,
    ui_acc_set_text: Offset,
    ui_acc_dup_child: Offset,
}

#[repr(C)]
pub struct UIStr<'p> {
    platform: &'p PlatformData,
    ptr: FfiPointer<'p, [u8; UI_STR_SIZE], c_void>,
}

/// High-level accessor for UIObject implementations.
#[repr(C)]
pub struct UIObjectAcc<'p> {
    platform: &'p PlatformData,
    ptr: FfiPointer<'p, [u8; UI_OBJ_ACC_SIZE], c_void>,
}

impl UiOffsets {
    pub fn load(config: &FfiConfig) -> Result<Self, &'static str> {
        Ok(Self {
            ui_str_constructor: config.get_function("ui-str-con").ok_or("UIStr()")?,
            ui_str_destructor: config.get_function("ui-str-des").ok_or("~UIStr()")?,
            ui_acc_constructor: config.get_function("ui-acc-con").ok_or("UIObjectAcc()")?,
            ui_acc_destructor: config.get_function("ui-acc-des").ok_or("~UIObjectAcc()")?,
            ui_acc_set_pos: config
                .get_function("ui-acc-pos")
                .ok_or("UIObjectAcc::setPos")?,
            ui_acc_set_text: config
                .get_function("ui-acc-text")
                .ok_or("UIObjectAcc::setText")?,
            ui_acc_dup_child: config
                .get_function("ui-acc-dup-child")
                .ok_or("UIObjectAcc::duplicateChild")?,
        })
    }
}

impl<'p> UIStr<'p> {
    pub fn from_ptr(platform: &'p PlatformData, ptr: *const c_void) -> Option<UIStr<'p>> {
        FfiPointer::from_ptr(ptr).map(|ptr| Self { platform, ptr })
    }

    pub fn new(platform: &'p PlatformData, text: *const c_char, make_clone: bool) -> UIStr<'p> {
        let mut buf = [0u8; UI_STR_SIZE];
        unsafe {
            (std::mem::transmute::<_, extern "C" fn(*mut u8, *const c_char, u32)>(
                platform
                    .ffi_offsets
                    .ui_offsets
                    .unwrap()
                    .ui_str_constructor
                    .as_fn(platform),
            ))(buf.as_mut_ptr(), text, if make_clone { 1 } else { 0 });
        }
        Self {
            platform,
            ptr: FfiPointer::new_owned(platform, buf, UIStr::destructor),
        }
    }

    unsafe fn destructor(res: &mut [u8; UI_STR_SIZE], platform: &'p PlatformData) {
        (std::mem::transmute::<_, extern "C" fn(*mut u8)>(
            platform
                .ffi_offsets
                .ui_offsets
                .unwrap()
                .ui_str_destructor
                .as_fn(platform),
        ))(res.as_mut_ptr());
    }
}

impl<'p> UIObjectAcc<'p> {
    pub fn from_ptr(platform: &'p PlatformData, ptr: *const c_void) -> Option<UIObjectAcc<'p>> {
        FfiPointer::from_ptr(ptr).map(|ptr| Self { platform, ptr })
    }

    pub fn new_from_id(platform: &'p PlatformData, id: u32) -> UIObjectAcc<'p> {
        let mut buf = [0u8; UI_OBJ_ACC_SIZE];
        unsafe {
            (std::mem::transmute::<_, extern "C" fn(*mut u8, u32)>(
                platform
                    .ffi_offsets
                    .ui_offsets
                    .unwrap()
                    .ui_acc_constructor
                    .as_fn(platform),
            ))(buf.as_mut_ptr(), id);
        }
        Self {
            platform,
            ptr: FfiPointer::new_owned(platform, buf, UIObjectAcc::destructor),
        }
    }

    pub fn duplicate_child(&self, child_name: *const c_char) -> u32 {
        unsafe {
            (std::mem::transmute::<_, extern "C" fn(*const c_void, *const c_char) -> u32>(
                self.platform
                    .ffi_offsets
                    .ui_offsets
                    .unwrap()
                    .ui_acc_dup_child
                    .as_fn(self.platform),
            ))(self.ptr.as_ptr(), child_name)
        }
    }

    pub fn set_pos(&self, pos: Point<i16>) {
        unsafe {
            (std::mem::transmute::<_, extern "C" fn(*const c_void, *const Point<i16>)>(
                self.platform
                    .ffi_offsets
                    .ui_offsets
                    .unwrap()
                    .ui_acc_set_pos
                    .as_fn(self.platform),
            ))(self.ptr.as_ptr(), &pos as *const _);
        }
    }

    pub fn set_text(&self, text: &UIStr<'_>) {
        unsafe {
            (std::mem::transmute::<_, extern "C" fn(*const c_void, *const c_void)>(
                self.platform
                    .ffi_offsets
                    .ui_offsets
                    .unwrap()
                    .ui_acc_set_text
                    .as_fn(self.platform),
            ))(self.ptr.as_ptr(), text.ptr.as_ptr());
        }
    }

    unsafe fn destructor(res: &mut [u8; UI_OBJ_ACC_SIZE], platform: &'p PlatformData) {
        (std::mem::transmute::<_, extern "C" fn(*mut u8)>(
            platform
                .ffi_offsets
                .ui_offsets
                .unwrap()
                .ui_acc_destructor
                .as_fn(platform),
        ))(res.as_mut_ptr());
    }
}
