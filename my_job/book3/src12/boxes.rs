use super::aabb::AABB;
use crate::aarect::{xy_rect, xz_rect, yz_rect};
use crate::hittable::HittableList;
use crate::vec3::Point3;
use crate::{hittable::HitRecord, hittable::Hittable, material::Material, ray::Ray};

use std::sync::Arc;

pub struct Box {
    min: Point3,
    max: Point3,
    sides: HittableList,
}

impl Box {
    pub fn new(p0: Point3, p1: Point3, ptr: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();
        sides.add(Arc::new(xy_rect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            ptr.clone(),
        )));
        sides.add(Arc::new(xy_rect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            ptr.clone(),
        )));

        sides.add(Arc::new(xz_rect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            ptr.clone(),
        )));
        sides.add(Arc::new(xz_rect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            ptr.clone(),
        )));

        sides.add(Arc::new(yz_rect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            ptr.clone(),
        )));
        sides.add(Arc::new(yz_rect::new(p0.y, p1.y, p0.z, p1.z, p0.x, ptr)));

        Self {
            min: p0,
            max: p1,
            sides,
        }
    }
}

impl Hittable for Box {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}
