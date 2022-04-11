use crate::PlatformData;
use hashbrown::HashMap;
use serde::Deserialize;
use skyline::hooks::InlineCtx;
use skyline::hooks::{A64HookFunction, A64InlineHook};
use skyline::libc::c_void;
use std::convert::TryInto;

pub mod hooks;
pub mod owned;
mod ui;

#[macro_export]
macro_rules! c_str {
    ($st:expr) => {
        concat!($st, '\0').as_ptr() as *const ::skyline::libc::c_char
    };
}

#[derive(Deserialize, Debug)]
pub struct FfiConfig {
    hooks: HashMap<String, Offset>,
    functions: HashMap<String, Offset>,
    registers: HashMap<String, Register>,
}

impl FfiConfig {
    pub fn get_hook(&self, key: &str) -> Option<Offset> {
        self.hooks
            .get(key)
            .and_then(|o| (o.offset != 0).then(|| o))
            .copied()
    }

    pub fn get_function(&self, key: &str) -> Option<Offset> {
        self.functions
            .get(key)
            .and_then(|o| (o.offset != 0).then(|| o))
            .copied()
    }

    pub fn get_register(&self, key: &str) -> Option<Register> {
        self.registers.get(key).copied()
    }
}

#[derive(Clone, Copy, Debug)]
enum RegisterType {
    X,
    W,
    R,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(from = "(char, usize)")]
pub struct Register {
    reg_type: RegisterType,
    index: usize,
}

#[derive(Debug)]
pub enum RegisterValue {
    RW(u32),
    X(u64),
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(from = "isize")]
pub struct Offset {
    offset: isize,
}

impl Offset {
    // TODO doc why unsafe, add alternative with PlatformData text_ptr
    pub(crate) unsafe fn as_fn(&self, platform: &PlatformData) -> *const () {
        platform.text_ptr.offset(self.offset)
    }

    pub(crate) unsafe fn patch_inline(
        &self,
        platform: &PlatformData,
        callback: unsafe extern "C" fn(&mut InlineCtx),
    ) {
        A64InlineHook(
            platform
                .text_ptr
                .offset::<c_void>(self.offset.try_into().unwrap()),
            callback as *const c_void,
        );
    }

    pub(crate) unsafe fn patch(
        &self,
        platform: &PlatformData,
        callback: *const c_void,
    ) -> *const c_void {
        let mut orig_fn: *mut c_void = std::ptr::null_mut();
        A64HookFunction(
            platform
                .text_ptr
                .offset::<c_void>(self.offset.try_into().unwrap()),
            callback,
            &mut orig_fn as *mut *mut c_void,
        );
        orig_fn as *const c_void
    }
}

impl Register {
    pub unsafe fn get(&self, inline_ctx: &InlineCtx) -> u64 {
        let regs = &inline_ctx.registers[self.index];
        match self.reg_type {
            RegisterType::X => *regs.x.as_ref(),
            RegisterType::W => (*regs.w.as_ref()).into(),
            RegisterType::R => (*regs.r.as_ref()).into(),
        }
    }

    pub unsafe fn set(&self, inline_ctx: &mut InlineCtx, val: RegisterValue) {
        let regs = &mut inline_ctx.registers[self.index];
        match (self.reg_type, val) {
            (RegisterType::X, RegisterValue::X(v)) => *regs.x.as_mut() = v,
            (RegisterType::W, RegisterValue::RW(v)) => *regs.w.as_mut() = v,
            (RegisterType::R, RegisterValue::RW(v)) => *regs.r.as_mut() = v,
            (t, v) => panic!("Incompatible register type {:?} with value {:?}", t, v),
        }
    }
}

impl From<isize> for Offset {
    fn from(addr: isize) -> Self {
        Self { offset: addr }
    }
}

impl From<(char, usize)> for Register {
    fn from((t, i): (char, usize)) -> Self {
        Self {
            index: i,
            reg_type: match t {
                'x' => RegisterType::X,
                'w' => RegisterType::W,
                'r' => RegisterType::R,
                t => panic!("Unsupported register type {}", t),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cfg() {
        let config = include_str!("../offsets/2.1.0.toml");
        let config: FfiConfig = toml::from_str(config).unwrap();
    }
}
