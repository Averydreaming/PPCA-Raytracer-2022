use super::material::Material;
use crate::bvh::BvhNode;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::pdf::{cosine_pdf, hittable_pdf, mixture_pdf, Pdf};
use crate::sphere::Sphere;
use crate::vec3::{Point3, RGBColor, Vec3};
use rand::Rng;
use std::sync::Arc;
#[derive(Copy, Clone)]
pub struct Ray {
    pub dir: Point3,
    pub orig: Vec3,
    pub tm: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            orig: origin,
            dir: direction,
            tm: time,
        }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}

impl Ray {
    /*pub fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> f64 {
        let oc = r.orig - center;
        let a = Vec3::dot(r.dir, r.dir);
        let b = 2.0 * Vec3::dot(oc, r.dir);
        let c = Vec3::dot(oc, oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        if (discriminant < 0.0) {
            return -1.0;
        } else {
            return (-b - discriminant.sqrt()) / (2. * a);
        }
    }*/
    pub fn ray_color(
        r: Ray,
        background: RGBColor,
        world: &BvhNode,
        lights: Arc<dyn Hittable>,
        depth: i32,
    ) -> RGBColor {
        if (depth <= 0) {
            return RGBColor::new(0., 0., 0.);
        }
        let mut srec: Option<HitRecord> = None;
        if let Some(rec) = world.hit(r, 0.001, f64::MAX) {
            if let Some(emitted) = rec.mat_ptr.emitted(rec.clone(), rec.u, rec.v, rec.p) {
                if let Some(srec) = rec.mat_ptr.scatter(r, &rec) {
                    if let Some(specular) = srec.specular_ray {
                        return srec.attenuation
                            * Ray::ray_color(specular, background, world, lights, depth - 1);
                    }
                    let light_ptr = Arc::new(hittable_pdf::new(lights.clone(), rec.p));
                    //let p1 = Arc::new(cosine_pdf::new(rec.normal));
                    let p = mixture_pdf::new(light_ptr, srec.pdf_ptr.unwrap());
                    let scattered = Ray::new(rec.p, p.generate(), r.tm);
                    let pdf_val = p.value(scattered.dir);
                    if let Some(pdf1) = rec.mat_ptr.scattering_pdf(r, rec.clone(), scattered) {
                        return emitted
                            + srec.attenuation
                                * Ray::ray_color(scattered, background, world, lights, depth - 1)
                                * pdf1
                                / pdf_val;
                    } else {
                        return emitted;
                    }
                } else {
                    return emitted;
                }
            } else {
                return background;
            }
        } else {
            return background;
        }
    }
}
