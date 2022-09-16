use crate::{gfx::Point, ui::Hit};

#[derive(Debug)]
pub struct DragHandler {
    translation: Point,
    touch_point: Option<Point>,
}

impl DragHandler {
    #[inline]
    pub fn new() -> Self {
        Self {
            translation: Point::ZERO,
            touch_point: None,
        }
    }

    #[inline]
    pub fn translation(&self) -> Point {
        self.translation
    }

    #[inline]
    pub fn is_held(&self) -> bool {
        self.touch_point.is_some()
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
            self.translation = cursor - point;
            true
        } else {
            false
        }
    }
}
