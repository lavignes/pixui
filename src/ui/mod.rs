mod border;
mod handler;
mod hbox;
mod hspan;
mod label;
mod margin;
mod overflow;
mod stack;
mod vbox;
mod vspan;

use std::{cell::RefCell, fmt::Debug};

pub use border::*;
pub use handler::*;
pub use hbox::*;
pub use hspan::*;
pub use label::*;
pub use margin::*;
pub use overflow::*;
pub use stack::*;
pub use vbox::*;
pub use vspan::*;

use crate::gfx::{Point, Rect, Size, WriteSurface};

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

pub trait Widget<I> {
    fn measure(&self, limits: Size) -> Size;

    fn render(&self, bounds: Rect, cursor: Point, surface: &mut dyn WriteSurface)
        -> Option<Hit<I>>;
}

struct Callbacks<M> {
    measure: Option<M>,
}

struct Interceptor<'a, I, M> {
    state: RefCell<Callbacks<M>>,
    child: &'a dyn Widget<I>,
}

impl<'a, I, M> Interceptor<'a, I, M> {
    #[inline]
    pub fn measure(measure: M, child: &'a dyn Widget<I>) -> Self {
        Self {
            state: RefCell::new(Callbacks {
                measure: Some(measure),
            }),
            child,
        }
    }
}

impl<'a, I, M: FnMut(Size, Size) -> Size> Widget<I> for Interceptor<'a, I, M> {
    #[inline]
    fn measure(&self, limits: Size) -> Size {
        let size = self.child.measure(limits);
        let mut state = self.state.borrow_mut();
        if let Some(measure) = &mut state.measure {
            measure(limits, size)
        } else {
            size
        }
    }

    #[inline]
    fn render(
        &self,
        bounds: Rect,
        cursor: Point,
        surface: &mut dyn WriteSurface,
    ) -> Option<Hit<I>> {
        self.child.render(bounds, cursor, surface)
    }
}
