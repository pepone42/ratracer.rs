use std::f64;
use scene::Scene;
use object::{Object, Ray, Intersection};
use color::Color;
use vector3::Vector3;
use light::Light;
use std::iter;
use camera::Camera;

const max_depth: f64 = 5.0;

pub struct Raytracer<'a> {
    pub scene: Scene<'a>,
}

impl<'a> Raytracer<'a> {
    pub fn new(scene: Scene<'a>) -> Self {
        Raytracer { scene }
    }

    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest = f64::INFINITY;
        let mut closes_inter: Option<Intersection> = None;
        for i in &self.scene.objects {
            if let Some(inter) = i.intersect(&ray) {
                if inter.dist < closest {
                    closes_inter = Some(Intersection {
                                            object: inter.object,
                                            ray: inter.ray,
                                            dist: inter.dist,
                                        });
                    closest = inter.dist;
                }
            }
        }
        closes_inter
    }

    fn testRay(&self, ray: &Ray) -> Option<f64> {
        if let Some(isect) = self.intersection(ray) {
            Some(isect.dist)
        } else {
            None
        }
    }

    fn getReflectionColor(&self,
                          obj: &Object,
                          pos: &Vector3,
                          normal: &Vector3,
                          reflectDir: &Vector3,
                          depth: f64)
                          -> Color {
        //return Color::new(0.1,0.2,0.1);
        obj.surface().reflect(&pos) *
        &self.traceRay(&Ray {
                            start: pos.clone(),
                            dir: reflectDir.clone(),
                        },
                       depth + 1.0)
    }

    fn getNaturalColor(&self, obj: &Object, pos: &Vector3, norm: &Vector3, rd: &Vector3) -> Color {
        let addLigh = |col: Color, light: &Light| {
            let ldis = &light.pos - &pos;
            let livec = ldis.norm();
            let neatIsect = self.testRay(&Ray {
                                              start: pos.clone(),
                                              dir: livec.clone(),
                                          });
            let isInShadow = match neatIsect {
                None => false,
                Some(i) => i <= ldis.len(),
            };
            if isInShadow {
                col
            } else {
                let illum = &livec ^ norm;
                let lcolor = if illum > 0.0 {
                    illum * &light.color
                } else {
                    Color::black()
                };
                let specular = &livec ^ &rd.norm();
                let scolor = if specular > 0.0 {
                    specular.powf(obj.surface().roughness()) * &light.color
                    //(obj.surface().roughness().powf(specular)) * &light.color
                } else {
                    Color::black()
                };
                &col +
                &(&(&obj.surface().diffuse(&pos) * &lcolor) +
                  &(&obj.surface().specular(&pos) * &scolor))
            }
        };
        self.scene
            .lights
            .iter()
            .fold(Color::black(), |acc, l| addLigh(acc, &l))
    }

    fn shade(&self, isect: &Intersection, depth: f64) -> Color {
        let d = &isect.ray.dir;
        let pos = &(isect.dist * d) + &isect.ray.start;
        let normal = isect.object.normal(&pos);
        let reflectDir = d - &(&((&normal ^ d) * &normal) * 2.0);
        let naturalColor = &Color::black() +
                           &self.getNaturalColor(isect.object, &pos, &normal, &reflectDir);
        let reflectedColor = if depth >= max_depth {
            Color::grey()
        } else {
            self.getReflectionColor(isect.object, &pos, &normal, &reflectDir, depth)
        };
        &naturalColor + &reflectedColor
    }
    fn traceRay(&self, ray: &Ray, depth: f64) -> Color {
        if let Some(isect) = self.intersection(ray) {
            self.shade(&isect, depth)
        } else {
            Color::black()
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, width: u32, height: u32) {

        let getPoint = |x: f64, y: f64, cam: &Camera| {
            let width = width as f64;
            let height = height as f64;
            let recenterX = |x| (x - (width / 2.0)) / 2.0 / width;
            let recenterY = |y| (y - (height / 2.0)) / 2.0 / height;
            (&(&(&cam.right * recenterX(x)) + &(&cam.up * recenterY(y))) + &cam.forward).norm()
        };
        for y in 0..height {
            for x in 0..width {
                let color =
                    self.traceRay(&Ray {
                                       start: self.scene.camera.pos.clone(),
                                       dir: getPoint(x as f64, y as f64, &self.scene.camera),
                                   },
                                  0.0);
                let c = color.to_pixel_color();
                buffer[(x + (height - y - 1) * width) as usize] =
                    (c.r as u32) << 16 | (c.g as u32) << 8 | (c.b as u32);
            }
        }
    }
}

