use vector3::Vector3;
use surface::Surface;
pub use ray::Ray;
pub use intersection::Intersection;

pub trait Object {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
    fn normal(&self, pos: Vector3) -> Vector3;
    fn surface(&self) -> &Surface;
}
