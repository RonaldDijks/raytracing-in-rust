use hittable::*;
use ray::Ray;
use vec3::Vec3;

mod hittable;
mod ray;
mod vec3;

fn write_color(Vec3(r, g, b): &Vec3) {
    let ir = (255.999 * r) as u8;
    let ig = (255.999 * g) as u8;
    let ib = (255.999 * b) as u8;
    println!("{} {} {}", ir, ig, ib);
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

    // world
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)),
    ]);

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::zero();
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

    // render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}     ", j);
        for i in 0..image_width {
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);
            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let r = Ray::new(origin, direction);
            let color = ray_color(&r, &world);
            write_color(&color);
        }
    }
    eprint!("\nDone\n");
}
