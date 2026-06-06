use approx::AbsDiffEq;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn try_normalized(&self) -> Option<Self> {
        let length = self.len();
        if length == 0.0 {
            None
        } else {
            Some(Self::new(self.x / length, self.y / length))
        }
    }

    pub fn normalized(&self) -> Self {
        self.try_normalized().unwrap_or(Self::new(0.0, 0.0))
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self * rhs.x, self * rhs.y)
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self::new(self.x / scalar, self.y / scalar)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl AbsDiffEq for Vec2 {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.x.abs_diff_eq(&other.x, epsilon) && self.y.abs_diff_eq(&other.y, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_vec2_operations() {
        let v1 = Vec2::new(3.0, 4.0);
        let v2 = Vec2::new(1.0, 2.0);

        assert_abs_diff_eq!(v1 + v2, Vec2::new(4.0, 6.0));
        assert_abs_diff_eq!(v1 - v2, Vec2::new(2.0, 2.0));
        assert_abs_diff_eq!(v1 * 2.0, Vec2::new(6.0, 8.0));
        assert_abs_diff_eq!(2.0 * v1, Vec2::new(6.0, 8.0));
        assert_abs_diff_eq!(v1 / 2.0, Vec2::new(1.5, 2.0));
        assert_abs_diff_eq!(-v1, Vec2::new(-3.0, -4.0));
        assert_abs_diff_eq!(v1.len(), 5.0);
        assert_abs_diff_eq!(v1.dot(v2), 11.0);
        assert_abs_diff_eq!(v1.normalized().len(), 1.0);
        assert_eq!(Vec2::new(0.0, 0.0).try_normalized(), None);
    }
}
