use std::fmt::Debug;

use crate::{
    gfx::{BlendMode, Color, Point, Rect, Scalar, Size, WriteSurface},
    ui::{Hit, Widget},
};

#[derive(Debug, Default)]
pub struct Border<'a, I> {
    pub id: Option<I>,
    pub child: Option<&'a dyn Widget<I>>,
    pub weight: Scalar,
    pub color: Color,
}

impl<'a, I: Copy + Debug> Widget<I> for Border<'a, I> {
    fn measure(&self, limits: Size) -> Size {
        let size = Size::new(self.weight, self.weight) * 2;
        if let Some(child) = self.child {
            let child_size = child.measure(limits - size);
            (child_size + size).limit(limits)
        } else {
            size.limit(limits)
        }
    }

    fn render(
        &self,
        bounds: Rect,
        cursor: Point,
        surface: &mut dyn WriteSurface,
    ) -> Option<Hit<I>> {
        surface.rect(bounds, self.color, BlendMode::Blend);
        if let Some(child) = self.child {
            let hit = child.render(
                bounds.inset(self.weight, self.weight, self.weight, self.weight),
                cursor,
                surface,
            );
            if hit.is_some() {
                return hit;
            }
        }
        Hit::from_test(self.id, bounds, cursor)
    }
}
