use std::{cell::Cell, lazy::SyncOnceCell, sync::RwLock};

use skyline::libc::c_void;

use super::{text::Text, Color4f, Point, Rect};

pub(in crate::ui) static RENDERER: SyncOnceCell<Renderer<'static>> = SyncOnceCell::new();

use crate::{
    ffi::{FfiConfig, Offset},
    PlatformData, StaticPtr,
};

#[derive(Debug)]
pub struct Renderer<'p> {
    platform: &'p PlatformData,
    foreign: RwLock<StaticPtr>, // TODO might violate
    offsets: Offsets,
}

#[derive(Debug)]
struct Offsets {
    deb_draw_get: Offset,
    set_color: Option<Offset>,
    render_rect_fill: Option<Offset>,
    render_rect_outline: Option<Offset>,
    get_screen_width: Option<Offset>,
    get_screen_height: Option<Offset>,
}

impl<'p> Renderer<'p> {
    pub fn load(config: &FfiConfig, platform: &'p PlatformData) -> Self {
        let offsets = Offsets {
            deb_draw_get: config
                .get_function("render-get")
                .expect("no render get function"),
            set_color: config.get_function("render-set-color"),
            render_rect_fill: config.get_function("render-rect-fill"),
            render_rect_outline: config.get_function("render-rect-outline"),
            get_screen_width: config.get_function("render-scr-width"),
            get_screen_height: config.get_function("render-scr-height"),
        };
        Self {
            foreign: RwLock::new(unsafe { StaticPtr::copy_of(std::ptr::null::<c_void>()) }),
            platform,
            offsets,
        }
    }

    pub fn text(&self, point: Point, text: Text<'_>) {
        self.platform
            .text_renderer
            .draw_text(self.platform, point.x as i32, point.z as i32, &text);
    }

    pub fn rect(&self, rect: &Rect, color: &Color4f) {
        match (self.get_foreign(), self.offsets.render_rect_fill) {
            (Some(foreign), Some(draw_rect)) => {
                self.color(color, foreign);
                unsafe {
                    offset_fn!(self.platform, draw_rect, (*const c_void, *const Rect))(
                        foreign,
                        rect as *const Rect,
                    )
                }
            }
            _ => {}
        }
    }

    pub fn rect_outline(&self, rect: &Rect, color: &Color4f) {
        match (self.get_foreign(), self.offsets.render_rect_outline) {
            (Some(foreign), Some(rect_outline)) => {
                self.color(color, foreign);
                unsafe {
                    offset_fn!(self.platform, rect_outline, (*const c_void, *const Rect))(
                        foreign,
                        rect as *const Rect,
                    );
                }
            }
            _ => {}
        }
    }

    pub fn get_screen_dimensions(&self) -> (u32, u32) {
        let width = match self.offsets.get_screen_width {
            Some(get_screen_width) => unsafe {
                offset_fn!(self.platform, get_screen_width, () -> u32)()
            },
            None => 1280,
        };
        let height = match self.offsets.get_screen_height {
            Some(get_screen_height) => unsafe {
                offset_fn!(self.platform, get_screen_height, () -> u32)()
            },
            None => 720,
        };
        (width, height)
    }

    fn color(&self, color: &Color4f, foreign: *const c_void) {
        if let Some(set_color) = self.offsets.set_color {
            unsafe {
                offset_fn!(self.platform, set_color, (*const c_void, *const Color4f))(
                    foreign,
                    color as *const Color4f,
                );
            }
        }
    }

    fn get_foreign(&self) -> Option<*const c_void> {
        let val = self.foreign.read().unwrap().inner::<c_void>();
        if val.is_null() {
            let ptr = unsafe {
                offset_fn!(self.platform, self.offsets.deb_draw_get, (u32) -> *const c_void)(
                    0xffffffff,
                )
            };
            if ptr.is_null() {
                None
            } else {
                let mut write_to = self.foreign.write().unwrap();
                *write_to = unsafe { StaticPtr::copy_of(ptr) };
                Some(ptr)
            }
        } else {
            Some(val)
        }
    }
}
