use crate::{
    gfx::{BlendMode, Color, Rect, Scalar, Size, WriteSurface},
    ui::Widget,
};

#[derive(Debug, Default)]
pub struct Border<'a> {
    pub id: Option<&'a str>,
    pub child: Option<&'a dyn Widget>,
    pub weight: Scalar,
    pub color: Color,
}

impl<'a> Widget for Border<'a> {
    fn id(&self) -> Option<&str> {
        self.id
    }

    fn measure(&self, limits: Size) -> Size {
        let size = Size::new(self.weight, self.weight) * 2;
        if let Some(child) = self.child {
            let child_size = child.measure(limits - size);
            (child_size + size).limit(limits)
        } else {
            size.limit(limits)
        }
    }

    fn render(&self, bounds: Rect, surface: &mut dyn WriteSurface) {
        surface.rect(bounds, self.color, BlendMode::Blend);
        if let Some(child) = self.child {
            child.render(
                bounds.inset(self.weight, self.weight, self.weight, self.weight),
                surface,
            );
        }
    }
}
