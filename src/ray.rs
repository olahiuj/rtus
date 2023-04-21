use crate::{point::Point, vec::Vec};

/* The struct for rays
 * A ray can be represented by giving its origin and its direction vector.
 * P is said to be on the Ray {origin, direct} iff exists t, P = origin + t * direct.
 */
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direct: Vec,
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            origin: Point::new(),
            direct: Vec::new(),
        }
    }

    pub fn from(origin: Point, direct: Vec) -> Ray {
        Ray { origin, direct }
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + (t * self.direct)
    }
}

pub struct HitRecord {
    pub t: f32,
    pub p: Point,
    pub n: Vec,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let ray = Ray::from(Point::from([0., 0., 0.]), Vec::from([1., 1., 1.]));
        assert_eq!(ray.at(0.).x(), 0.)
    }
}
