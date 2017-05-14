use std::f64;
use vector3::Vector3;
use surface::Surface;
use object::{Object, Ray, Intersection};

pub struct Plane<'a> {
    norm: Vector3,
    offset: f64,
    surface: &'a Surface,
}

impl<'a> Plane<'a> {
    pub fn new(norm: Vector3, offset: f64, surface: &'a Surface) -> Self {
        Plane {
            norm,
            offset,
            surface,
        }
    }
}

impl<'a> Object for Plane<'a> {
    fn normal(&self, pos: Vector3) -> Vector3 {
        let _ = pos;
        self.norm
    }
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let denom = self.norm ^ ray.dir;
        if denom > 0.0 {
            None
        } else {
            let dist = ((self.norm ^ ray.start) + self.offset) / (-denom);
            Some(Intersection {
                     object: self,
                     ray,
                     dist,
                 })
        }
    }
    fn surface(&self) -> &Surface {
        self.surface
    }
}
