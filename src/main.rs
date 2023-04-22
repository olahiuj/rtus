use camera::Camera;
use color::Color;
use image::{Image, Ppm};
use point::Point;
use ray::Ray;
use scene::Scene;
use shape::Sphere;
use std::ops::Add;
use vec::Vec;

mod camera;
mod color;
mod image;
mod material;
mod point;
mod ray;
mod scene;
mod shape;
mod vec;

const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

fn sample(ray: Ray, scene: &Scene, depth: u32) -> Vec {
    if depth >= MAX_DEPTH {
        return Vec::new();
    }
    scene
        .hit(ray, 0.001, f32::MAX)
        .map(|record| {
            if let Some((attenuation, scattered_ray)) = record.material.scatter(ray, record) {
                sample(scattered_ray, scene, depth + 1).scale(attenuation)
            } else {
                Vec::from([0., 0., 0.])
            }
        })
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

    scene.push(Sphere::new(
        Point::from([0., 0., -1.]),
        0.5,
        &material::CENTER_MATERIAL,
    ));
    scene.push(Sphere::new(
        Point::from([0., -100.5, -1.]),
        100.,
        &material::GROUND_MATERIAL,
    ));
    scene.push(Sphere::new(
        Point::from([-1., 0., -1.]),
        0.5,
        &material::LEFT_MATERIAL,
    ));
    scene.push(Sphere::new(
        Point::from([1., 0., -1.]),
        0.5,
        &material::RIGHT_MATERIAL,
    ));

    for j in 0..image.height {
        for i in 0..image.width {
            let color_vec = (0..SAMPLES_PER_PIXEL)
                .map(|_| {
                    let u = (i as f32 + rand::random::<f32>()) / (image.width as f32 - 1.);
                    let v = (j as f32 + rand::random::<f32>()) / (image.height as f32 - 1.);
                    sample(camera.get_ray(u, v), &scene, 0)
                })
                .fold(Vec::new(), Add::add)
                / (SAMPLES_PER_PIXEL as f32);

            let color = Color::from(color_vec);
            image.plot(j, i, color);
        }
    }

    image.to_file("/tmp/fig.ppm").expect("to_file err");
}
