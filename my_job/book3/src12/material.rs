use crate::hittable;
use crate::onb::Onb;
use crate::pdf::{cosine_pdf, Pdf};
use crate::texture::{Solid_Color, Texture};
use crate::vec3::Point3;
use crate::{hittable::HitRecord, ray::Ray, vec3::RGBColor, vec3::Vec3};
use rand::Rng;
use std::f64::consts::PI;
use std::sync::Arc;
/*pub struct Material {
}

impl Material {

}
#[derive(Copy, Clone, Default)]
pub struct Material{
}*/

pub struct Scattered_Record {
    pub specular_ray: Option<Ray>,
    pub attenuation: RGBColor,
    pub pdf_ptr: Option<Arc<dyn Pdf>>,
}
impl Scattered_Record {
    pub fn new(
        specular_ray: Option<Ray>,
        attenuation: RGBColor,
        pdf_ptr: Option<Arc<dyn Pdf>>,
    ) -> Self {
        Self {
            specular_ray,
            attenuation,
            pdf_ptr,
        }
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<Scattered_Record> {
        return None;
    }
    fn scattering_pdf(&self, r_in: Ray, rec: HitRecord, scattered: Ray) -> Option<f64> {
        return Some(0.);
    }
    fn emitted(&self, rec: HitRecord, u: f64, v: f64, p: Point3) -> Option<RGBColor> {
        return Some(RGBColor::new(0., 0., 0.));
    }
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: RGBColor) -> Self {
        Self {
            albedo: Arc::new(Solid_Color::new(a.x, a.y, a.z)),
        }
    }
    pub fn new_arc(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<Scattered_Record> {
        /*let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered=Ray::new(rec.p, Vec3::unit_vector(scatter_direction), r_in.tm),
        return Some(
            (self.albedo.value(rec.u, rec.v, rec.p),
            scattered.clone();
            Vec3::dot(rec.normal,scatter.direction)/PI,)
        );
        let uvw = Onb::build_from_w(rec.normal);
        let direction = uvw.local_vec(Vec3::random_cosine_direction());
        let scattered = Ray::new(rec.p, Vec3::unit_vector(direction), r_in.tm);
        let alb = self.albedo.value(rec.u, rec.v, rec.p);
        let pdf = Vec3::dot(uvw.w(), scattered.dir) / PI;
        Some((alb, scattered, pdf))*/
        return Some(Scattered_Record::new(
            None,
            self.albedo.value(rec.u, rec.v, rec.p),
            Some(Arc::new(cosine_pdf::new(rec.normal))),
        ));
    }
    fn scattering_pdf(&self, r_in: Ray, rec: HitRecord, scattered: Ray) -> Option<f64> {
        let cosine = Vec3::dot(rec.normal, Vec3::unit_vector(scattered.dir));
        if (cosine < 0.) {
            return Some(0.);
        } else {
            return Some(cosine / PI);
        }
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
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<Scattered_Record> {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.dir), rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            0.,
        );
        return Some(Scattered_Record::new(Some(scattered), self.albedo, None));
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
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<Scattered_Record> {
        let attenuation = RGBColor::new(1.0, 1.0, 1.0);
        let mut refraction_ratio = self.ir;
        if rec.front_face {
            refraction_ratio = 1.0 / self.ir;
        }
        let unit_direction = Vec3::unit_vector(r_in.dir);
        let cos_theta = f64::min(Vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let mut cannot_refract = false;
        if refraction_ratio * sin_theta > 1.0 {
            cannot_refract = true;
        }
        let mut direction = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
        let mut rnd = rand::thread_rng();
        if (cannot_refract || self.reflectance(cos_theta, refraction_ratio) > rnd.gen()) {
            direction = Vec3::reflect(unit_direction, rec.normal);
        }
        let mut scattered = Ray::new(rec.p, direction, r_in.tm);
        return Some(Scattered_Record::new(
            Some(scattered),
            RGBColor::new(1., 1., 1.),
            None,
        ));
    }
}

pub struct Diffuse_Light {
    emit: Arc<dyn Texture>,
}
impl Diffuse_Light {
    pub fn new(c: RGBColor) -> Self {
        Self {
            emit: Arc::new(Solid_Color::new(c.x, c.y, c.z)),
        }
    }
    pub fn new_arc(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }
}
impl Material for Diffuse_Light {
    /*fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<Scattered_Record> {
        return None;
    }*/
    fn emitted(&self, rec: HitRecord, u: f64, v: f64, p: Point3) -> Option<RGBColor> {
        if rec.front_face {
            return Some(self.emit.value(u, v, p));
        } else {
            return Some(RGBColor::new(0., 0., 0.));
        }
    }
}

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(c: RGBColor) -> Self {
        Self {
            albedo: Arc::new(Solid_Color::new(c.x, c.y, c.z)),
        }
    }
    pub fn new_arc(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Isotropic {
    /* fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Scattered_Record)> {
        Some(Scattered_Record::new(
            Some(Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.tm)),
            self.albedo.value(rec.u, rec.v, rec.p),
            None,
        ))
    }*/
}
/*
pub trait Material: Send + Sync {
    // fn scatter(&self, _r_in: Ray, _rec: &HitRecord) -> Option<(Color, Ray, f64)> {
    //     None
    // }
    fn scatter(&self, _r_in: Ray, _rec: &HitRecord) -> Option<Scatter_Record> {
        None
    }
    fn scattering_pdf(&self, _r_in: Ray, _rec: &HitRecord, _scattered: Ray) -> f64 {
        0.
    }
    fn emitted(&self, _r_in: Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: Point3) -> RGBColor {
        RGBColor::new(0., 0., 0.)
    }
}

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    #[allow(dead_code)]
    pub fn new(a: RGBColor) -> Self {
        Self {
            albedo: Arc::new(Solid_Color::new(a.x, a.y, a.z)),
        }
    }
    pub fn new_arc(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    /*
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        /*
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction.to_unit(), r_in.tm); // XXX
        let alb = self.albedo.value(rec.u, rec.v, rec.p);
        let pdf = Vec3::dot(rec.normal, scattered.dir) / PI;
        Some((alb, scattered, pdf))
        */
        /*
        let direction = random_in_hemisphere(rec.normal);
        let scattered = Ray::new(rec.p, direction.to_unit(), r_in.tm); // XXX
        let alb = self.albedo.value(rec.u, rec.v, rec.p);
        let pdf = 0.5 / PI;
        Some((alb, scattered, pdf))
        */
        let uvw = Onb::build_from_w(rec.normal);
        let direction = uvw.local_vec(random_cosine_direction());
        let scattered = Ray::new(rec.p, direction.to_unit(), r_in.tm);
        let alb = self.albedo.value(rec.u, rec.v, rec.p);
        let pdf = Vec3::dot(uvw.w(), scattered.dir) / PI;
        Some((alb, scattered, pdf))
    }
    */
    fn scatter(&self, _r_in: Ray, rec: &HitRecord) -> Option<Scatter_Record> {
        Some(Scatter_Record::new(
            None,
            self.albedo.value(rec.u, rec.v, rec.p),
            Some(Arc::new(cosine_pdf::new(rec.normal))),
        ))
    }
    fn scattering_pdf(&self, _r_in: Ray, rec: &HitRecord, scattered: Ray) -> f64 {
        let cosine = Vec3::dot(rec.normal, Vec3::unit_vector(scattered.dir));
        if cosine < 0. {
            0.
        } else {
            cosine / PI
        }
    }
}

pub struct Metal {
    albedo: RGBColor,
    fuzz: f64,
}

impl Metal {
    #[allow(dead_code)]
    pub fn new(a: RGBColor, f: f64) -> Self {
        Self {
            albedo: a,
            fuzz: if f < 1. { f } else { 1. },
        }
    }
}

impl Material for Metal {
    /*
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        let reflected = reflect(r_in.dir.to_unit(), rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + random_in_unit_sphere() * self.fuzz,
            r_in.tm,
        );
        if Vec3::dot(scattered.dir, rec.normal) > 0. {
            Some((self.albedo, scattered, 0.))
        } else {
            None
        }
    }
    */
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<Scatter_Record> {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.dir), rec.normal);
        Some(Scatter_Record::new(
            Some(Ray::new(
                rec.p,
                reflected + Vec3::random_in_unit_sphere() * self.fuzz,
                0.,
            )),
            self.albedo,
            None,
        ))
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    #[allow(dead_code)]
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cos: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cos).powi(5)
    }
}

