use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, Eq, Clone, Debug, Hash, Copy)]
pub struct Point2D {
    pub x: isize,
    pub y: isize,
}

pub const ZERO: Point2D = Point2D { x: 0, y: 0 };
pub const UP: Point2D = Point2D { x: 0, y: -1 };
pub const RIGHT: Point2D = Point2D { x: 1, y: 0 };
pub const DOWN: Point2D = Point2D { x: 0, y: 1 };
pub const LEFT: Point2D = Point2D { x: -1, y: 0 };

impl Point2D {
    pub fn rotate_clockwise(&self) -> Point2D {
        if self.eq(&UP) {
            RIGHT
        } else if self.eq(&RIGHT) {
            DOWN
        } else if self.eq(&DOWN) {
            LEFT
        } else if self.eq(&LEFT) {
            UP
        } else {
            panic!("unknown rotation vector")
        }
    }
}

impl Add<Self> for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Self> for Point2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<isize> for Point2D {
    type Output = Point2D;

    fn mul(self, scalar: isize) -> Self {
        Point2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<usize> for Point2D {
    type Output = Point2D;

    fn mul(self, scalar: usize) -> Self {
        let scalar: isize = scalar as isize;
        Point2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
