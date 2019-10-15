mod image;
mod ray;
mod sphere;
mod vec3;

use image::Image;
use image::RGB;
use ray::Ray;
use sphere::Sphere;
use std::time::Instant;
use vec3::Vec3;

#[allow(unused_assignments)]
fn main() {
    let w = 600; // width
    let h = 600; // height

    let white = Vec3::new(255.0, 255.0, 255.0); // white
    let black = Vec3::new(0.0, 0.0, 0.0); // black
    let color = Vec3::new(134.0, 248.0, 50.0); // RGB Color Values

    let sph = Sphere::new(Vec3::new(w as f32 * 0.5, h as f32 * 0.5, 100.0), 100.0); // object
    let light = Sphere::new(Vec3::new(w as f32 * 0.25, h as f32 * 0.75, 5.0), 50.0); // light

    let mut img = Image::new(w as u32, h as u32); // imag

    let time = Instant::now(); // time before starting
    for x in 0..w {
        for y in 0..h {
            let mut pixel_color = black;
            let r = Ray::new(Vec3::new(x as f32, y as f32, 0.0), Vec3::new(0.0, 0.0, 1.0));
            let (status, t) = sph.intersect(r);

            if status == true {
                let pi = r.o + r.d * t;
                let l = light.c - pi;
                let n = sph.normal(pi);
                let dt = Vec3::dot(l.normalize(), n.normalize());

                pixel_color = (color + white * dt) * 0.5;

                img.set_pixel(x as u32, y as u32, RGB::new(pixel_color));
            }
        }
    }
    let t2 = Instant::now(); // completion time

    img.write_file("output\\image.ppm")
        .expect("Could not write file");
    println!("Done {:?}", t2 - time);
}
