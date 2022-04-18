use std::{cell::RefCell, convert::TryInto, lazy::OnceCell};

use crate::{input::PadData, PlatformData};

use super::{
    container::{Container, List, ListHandler, ListIndex},
    render::Renderer,
    text::{Text, TextWidget},
    Color4f, Point, Widget,
};

#[derive(Debug)]
struct ModulesHandler;

thread_local! {
    static CACHED_UI: OnceCell<RefCell<Container>> = OnceCell::new();
}

pub(crate) fn render(platform: &PlatformData, renderer: &Renderer, inputs: PadData) {
    CACHED_UI.with(|ui| {
        let root = {
            match ui.get() {
                Some(root) => root,
                None => {
                    init(platform, renderer, &ui);
                    ui.get().expect("ui not initialized")
                }
            }
        };

        update(&mut root.borrow_mut());
        let root = root.borrow();

        if !inputs.is_empty() {
            root.handle_input(inputs);
        }

        let screen = renderer.get_screen_dimensions();
        let half_width = screen.0 / 2;
        root.render(&Point::new(half_width.try_into().unwrap(), 0), renderer);
    });
}

fn init(platform: &PlatformData, renderer: &Renderer, dest: &OnceCell<RefCell<Container>>) {
    let screen = renderer.get_screen_dimensions();
    let half_width = screen.0 / 2;

    let title = TextWidget::new(
        Text::new(c_str_ref!("Mod Ardain")).scale(1.3).shadow(true),
        Point::new(10, 10),
    );

    let mut test_list = List::new(true, None, box ModulesHandler);
    test_list.push(TextWidget::at_root(Text::new(c_str_ref!("Item 1"))));
    test_list.push(TextWidget::at_root(Text::new(c_str_ref!("Item 2"))));
    test_list.push(TextWidget::at_root(Text::new(c_str_ref!("Item 3"))));

    let root = Container::new(
        Color4f::from_rgba(0.0, 0.0, 0.0, 0.7),
        (half_width, screen.1),
        vec![box title, box test_list],
    );
    dest.set(RefCell::new(root)).expect("couldn't init ui");
}

fn update(root: &mut Container) {}

impl ListHandler for ModulesHandler {
    fn on_select(&self, list: &List, from: Option<ListIndex>, to: ListIndex) {}
}
