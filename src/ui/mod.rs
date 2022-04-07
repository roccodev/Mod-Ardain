pub mod text;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Color4f {
    r: f32,
    g: f32,
    b: f32,
    alpha: f32,
}

impl Color4f {
    pub fn from_rgba(r: f32, g: f32, b: f32, alpha: f32) -> Color4f {
        Color4f { r, g, b, alpha }
    }
}
