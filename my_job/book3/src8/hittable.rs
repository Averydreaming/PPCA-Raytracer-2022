use crate::aabb::AABB;
use crate::material::Material;
use crate::{ray::Ray, vec3::Point3, vec3::Vec3};
use std::sync::Arc;
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,    //碰撞点
    pub normal: Vec3, //法向量
    pub u: f64,
    pub v: f64,
    pub t: f64,
    pub front_face: bool, //光线朝里还是朝外
    pub mat_ptr: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        _p: Point3,
        _normal: Vec3,
        _t: f64,
        _u: f64,
        _v: f64,
        _front_face: bool,
        _mat_ptr: Arc<dyn Material>,
    ) -> HitRecord {
        HitRecord {
            p: _p,
            normal: _normal,
            t: _t,
            u: _u,
            v: _v,
            front_face: _front_face,
            mat_ptr: _mat_ptr,
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
pub trait Hittable: Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        0.0
    }
    fn random(&self, o: Vec3) -> Vec3 {
        Vec3::new(1., 0., 0.)
    }
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
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }
        let mut output_box = AABB::new(
            Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        );

        for object in &self.objects {
            if let Some(temp_box) = object.bounding_box(time0, time1) {
                output_box = AABB::surrounding_box(output_box, temp_box);
            } else {
                return None;
            }
        }
        return Some(output_box);
    }
}
#[derive(Clone)]
pub struct Translate {
    ptr: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            ptr: p,
            offset: displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.orig - self.offset, r.dir, r.tm);
        if let Some(mut rec) = self.ptr.hit(moved_r, t_min, t_max) {
            rec.p += self.offset;
            rec.set_face_normal(moved_r, rec.normal);
            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if let Some(output_box) = self.ptr.bounding_box(time0, time1) {
            Some(AABB::new(
                output_box.minimum + self.offset,
                output_box.maximum + self.offset,
            ))
        } else {
            None
        }
    }
}

pub struct RotateY {
    ptr: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        if let Some(output_box) = p.bounding_box(0., 1.) {
            let mut minimum = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
            let mut maximum = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x =
                            i as f64 * output_box.maximum.x + (1 - i) as f64 * output_box.minimum.x;
                        let y =
                            j as f64 * output_box.maximum.y + (1 - j) as f64 * output_box.minimum.y;
                        let z =
                            k as f64 * output_box.maximum.z + (1 - k) as f64 * output_box.minimum.z;

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(newx, y, newz);

                        for c in 0..3 {
                            minimum[c] = minimum[c].min(tester[c]);
                            maximum[c] = maximum[c].max(tester[c]);
                        }
                    }
                }
            }
            Self {
                ptr: p,
                sin_theta,
                cos_theta,
                bbox: Some(AABB::new(minimum, maximum)),
            }
        } else {
            Self {
                ptr: p,
                sin_theta,
                cos_theta,
                bbox: None,
            }
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        self.bbox
    }
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.orig;
        let mut direction = r.dir;

        origin[0] = self.cos_theta * r.orig[0] - self.sin_theta * r.orig[2];
        origin[2] = self.sin_theta * r.orig[0] + self.cos_theta * r.orig[2];

        direction[0] = self.cos_theta * r.dir[0] - self.sin_theta * r.dir[2];
        direction[2] = self.sin_theta * r.dir[0] + self.cos_theta * r.dir[2];

        let rotated_r = Ray::new(origin, direction, r.tm);

        if let Some(mut rec) = self.ptr.hit(rotated_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            rec.p = p;
            rec.set_face_normal(rotated_r, normal);

            Some(rec)
        } else {
            None
        }
    }
}
#[derive(Clone)]
pub struct flip_face {
    ptr: Arc<dyn Hittable>,
}

impl flip_face {
    pub fn new(p: Arc<dyn Hittable>) -> Self {
        Self { ptr: p }
    }
}

impl Hittable for flip_face {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec) = self.ptr.hit(r, t_min, t_max) {
            rec.front_face = !rec.front_face;
            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        return self.ptr.bounding_box(time0, time1);
    }
}
