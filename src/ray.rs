use crate::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    direction: Vec3,
    origin: Point3,
}

impl Ray {
    pub fn new(direction: &Vec3, origin: &Point3) -> Self {
        Self {
            direction: *direction,
            origin: *origin,
        }
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (t * self.direction)
    }
}