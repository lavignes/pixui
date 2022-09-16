use std::ops::{Add, Div, Mul, Sub};

pub type Scalar = i32;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: Scalar,
    pub y: Scalar,
}

impl Point {
    pub const ZERO: Self = Self::new(0, 0);

    #[inline]
    pub const fn new(x: Scalar, y: Scalar) -> Self {
        Self { x, y }
    }
}

impl<T: Into<Scalar>, U: Into<Scalar>> From<(T, U)> for Point {
    #[inline]
    fn from(tuple: (T, U)) -> Self {
        Self::new(tuple.0.into(), tuple.1.into())
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Mul<T> for Point
where
    T: Into<Scalar> + Copy,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs.into(), self.y * rhs.into())
    }
}

impl<T> Div<T> for Point
where
    T: Into<Scalar> + Copy,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs.into(), self.y / rhs.into())
    }
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct Size {
    pub(crate) width: Scalar,
    pub(crate) height: Scalar,
}

impl Size {
    pub const ZERO: Self = Self::new(0, 0);
    pub const MAX: Self = Self::new(Scalar::MAX, Scalar::MAX);

    #[inline]
    pub const fn new(width: Scalar, height: Scalar) -> Self {
        Self { width, height }
    }

    #[inline]
    pub const fn width(&self) -> Scalar {
        self.width
    }

    #[inline]
    pub const fn height(&self) -> Scalar {
        self.height
    }

    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.width == 0 && self.height == 0
    }

    #[inline]
    pub fn limit(&self, size: Self) -> Self {
        Self {
            width: self.width.min(size.width),
            height: self.height.min(size.height),
        }
    }

    #[inline]
    pub fn max(&self, size: Self) -> Self {
        Self {
            width: self.width.max(size.width),
            height: self.height.max(size.height),
        }
    }
}

impl<T: Into<Scalar>, U: Into<Scalar>> From<(T, U)> for Size {
    #[inline]
    fn from(tuple: (T, U)) -> Self {
        Self::new(tuple.0.into(), tuple.1.into())
    }
}

impl Add for Size {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl Sub for Size {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.width - rhs.width, self.height - rhs.height)
    }
}

impl<T> Mul<T> for Size
where
    T: Into<Scalar> + Copy,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.width * rhs.into(), self.height * rhs.into())
    }
}

impl<T> Div<T> for Size
where
    T: Into<Scalar> + Copy,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.width / rhs.into(), self.height / rhs.into())
    }
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub const ZERO: Self = Self::sized(Size::ZERO);

    #[inline]
    pub const fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    #[inline]
    pub const fn sized(size: Size) -> Self {
        Self::new(Point::ZERO, size)
    }

    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.size.is_zero()
    }

    #[inline]
    pub const fn width(&self) -> Scalar {
        self.size.width
    }

    #[inline]
    pub const fn height(&self) -> Scalar {
        self.size.height
    }

    #[inline]
    pub const fn top(&self) -> Scalar {
        self.origin.y
    }

    #[inline]
    pub const fn left(&self) -> Scalar {
        self.origin.x
    }

    #[inline]
    pub const fn bottom(&self) -> Scalar {
        self.origin.y
            + (if (self.size.height - 1) < 0 {
                0
            } else {
                self.size.height - 1
            })
    }

    #[inline]
    pub const fn right(&self) -> Scalar {
        self.origin.x
            + (if (self.size.width - 1) < 0 {
                0
            } else {
                self.size.width - 1
            })
    }

    #[inline]
    pub const fn inset(&self, top: Scalar, left: Scalar, bottom: Scalar, right: Scalar) -> Self {
        Self::new(
            Point::new(self.origin.x + left, self.origin.y + top),
            Size::new(
                self.size.width - (left + right),
                self.size.height - (top + bottom),
            ),
        )
    }

    #[inline]
    pub const fn contains(&self, point: Point) -> bool {
        if point.x < self.origin.x || point.x > self.right() {
            return false;
        }
        if point.y < self.origin.y || point.y > self.bottom() {
            return false;
        }
        true
    }
}

impl<S: Into<Scalar>, T: Into<Scalar>, U: Into<Scalar>, V: Into<Scalar>> From<(S, T, U, V)>
    for Rect
{
    #[inline]
    fn from(tuple: (S, T, U, V)) -> Self {
        Self {
            origin: Point {
                x: tuple.0.into(),
                y: tuple.1.into(),
            },
            size: Size {
                width: tuple.2.into(),
                height: tuple.3.into(),
            },
        }
    }
}
