use crate::{hittable::HitRecord, ray::Ray, vec3::Point3, vec3::RGBColor, vec3::Vec3};
use std::sync::Arc;

/*pub struct Material {
}

impl Material {

}
#[derive(Copy, Clone, Default)]
pub struct Material{
}*/
pub trait Material {
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
    fn scatter(&self, _r_in: Ray, rec: &HitRecord) -> Option<(RGBColor, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        return Some((self.albedo, Ray::new(rec.p, scatter_direction)));
    }
}

pub struct Metal {
    albedo: RGBColor,
}

impl Metal {
    pub fn new(a: RGBColor) -> Self {
        Self { albedo: a }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(RGBColor, Ray)> {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.dir), rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        if Vec3::dot(scattered.dir, rec.normal) > 0. {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
