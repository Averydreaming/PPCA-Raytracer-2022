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
    pub fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> bool {
        let oc = r.orig - center;
        let a = Vec3::dot(r.dir, r.dir);
        let b = 2.0 * Vec3::dot(oc, r.dir);
        let c = Vec3::dot(oc, oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        if (discriminant > 0.0) {
            return true;
        } else {
            return false;
        }
    }
    pub fn ray_color(r: Ray) -> Vec3 {
        if (Ray::hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r) == true) {
            return RGBColor::new(1.0, 0.0, 0.0);
        }
        let unit_direction = Vec3::unit_vector(r.dir);
        let t = 0.5 * (unit_direction.y + 1.0);
        return RGBColor::new(1.0, 1.0, 1.0) * (1.0 - t) + RGBColor::new(0.5, 0.7, 1.0) * t;
    }
}
