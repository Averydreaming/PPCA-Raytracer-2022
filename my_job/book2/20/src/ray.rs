use super::material::Material;
use crate::bvh::BvhNode;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::sphere::Sphere;
use crate::vec3::{Point3, RGBColor, Vec3};
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
            if let Some(emitted) = rec.mat_ptr.emitted(rec.u, rec.v, rec.p) {
                if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(r, &rec) {
                    return emitted
                        + attenuation * Ray::ray_color(scattered, background, world, depth - 1);
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
