use crate::vec3::Vec3;
use image::Rgb;

pub type Color = Vec3;

impl Color {
    pub fn r(&self) -> f64 {
        self.e[0]
    }

    pub fn g(&self) -> f64 {
        self.e[1]
    }

    pub fn b(&self) -> f64 {
        self.e[2]
    }
}

pub fn write_color(color: Color, pixel: &mut Rgb<u8>) {
    let ir = (255.999 * color.x()) as u8;
    let ig = (255.999 * color.y()) as u8;
    let ib = (255.999 * color.z()) as u8;
    *pixel = image::Rgb([ir, ig, ib])
}
