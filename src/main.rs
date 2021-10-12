mod camera;
mod hittable;
mod ray;
mod vec3;

use camera::Camera;
use hittable::*;
use ray::Ray;
use vec3::Vec3;

use rand::Rng;

impl Vec3 {
    pub fn format_color(&self, samples_per_pixel: u32) -> String {
        let ir = (256.0 * (self.0 / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self.1 / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self.2 / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        format!("{} {} {}", ir, ig, ib)
    }
}

fn ray_color(ray: &Ray, hittable: &dyn Hittable, depth: u64) -> Vec3 {
    if depth == 0 {
        Vec3::zero()
    } else if let Some(rec) = hittable.hit(ray, 0.001, f64::INFINITY) {
        let target = rec.position + rec.normal + Vec3::random_in_unit_sphere().normalized();
        let ray = Ray::new(rec.position, target - rec.position);
        0.5 * ray_color(&ray, hittable, depth - 1)
    } else {
        let unit_direction = ray.direction.normalized();
        let t = 0.5 * (unit_direction.1 + 1.0);
        (1.0 - t) * Vec3::one() + t * Vec3(0.5, 0.7, 1.0)
    }

}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 5;

    //rng
    let mut rng = rand::thread_rng();

    // world
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)),
    ]);

    // camera
    let camera = Camera::default();


    // render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}     ", j);
        for i in 0..image_width {
            let mut pixel_color = Vec3::zero();
            for _ in 0..samples_per_pixel {
                let u: f64 = rng.gen();
                let v: f64 = rng.gen();

                let u = (i as f64 + u) / (image_width - 1) as f64;
                let v = (j as f64 + v) / (image_height - 1) as f64;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }
            println!("{}", pixel_color.format_color(samples_per_pixel));
        }
    }
    eprint!("\nDone\n");
}
