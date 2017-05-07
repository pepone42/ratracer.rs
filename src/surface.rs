use color::Color;
use vector3::Vector3;
// #[derive(Debug)]
// pub struct Surface {
//     pub roughness: f64,
// }

// impl Surface {
//     pub fn new(roughness: f64) -> Self {
//         Surface { roughness }
//     }
// }

pub trait Surface {
    fn diffuse(&self, pos: &Vector3) -> Color;
    fn specular(&self, pos: &Vector3) -> Color;
    fn reflect(&self, pos: &Vector3) -> f64;
    fn roughness(&self) -> f64;
}
#[derive(Debug)]
pub struct Shiny;
impl Surface for Shiny {
    fn diffuse(&self, pos: &Vector3) -> Color {
        Color::white()
    }
    fn specular(&self, pos: &Vector3) -> Color {
        Color::grey()
    }
    fn reflect(&self, pos: &Vector3) -> f64 {
        0.7
    }
    fn roughness(&self) -> f64 {
        250.0
    }
}
#[derive(Debug)]
pub struct CheckBoard;
impl Surface for CheckBoard {
    fn diffuse(&self, pos: &Vector3) -> Color {
        if (f64::floor(pos.z) + f64::floor(pos.x)) % 2.0 != 0.0 {
            Color::white()
        } else {
            Color::black()
        }
    }
    fn specular(&self, pos: &Vector3) -> Color {
        Color::white()
    }
    fn reflect(&self, pos: &Vector3) -> f64 {
        if (f64::floor(pos.z) + f64::floor(pos.x)) % 2.0 != 0.0 {
            0.1
        } else {
            0.7
        }
    }
    fn roughness(&self) -> f64 {
        150.0
    }
}
