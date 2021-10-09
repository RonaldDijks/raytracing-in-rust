use vec3::Vec3;

mod vec3;

fn write_color(Vec3(r, g, b): &Vec3) {
    let ir = (255.999 * r) as u8;
    let ig = (255.999 * g) as u8;
    let ib = (255.999 * b) as u8;
    println!("{} {} {}", ir, ig, ib);
}

fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}     ", j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;
            let color = Vec3(r, g, b);
            write_color(&color);
        }
    }
    eprint!("\nDone\n");
}
