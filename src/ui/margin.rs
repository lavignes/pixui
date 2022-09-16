use crate::{
    gfx::{Point, Rect, Scalar, Size, WriteSurface},
    ui::{Hit, Widget},
};

#[derive(Default)]
pub struct Margin<'a, I> {
    pub id: Option<I>,
    pub child: Option<&'a dyn Widget<I>>,
    pub top: Scalar,
    pub left: Scalar,
    pub bottom: Scalar,
    pub right: Scalar,
}

impl<'a, I: Copy> Widget<I> for Margin<'a, I> {
    fn measure(&self, limits: Size) -> Size {
        let size: Size = (self.left + self.right, self.top + self.bottom).into();
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
        if let Some(child) = self.child {
            if let Some(hit) = child.render(
                bounds.inset(self.top, self.left, self.bottom, self.right),
                cursor,
                surface,
            ) {
                return Some(hit);
            }
        }
        Hit::from_test(self.id, bounds, cursor)
    }
}
