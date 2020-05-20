use std::fs;

mod maths;
use maths::vector::Vector;
use maths::vector::normalise;

mod ray;
use ray::Ray;

fn radiance(ray_in: Ray) -> Vector {
    let unit_dir = normalise(ray_in.direction());
    let t: f64 = (unit_dir.y() + 1.0) * 0.5;

    Vector::new(0.0, 0.0, 0.0) * (1.0 - t) - Vector::new(0.5, 0.7, 1.0) * t
}

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * ASPECT_RATIO;
    let focal_length: f64 = 1.0;

    let origin = Vector::new(0.0, 0.0, 0.0);
    let horizontal = Vector::new(viewport_width, 0.0, 0.0);
    let vertical = Vector::new(0.0, viewport_height, 0.0);
    let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - Vector::new(0.0, 0.0, focal_length);
    
    let mut data = String::from(format!("P3\n{} {}\n255\n", IMAGE_HEIGHT, IMAGE_HEIGHT));
    for y in (0..IMAGE_HEIGHT).rev() {
        for x in 0..IMAGE_WIDTH {
            let u: f64 = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let v: f64 = y as f64 / (IMAGE_HEIGHT - 1) as f64;

            let r = Ray::new(&origin, &(lower_left + horizontal * u + vertical * v - origin));
            let colour = radiance(r) * 255.99;

            data = format!("{}{}\n", data, colour);
        }
    }

    fs::write("image.ppm", data).expect("Unable to write file");
}
