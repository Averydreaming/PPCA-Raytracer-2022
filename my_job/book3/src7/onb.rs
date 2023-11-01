use crate::aabb::AABB;
use crate::material::Material;
use crate::{ray::Ray, vec3::Point3, vec3::Vec3};
use std::sync::Arc;
pub struct Onb {
    pub axis: [Vec3; 3],
}
impl Onb {
    pub fn u(&self) -> Vec3 {
        return self.axis[0];
    }
    pub fn v(&self) -> Vec3 {
        return self.axis[1];
    }
    pub fn w(&self) -> Vec3 {
        return self.axis[2];
    }
    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        return self.u() * a + self.v() * b + self.w() * c;
    }
    pub fn local_vec(&self, a: Vec3) -> Vec3 {
        return self.u() * a.x + self.v() * a.y + self.w() * a.z;
    }
    pub fn build_from_w(n: Vec3) -> Self {
        let w = Vec3::unit_vector(n);
        let mut a = Vec3::new(0., 0., 0.);
        if (w.x.abs() > 0.9) {
            a = Vec3::new(0., 1., 0.);
        } else {
            a = Vec3::new(1., 0., 0.);
        }
        let v = Vec3::unit_vector(Vec3::cross(w, a));
        let u = Vec3::cross(w, v);
        Self { axis: [u, v, w] }
    }
}
