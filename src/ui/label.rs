use crate::{
    gfx::{BlendMode, Color, Font, Rect, Size, WriteSurface},
    ui::Widget,
};

#[derive(Debug)]
pub struct Label<'a> {
    pub id: Option<&'a str>,
    pub font: Option<&'a dyn Font>,
    pub color: Color,
    pub text: &'a str,
}

impl<'a> Default for Label<'a> {
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

impl<'a> Widget for Label<'a> {
    fn id(&self) -> Option<&str> {
        self.id
    }

    fn measure(&self, limits: Size) -> Size {
        if let Some(font) = self.font {
            font.measure(limits, self.text).limit(limits)
        } else {
            Size::ZERO
        }
    }

    fn render(&self, bounds: Rect, surface: &mut dyn WriteSurface) {
        if let Some(font) = self.font {
            font.render(bounds, self.text, surface, self.color, BlendMode::Blend)
        }
    }
}
