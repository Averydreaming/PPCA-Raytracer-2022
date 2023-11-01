use crate::vec3::{Point3, RGBColor, Vec3};

#[derive(Copy, Clone)]
pub struct Ray {
    pub dir: Point3,
    pub orig: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}

impl Ray {
    pub fn ray_color(r: Ray) -> Vec3 {
        let unit_direction = Vec3::unit_vector(r.dir);
        let t = 0.5 * (unit_direction.y + 1.0);
        return RGBColor::new(1.0, 1.0, 1.0) * (1.0 - t) + RGBColor::new(0.5, 0.7, 1.0) * t;
    }
}
