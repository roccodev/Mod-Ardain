use std::{cell::Cell, convert::TryInto, num::NonZeroUsize};

use super::{render::Renderer, Color4f, Point, Rect, Widget};

type ListIndex = NonZeroUsize;

pub struct Container<'w> {
    dimensions: (u32, u32),
    color: Color4f,
    children: Vec<&'w dyn Widget>,
}

pub struct List<'c> {
    selectable: bool,
    selected: Cell<Option<NonZeroUsize>>,
    max_height: u32,
    children: Vec<Container<'c>>,
}

pub trait ListHandler {
    fn on_select(list: &List<'_>, from: Option<ListIndex>, to: ListIndex);
}

impl<'w> Container<'w> {
    pub fn new(color: Color4f, dimensions: (u32, u32), items: Vec<&'w dyn Widget>) -> Self {
        Self {
            dimensions,
            color,
            children: items,
        }
    }
}

impl<'c> List<'c> {
    pub fn push<W: Into<Box<dyn Widget>>>(&mut self, widget: W) {
        let widget = widget.into();
        let wrapped = Container {
            dimensions: (widget.get_width(), widget.get_height()), // TODO set from parent
            color: Color4f::default(),
            children: vec![/*&widget*/],
        };
        self.children.push(wrapped);
    }
}

impl<'w> Widget for Container<'w> {
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

impl<'c> Widget for List<'c> {
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
        0 // TODO: from parent
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
