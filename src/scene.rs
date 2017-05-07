use object::Object;
use light::Light;
use camera::Camera;

pub struct Scene<'a> {
    pub objects: Vec<&'a Object>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}