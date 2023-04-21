use crate::{point::Point, ray::Ray, vec::Vec};

pub struct Camera {
    origin: Point,
    horizontal: Vec,
    vertical: Vec,
    lower_left: Vec,
}

impl Camera {
    const DEFAULT_FOCAL_DEPTH: f32 = 1.;

    pub fn new() -> Camera {
        Camera::default()
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::from(
            self.origin,
            self.lower_left + u * self.horizontal + v * self.vertical,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        let origin = Point::new();
        let horizontal = Vec::from([3.56, 0., 0.]);
        let vertical = Vec::from([0., 2., 0.]);
        let lower_left = (origin
            - horizontal / 2.
            - vertical / 2.
            - Vec::from([0., 0., Camera::DEFAULT_FOCAL_DEPTH]))
            - origin;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left,
        }
    }
}
