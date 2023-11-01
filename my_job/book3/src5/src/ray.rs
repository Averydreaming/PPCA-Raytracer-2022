use super::material::Material;
use crate::bvh::BvhNode;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::sphere::Sphere;
use crate::vec3::{Point3, RGBColor, Vec3};
use rand::Rng;
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
    pub fn ray_color(r: Ray, background: RGBColor, world: &BvhNode, depth: i32) -> RGBColor {
        if (depth <= 0) {
            return RGBColor::new(0., 0., 0.);
        }
        let mut rec: Option<HitRecord> = None;
        if let Some(rec) = world.hit(r, 0.001, f64::MAX) {
            if let Some(emitted) = rec.mat_ptr.emitted(rec.clone(), rec.u, rec.v, rec.p) {
                if let Some((attenuation, scattered, pdf)) = rec.mat_ptr.scatter(r, &rec) {
                    let mut rng = rand::thread_rng();
                    let on_light = Point3::new(
                        rng.gen_range(213.0..343.0),
                        554.,
                        rng.gen_range(227.0..332.0),
                    );
                    let to_light = on_light - rec.p;
                    let distance_squared = to_light.length_squared();
                    let to_light = Vec3::unit_vector(to_light);
                    if Vec3::dot(to_light, rec.normal) < 0. {
                        return emitted;
                    }
                    let light_area = (343. - 213.) * (332. - 227.);
                    let light_cosine = (to_light.y).abs();
                    if light_cosine < 0.000001 {
                        return emitted;
                    }
                    let pdf = distance_squared / (light_cosine * light_area);
                    let scattered = Ray::new(rec.p, to_light, r.tm);
                    if let Some(pdf1) = rec.mat_ptr.scattering_pdf(r, rec.clone(), scattered) {
                        return emitted
                            + attenuation
                                * Ray::ray_color(scattered, background, world, depth - 1)
                                * pdf1
                                / pdf;
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
