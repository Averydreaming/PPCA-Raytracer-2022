use super::aabb::AABB;
use crate::vec3::{Point3, Vec3};
use crate::{hittable::HitRecord, hittable::Hittable, material::Material, ray::Ray};
use std::f64::consts::PI;
use std::sync::Arc;
pub struct xy_rect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mp: Arc<dyn Material>,
}
impl xy_rect {
    pub fn new(_x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64, _mp: Arc<dyn Material>) -> Self {
        Self {
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
            mp: _mp,
        }
    }
}
impl Hittable for xy_rect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.z) / r.dir.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x + t * r.dir.x;
        let y = r.orig.y + t * r.dir.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let outward_normal = Vec3::new(0., 0., 1.);
        let mut rec = HitRecord::new(
            r.at(t),
            outward_normal,
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
            true,
            self.mp.clone(),
        );

        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let output_box = AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        return Some(output_box);
    }
}
