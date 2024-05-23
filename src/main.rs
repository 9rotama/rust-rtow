mod color;
mod vec3;

use crate::{color::write_color, color::Color};
use std::time;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut img = image::RgbImage::new(image_width, image_height);

    let start = time::Instant::now();

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        print!(
            "\rpixel remaining: {} (u_u)     ",
            (image_width * image_height) - ((x + 1) * (y + 1))
        );
        let r: f64 = x as f64 / (image_width - 1) as f64;
        let g: f64 = y as f64 / (image_height - 1) as f64;
        let b: f64 = 0.0;
        let color = Color::new([r, g, b]);

        write_color(color, pixel)
    }

    let end = start.elapsed();

    img.save("result.png").unwrap();
    println!("\ntook {} ms.", end.as_millis())
}
