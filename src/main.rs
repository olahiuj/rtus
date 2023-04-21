mod color;
mod image;
mod point;
mod ray;
mod scene;
mod shape;
mod sphere;
mod vec;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::image::{Image, Ppm};
    use crate::{color::Color, point::Point, ray::Ray, scene::Scene, sphere::Sphere, vec::Vec};

    #[test]
    fn test_chap05() {
        let mut image = Ppm::new();

        let origin = Point::from([0., 0., 0.]);
        let lower_left = Vec::from([-2., -1., -1.]);
        let horizontal = Vec::from([4., 0., 0.]);
        let vertical = Vec::from([0., 2., 0.]);

        let mut scene = Scene::new();
        scene.push(Sphere::new(Point::from([0., 0., -1.]), 0.5));
        scene.push(Sphere::new(Point::from([0., -100.5, -1.]), 100.));

        for j in 0..image.height {
            for i in 0..image.width {
                let u = (i as f32) / (image.width as f32);
                let v = (j as f32) / (image.height as f32);
                let ray = Ray::from(origin, lower_left + u * horizontal + v * vertical);

                let color_vec = scene
                    .hit(ray, 0., f32::MAX)
                    .map(|x| 0.5 * (x.n + Vec::from([1., 1., 1.])))
                    .unwrap_or({
                        let unit_direct = ray.direct.to_unit();
                        let t = 0.5 * (unit_direct.y() + 1.);
                        (1. - t) * Vec::from([1., 1., 1.]) + t * Vec::from([0.5, 0.7, 1.])
                    });

                let color = Color::from(color_vec);
                image.plot(j as usize, i as usize, color);
            }
        }

        assert_eq!(image.height, 100);
        assert_eq!(image.width, 200);

        image.to_file("/tmp/chap05.ppm").expect("to_file err");
    }
}
