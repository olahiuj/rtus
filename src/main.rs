use std::ops::Add;

use camera::Camera;
use color::Color;
use image::{Image, Ppm};
use point::Point;
use rand::Rng;
use scene::Scene;
use sphere::Sphere;
use vec::Vec;

mod camera;
mod color;
mod image;
mod point;
mod ray;
mod scene;
mod shape;
mod sphere;
mod vec;

fn sample(camera: &Camera, scene: &Scene, u: f32, v: f32) -> Vec {
    let ray = camera.get_ray(u, v);
    scene
        .hit(ray, 0., f32::MAX)
        .map(|x| 0.5 * (x.n + Vec::from([1., 1., 1.])))
        .unwrap_or({
            let unit_direct = ray.direct.to_unit();
            let t = 0.5 * (unit_direct.y() + 1.);
            (1. - t) * Vec::from([1., 1., 1.]) + t * Vec::from([0.5, 0.7, 1.])
        })
}

fn main() {
    let camera = Camera::new();
    let mut image = Ppm::new();
    let mut scene = Scene::new();

    scene.push(Sphere::new(Point::from([0., 0., -1.]), 0.5));
    scene.push(Sphere::new(Point::from([0., -100.5, -1.]), 100.));

    const SAMPLES_PER_PIXEL: u32 = 100;
    let mut rng = rand::thread_rng();

    for j in 0..image.height {
        for i in 0..image.width {
            let color_vec = (0..SAMPLES_PER_PIXEL)
                .map(|_| {
                    let u = (i as f32 + rng.gen::<f32>()) / (image.width as f32 - 1.);
                    let v = (j as f32 + rng.gen::<f32>()) / (image.height as f32 - 1.);
                    sample(&camera, &scene, u, v)
                })
                .fold(Vec::new(), Add::add)
                / (SAMPLES_PER_PIXEL as f32);

            let color = Color::from(color_vec);
            image.plot(j, i, color);
        }
    }

    image.to_file("/tmp/chap05.ppm").expect("to_file err");
}
