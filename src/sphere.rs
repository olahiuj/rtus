use crate::{
    point::Point,
    ray::{HitRecord, Ray},
    shape::Shape,
};

pub struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direct * ray.direct;
        let b = oc * ray.direct;
        let c = oc * oc - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant < 0. {
            return None;
        }
        let t = (-b - discriminant.sqrt()) / a;
        if t > t_max || t < t_min {
            let t = (-b + discriminant.sqrt()) / a;
            if t > t_max || t < t_min {
                return None;
            }
        }
        let p = ray.at(t);
        Some(HitRecord::new(ray, t, (p - self.center) / self.radius))
    }
}
