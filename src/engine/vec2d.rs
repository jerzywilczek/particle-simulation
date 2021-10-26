use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Clone, Copy)]
/// A struct implementing a 2d vector, which supports all basic operations
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Vec2d {
    /// Creates a new Vec2d
    pub fn new(x: f64, y: f64) -> Vec2d {
        Vec2d { x, y }
    }

    /// Calculates the length of a Vec2d
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Add for Vec2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2d::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Neg for Vec2d {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2d::new(-self.x, -self.y)
    }
}

impl Sub for Vec2d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl SubAssign for Vec2d {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl PartialEq for Vec2d {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f64 = 10e-12;
        (self.x - other.x).abs() < EPS && (self.y - other.y).abs() < EPS
    }
}

impl Debug for Vec2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entry(&self.x)
            .entry(&self.y)
            .finish()
    }
}

impl Mul<f64> for Vec2d {
    type Output = Vec2d;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2d::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Vec2d> for f64 {
    type Output = Vec2d;

    fn mul(self, rhs: Vec2d) -> Self::Output {
        return rhs * self;
    }
}

impl Div<f64> for Vec2d {
    type Output = Vec2d;

    fn div(self, rhs: f64) -> Self::Output {
        Vec2d::new(self.x / rhs, self.y / rhs)
    }
}

impl From<[f64; 2]> for Vec2d {
    fn from([x, y]: [f64; 2]) -> Self {
        Vec2d::new(x, y)
    }
}

impl From<Vec2d> for [f64; 2] {
    fn from(v: Vec2d) -> Self {
        [v.x, v.y]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn equals() {
        let v1 = Vec2d::new(5.0, 10.0);
        let v2 = Vec2d::new(5.0 + 10e-14, 10.0 + 10e-14);
        let v3 = Vec2d::new(6.0, 6.0);

        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }

    #[test]
    fn add() {
        let mut v1 = Vec2d::new(2.0, 3.0);
        let v2 = Vec2d::new(3.0, 2.0);
        let v3 = Vec2d::new(5.0, 5.0);

        assert_eq!(v1 + v2, v3);
        v1 += v2;
        assert_eq!(v1, v3);
    }

    #[test]
    fn sub() {
        let mut v1 = Vec2d::new(2.0, 3.0);
        let v2 = Vec2d::new(3.0, 2.0);
        let v3 = Vec2d::new(-1.0, 1.0);

        assert_eq!(v1 - v2, v3);
        v1 -= v2;
        assert_eq!(v1, v3);
    }

    #[test]
    fn len() {
        let v1 = Vec2d::new(3.0, 4.0);

        assert!((v1.len() - 5.0).abs() < 10e-12);
    }

    use crate::engine::Vec2d;
}