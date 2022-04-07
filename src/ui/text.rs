use std::ffi::CStr;

use skyline::libc::c_char;

use crate::{
    ffi::{FfiConfig, Offset},
    PlatformData,
};

use super::Color4f;

#[derive(Debug)]
pub struct TextRenderer {
    draw_text_fn: Offset,
    draw_text_color_fn: Option<Offset>,
    draw_text_scale_fn: Option<Offset>,
}

pub struct Text<'s> {
    text: &'s CStr,
    color: Option<Color4f>,
    scale: f32,
}

impl TextRenderer {
    pub fn new(ffi_cfg: &FfiConfig) -> Self {
        Self {
            draw_text_fn: ffi_cfg
                .get_function("draw-font")
                .expect("no draw-font in offsets"),
            draw_text_color_fn: ffi_cfg.get_function("draw-font-color"),
            draw_text_scale_fn: ffi_cfg.get_function("draw-font-scale"),
        }
    }
}

impl<'s> Text<'s> {
    pub fn new<'t: 's, T: 't + AsRef<CStr>>(text: &'t T) -> Text<'s> {
        Self {
            text: text.as_ref(),
            color: None,
            scale: 0f32,
        }
    }

    pub fn color(self, r: f32, g: f32, b: f32, alpha: f32) -> Text<'s> {
        Self {
            color: Some(Color4f::from_rgba(r, g, b, alpha)),
            ..self
        }
    }

    pub fn scale(self, scale: f32) -> Text<'s> {
        Self { scale, ..self }
    }
}

impl TextRenderer {
    pub(crate) fn draw_text<'s>(&self, platform: &PlatformData, x: i32, y: i32, text: Text<'s>) {
        if let Some(color) = text.color {
            if let Some(draw_text_color_fn) = self.draw_text_color_fn {
                unsafe {
                    let f: extern "C" fn(*const Color4f) =
                        std::mem::transmute(draw_text_color_fn.as_fn(platform));
                    (f)(&color as *const Color4f)
                }
            }
        }
        if text.scale != 0f32 {
            if let Some(draw_text_scale_fn) = self.draw_text_scale_fn {
                unsafe {
                    let f: extern "C" fn(f32, f32) =
                        std::mem::transmute(draw_text_scale_fn.as_fn(platform));
                    (f)(text.scale, text.scale);
                }
            }
        }
        unsafe {
            let f: extern "C" fn(i32, i32, *const c_char) =
                std::mem::transmute(self.draw_text_fn.as_fn(platform));
            (f)(x, y, text.text.as_ptr() as *const u8);
        }
    }
}
