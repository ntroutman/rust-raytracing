use std::fmt;
use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn from(origin: Point3, dir: Vec3) -> Ray {
        return Ray { origin, dir };
    }

    pub fn at(&self, t: f32) -> Point3 {
        return &self.origin + t * &self.dir;
    }

    pub fn direction(&self) -> &Vec3 {
        return &self.dir;
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}->{}>", self.origin, self.dir)
    }
}
