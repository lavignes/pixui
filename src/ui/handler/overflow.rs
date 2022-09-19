use crate::{
    gfx::{Point, Size},
    ui::{Hit, Interceptor, Overflow, Widget},
};

#[derive(Debug)]
pub struct OverflowHandler {
    offset: Point,
    content_size: Size,
    touch_point: Option<Point>,
}

impl OverflowHandler {
    #[inline]
    pub fn new() -> Self {
        Self {
            offset: Point::ZERO,
            content_size: Size::ZERO,
            touch_point: None,
        }
    }

    #[inline]
    pub fn offset(&self) -> Point {
        self.offset
    }

    #[inline]
    pub fn is_held(&self) -> bool {
        self.touch_point.is_some()
    }

    pub fn measure<'a, I: Copy>(&'a mut self, child: &'a Overflow<'a, I>) -> impl Widget<I> + 'a {
        Interceptor::measure(
            |size, limit| {
                self.content_size = size;
                limit
            },
            child,
        )
    }

    pub fn handle<I: Copy + PartialEq>(
        &mut self,
        id: I,
        touch: bool,
        cursor: Point,
        hit: Option<&Hit<I>>,
    ) -> bool {
        if let Some(hit) = hit {
            if touch {
                if self.touch_point.is_none() && hit.id() == &id {
                    self.touch_point = Some(cursor - hit.bounds().origin);
                }
            } else {
                self.touch_point = None;
            }
        }
        if let Some(point) = self.touch_point {
            self.touch_point = Some(cursor);
            let offset = self.offset + (point - cursor);
            self.offset = Point::new(0, offset.y.max(0).min(self.content_size.height));
            true
        } else {
            false
        }
    }
}
