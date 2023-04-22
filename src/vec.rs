use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy)]
pub struct Vec {
    coeff: [f32; 3],
}

impl Vec {
    pub fn new() -> Vec {
        Vec::default()
    }

    pub const fn new_const(coeff: [f32; 3]) -> Vec {
        Vec { coeff }
    }

    pub fn new_rand_unit_sphere() -> Vec {
        loop {
            let vec = Vec::from([
                rand::random::<f32>() * 2. - 1.,
                rand::random::<f32>() * 2. - 1.,
                rand::random::<f32>() * 2. - 1.,
            ]);
            if vec.len() <= 1. {
                break vec;
            }
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

    pub fn cross(&self, rhs: &Vec) -> Vec {
        Vec::from([
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        ])
    }

    pub fn reflect(v: Vec, n: Vec) -> Vec {
        v + 2. * (v * n).abs() * n
    }

    pub fn near_zero(&self) -> bool {
        self.len() < 1e-8
    }

    pub fn scale(&self, rhs: Vec) -> Vec {
        Vec::from([
            self.x().mul(rhs.x()),
            self.y().mul(rhs.y()),
            self.z().mul(rhs.z()),
        ])
    }

    pub fn len(&self) -> f32 {
        self.coeff.map(|x| x * x).iter().fold(0., f32::add).sqrt()
    }

    pub fn to_unit(self) -> Vec {
        Vec::from(self.coeff.map(|x| x / self.len()))
    }
}

impl From<[f32; 3]> for Vec {
    fn from(value: [f32; 3]) -> Self {
        Vec { coeff: value }
    }
}

impl Neg for Vec {
    type Output = Vec;
    fn neg(self) -> Self::Output {
        Vec::from(self.coeff.map(|x| -x))
    }
}

impl AddAssign<Vec> for Vec {
    fn add_assign(&mut self, rhs: Vec) {
        *self.x_mut() += rhs.x();
        *self.y_mut() += rhs.y();
        *self.z_mut() += rhs.z();
    }
}

impl SubAssign<Vec> for Vec {
    fn sub_assign(&mut self, rhs: Vec) {
        *self.x_mut() -= rhs.x();
        *self.y_mut() -= rhs.y();
        *self.z_mut() -= rhs.z();
    }
}

impl Mul<Vec> for Vec {
    type Output = f32;
    fn mul(self, rhs: Vec) -> Self::Output {
        let mut result = 0.;
        for i in 0..=2 {
            result += self.at(i) * rhs.at(i)
        }
        result
    }
}

impl Mul<f32> for Vec {
    type Output = Vec;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut vec = self;
        vec *= rhs;
        vec
    }
}

impl Div<f32> for Vec {
    type Output = Vec;
    fn div(self, rhs: f32) -> Self::Output {
        let mut vec = self;
        vec /= rhs;
        vec
    }
}

impl Mul<Vec> for f32 {
    type Output = Vec;
    fn mul(self, rhs: Vec) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for Vec {
    fn mul_assign(&mut self, rhs: f32) {
        self.coeff.iter_mut().for_each(|x| *x *= rhs);
    }
}

impl DivAssign<f32> for Vec {
    fn div_assign(&mut self, rhs: f32) {
        self.coeff.iter_mut().for_each(|x| *x /= rhs);
    }
}

impl Add<Vec> for Vec {
    type Output = Vec;
    fn add(self, rhs: Vec) -> Self::Output {
        let mut vec = self;
        vec += rhs;
        vec
    }
}

impl Sub<Vec> for Vec {
    type Output = Vec;
    fn sub(self, rhs: Vec) -> Self::Output {
        let mut vec = self;
        vec -= rhs;
        vec
    }
}

impl Display for Vec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.x(), self.y(), self.z())
    }
}

impl Default for Vec {
    fn default() -> Self {
        Vec {
            coeff: [0., 0., 0.],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op() {
        let v1 = Vec::from([1., 1., 4.]);
        let v2 = Vec::from([5., 1., 4.]);
        let v1_v2 = v2 - v1;
        assert_eq!(v1_v2.x(), 4.);
        assert_eq!(v1_v2.y(), 0.);
        assert_eq!(v1_v2.z(), 0.);
        assert_eq!((2. * v1).z(), 8.);
        assert_eq!((v1 / 2.).z(), 2.);
        assert_eq!(v1 * v2, 22.);

        let v2_v1 = v1 - v2;
        assert_eq!(v2_v1.x(), -4.);

        let vi = Vec::from([1., 1., -1.]);
        let n = Vec::from([0., 0., 1.]);
        let r = Vec::reflect(vi, n);
        assert_eq!(r.at(0), 1.);
        assert_eq!(r.at(1), 1.);
        assert_eq!(r.at(2), 1.);
    }

    #[test]
    fn test_len() {
        assert_eq!(Vec::from([1., 2., 2.]).len(), 3.);
        assert_eq!(Vec::from([3., 4., 12.]).len(), 13.);
    }

    #[test]
    fn test_unit() {
        let v = Vec::from([8., 9., 12.]);
        let v_unit = v.to_unit();
        assert_eq!(v_unit.x(), 8. / 17.);
        assert_eq!(v_unit.y(), 9. / 17.);
        assert_eq!(v_unit.z(), 12. / 17.);
    }
}
