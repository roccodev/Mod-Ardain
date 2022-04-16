use std::convert::TryInto;

use crate::{input::PadData, PlatformData};

use super::{
    container::Container,
    render::Renderer,
    text::{Text, TextWidget},
    Color4f, Point, Widget,
};

pub(crate) fn render(platform: &PlatformData, renderer: &Renderer, inputs: PadData) {
    let screen = renderer.get_screen_dimensions();
    let half_width = screen.0 / 2;

    let title = TextWidget::new(
        Text::new(c_str_ref!("Mod Ardain")).scale(1.3).shadow(true),
        Point::new(10, 10),
    );

    let root = Container::new(
        Color4f::from_rgba(0.0, 0.0, 0.0, 0.7),
        (half_width, screen.1),
        vec![box title],
    );
    root.render(&Point::new(half_width.try_into().unwrap(), 0), renderer);
}
