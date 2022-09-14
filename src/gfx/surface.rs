use std::borrow::{Borrow, BorrowMut};

use crate::gfx::{BlendMode, Color, Point, Rect, Scalar, Size};

pub trait Surface {
    fn bounds(&self) -> Rect;
}

pub trait ReadSurface: Surface {
    fn read(&self, point: Point) -> Option<Color>;
}

pub trait WriteSurface: Surface {
    fn write(&mut self, point: Point, color: Color, blend: BlendMode);

    fn line(&mut self, from: Point, to: Point, color: Color, blend: BlendMode) {
        let dx = (to.x - from.x).abs();
        let dy = (to.y - from.y).abs();
        let sx = if from.x < to.x { 1 } else { -1 };
        let sy = if from.y < to.y { 1 } else { -1 };
        let mut err = (if dx > dy { dx } else { -dy }) / 2;
        let mut point = from;
        loop {
            self.write(point, color, blend);
            if point == to {
                break;
            }
            let err2 = err;
            if err2 > -dx {
                err -= dy;
                point.x += sx;
            }
            if err2 < dy {
                err += dx;
                point.y += sy;
            }
        }
    }

    fn rect(&mut self, rect: Rect, color: Color, blend: BlendMode) {
        let top = rect.top();
        let bottom = rect.bottom();
        for y in top..=bottom {
            if y == top || y == bottom {
                for x in rect.left()..=rect.right() {
                    self.write((x, y).into(), color, blend);
                }
            } else {
                self.write((rect.left(), y).into(), color, blend);
                self.write((rect.right(), y).into(), color, blend);
            }
        }
    }

    fn fill(&mut self, rect: Rect, color: Color, blend: BlendMode) {
        for y in rect.top()..=rect.bottom() {
            for x in rect.left()..=rect.right() {
                self.write((x, y).into(), color, blend);
            }
        }
    }

    fn blit(&mut self, from: Rect, to: Point, src: &dyn ReadSurface, blend: BlendMode) {
        for y in from.top()..=from.bottom() {
            for x in from.left()..=from.right() {
                let point = (x, y).into();
                if let Some(color) = src.read(point) {
                    self.write(to + point, color, blend);
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SliceSurface<S> {
    slice: S,
    stride: Scalar,
    bounds: Rect,
}

pub type VecSurface = SliceSurface<Vec<Color>>;

impl VecSurface {
    #[inline]
    pub fn from_size(size: Size) -> Self {
        Self::new(
            vec![Color::BLACK; (size.width * size.height) as usize],
            size.width,
            Rect::sized(size),
        )
    }
}

impl<S> SliceSurface<S> {
    #[inline]
    pub const fn new(slice: S, stride: Scalar, bounds: Rect) -> Self {
        Self {
            slice,
            stride,
            bounds,
        }
    }
}

impl<S> Surface for SliceSurface<S>
where
    S: Borrow<[Color]>,
{
    #[inline]
    fn bounds(&self) -> Rect {
        self.bounds
    }
}

impl<S> ReadSurface for SliceSurface<S>
where
    S: Borrow<[Color]>,
{
    fn read(&self, point: Point) -> Option<Color> {
        let size = self.bounds.size;
        if point.x < 0 || point.x >= size.width {
            return None;
        }
        if point.y < 0 || point.y >= size.height {
            return None;
        }
        let point = self.bounds.origin + point;
        let offset = point.x + point.y * self.stride;
        Some(self.slice.borrow()[offset as usize])
    }
}

impl<S> WriteSurface for SliceSurface<S>
where
    S: BorrowMut<[Color]>,
{
    fn write(&mut self, point: Point, color: Color, blend: BlendMode) {
        let size = self.bounds.size;
        if point.x < 0 || point.x >= size.width {
            return;
        }
        if point.y < 0 || point.y >= size.height {
            return;
        }
        let point = self.bounds.origin + point;
        let offset = point.x + point.y * self.stride;
        let dst = &mut self.slice.borrow_mut()[offset as usize];
        *dst = blend.blend(color, *dst);
    }
}

#[derive(Copy, Clone)]
pub struct U8SliceSurface<S> {
    slice: S,
    stride: Scalar,
    bounds: Rect,
}

impl<S> U8SliceSurface<S> {
    #[inline]
    pub fn new(slice: S, stride: Scalar, bounds: Rect) -> Self {
        Self {
            slice,
            stride,
            bounds,
        }
    }
}

impl<S> Borrow<[u8]> for &U8SliceSurface<S>
where
    S: Borrow<[u8]>,
{
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.slice.borrow()
    }
}

impl<S> Surface for U8SliceSurface<S> {
    #[inline]
    fn bounds(&self) -> Rect {
        self.bounds
    }
}

impl<S> ReadSurface for U8SliceSurface<S>
where
    S: Borrow<[u8]>,
{
    #[inline]
    fn read(&self, point: Point) -> Option<Color> {
        let size = self.bounds.size;
        if point.x < 0 || point.x >= size.width {
            return None;
        }
        if point.y < 0 || point.y >= size.height {
            return None;
        }
        let point = self.bounds.origin + point;
        let offset = (point.x + point.y * self.stride) * 4;
        let slice = self.slice.borrow();
        Some(Color::new(
            slice[offset as usize + 0],
            slice[offset as usize + 1],
            slice[offset as usize + 2],
            slice[offset as usize + 3],
        ))
    }
}

impl<S> WriteSurface for U8SliceSurface<S>
where
    S: BorrowMut<[u8]>,
{
    #[inline]
    fn write(&mut self, point: Point, color: Color, blend: BlendMode) {
        let size = self.bounds.size;
        if point.x < 0 || point.x >= size.width {
            return;
        }
        if point.y < 0 || point.y >= size.height {
            return;
        }
        let point = self.bounds.origin + point;
        let offset = (point.x + point.y * self.stride) * 4;
        let slice = self.slice.borrow_mut();

        let red = &mut slice[offset as usize + 0];
        *red = blend.blend_channel(color.red, color.alpha, *red);
        let green = &mut slice[offset as usize + 1];
        *green = blend.blend_channel(color.green, color.alpha, *green);
        let blue = &mut slice[offset as usize + 2];
        *blue = blend.blend_channel(color.blue, color.alpha, *blue);
        let alpha = &mut slice[offset as usize + 3];
        *alpha = blend.blend_channel(color.alpha, color.alpha, *alpha);
    }
}
