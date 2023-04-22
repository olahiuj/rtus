use crate::{
    ray::{HitRecord, Ray},
    vec::Vec,
};

pub static CENTER_MATERIAL: Lambertian = Lambertian::new_const([0.7, 0.3, 0.3]);
pub static GROUND_MATERIAL: Lambertian = Lambertian::new_const([0.8, 0.8, 0.]);
pub static LEFT_MATERIAL: Metal = Metal::new_const([0.8, 0.8, 0.8], 0.3);
pub static RIGHT_MATERIAL: Metal = Metal::new_const([0.8, 0.6, 0.2], 1.);

pub trait Material {
    fn scatter(&self, ray: Ray, record: HitRecord) -> Option<(Vec, Ray)>;
}

pub struct Lambertian {
    albedo: Vec,
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, record: HitRecord) -> Option<(Vec, Ray)> {
        let emit = record.n + Vec::new_rand_unit_sphere();
        if emit.near_zero() {
            None
        } else {
            Some((self.albedo, Ray::from(record.p, emit)))
        }
    }
}

impl Lambertian {
    pub const fn new_const(coeff: [f32; 3]) -> Self {
        Lambertian {
            albedo: Vec::new_const(coeff),
        }
    }
}

pub struct Metal {
    albedo: Vec,
    fuzz: f32,
}

impl Metal {
    pub const fn new_const(coeff: [f32; 3], fuzz: f32) -> Metal {
        Metal {
            albedo: Vec::new_const(coeff),
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, record: HitRecord) -> Option<(Vec, Ray)> {
        let emit =
            Vec::reflect(ray.direct.to_unit(), record.n) + Vec::new_rand_unit_sphere() * self.fuzz;
        if emit * record.n < 0. {
            None
        } else {
            Some((self.albedo, Ray::from(record.p, emit)))
        }
    }
}
