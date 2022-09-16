use std::cell::RefCell;

use crate::{
    gfx::{BlendMode, Color, Font, Point, Rect, Size, WriteSurface},
    ui::{Hit, Widget},
};

pub struct Label<'a, I> {
    pub id: Option<I>,
    pub font: Option<&'a dyn Font>,
    pub color: Color,
    pub text: &'a str,
}

impl<'a, I> Default for Label<'a, I> {
    #[inline]
    fn default() -> Self {
        Self {
            id: None,
            font: None,
            color: Color::BLACK,
            text: "",
        }
    }
}

impl<'a, I: Copy> Widget<I> for Label<'a, I> {
    fn measure(&self, limits: Size) -> Size {
        if let Some(font) = self.font {
            font.measure(limits, self.text)
        } else {
            Size::ZERO
        }
    }

    fn render(
        &self,
        bounds: Rect,
        cursor: Point,
        surface: &mut dyn WriteSurface,
    ) -> Option<Hit<I>> {
        if let Some(font) = self.font {
            font.render(bounds, self.text, surface, self.color, BlendMode::Blend)
        }
        Hit::from_test(self.id, bounds, cursor)
    }
}
