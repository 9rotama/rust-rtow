fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut img = image::RgbImage::new(image_width, image_height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r: f64 = x as f64 / (image_width - 1) as f64;
        let g: f64 = y as f64 / (image_height - 1) as f64;
        let b: f64 = 0.0;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        *pixel = image::Rgb([ir, ig, ib])
    }
    img.save("result.png").unwrap();
}
