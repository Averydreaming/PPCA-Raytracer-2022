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
        let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        if Vec3::dot(scattered.dir, rec.normal) > 0. {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
/*class dielectric : public material {
    public:
        dielectric(double index_of_refraction) : ir(index_of_refraction) {}

        virtual bool scatter(
            const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered
        ) const override {
            attenuation = color(1.0, 1.0, 1.0);
            double refraction_ratio = rec.front_face ? (1.0/ir) : ir;

            vec3 unit_direction = unit_vector(r_in.direction());
            vec3 refracted = refract(unit_direction, rec.normal, refraction_ratio);

            scattered = ray(rec.p, refracted);
            return true;
        }

    public:
        double ir; // Index of Refraction
}; */
pub struct Dielectric {
    ir: f64,
}
impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
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
        let mut refracted = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
        let mut scattered = Ray::new(rec.p, refracted);
        return Some((attenuation, scattered));
    }
}
