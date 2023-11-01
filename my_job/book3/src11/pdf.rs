use crate::aabb::AABB;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::onb::Onb;
use crate::{ray::Ray, vec3::Point3, vec3::Vec3};
use rand::Rng;
use std::f64::consts::PI;
use std::sync::Arc;
pub trait Pdf: Send + Sync {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
pub struct cosine_pdf {
    uvw: Onb,
}
impl cosine_pdf {
    pub fn new(x: Vec3) -> Self {
        Self {
            uvw: Onb::build_from_w(x),
        }
    }
}
impl Pdf for cosine_pdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine = Vec3::dot(Vec3::unit_vector(direction), self.uvw.w());
        if cosine <= 0. {
            return 0.;
        } else {
            return cosine / PI;
        }
    }

    fn generate(&self) -> Vec3 {
        return self.uvw.local_vec(Vec3::random_cosine_direction());
    }
}

pub struct hittable_pdf {
    o: Point3,
    ptr: Arc<dyn Hittable>,
}
impl hittable_pdf {
    pub fn new(p: Arc<dyn Hittable>, origin: Point3) -> Self {
        Self { ptr: p, o: origin }
    }
}
impl Pdf for hittable_pdf {
    fn value(&self, direction: Vec3) -> f64 {
        return self.ptr.pdf_value(self.o, direction);
    }

    fn generate(&self) -> Vec3 {
        return self.ptr.random(self.o);
    }
}
pub struct mixture_pdf {
    p0: Arc<dyn Pdf>,
    p1: Arc<dyn Pdf>,
}
impl mixture_pdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self { p0, p1 }
    }
}

impl Pdf for mixture_pdf {
    fn generate(&self) -> Vec3 {
        if rand::thread_rng().gen_range(0.0..1.0) < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }
}
