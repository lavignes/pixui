use crate::{
    gfx::{BlendMode, Color, Point, Rect, Scalar, Size, Surface, WriteSurface},
    ui::{Hit, Widget},
};

const SLIDER_WIDTH: Scalar = 16;

struct WriteClipper<'a> {
    surface: &'a mut dyn WriteSurface,
    bounds: Rect,
}

impl<'a> Surface for WriteClipper<'a> {
    fn bounds(&self) -> Rect {
        self.surface.bounds()
    }
}

impl<'a> WriteSurface for WriteClipper<'a> {
    fn write(&mut self, point: Point, color: Color, blend: BlendMode) {
        // clip
        if !self.bounds.contains(point) {
            return;
        }
        self.surface.write(point, color, blend);
    }
}

#[derive(Default)]
pub struct Overflow<'a, I> {
    pub id: Option<I>,
    pub child: Option<&'a dyn Widget<I>>,
    pub offset: Point,
}

impl<'a, I: Copy> Widget<I> for Overflow<'a, I> {
    fn measure(&self, limits: Size) -> Size {
        if let Some(child) = self.child {
            child.measure(Size::new(limits.width, Scalar::MAX)) + Size::new(SLIDER_WIDTH, 0)
        } else {
            Size::new(SLIDER_WIDTH, 0)
        }
    }

    fn render(
        &self,
        bounds: Rect,
        cursor: Point,
        surface: &mut dyn WriteSurface,
    ) -> Option<Hit<I>> {
        let mut hit = None;
        let mut size = bounds.size;
        if let Some(child) = self.child {
            size = child.measure(Size::new(bounds.size.width, Scalar::MAX));
            let inner_bounds = Rect::new(bounds.origin - self.offset, size);
            // clip the cursor
            let cursor = if !bounds.inset(0, 0, 0, SLIDER_WIDTH).contains(cursor) {
                Point::MAX
            } else {
                cursor
            };
            hit = child.render(inner_bounds, cursor, &mut WriteClipper { surface, bounds });
        }

        surface.fill(
            Rect::new(
                (bounds.right() - SLIDER_WIDTH + 1, bounds.origin.y).into(),
                (SLIDER_WIDTH, bounds.size.height).into(),
            ),
            Color::opaque(32, 32, 32),
            BlendMode::None,
        );

        let scale = bounds.size.height as f32 / size.height as f32;
        let offset = (scale * self.offset.y as f32) as Scalar;
        surface.fill(
            Rect::new(
                (
                    bounds.right() - SLIDER_WIDTH + 1,
                    (bounds.origin.y + offset)
                        .min(bounds.bottom() - SLIDER_WIDTH)
                        .max(bounds.origin.y),
                )
                    .into(),
                (SLIDER_WIDTH, SLIDER_WIDTH).into(),
            ),
            Color::opaque(127, 127, 127),
            BlendMode::None,
        );

        if hit.is_some() {
            return hit;
        }
        Hit::from_test(self.id, bounds, cursor)
    }
}
