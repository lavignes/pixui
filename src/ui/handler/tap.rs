use crate::ui::Hit;

#[derive(Debug)]
pub struct TapHandler {
    held: bool,
    tapped: bool,
}

impl TapHandler {
    #[inline]
    pub fn new() -> Self {
        Self {
            held: false,
            tapped: false,
        }
    }

    #[inline]
    pub fn is_held(&self) -> bool {
        self.held
    }

    #[inline]
    pub fn is_tapped(&self) -> bool {
        self.tapped
    }

    pub fn handle<I: Copy + PartialEq>(
        &mut self,
        id: I,
        touch: bool,
        hit: Option<&Hit<I>>,
    ) -> bool {
        let held = self.held;
        self.tapped = false;
        self.held = false;
        if let Some(hit) = hit {
            if hit.id() == &id {
                if touch {
                    self.held = true;
                } else {
                    if held {
                        self.tapped = true;
                    }
                }
            }
        }
        self.tapped
    }
}
