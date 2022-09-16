mod border;
mod hbox;
mod label;
mod margin;
mod stack;
mod vbox;

use std::fmt::Debug;

pub use border::*;
pub use hbox::*;
pub use label::*;
pub use margin::*;
pub use stack::*;
pub use vbox::*;

use crate::gfx::{BlendMode, Color, Point, Rect, Size, WriteSurface};

pub struct UI {}

#[derive(Debug)]
pub struct Hit<I> {
    id: I,
    bounds: Rect,
}

impl<I> Hit<I> {
    pub fn from_test(id: Option<I>, bounds: Rect, cursor: Point) -> Option<Hit<I>> {
        if let Some(id) = id {
            if bounds.contains(cursor) {
                return Some(Hit { id, bounds });
            }
        }
        None
    }

    #[inline]
    pub fn id(&self) -> &I {
        &self.id
    }

    #[inline]
    pub fn bounds(&self) -> Rect {
        self.bounds
    }
}

#[derive(Debug, Default)]
pub struct Feedback<I> {
    size: Size,
    hit: Option<Hit<I>>,
}

impl<I> Feedback<I> {
    #[inline]
    pub fn size(&self) -> Size {
        self.size
    }

    #[inline]
    pub fn hit(&self) -> Option<&Hit<I>> {
        self.hit.as_ref()
    }
}

impl UI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render<I>(
        &mut self,
        bounds: Rect,
        surface: &mut dyn WriteSurface,
        cursor: Point,
        root: &dyn Widget<I>,
    ) -> Feedback<I> {
        let size = root.measure(bounds.size);
        Feedback {
            size,
            hit: root.render(Rect::new(bounds.origin, size), cursor, surface),
        }
    }
}

pub trait Widget<I>: Debug {
    fn measure(&self, limits: Size) -> Size;

    fn render(&self, bounds: Rect, cursor: Point, surface: &mut dyn WriteSurface)
        -> Option<Hit<I>>;
}
