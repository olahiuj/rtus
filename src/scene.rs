use crate::{
    ray::{HitRecord, Ray},
    shape::Shape,
};

pub struct Scene {
    shapes: Vec<Box<dyn Shape>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { shapes: Vec::new() }
    }

    pub fn push<T: Shape + 'static>(&mut self, obj: T) {
        self.shapes.push(Box::from(obj));
    }

    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_point: Option<HitRecord> = None;
        let mut t_max = t_max;
        for obj in &self.shapes {
            if let Some(hp) = obj.hit(ray, t_min, t_max) {
                t_max = hp.t;
                hit_point = Some(hp);
            }
        }
        hit_point
    }
}
