use std::{
    borrow::Cow,
    cell::Cell,
    convert::TryInto,
    fmt::Debug,
    num::{NonZeroU32, NonZeroUsize},
};

use crate::input::{self, PadButton, PadData};

use super::{render::Renderer, Color4f, Point, Rect, Widget};

pub type ListIndex = NonZeroUsize;

#[derive(Debug)]
pub struct Container {
    dimensions: (u32, u32),
    color: Color4f,
    children: Vec<Box<dyn Widget>>,
}

#[derive(Debug)]
pub struct List {
    selectable: bool,
    selected: Cell<Option<NonZeroUsize>>,
    handler: Box<dyn ListHandler>,
    max_height: Option<NonZeroU32>,
    children: Vec<Container>,
}

pub trait ListHandler: Debug {
    fn on_select(&self, list: &List, from: Option<ListIndex>, to: ListIndex);
}

impl Container {
    pub fn new(color: Color4f, dimensions: (u32, u32), items: Vec<Box<dyn Widget>>) -> Self {
        Self {
            dimensions,
            color,
            children: items,
        }
    }
}

impl List {
    pub fn new(
        selectable: bool,
        max_height: Option<NonZeroU32>,
        handler: Box<dyn ListHandler>,
    ) -> Self {
        Self {
            selectable,
            selected: Cell::new(None),
            handler,
            max_height,
            children: Vec::new(),
        }
    }

    pub fn push<W: 'static + Widget, B: Into<Box<W>>>(&mut self, widget: B) {
        let widget = widget.into();
        let wrapped = Container::new(
            Color4f::default(),
            (widget.get_width(), widget.get_height()),
            vec![widget],
        );
        self.children.push(wrapped);
    }
}

impl Widget for Container {
    fn render(&self, base_pos: &Point, renderer: &Renderer<'_>) {
        renderer.rect(
            &Rect::from_point_dimensions(*base_pos, self.dimensions),
            &self.color,
        );
        let mut pos = *base_pos;
        for child in &self.children {
            child.render(&pos, renderer);
            pos.add(0, child.get_height().try_into().unwrap());
        }
    }

    fn handle_input(&self, inputs: PadData) {
        for child in &self.children {
            child.handle_input(inputs);
        }
    }

    fn get_width(&self) -> u32 {
        self.dimensions.0
    }

    fn get_height(&self) -> u32 {
        self.dimensions.1
    }
}

impl Widget for List {
    fn render(&self, base_pos: &Point, renderer: &Renderer<'_>) {
        let mut pos = *base_pos;
        let selected = self
            .selected
            .get()
            .map(NonZeroUsize::get)
            .unwrap_or_default();
        for (i, child) in self.children.iter().enumerate() {
            if selected > 0 && i == selected - 1 {
                renderer.rect(
                    &Rect::from_point_dimensions(pos, (self.get_width(), child.get_height())),
                    &Color4f::from_rgba(1.0, 0.0, 0.0, 0.8),
                );
            }
            child.render(&pos, renderer);
            pos.add(0, child.get_height().try_into().unwrap());
        }
    }

    fn handle_input(&self, inputs: PadData) {
        if !self.selectable {
            return;
        }
        if inputs.contains(PadButton::LeftStickDown) || inputs.contains(PadButton::DpadDown) {
            self.selected.update(|old| {
                let new_index = old
                    .map(NonZeroUsize::get)
                    .unwrap_or_default()
                    .saturating_add(1);
                if new_index - 1 >= self.children.len() {
                    old
                } else {
                    Some(unsafe { NonZeroUsize::new_unchecked(new_index) })
                }
            });
        } else if inputs.contains(PadButton::LeftStickUp) || inputs.contains(PadButton::DpadUp) {
            self.selected.update(|old| {
                let new_index = old
                    .map(NonZeroUsize::get)
                    .unwrap_or_default()
                    .saturating_sub(1)
                    .max(1);
                if new_index - 1 >= self.children.len() {
                    old
                } else {
                    Some(unsafe { NonZeroUsize::new_unchecked(new_index) })
                }
            });
        } else if let Some(selected) = self.selected.get() {
            // Propagate inputs to children
            let item = &self.children[selected.get() - 1];
            item.handle_input(inputs);
        }
    }

    fn get_width(&self) -> u32 {
        100 // TODO: from parent
    }

    fn get_height(&self) -> u32 {
        let mut height = 0;
        let max_height = self.max_height.map(NonZeroU32::get).unwrap_or_default();
        for child in &self.children {
            let child_height = child.get_height();
            if max_height > 0 && height + child_height > max_height {
                return height;
            }
            height += child_height;
        }
        height
    }
}
