use super::vec3::{Point3, RGBColor};
use crate::perlin::Perlin;
use image::math::utils::clamp;
use image::RgbImage;
use std::str;
use std::sync::Arc;
pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> RGBColor;
}
//Solid_Color
#[derive(Clone, Copy)]
pub struct Solid_Color {
    pub color_value: RGBColor,
}

impl Solid_Color {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self {
            color_value: RGBColor::new(a, b, c),
        }
    }
}

impl Texture for Solid_Color {
    fn value(&self, u: f64, v: f64, p: Point3) -> RGBColor {
        return self.color_value;
    }
}
// Checker_Texture
pub struct Checker_Texture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}
impl Checker_Texture {
    pub fn new(oddc: RGBColor, evenc: RGBColor) -> Self {
        Self {
            odd: Arc::new(Solid_Color::new(oddc.x, oddc.y, oddc.z)),
            even: Arc::new(Solid_Color::new(evenc.x, evenc.y, evenc.z)),
        }
    }
}
impl Texture for Checker_Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> RGBColor {
        let sines = (10.0 * p.x).sin() * (p.y * 10.0).sin() * (p.z * 10.0).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}
//Noise_Texture
pub struct Noise_Texture {
    pub noise: Perlin,
    pub sc: f64,
}
impl Noise_Texture {
    pub fn new(sc: f64) -> Self {
        Self {
            noise: Perlin::new(),
            sc: sc,
        }
    }
}
impl Texture for Noise_Texture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> RGBColor {
        return RGBColor::new(1., 1., 1.)
            * 0.5
            * (1.0 + (self.sc * p.z + 10.0 * self.noise.turb(p)).sin());
    }
}
//Image_Texture
/*pub struct Image_Texture {
    pub width: i32,
    pub height: i32,
    pub bytes_per_scanline: i32,
}
impl Image_Texture  {
    pub fn new(sc: f64) -> Self {
        Self {
            noise: Perlin::new(),
            sc: sc,
        }
    }
}
impl Texture for Image_Texture  {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> RGBColor {
        return RGBColor::new(1., 1., 1.)
            * 0.5
            * (1.0 + (self.sc * p.z + 10.0 * self.noise.turb(p)).sin());
    }
}*/

pub struct Image_Texture {
    pub width: i32,
    pub height: i32,
    pub bytes_per_scanline: i32,
    pub img: RgbImage,
}

impl Image_Texture {
    pub fn new(filename: &str) -> Self {
        Self {
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
            img: image::open(filename).expect("failed").to_rgb8(),
        }
    }
}
impl Texture for Image_Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> RGBColor {
        let u = f64::clamp(u, 0.0, 1.0);
        let v = 1.0 - f64::clamp(v, 0.0, 1.0);
        let mut i = (u * ((self.img.width()) as f64)) as i32;
        let mut j = (v * ((self.img.height()) as f64)) as i32;
        if i >= self.img.width() as i32 {
            i = self.img.width() as i32 - 1;
        }
        if j >= self.img.height() as i32 {
            j = self.img.height() as i32 - 1;
        }
        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i as u32, j as u32);
        return RGBColor::new(
            color_scale * (pixel[0] as f64),
            color_scale * (pixel[1] as f64),
            color_scale * (pixel[2] as f64),
        );
    }
}
