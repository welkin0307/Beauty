use super::vector::{Vector3, Point3};

pub struct Ray{
    orig:Point3,
    dir:Vector3
}

impl Ray{
    pub fn new(origin:Point3, direction:Vector3) -> Ray{
        Ray{
            orig:origin,
            dir:direction
        }
    }

    pub fn origin(&self) -> Point3{
        self.orig
    }

    pub fn direction(&self) -> Vector3{
        self.dir
    }

    pub fn at(&self, t:f64) -> Point3{
        self.orig+t*self.dir
    }
}
