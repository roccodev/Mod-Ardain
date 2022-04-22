use std::ffi::CStr;

use serde::{Deserialize, Serialize};

use crate::ui::{
    container::{self, Container, ListHandler},
    render::Renderer,
    text::{Text, TextWidget},
    Color4f, Point, Widget,
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct RuntimeConfig {
    pub ui_visible: bool,
    pub blade_create_enable_save: bool,
    return_title: bool,
    infinite_flutterheart: bool,
    chain_attack_rate_fix: bool,
    blade_create_default_sel: BladeCreateDefault,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum BladeCreateDefault {
    Common = 1 << 0,
    Rare = 1 << 1,
    Legendary = 1 << 2,
    Worst = 1 << 3,
    Best = 1 << 4,
    Bravery = 1 << 5,
    Truth = 1 << 6,
    Compassion = 1 << 7,
    Justice = 1 << 8,
    NoRarity = 1 << 9,
    NoIdea = 1 << 10,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            ui_visible: true,
            blade_create_enable_save: true,
            return_title: true,
            infinite_flutterheart: true,
            chain_attack_rate_fix: true,
            blade_create_default_sel: BladeCreateDefault::Best,
        }
    }
}

// UI elements

struct ConfigEntryWidget<T: Fn()> {
    inner: Container,
    toggle_func: T,
}

macro_rules! cfg_entry {
    ($name:expr, $field:tt) => {{
        let widget = ConfigEntryWidget::new(c_str_ref!($name), || {
            let mut cfg = crate::get_platform_data().config.write().unwrap();
            cfg.$field ^= true;
        });
        Box::new(widget)
    }};
}

pub fn get_ui_widgets() -> Vec<Box<dyn Widget>> {
    let item = cfg_entry!("Show UI", ui_visible);
    vec![
        item,
        cfg_entry!("Enable Save in Create Blade", blade_create_enable_save),
    ]
}

impl<T: Fn()> ConfigEntryWidget<T> {
    fn new(name: &'static CStr, toggle_func: T) -> Self {
        let text = TextWidget::new(Text::new(name), Point::default());
        let container = Container::new(Color4f::default(), (100, 10), vec![box text]);
        Self {
            inner: container,
            toggle_func,
        }
    }
}

impl<T: Fn()> Widget for ConfigEntryWidget<T> {
    fn render(&self, base_pos: &crate::ui::Point, renderer: &Renderer<'_>) {
        self.inner.render(base_pos, renderer);
    }

    fn handle_input(&self, inputs: crate::input::PadData) -> bool {
        if inputs.is_click() {
            (self.toggle_func)();
            true
        } else {
            false
        }
    }

    fn get_width(&self) -> u32 {
        self.inner.get_width()
    }

    fn get_height(&self) -> u32 {
        self.inner.get_height()
    }
}
