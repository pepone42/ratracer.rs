use vector3::Vector3;
use color::Color;

#[derive(Debug)]
pub struct Light {
    pub pos: Vector3,
    pub color: Color,
}

impl Light {
    pub fn new(pos: Vector3, color: Color) -> Self {
        Light { pos, color }
    }
}
