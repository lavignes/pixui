#[derive(Copy, Clone, Debug)]
pub enum BlendMode {
    None,
    Blend,
}

impl BlendMode {
    #[inline]
    pub(crate) fn blend(&self, src: Color, dst: Color) -> Color {
        match self {
            Self::None => src,
            Self::Blend => {
                if src.alpha == 0 {
                    dst
                } else {
                    src
                }
            }
        }
    }

    #[inline]
    pub(crate) fn blend_channel(&self, src: u8, src_alpha: u8, dst: u8) -> u8 {
        match self {
            Self::None => src,
            Self::Blend => {
                if src_alpha == 0 {
                    dst
                } else {
                    src
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Color {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
    pub(crate) alpha: u8,
}

impl Default for Color {
    #[inline]
    fn default() -> Self {
        Color::BLACK
    }
}

impl Color {
    pub const BLACK: Self = Self::opaque(0, 0, 0);
    pub const WHITE: Self = Self::opaque(255, 255, 255);

    #[inline]
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    #[inline]
    pub const fn opaque(red: u8, green: u8, blue: u8) -> Self {
        Self::new(red, green, blue, 255)
    }

    #[inline]
    pub const fn red(&self) -> u8 {
        self.red
    }

    #[inline]
    pub const fn green(&self) -> u8 {
        self.green
    }

    #[inline]
    pub const fn blue(&self) -> u8 {
        self.blue
    }

    #[inline]
    pub const fn alpha(&self) -> u8 {
        self.alpha
    }
}
