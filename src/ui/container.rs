use std::{borrow::Cow, cell::Cell, convert::TryInto, num::NonZeroUsize};

use super::{render::Renderer, Color4f, Point, Rect, Widget};

type ListIndex = NonZeroUsize;

pub struct Container {
    dimensions: (u32, u32),
    color: Color4f,
    children: Vec<Box<dyn Widget>>,
}

pub struct List {
    selectable: bool,
    selected: Cell<Option<NonZeroUsize>>,
    handler: Box<dyn ListHandler>,
    max_height: u32,
    children: Vec<Container>,
}

pub trait ListHandler {
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
    pub fn push<W: Into<Box<dyn Widget>>>(&mut self, widget: W) {
        let widget = widget.into();
        let wrapped = Container::new(
            Color4f::default(),
            (widget.get_width(), widget.get_height()),
            vec![widget.into()],
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
        let selected = self.selected.get().map(|i| i.get()).unwrap_or(0);
        for (i, child) in self.children.iter().enumerate() {
            if i == selected - 1 {
                renderer.rect(
                    &Rect::from_point_dimensions(pos, (self.get_width(), child.get_height())),
                    &Color4f::from_rgba(1.0, 0.0, 0.0, 0.8),
                );
            }
            child.render(&pos, renderer);
            pos.add(0, child.get_height().try_into().unwrap());
        }
    }

    fn get_width(&self) -> u32 {
        100 // TODO: from parent
    }

    fn get_height(&self) -> u32 {
        let mut height = 0;
        for child in &self.children {
            let child_height = child.get_height();
            if height + child_height > self.max_height {
                return height;
            }
            height += child_height;
        }
        height
    }
}
