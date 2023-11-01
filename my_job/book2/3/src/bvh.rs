use std::cmp::Ordering;
use std::sync::Arc;

use rand::Rng;

use super::aabb::AABB;
use crate::hittable::{Hittable, HittableList};

#[derive(Clone)]
pub struct BvhNode {
    Box: AABB,
    left: Option<Arc<dyn Hittable>>,
    right: Option<Arc<dyn Hittable>>,
}

impl Hittable for BvhNode {
    fn hit(
        &self,
        r: crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        if !self.Box.hit(r, t_min, t_max) {
            return None;
        }
        if let Some(rec1) = self.left.as_ref().unwrap().hit(r, t_min, t_max) {
            if let Some(rec2) = self.right.as_ref().unwrap().hit(r, t_min, rec1.t) {
                Some(rec2)
            } else {
                Some(rec1)
            }
        } else {
            if let Some(rec2) = self.right.as_ref().unwrap().hit(r, t_min, t_max) {
                Some(rec2)
            } else {
                None
            }
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.Box)
    }
}
impl BvhNode {
    pub fn new(mut src_objects: Vec<Arc<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        let mut objects = src_objects;
        let axis = rand::thread_rng().gen_range(0..3);
        let objects_span = objects.len();
        let mut left: Option<Arc<dyn Hittable>> = None;
        let mut right: Option<Arc<dyn Hittable>> = None;

        if objects_span == 0 {
            panic!("BvhNode::new: Get empty vec");
        }
        if objects_span == 1 {
            let obj0 = objects.pop().unwrap();
            left = Some(obj0.clone());
            right = Some(obj0);
        } else if objects_span == 2 {
            let obj0 = objects.pop().unwrap();
            let obj1 = objects.pop().unwrap();
            if BvhNode::box_compare(&obj0, &obj1, axis) == Ordering::Less {
                left = Some(obj0);
                right = Some(obj1);
            } else {
                left = Some(obj1);
                right = Some(obj0);
            }
        } else {
            objects.sort_by(|a, b| BvhNode::box_compare(a, b, axis));

            let mut left_vec = objects;
            let right_vec = left_vec.split_off(objects_span / 2);

            left = Some(Arc::new(BvhNode::new(left_vec, time0, time1)));
            right = Some(Arc::new(BvhNode::new(right_vec, time0, time1)));
        }

        if let Some(left_box) = left.as_ref().unwrap().bounding_box(time0, time1) {
            if let Some(right_box) = right.as_ref().unwrap().bounding_box(time0, time1) {
                Self {
                    Box: AABB::surrounding_box(left_box, right_box),
                    left,
                    right,
                }
            } else {
                panic!("BvhNode::new: No bounding box in bvh_node constructor.");
            }
        } else {
            panic!("BvhNode::new: No bounding box in bvh_node constructor.");
        }
    }

    pub fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
        let box_a = a.bounding_box(0., 0.).unwrap();
        let box_b = b.bounding_box(0., 0.).unwrap();
        if box_a.minimum[axis] < box_b.minimum[axis] {
            Ordering::Less
        } else if box_a.minimum[axis] > box_b.minimum[axis] {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    pub fn new_list(list: &HittableList, time0: f64, time1: f64) -> Self {
        BvhNode::new(list.objects.clone(), time0, time1)
    }
}
