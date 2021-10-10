mod camera;
mod hittable;
mod ray;
mod vec3;

use camera::Camera;
use hittable::*;
use rand::Rng;
use ray::Ray;
use vec3::Vec3;

fn write_color(pixel_color: &Vec3, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let Vec3(r, g, b) = pixel_color * scale;
    let r = (r.clamp(0.0, 0.999) * 256.0) as u8;
    let g = (g.clamp(0.0, 0.999) * 256.0) as u8;
    let b = (b.clamp(0.0, 0.999) * 256.0) as u8;
    println!("{} {} {}", r, g, b);
}

fn ray_color(ray: &Ray, hittable: &dyn Hittable) -> Vec3 {
    if let Some(rec) = hittable.hit(ray, 0.0, f64::INFINITY) {
        return 0.5 * (rec.normal.normal + Vec3::one());
    }

    let t = 0.5 * (ray.direction.unit().1 + 1.0);
    (1.0 - t) * Vec3::one() + t * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // world
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)),
    ]);

    // camera
    let camera = Camera::default();

    let mut rng = rand::thread_rng();

    // render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}     ", j);
        for i in 0..image_width {
            let mut pixel_color = Vec3::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }
            write_color(&pixel_color, samples_per_pixel);
        }
    }
    eprint!("\nDone\n");
}
