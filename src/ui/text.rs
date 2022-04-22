use std::ffi::CStr;

use skyline::libc::{c_char, c_void};

use crate::{
    ffi::{FfiConfig, Offset},
    input::PadData,
    PlatformData,
};

use super::{Color4f, Point, Widget};

#[derive(Debug)]
pub struct TextRenderer {
    draw_text_fn: Offset,
    draw_text_color_fn: Option<Offset>,
    draw_text_scale_fn: Option<Offset>,
}

#[derive(Debug)]
pub struct Text<'s> {
    text: &'s CStr,
    color: Option<Color4f>,
    scale: f32,
    shadow: bool,
}

#[derive(Debug)]
pub struct TextWidget<'s> {
    text: Text<'s>,
    pos: Point,
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
    pub fn new<'t: 's, T: 't + AsRef<CStr> + ?Sized>(text: &'t T) -> Text<'s> {
        Self {
            text: text.as_ref(),
            color: None,
            scale: 0f32,
            shadow: false,
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

    pub fn shadow(self, shadow: bool) -> Text<'s> {
        Self { shadow, ..self }
    }
}

impl TextRenderer {
    pub(crate) fn draw_text<'s>(&self, platform: &PlatformData, x: i32, y: i32, text: &Text<'s>) {
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
            unsafe {
                self.set_scale(platform, (text.scale, text.scale));
            }
        }
        unsafe {
            let f: extern "C" fn(i16, i16, *const c_char) =
                std::mem::transmute(self.draw_text_fn.as_fn(platform));
            (f)(x as i16, y as i16, text.text.as_ptr() as *const u8);
        }

        // Reset state
        if text.scale != 0f32 {
            unsafe {
                self.set_scale(platform, (1.0, 1.0));
            }
        }
    }

    unsafe fn set_scale(&self, platform: &PlatformData, scale: (f32, f32)) {
        let set_scale_fn = match self.draw_text_scale_fn {
            Some(f) => f,
            None => return,
        };
        // CacheDraw::fontScale is inlined in 2.1.0, thus we call
        // FontLayer::fontScale, which will use the DevFontLayer if DebDraw
        // hasn't been initialized.
        // In that case we need to skip it, so we give a special pointer so that
        // *(ptr + 8) == 0, i.e. *((ptr as *const u64).offset(1)) == 0
        let buf = [0u64; 2];
        offset_fn!(platform, set_scale_fn, (*const c_void, f32, f32))(
            buf.as_ptr() as *const _,
            scale.0,
            scale.1,
        );
    }
}

impl<'s> TextWidget<'s> {
    pub fn new(text: Text<'s>, pos: Point) -> Self {
        Self { text, pos }
    }

    pub fn at_root(text: Text<'s>) -> Self {
        Self::new(text, Point::default())
    }
}

impl<'s> Widget for TextWidget<'s> {
    fn render(&self, base_pos: &super::Point, renderer: &super::render::Renderer) {
        let mut point = *base_pos;
        point.add(self.pos.x, self.pos.z);
        renderer.text(point, &self.text);
    }

    fn handle_input(&self, inputs: PadData) {
        // no-op
    }

    fn get_width(&self) -> u32 {
        100 // TODO
    }

    fn get_height(&self) -> u32 {
        20 // TODO
    }
}
