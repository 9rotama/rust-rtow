mod color;
mod point3;
mod ray;
mod vec3;

use std::time;
use vec3::unit_vector;

use {color::write_color, color::Color, point3::Point3, ray::Ray, vec3::dot, vec3::Vec3};

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = center.clone() - r.origin();
    let a = dot(&(r.direction()), &(r.direction()));
    let b = -2.0 * dot(&(r.direction()), &oc);
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new([0.0, 0.0, -1.0]), 0.5, r) {
        return Color::new([1.0, 0.0, 0.0]);
    }
    let unit_direction = unit_vector(&r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::new([1.0, 1.0, 1.0]) + a * Color::new([0.5, 0.7, 1.0]);
}

fn main() {
    // calc image width & height
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 512;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new([0.0, 0.0, 0.0]);

    // viewport
    let viewport_u = Vec3::new([viewport_width, 0.0, 0.0]);
    let viewport_v = Vec3::new([0.0, -viewport_height, 0.0]);

    let pixel_delta_u = viewport_u.clone() / image_width as f64;
    let pixel_delta_v = viewport_v.clone() / image_height as f64;

    let viewport_upper_left = camera_center.clone()
        - Vec3::new([0.0, 0.0, focal_length])
        - viewport_u.clone() / 2.0
        - viewport_v.clone() / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u.clone() + pixel_delta_v.clone());

    let mut img = image::RgbImage::new(image_width, image_height);

    let start = time::Instant::now();

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        print!(
            "\rpixel remaining: {} (u_u)     ",
            (image_width * image_height) - ((x + 1) * (y + 1))
        );

        let pixel_center = pixel00_loc.clone()
            + (x as f64 * pixel_delta_u.clone())
            + (y as f64 * pixel_delta_v.clone());
        let ray_direction = pixel_center - camera_center.clone();

        let pixel_color = ray_color(&Ray::new(camera_center.clone(), ray_direction));

        write_color(pixel_color, pixel)
    }

    let end = start.elapsed();

    img.save("result.png").unwrap();
    println!("\ntook {} ms.", end.as_millis())
}
