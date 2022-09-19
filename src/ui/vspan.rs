use crate::{
    gfx::{Point, Rect, Size, WriteSurface},
    ui::{Hit, Widget},
};

#[derive(Default)]
pub struct VSpan<'a, I> {
    pub id: Option<I>,
    pub child: Option<&'a dyn Widget<I>>,
}

impl<'a, I: Copy> Widget<I> for VSpan<'a, I> {
    fn measure(&self, limits: Size) -> Size {
        let mut width = 0;
        if let Some(child) = self.child {
            width = child.measure(limits).width;
        }
        Size::new(width, limits.height)
    }

    fn render(
        &self,
        bounds: Rect,
        cursor: Point,
        surface: &mut dyn WriteSurface,
    ) -> Option<Hit<I>> {
        if let Some(child) = self.child {
            let hit = child.render(bounds, cursor, surface);
            if hit.is_some() {
                return hit;
            }
        }
        Hit::from_test(self.id, bounds, cursor)
    }
}
