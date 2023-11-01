use crate::vec3::{Point3, RGBColor, Vec3};
use crate::{hittable::HitRecord, hittable::Hittable, ray::Ray};
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}
impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared(); //长度平方
        let half_b = Vec3::dot(oc, r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return None;
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a; //求出方程的根
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return None;
                }
            }

            let mut rec = HitRecord::new(r.at(root), Vec3::new(0., 0., 0.), root, true);
            let outward_normal = (rec.p - self.center) / self.radius;
            rec.set_face_normal(r, outward_normal);
            return Some(rec);
        }
    }
}
