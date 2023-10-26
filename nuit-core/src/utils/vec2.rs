use serde::{Serialize, Deserialize};
use std::{fmt, ops::{Add, Sub, Mul, Div, DivAssign, MulAssign, AddAssign, SubAssign, Neg}};

use super::Zero;

/// A 2D vector (or position).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Zero for Vec2<T> where T: Zero {
    /// The coordinate origin or zero direction vector, i.e. (0, 0).
    const ZERO: Self = Self::new(T::ZERO, T::ZERO);
}

impl<T> Vec2<T> {
    /// Creates a new vector from the given cube components.
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Vec2<T> where T: Add<Output = T> + Mul<Output = T> + Copy {
    /// The squared length of this vector.
    #[inline]
    pub fn squared_length(self) -> T { self.x * self.x + self.y * self.y }
}

impl<T> Add for Vec2<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> AddAssign<Vec2<T>> for Vec2<T> where T: AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Vec2<T> where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> SubAssign<Vec2<T>> for Vec2<T> where T: SubAssign {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Mul<T> for Vec2<T> where T: Mul<Output = T> + Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T> MulAssign<T> for Vec2<T> where T: MulAssign + Copy {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> Div<T> for Vec2<T> where T: Div<Output = T> + Copy {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<T> DivAssign<T> for Vec2<T> where T: DivAssign + Copy {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T> Neg for Vec2<T> where T: Neg<Output = T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl<T> fmt::Display for Vec2<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
