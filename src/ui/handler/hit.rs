use crate::ui::Hit;

#[derive(Debug)]
pub struct HitHandler {
    hit: bool,
}

impl HitHandler {
    #[inline]
    pub fn new() -> Self {
        Self { hit: false }
    }

    #[inline]
    pub fn is_hit(&self) -> bool {
        self.hit
    }

    #[inline]
    pub fn handle<I: Copy + PartialEq>(&mut self, id: I, hit: Option<&Hit<I>>) -> bool {
        self.hit = false;
        if let Some(hit) = hit {
            self.hit = hit.id() == &id;
        }
        self.hit
    }
}
