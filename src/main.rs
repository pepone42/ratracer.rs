extern crate minifb;

mod vector3;
mod color;
mod camera;
mod ray;
mod object;
mod surface;
mod sphere;
mod plane;
mod intersection;
mod light;
mod scene;
mod raytracer;

use minifb::{Key, WindowOptions, Window};

use vector3::Vector3;
use color::Color;
use scene::Scene;
use plane::Plane;
use sphere::Sphere;
use surface::{CheckBoard, Shiny};
use light::Light;
use camera::Camera;
use raytracer::Raytracer;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

const CHECKBOARD: &CheckBoard = &CheckBoard {};
const SHINY: &Shiny = &Shiny {};

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let plane: Plane = Plane::new(Vector3::new(0.0, 1.0, 0.0), 0.0, CHECKBOARD);
    let sphere1: Sphere = Sphere::new(Vector3::new(0.0, 1.0, -0.25), 1.0, SHINY);
    let sphere2: Sphere = Sphere::new(Vector3::new(-1.0, 0.5, 1.5), 0.5, SHINY);

    let s = Scene {
        objects: vec![&plane, &sphere1, &sphere2],
        lights: vec![Light::new(Vector3::new(-2.0, 2.5, 0.0), Color::new(0.49, 0.07, 0.07)),
                     Light::new(Vector3::new(1.5, 2.5, 1.5), Color::new(0.07, 0.07, 0.49)),
                     Light::new(Vector3::new(1.5, 2.5, -1.5), Color::new(0.07, 0.49, 0.071)),
                     Light::new(Vector3::new(0.0, 3.5, 0.0), Color::new(0.21, 0.21, 0.35))],
        camera: Camera::new(Vector3::new(3.0, 2.0, 4.0), Vector3::new(-1.0, 0.5, 0.0)),
    };

    let mut window = Window::new("Raytrace - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default())
            .unwrap_or_else(|e| {
                                panic!("{}", e);
                            });

    let rt = Raytracer::new(s);
    
    rt.render(&mut buffer, WIDTH as u32, HEIGHT as u32, 4);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        window.update_with_buffer(&buffer);
    }
}
