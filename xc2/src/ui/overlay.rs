use std::{cell::OnceCell, cell::RefCell, convert::TryInto, sync::atomic::AtomicBool};

use crate::{input::PadData, PlatformData};

use super::{
    container::{Container, List, ListHandler, ListIndex},
    render::Renderer,
    text::{Text, TextWidget},
    Color4f, Line, Point, Widget,
};

#[derive(Debug)]
struct ModulesHandler;

thread_local! {
    static CACHED_UI: OnceCell<RefCell<Container>> = OnceCell::new();
}

pub(crate) fn render(platform: &PlatformData, renderer: &Renderer, inputs: PadData) -> bool {
    CACHED_UI.with(|ui| {
        let root = {
            match ui.get() {
                Some(root) => root,
                None => {
                    init(platform, renderer, ui);
                    ui.get().expect("ui not initialized")
                }
            }
        };

        update(&mut root.borrow_mut());
        let root = root.borrow();

        let mut input_handled = false;
        if !inputs.is_empty() {
            if root.handle_input(inputs) {
                input_handled = true;
            }
        }

        let screen = renderer.get_screen_dimensions();
        let half_width = screen.0 / 2;
        root.render(&Point::new(half_width.try_into().unwrap(), 0), renderer);

        input_handled
    })
}

fn init(platform: &PlatformData, renderer: &Renderer, dest: &OnceCell<RefCell<Container>>) {
    let screen = renderer.get_screen_dimensions();
    let half_width = screen.0 / 2;

    let title = TextWidget::new(
        Text::new(c_str_ref!("Mod Ardain")).scale(1.3).shadow(true),
        Point::new(10, 10),
    );

    let separator = Line::new(
        (100, 10),
        (100, (screen.1 - 10) as i32),
        Color4f::from_rgba(1.0, 1.0, 1.0, 1.0),
    );

    let mut test_list = List::new(true, None, box ModulesHandler);
    test_list.append(crate::config::get_ui_widgets());

    let root = Container::new(
        Color4f::from_rgba(0.0, 0.0, 0.0, 0.7),
        (half_width, screen.1),
        vec![box title, box test_list, box separator],
    );
    if dest.set(RefCell::new(root)).is_err() {
        panic!("Couldn't init UI");
    }
}

fn update(root: &mut Container) {}

impl ListHandler for ModulesHandler {
    fn on_select(&self, list: &List, from: Option<ListIndex>, to: ListIndex) {}
}
