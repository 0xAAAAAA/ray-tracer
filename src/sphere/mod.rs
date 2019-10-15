use crate::ray::Ray;
use crate::vec3::Vec3;
use std::clone::Clone;
use std::marker::Copy;

pub struct Sphere {
    pub c: Vec3,
    pub r: f32,
}

#[allow(dead_code, unused_variables)]
impl Sphere {
    pub fn new(c: Vec3, r: f32) -> Sphere {
        Sphere { c: c, r: r }
    }

    pub fn normal(self, pi: Vec3) -> Vec3 {
        (pi - self.c) * (-1.0 / self.r)
    }

    pub fn intersect(self, r: Ray) -> (bool, f32) {
        let o = r.o;
        let d = r.d;
        let oc = o - self.c;
        let b = Vec3::dot(oc, d) * 2.0;
        let c = Vec3::sdot(&oc, &oc) - (self.r * self.r);
        let mut disc = b * b - (c * 4.0);

        if disc < 1e-4 {
            return (false, 0.0);
        } else {
            disc = disc.sqrt();
            let t0 = -b - disc;
            let t1 = -b + disc;
            return (true, if t0 < t1 { t0 } else { t1 });
        }
    }
}

impl Copy for Sphere {}

impl Clone for Sphere {
    fn clone(&self) -> Self {
        *self
    }
}
