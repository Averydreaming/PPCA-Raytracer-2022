use super::aabb::AABB;
use crate::material::Isotropic;
use crate::texture::Texture;
use crate::vec3::{RGBColor, Vec3};
use crate::{hittable::HitRecord, hittable::Hittable, material::Material, ray::Ray};
use rand::Rng;
use std::f64::consts::E;
use std::f64::INFINITY;
use std::sync::Arc;
//恒定密度介质

pub struct Constant_Medium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}
impl Constant_Medium {
    pub fn new(b: Arc<dyn Hittable>, d: f64, c: RGBColor) -> Self {
        Self {
            boundary: b,
            phase_function: Arc::new(Isotropic::new(c)),
            neg_inv_density: -1. / d,
        }
    }
    pub fn new_arc(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> Self {
        Self {
            boundary: b,
            phase_function: Arc::new(Isotropic::new_arc(a)),
            neg_inv_density: -1. / d,
        }
    }
}

impl Hittable for Constant_Medium {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        return self.boundary.bounding_box(_time0, _time1);
    }
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(r, -INFINITY, INFINITY) {
            if let Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, INFINITY) {
                if (rec1.t < t_min) {
                    rec1.t = t_min;
                }
                if (rec2.t > t_max) {
                    rec2.t = t_max;
                }
                if (rec1.t >= rec2.t) {
                    return None;
                }
                if (rec1.t < 0.) {
                    rec1.t = 0.;
                }
                let ray_length = r.dir.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let mut rnd: f64 = rand::thread_rng().gen();
                let hit_distance = self.neg_inv_density * (rnd.log(E));
                if (hit_distance > distance_inside_boundary) {
                    return None;
                }
                let mut rec = HitRecord::new(
                    r.at(rec1.t.clone() + hit_distance.clone() / ray_length.clone()),
                    Vec3::new(1., 0., 0.),
                    rec1.t + hit_distance / ray_length,
                    0.,
                    0.,
                    true,
                    self.phase_function.clone(),
                );
                return Some(rec);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}
