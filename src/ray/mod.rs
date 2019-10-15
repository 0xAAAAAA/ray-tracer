use crate::vec3::Vec3;
use std::marker::Copy;
use std::clone::Clone;

pub struct Ray {
    pub o: Vec3,
    pub d: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray { o: o, d: d }
    }
}

impl Copy for Ray {}

impl Clone for Ray {
    fn clone(&self) -> Self {
        *self
    }
}