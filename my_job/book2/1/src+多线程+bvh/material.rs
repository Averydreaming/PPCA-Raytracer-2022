use crate::{hittable::HitRecord, ray::Ray, vec3::Point3, vec3::RGBColor, vec3::Vec3};
use rand::Rng;
use std::sync::Arc;
/*pub struct Material {
}

impl Material {

}
#[derive(Copy, Clone, Default)]
pub struct Material{
}*/
pub trait Material: Send + Sync {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(RGBColor, Ray)>;
}

pub struct Lambertian {
    pub albedo: RGBColor,
}

impl Lambertian {
    pub fn new(a: RGBColor) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(RGBColor, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        return Some((self.albedo, Ray::new(rec.p, scatter_direction, r_in.tm)));
    }
}

pub struct Metal {
    albedo: RGBColor,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: RGBColor, mut f: f64) -> Self {
        if (f > 1.0) {
            f = 1.0;
        }
        Self { albedo: a, fuzz: f }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(RGBColor, Ray)> {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.dir), rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            r_in.tm,
        );
        if Vec3::dot(scattered.dir, rec.normal) > 0. {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
pub struct Dielectric {
    ir: f64,
}
impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }
    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        return r0 + (1.0 - cosine).powi(5) * (1.0 - r0);
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(RGBColor, Ray)> {
        let mut attenuation = RGBColor::new(1.0, 1.0, 1.0);
        let mut refraction_ratio = self.ir;
        if (rec.front_face) {
            refraction_ratio = 1.0 / self.ir;
        }
        let mut unit_direction = Vec3::unit_vector(r_in.dir);
        let mut cos_theta = f64::min(Vec3::dot(-unit_direction, rec.normal), 1.0);
        let mut sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let mut cannot_refract = false;
        if (refraction_ratio * sin_theta > 1.0) {
            cannot_refract = true;
        }
        let mut direction = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
        let mut rnd = rand::thread_rng();
        if (cannot_refract || self.reflectance(cos_theta, refraction_ratio) > rnd.gen()) {
            direction = Vec3::reflect(unit_direction, rec.normal);
        }
        let mut scattered = Ray::new(rec.p, direction, r_in.tm);
        return Some((attenuation, scattered));
    }
}
