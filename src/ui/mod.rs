use std::fmt::Debug;

use crate::get_platform_data;
use crate::input::PadData;
use crate::{ffi::FfiConfig, PlatformData};
use render::Renderer;

pub mod container;
pub mod overlay;
pub mod render;
pub mod text;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Color4f {
    r: f32,
    g: f32,
    b: f32,
    alpha: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: i16,
    z: i16,
}

#[repr(C)]
pub struct Rect {
    x: i32,
    z: i32,
    width: u32,
    height: u32,
}

pub(crate) fn load(config: &FfiConfig, platform: &'static PlatformData) {
    let renderer = Renderer::load(config, platform);
    render::RENDERER.set(renderer).unwrap();
}

pub fn get_renderer() -> Option<&'static Renderer<'static>> {
    render::RENDERER.get()
}

impl Point {
    pub const fn new(x: i16, z: i16) -> Point {
        Self { x, z }
    }

    pub const fn default() -> Self {
        Self::new(0, 0)
    }

    pub fn add(&mut self, x: i16, z: i16) {
        self.x += x;
        self.z += z;
    }
}

pub trait Widget {
    fn render(&self, base_pos: &Point, renderer: &Renderer<'_>);
    fn handle_input(&self, inputs: PadData) -> bool;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
}

impl Color4f {
    pub const fn default() -> Color4f {
        Color4f::from_rgba(0.0, 0.0, 0.0, 0.0)
    }

    pub const fn from_rgba(r: f32, g: f32, b: f32, alpha: f32) -> Color4f {
        Color4f { r, g, b, alpha }
    }
}

impl Rect {
    pub fn from_points(start: Point, end: Point) -> Self {
        let width = (end.x - start.x).abs() as u32;
        let height = (end.z - start.z).abs() as u32;
        Self::from_point_dimensions(start, (width, height))
    }

    pub fn from_point_dimensions(start: Point, dimensions: (u32, u32)) -> Self {
        Self {
            x: start.x as i32,
            z: start.z as i32,
            width: dimensions.0,
            height: dimensions.1,
        }
    }

    pub fn render(&self, color: &Color4f) {
        let platform = get_platform_data();
        if let Some(draw_rect) = platform.ffi_offsets.draw_square_2d {
            unsafe {
                (std::mem::transmute::<_, extern "C" fn(*const Rect, *const Color4f)>(
                    draw_rect.as_fn(platform),
                ))(self as *const Rect, color as *const Color4f);
            }
        }
    }
}

impl Default for Color4f {
    fn default() -> Self {
        Self::default()
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::default()
    }
}
