use vector3::Vector3;
use surface::Surface;
pub use ray::Ray;
pub use intersection::Intersection;

// //#[derive(Debug)]
// pub struct Object {
//     surface: Box<Surface>,
// }

// impl Object {
//     pub fn new(surface : Surface) ->Self {
//         Object { surface }
//     }
// }

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn normal(&self, pos: &Vector3) -> Vector3;
    fn surface(&self) -> &Surface;
}
