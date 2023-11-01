use super::ray::Ray;
use super::vec3::{Point3, RGBColor};
use std::sync::Arc;
pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> RGBColor;
}
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
        let mut sines = (10.0 * p.x).sin() * (p.y * 10.0).sin() * (p.z * 10.0).sin();
        if (sines < 0.0) {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}
