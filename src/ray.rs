use crate::{material::Material, point::Point, vec::Vec};

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
    pub fn from(origin: Point, direct: Vec) -> Ray {
        Ray { origin, direct }
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + (t * self.direct)
    }
}

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Point,
    pub n: Vec,
    pub material: &'a dyn Material,
    is_front: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(ray: Ray, t: f32, outward_n: Vec, material: &dyn Material) -> HitRecord {
        let p = ray.at(t);
        let is_front = (ray.direct * outward_n) < 0.;
        let n = if is_front { outward_n } else { -outward_n };
        HitRecord {
            t,
            p,
            n,
            is_front,
            material,
        }
    }
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
