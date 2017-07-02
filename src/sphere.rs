use std::f64;
use vector3::Vector3;
use surface::Surface;
use object::{Object, Ray, Intersection};

pub struct Sphere<'a> {
    //object: Object,
    pub radius: f64,
    pub center: Vector3,
    surface: &'a Surface,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Vector3, radius: f64, surface: &'a Surface) -> Self {
        Sphere {
            center,
            radius,
            surface,
        }
    }
}

impl<'a> Object for Sphere<'a> {
    fn normal(&self, pos: Vector3) -> Vector3 {
        (pos - self.center).norm()
    }
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let eo = self.center - ray.start;
        let v = eo ^ ray.dir;
        let mut dist = 0f64;
        if v >= 0.0 {
            let disc = (self.radius * self.radius) - ((eo ^ eo) - (v * v));
            if disc >= 0.0 {
                dist = v - disc.sqrt();
            }
        }
        if dist == 0.0 {
            None
        } else {
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
