use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::sphere::Sphere;
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
    pub fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> f64 {
        let oc = r.orig - center;
        let a = Vec3::dot(r.dir, r.dir);
        let b = 2.0 * Vec3::dot(oc, r.dir);
        let c = Vec3::dot(oc, oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        if (discriminant < 0.0) {
            return -1.0;
        } else {
            return (-b - discriminant.sqrt()) / (2. * a);
        }
    }
    pub fn ray_color(r: Ray, world: &HittableList) -> RGBColor {
        let mut rec: Option<HitRecord> = None;
        if let Some(rec) = world.hit(r, 0., f64::MAX) {
            return (rec.normal + RGBColor::new(1., 1., 1.)) * 0.5;
        } else {
            let unit_direction = Vec3::unit_vector(r.dir);
            let t = 0.5 * (unit_direction.y + 1.0);
            return RGBColor::new(1., 1., 1.) * (1. - t) + RGBColor::new(0.5, 0.7, 1.) * t;
        }
    }
}
