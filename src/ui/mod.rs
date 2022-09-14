mod border;
mod label;
mod margin;

use std::fmt::Debug;

pub use border::*;
pub use label::*;
pub use margin::*;

use crate::gfx::{Point, Rect, Size, WriteSurface};

struct Input {
    hover: Point,
    touch: Point,
}

pub struct UI {
    // last_input: Input,
    // current_input: Input,
}

// struct Touchable / struct Touchable
// struct Receiver ?

impl UI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &mut self,
        bounds: Rect,
        surface: &mut dyn WriteSurface,
        root: &dyn Widget,
    ) -> Size {
        let limits = bounds.size;
        let size = root.measure(limits);
        root.render(Rect::new(bounds.origin, size), surface);
        size
    }
}

pub trait Widget: Debug {
    fn id(&self) -> Option<&str> {
        None
    }

    fn measure(&self, limits: Size) -> Size;

    fn render(&self, bounds: Rect, surface: &mut dyn WriteSurface);
}
