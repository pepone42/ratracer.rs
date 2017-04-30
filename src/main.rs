mod vector3;
mod color;
mod camera;

use vector3::Vector3;
use color::Color;

fn main() {
    let v = Vector3::new(1.0,2.0,3.0);
    println!("{:?}",3.0 * &v);
    let col = Color::white(); 
}
