use crate::{
    gfx::{Rect, Scalar, Size, WriteSurface},
    ui::Widget,
};

#[derive(Debug, Default)]
pub struct Margin<'a> {
    pub id: Option<&'a str>,
    pub child: Option<&'a dyn Widget>,
    pub top: Scalar,
    pub left: Scalar,
    pub bottom: Scalar,
    pub right: Scalar,
}

impl<'a> Widget for Margin<'a> {
    fn id(&self) -> Option<&str> {
        self.id
    }

    fn measure(&self, limits: Size) -> Size {
        let margin_size: Size = (self.left + self.right, self.top + self.bottom).into();
        if let Some(child) = self.child {
            let child_size = child.measure(Size::new(
                limits.width - margin_size.width,
                limits.height - margin_size.height,
            ));
            (child_size + margin_size).limit(limits)
        } else {
            margin_size.limit(limits)
        }
    }

    fn render(&self, bounds: Rect, surface: &mut dyn WriteSurface) {
        if let Some(child) = self.child {
            child.render(
                bounds.inset(self.top, self.left, self.bottom, self.right),
                surface,
            );
        }
    }
}
