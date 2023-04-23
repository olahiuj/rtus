use crate::{
    ray::{HitRecord, Ray},
    vec::Vec,
};

pub static CENTER_MATERIAL: Lambertian = Lambertian::new_const([0.1, 0.2, 0.5]);
pub static GROUND_MATERIAL: Lambertian = Lambertian::new_const([0.8, 0.8, 0.]);
// pub static LEFT_MATERIAL: Metal = Metal::new_const([0.8, 0.8, 0.8], 0.3);
pub static LEFT_MATERIAL: Dielectric = Dielectric::new_const(1.5);
pub static RIGHT_MATERIAL: Metal = Metal::new_const([0.8, 0.6, 0.2], 0.);

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

pub struct Dielectric {
    refract_index: f32,
}

impl Dielectric {
    pub const fn new_const(refract_index: f32) -> Dielectric {
        Dielectric { refract_index }
    }

    fn reflectance(cos_theta: f32, refract_ratio: f32) -> f32 {
        let r0 = (1. - refract_ratio) / (1. + refract_ratio);
        let r0 = r0 * r0;
        r0 + (1. - r0) * f32::powi(1. - cos_theta, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, record: HitRecord) -> Option<(Vec, Ray)> {
        let refract_ratio = if record.is_front {
            1. / self.refract_index
        } else {
            self.refract_index
        };
        let cos_theta = f32::min(-ray.direct.to_unit() * record.n, 1.);
        let sin_theta = f32::sqrt(1. - cos_theta * cos_theta);

        let emit = if refract_ratio * sin_theta > 1.
            || Self::reflectance(cos_theta, refract_ratio) > rand::random()
        {
            Vec::reflect(ray.direct.to_unit(), record.n)
        } else {
            Vec::refract(ray.direct.to_unit(), record.n, refract_ratio)
        };
        Some((Vec::from([1., 1., 1.]), Ray::from(record.p, emit)))
    }
}
