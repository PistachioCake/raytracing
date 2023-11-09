use core::alloc::Allocator;

use crate::ray::Ray;

use super::{HitRecord, Hittable, Interval, AABB};

pub struct BvhNode<'a> {
    l: &'a dyn Hittable,
    r: &'a dyn Hittable,
    aabb: AABB<f32>,
}

impl<'a> BvhNode<'a> {
    pub fn new<'b: 'a, A: Allocator + Copy + 'a>(
        mut objects: Vec<&'a dyn Hittable>,
        alloc: A,
    ) -> Self {
        if objects.len() < 2 {
            panic!("Cannot construct a BvhNode with < 2 elements!")
        }

        Self::construct(alloc, &mut objects, 0)
    }

    fn construct<'b: 'a, A: Allocator + Copy + 'a>(
        alloc: A,
        objects: &mut [&'a dyn Hittable],
        axis: usize,
    ) -> Self {
        let in_alloc = |node| Box::leak(Box::new_in(node, alloc));

        // base cases
        if objects.len() == 2 {
            return Self::from_two(objects[0], objects[1]);
        }
        if objects.len() == 3 {
            let node_a = in_alloc(Self::from_two(objects[0], objects[1]));
            return Self::from_two(node_a, objects[2]);
        }

        // recursive case - split into two a la k-d trees, and then recurse
        let split_idx = objects.len() / 2;
        objects.select_nth_unstable_by(split_idx, |l, r| {
            let l = l.bounding_box()[axis].min;
            let r = r.bounding_box()[axis].min;
            l.partial_cmp(&r).unwrap()
        });

        let (l, r) = objects.split_at_mut(split_idx);
        let axis = (axis + 1) % 3;
        let l = in_alloc(Self::construct(alloc, l, axis));
        let r = in_alloc(Self::construct(alloc, r, axis));

        Self::from_two(l, r)
    }

    fn from_two(l: &'a dyn Hittable, r: &'a dyn Hittable) -> Self {
        BvhNode {
            l,
            r,
            aabb: l.bounding_box().combine(r.bounding_box()),
        }
    }
}

impl Hittable for BvhNode<'_> {
    fn hit(&self, ray: &Ray, mut ray_t: Interval<f32>) -> Option<HitRecord> {
        if !self.aabb.hit(ray, ray_t) {
            return None;
        }

        let hit_l = self.l.hit(ray, ray_t);
        if let Some(HitRecord { t, .. }) = hit_l {
            ray_t.max = t;
        }
        let hit_r = self.r.hit(ray, ray_t);

        hit_r.or(hit_l)
    }

    fn bounding_box(&self) -> AABB<f32> {
        self.aabb
    }
}
