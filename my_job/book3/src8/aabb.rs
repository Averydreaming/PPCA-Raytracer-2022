use super::hittable::HitRecord;
use super::ray::Ray;
use super::vec3::Point3;
#[derive(Clone, Copy)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }

    pub fn hit(&self, r: Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..3 {
            let d = 1. / r.dir[i];
            let mut t0 = (self.minimum[i] - r.orig[i]) * d;
            let mut t1 = (self.maximum[i] - r.orig[i]) * d;
            if d < 0. {
                std::mem::swap(&mut t0, &mut t1);
            }
            if t0 > t_min {
                t_min = t0;
            }
            if t1 < t_max {
                t_max = t1;
            }
            if t_max <= t_min {
                return false;
            }
        }
        return true;
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        let small = Point3::new(
            f64::min(box0.minimum.x, box1.minimum.x),
            f64::min(box0.minimum.y, box1.minimum.y),
            f64::min(box0.minimum.z, box1.minimum.z),
        );
        let large = Point3::new(
            f64::max(box0.maximum.x, box1.maximum.x),
            f64::max(box0.maximum.y, box1.maximum.y),
            f64::max(box0.maximum.z, box1.maximum.z),
        );
        AABB::new(small, large)
    }
}
