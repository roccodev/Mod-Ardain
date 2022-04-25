use std::fmt::Debug;
use std::ops::{Add, AddAssign};

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

/// A 2D point.
///
/// This can hold any number type. The game uses Pnt<short> in the UI library,
/// and Pnt<int> for almost everything else.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Point<N = i32> {
    x: N,
    z: N,
}

#[repr(C)]
pub struct Rect {
    x: i32,
    z: i32,
    width: u32,
    height: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    start: Point,
    end: Point,
    color: Color4f,
}

pub(crate) fn load(config: &FfiConfig, platform: &'static PlatformData) {
    let renderer = Renderer::load(config, platform);
    render::RENDERER.set(renderer).unwrap();
}

pub fn get_renderer() -> Option<&'static Renderer<'static>> {
    render::RENDERER.get()
}

impl<N> Point<N> {
    pub const fn new(x: N, z: N) -> Point<N> {
        Self { x, z }
    }
}

impl<N: AddAssign> Point<N> {
    pub fn add(&mut self, x: N, z: N) {
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

impl Line {
    pub fn new<P: Into<Point>>(start: P, end: P, color: Color4f) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
            color,
        }
    }

    pub fn render(&self) {
        let platform = get_platform_data();
        if let Some(draw_line) = platform.ffi_offsets.draw_line_2d {
            unsafe {
                offset_fn!(
                    platform,
                    draw_line,
                    (*const Point, *const Point, *const Color4f)
                )(
                    &self.start as *const _,
                    &self.end as *const _,
                    &self.color as *const _,
                );
            }
        }
    }
}

impl Widget for Line {
    fn render(&self, base_pos: &Point, renderer: &Renderer<'_>) {
        let with_offset = Self {
            start: self.start + *base_pos,
            end: self.end + *base_pos,
            color: self.color,
        };
        with_offset.render();
    }

    fn handle_input(&self, inputs: PadData) -> bool {
        // no-op
        false
    }

    fn get_width(&self) -> u32 {
        (self.end.x - self.start.x).max(1) as u32
    }

    fn get_height(&self) -> u32 {
        (self.end.z - self.start.z).max(1) as u32
    }
}

impl Default for Color4f {
    fn default() -> Self {
        Self::default()
    }
}

impl<N: Default> Default for Point<N> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            z: Default::default(),
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from(coords: (i32, i32)) -> Self {
        Self::new(coords.0, coords.1)
    }
}

impl<N: Add<Output = N>> Add for Point<N> {
    type Output = Point<N>;

    fn add(self, rhs: Point<N>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            z: self.z + rhs.z,
        }
    }
}
