use crate::{
    material::Material,
    point::Point,
    ray::{HitRecord, Ray},
};

pub trait Shape {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere<'a> {
    center: Point,
    radius: f32,
    material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new<'b: 'a>(center: Point, radius: f32, material: &'b dyn Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Shape for Sphere<'a> {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direct * ray.direct;
        let b = oc * ray.direct;
        let c = oc * oc - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant < 0. {
            return None;
        }
        let mut t = (-b - discriminant.sqrt()) / a;
        if t > t_max || t < t_min {
            t = (-b + discriminant.sqrt()) / a;
            if t > t_max || t < t_min {
                return None;
            }
        }
        let p = ray.at(t);
        Some(HitRecord::new(
            ray,
            t,
            (p - self.center) / self.radius,
            self.material,
        ))
    }
}
