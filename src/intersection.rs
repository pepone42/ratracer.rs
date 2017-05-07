use object::Object;
use ray::Ray;

//#[derive(Debug,Clone)]
pub struct Intersection<'a> {
    pub object: &'a Object,
    pub ray: Ray,
    pub dist: f64,
}
// TODO create a constructor