impl Material for Dielectric {
    /*
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.dir.to_unit();
        // let refracted = refract(unit_direction, rec.normal, refraction_ratio);
        let cos_theta = f64::min(Vec3::dot(-unit_direction, rec.normal), 1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;

        let mut rng = rand::thread_rng();
        let random_double: f64 = rng.gen_range(0.0..1.0);
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double
        {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };

        Some((
            Color::new(1., 1., 1.),
            Ray::new(rec.p, direction, r_in.tm),
            0.,
        ))
    }
    */
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<Scatter_Record> {
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };
        let unit_direction = Vec3::unit_vector(r_in.dir);
        // let refracted = refract(unit_direction, rec.normal, refraction_ratio);
        let cos_theta = f64::min(Vec3::dot(-unit_direction, rec.normal), 1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;

        let mut rng = rand::thread_rng();
        let random_double: f64 = rng.gen_range(0.0..1.0);
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double
        {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };
        Some(Scatter_Record::new(
            Some(Ray::new(rec.p, direction, r_in.tm)),
            RGBColor::new(1., 1., 1.),
            None,
        ))
    }
}

pub struct Diffuse_Light {
    emit: Arc<dyn Texture>,
}

impl Diffuse_Light {
    pub fn new(c: RGBColor) -> Self {
        Self {
            emit: Arc::new(Solid_Color::new(c.x, c.y, c.z)),
        }
    }
    #[allow(dead_code)]
    pub fn new_arc(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }
}

impl Material for Diffuse_Light {
    /*
    fn scatter(&self, _r_in: Ray, _rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        None
    }
    */
    fn emitted(&self, _r_in: Ray, rec: &HitRecord, u: f64, v: f64, p: Point3) -> RGBColor {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            RGBColor::new(0., 0., 0.)
        }
    }
}

pub struct Isotropic {
    #[allow(dead_code)]
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(c: RGBColor) -> Self {
        Self {
            albedo: Arc::new(Solid_Color::new(c.x, c.y, c.z)),
        }
    }
    pub fn new_arc(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Isotropic {
    /*
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        Some((
            self.albedo.value(rec.u, rec.v, rec.p),
            Ray::new(rec.p, random_in_unit_sphere(), r_in.tm),
            0.,
        ))
    }
    */
}

pub struct Scatter_Record {
    pub specular_ray: Option<Ray>,
    pub attenuation: RGBColor,
    pub pdf_ptr: Option<Arc<dyn Pdf>>,
}

impl Scatter_Record {
    pub fn new(
        specular_ray: Option<Ray>,
        attenuation: RGBColor,
        pdf_ptr: Option<Arc<dyn Pdf>>,
    ) -> Self {
        Self {
            specular_ray,
            attenuation,
            pdf_ptr,
        }
    }
}
*/
