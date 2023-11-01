use crate::{ray::Ray, vec3::Point3, vec3::Vec3};
use std::sync::Arc;
#[derive(Copy, Clone, Default)]
pub struct HitRecord {
    pub p: Point3,    //碰撞点
    pub normal: Vec3, //法向量
    pub t: f64,
    pub front_face: bool, //光线朝里还是朝外
}

impl HitRecord {
    pub fn new(_p: Point3, _normal: Vec3, _t: f64, _front_face: bool) -> HitRecord {
        HitRecord {
            p: _p,
            normal: _normal,
            t: _t,
            front_face: _front_face,
        }
    }
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.dir, outward_normal) < 0.;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = Vec3::new(0., 0., 0.) - outward_normal;
        }
    }
}
pub trait Hittable {
    //继承类
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    //里面有sphere and so on
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
        return temp_rec;
    }
}
