use crate::{
    gfx::{Point, Rect, Size, WriteSurface},
    ui::{Hit, Widget},
};

#[derive(Default)]
pub struct Stack<'a, I> {
    pub id: Option<I>,
    pub children: &'a [&'a dyn Widget<I>],
}

impl<'a, I: Copy> Widget<I> for Stack<'a, I> {
    fn measure(&self, limits: Size) -> Size {
        let mut max_size = Size::ZERO;
        for child in self.children {
            max_size = max_size.max(child.measure(limits));
        }
        max_size.limit(limits)
    }

    fn render(
        &self,
        bounds: Rect,
        cursor: Point,
        surface: &mut dyn WriteSurface,
    ) -> Option<Hit<I>> {
        let mut hit = None;
        for child in self.children {
            let child_hit = child.render(bounds, cursor, surface);
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
