use std::{
    fmt::Display,
    ops::{Add, AddAssign, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    coeff: [f32; 3],
}

impl Point {
    pub fn new() -> Point {
        Point {
            coeff: [0., 0., 0.],
        }
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        self.at_mut(0)
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        self.at_mut(1)
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        self.at_mut(2)
    }

    pub fn x(&self) -> f32 {
        self.at(0)
    }

    pub fn y(&self) -> f32 {
        self.at(1)
    }

    pub fn z(&self) -> f32 {
        self.at(2)
    }

    pub fn at_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.coeff[index]
    }

    pub fn at(&self, index: usize) -> f32 {
        self.coeff[index]
    }
}

impl From<[f32; 3]> for Point {
    fn from(value: [f32; 3]) -> Self {
        Point { coeff: value }
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self::Output {
        Point::from(self.coeff.map(|x| -x))
    }
}

impl AddAssign<Vec> for Point {
    fn add_assign(&mut self, rhs: Vec) {
        *self.x_mut() += rhs.x();
        *self.y_mut() += rhs.y();
        *self.z_mut() += rhs.z();
    }
}

impl SubAssign<Vec> for Point {
    fn sub_assign(&mut self, rhs: Vec) {
        *self.x_mut() -= rhs.x();
        *self.y_mut() -= rhs.y();
        *self.z_mut() -= rhs.z();
    }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, rhs: f32) {
        self.coeff.iter_mut().for_each(|x| *x *= rhs);
    }
}

impl DivAssign<f32> for Point {
    fn div_assign(&mut self, rhs: f32) {
        self.coeff.iter_mut().for_each(|x| *x /= rhs);
    }
}

impl Add<Vec> for Point {
    type Output = Point;
    fn add(self, rhs: Vec) -> Self::Output {
        let mut vec = self;
        vec += rhs;
        vec
    }
}

impl Sub<Vec> for Point {
    type Output = Point;
    fn sub(self, rhs: Vec) -> Self::Output {
        let mut vec = self;
        vec -= rhs;
        vec
    }
}

impl Sub<Point> for Point {
    type Output = Vec;
    fn sub(self, rhs: Point) -> Self::Output {
        let delta = self;
        Vec::from([
            delta.x() - rhs.x(),
            delta.y() - rhs.y(),
            delta.z() - rhs.z(),
        ])
    }
}

impl Mul<f32> for Point {
    type Output = Point;
    fn mul(self, rhs: f32) -> Self::Output {
        Point::from(self.coeff.map(|x| x * rhs))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.x(), self.y(), self.z())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op() {
        let v1 = Point::from([1., 1., 4.]);
        let v2 = Point::from([5., 1., 4.]);
        let v1_v2 = v2 - v1;
        assert_eq!(v1_v2.x(), 4.);
        assert_eq!(v1_v2.y(), 0.);
        assert_eq!(v1_v2.z(), 0.);
    }
}
