use std::fmt::Debug;

use crate::{
    gfx::{Point, Rect, Scalar, Size, WriteSurface},
    ui::{Hit, Widget},
};

#[derive(Debug, Default)]
pub struct VBox<'a, I> {
    pub id: Option<I>,
    pub children: &'a [&'a dyn Widget<I>],
}

impl<'a, I: Copy + Debug> Widget<I> for VBox<'a, I> {
    fn measure(&self, limits: Size) -> Size {
        let mut size = Size::ZERO;
        for child in self.children {
            let child_size = child.measure(limits);
            size.width = size.width.max(child_size.width);
            size.height += child_size.height;
        }
        size.limit(limits)
    }

    fn render(
        &self,
        bounds: Rect,
        cursor: Point,
        surface: &mut dyn WriteSurface,
    ) -> Option<Hit<I>> {
        let mut height = 0;
        for child in self.children {
            height += child.measure(bounds.size).height;
        }
        let scale = bounds.size.height as f32 / height as f32;

        let mut hit = None;
        let mut origin = bounds.origin;
        for child in self.children {
            let size = child.measure(bounds.size);
            let height = scale * size.height as f32;
            let child_hit = child.render(
                Rect::new(origin, Size::new(size.width, height as Scalar)),
                cursor,
                surface,
            );
            origin.y += height as Scalar;
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
