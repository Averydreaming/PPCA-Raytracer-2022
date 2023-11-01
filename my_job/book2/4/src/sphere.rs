use super::aabb::AABB;
use crate::vec3::{Point3, Vec3};
use crate::{hittable::HitRecord, hittable::Hittable, material::Material, ray::Ray};
use std::f64::consts::PI;
use std::sync::Arc;
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}
impl Sphere {
    pub fn new(_center: Point3, _radius: f64, _mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            center: _center,
            radius: _radius,
            mat_ptr: _mat_ptr,
        }
    }
    fn get_sphere_uv(&self, p: Point3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = f64::atan2(-p.z, p.x) + PI;
        let mut u = phi / (2.0 * PI);
        let mut v = theta / PI;
        return (u, v);
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
            let outward_normal = (r.at(root) - self.center) / self.radius;
            let (u, v) = self.get_sphere_uv(outward_normal);
            let mut rec = HitRecord::new(
                r.at(root),
                Vec3::new(0., 0., 0.),
                root,
                u,
                v,
                true,
                self.mat_ptr.clone(),
            );

            rec.set_face_normal(r, outward_normal);

            return Some(rec);
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let box0 = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        return Some(AABB::surrounding_box(box0, box1));
    }
}

pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}
impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat_ptr: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat_ptr,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        return self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0));
    }

    fn get_sphere_uv(&self, p: Point3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = f64::atan2(-p.z, p.x) + PI;
        let mut u = phi / (2.0 * PI);
        let mut v = theta / PI;
        return (u, v);
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center(r.tm);
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(oc, r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let outward_normal = (r.at(root) - self.center(r.tm)) / self.radius;
        let (u, v) = self.get_sphere_uv(outward_normal);
        let mut rec = HitRecord::new(
            r.at(root),
            outward_normal,
            root,
            u,
            v,
            false,
            self.mat_ptr.clone(),
        );

        rec.set_face_normal(r, outward_normal);
        return Some(rec);
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let box0 = AABB::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        return Some(AABB::surrounding_box(box0, box1));
    }
}
