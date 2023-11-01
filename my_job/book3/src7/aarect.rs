use super::aabb::AABB;
use crate::vec3::{Point3, Vec3};
use crate::{hittable::HitRecord, hittable::Hittable, material::Material, ray::Ray};
use rand::Rng;
use std::f64::INFINITY;
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

pub struct xz_rect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mp: Arc<dyn Material>,
}
impl xz_rect {
    pub fn new(_x0: f64, _x1: f64, _z0: f64, _z1: f64, _k: f64, _mp: Arc<dyn Material>) -> Self {
        Self {
            x0: _x0,
            x1: _x1,
            z0: _z0,
            z1: _z1,
            k: _k,
            mp: _mp,
        }
    }
}
impl Hittable for xz_rect {
    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        if let Some(rec) = self.hit(Ray::new(o, v, 0.), 0.001, INFINITY) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let dis_sqr = rec.t * rec.t * v.length_squared();
            let cos = (Vec3::dot(v, rec.normal) / v.length()).abs();
            dis_sqr / (cos * area)
        } else {
            0.
        }
    }
    fn random(&self, o: Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let random_point = Point3::new(
            rng.gen_range(self.x0..self.x1),
            self.k,
            rng.gen_range(self.z0..self.z1),
        );
        return random_point - o;
    }
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.y) / r.dir.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x + t * r.dir.x;
        let z = r.orig.z + t * r.dir.z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = Vec3::new(0., 1., 0.);
        let mut rec = HitRecord::new(
            r.at(t),
            outward_normal,
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0),
            true,
            self.mp.clone(),
        );

        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let output_box = AABB::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        );
        return Some(output_box);
    }
}

pub struct yz_rect {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mp: Arc<dyn Material>,
}
impl yz_rect {
    pub fn new(_y0: f64, _y1: f64, _z0: f64, _z1: f64, _k: f64, _mp: Arc<dyn Material>) -> Self {
        Self {
            y0: _y0,
            y1: _y1,
            z0: _z0,
            z1: _z1,
            k: _k,
            mp: _mp,
        }
    }
}
impl Hittable for yz_rect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.x) / r.dir.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.orig.y + t * r.dir.y;
        let z = r.orig.z + t * r.dir.z;

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = Vec3::new(1., 0., 0.);
        let mut rec = HitRecord::new(
            r.at(t),
            outward_normal,
            t,
            (y - self.y0) / (self.y1 - self.y0),
            (z - self.z0) / (self.z1 - self.z0),
            true,
            self.mp.clone(),
        );

        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let output_box = AABB::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        );
        return Some(output_box);
    }
}
