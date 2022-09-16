use std::fmt::Debug;

use crate::{
    gfx::{Point, Rect, Scalar, Size, WriteSurface},
    ui::{Hit, Widget},
};

#[derive(Default)]
pub struct HBox<'a, I> {
    pub id: Option<I>,
    pub children: &'a [&'a dyn Widget<I>],
}

impl<'a, I: Copy + Debug> Widget<I> for HBox<'a, I> {
    fn measure(&self, limits: Size) -> Size {
        let mut size = Size::ZERO;
        for child in self.children {
            let child_size = child.measure(limits);
            size.width += child_size.width;
            size.height = size.height.max(child_size.height);
        }
        size.limit(limits)
    }

    fn render(
        &self,
        bounds: Rect,
        cursor: Point,
        surface: &mut dyn WriteSurface,
    ) -> Option<Hit<I>> {
        let mut width = 0;
        for child in self.children {
            width += child.measure(bounds.size).width;
        }
        let scale = bounds.size.width as f32 / width as f32;

        let mut hit = None;
        let mut origin = bounds.origin;
        for child in self.children {
            let size = child.measure(bounds.size);
            let width = scale * size.width as f32;
            let child_hit = child.render(
                Rect::new(origin, Size::new(width as Scalar, size.height)),
                cursor,
                surface,
            );
            origin.x += width as Scalar;
            if child_hit.is_some() {
                hit = child_hit;
            }
        }
        if hit.is_some() {
            return hit;
        }
        Hit::from_test(self.id, bounds, cursor)
    }
}
