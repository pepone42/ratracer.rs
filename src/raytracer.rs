use std::f64;
use scene::Scene;
use object::{Object, Ray, Intersection};
use color::Color;
use vector3::Vector3;
use light::Light;
use camera::Camera;

const MAX_DEPTH: f64 = 5.0;

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

    fn test_ray(&self, ray: &Ray) -> Option<f64> {
        if let Some(isect) = self.intersection(ray) {
            Some(isect.dist)
        } else {
            None
        }
    }

    fn get_reflection_color(&self,
                            obj: &Object,
                            pos: Vector3,
                            normal: Vector3,
                            reflect_dir: Vector3,
                            depth: f64)
                            -> Color {
        obj.surface().reflect(pos) *
        self.trace_ray(&Ray {
                             start: pos.clone(),
                             dir: reflect_dir.clone(),
                         },
                        depth + 1.0)
    }

    fn get_natural_color(&self,
                         obj: &Object,
                         pos: Vector3,
                         norm: Vector3,
                         rd: Vector3)
                         -> Color {
        let add_ligh = |col: Color, light: &Light| {
            let ldis = light.pos - pos;
            let livec = ldis.norm();
            let neat_isect = self.test_ray(&Ray {
                                                start: pos,
                                                dir: livec,
                                            });
            let is_in_shadow = match neat_isect {
                None => false,
                Some(i) => i <= ldis.len(),
            };
            if is_in_shadow {
                col
            } else {
                let illum = livec ^ norm;
                let lcolor = if illum > 0.0 {
                    illum * light.color
                } else {
                    Color::black()
                };
                let specular = livec ^ rd.norm();
                let scolor = if specular > 0.0 {
                    specular.powf(obj.surface().roughness()) * light.color
                } else {
                    Color::black()
                };
                col +
                ((obj.surface().diffuse(pos) * lcolor) +
                  (obj.surface().specular(pos) * scolor))
            }
        };
        self.scene
            .lights
            .iter()
            .fold(Color::black(), |acc, l| add_ligh(acc, &l))
    }

    fn shade(&self, isect: &Intersection, depth: f64) -> Color {
        let d = isect.ray.dir;
        let pos = (isect.dist * d) + isect.ray.start;
        let normal = isect.object.normal(pos);
        let reflect_dir = d - (((normal ^ d) * normal) * 2.0);
        let natural_color = Color::black() +
                            self.get_natural_color(isect.object, pos, normal, reflect_dir);
        let reflected_color = if depth >= MAX_DEPTH {
            Color::grey()
        } else {
            self.get_reflection_color(isect.object, pos, normal, reflect_dir, depth)
        };
        natural_color + reflected_color
    }
    fn trace_ray(&self, ray: &Ray, depth: f64) -> Color {
        if let Some(isect) = self.intersection(ray) {
            self.shade(&isect, depth)
        } else {
            Color::black()
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, width: u32, height: u32, oversampling: u32) {

        let get_point = |x: f64, y: f64, cam: &Camera| {
            let width = (width * oversampling) as f64;
            let height = (height * oversampling) as f64;
            let recenter_x = |x| (x - (width / 2.0)) / 2.0 / width;
            let recenter_y = |y| (y - (height / 2.0)) / 2.0 / height;
            (((cam.right * recenter_x(x)) + (cam.up * recenter_y(y))) + cam.forward).norm()
        };
        for y in 0..height {
            for x in 0..width {
                let color = {
                    let mut color = Color::black();
                    for xx in 0..oversampling {
                        for yy in 0..oversampling {
                            color =
                                color +
                                self.trace_ray(&Ray {
                                                     start: self.scene.camera.pos.clone(),
                                                     dir: get_point((x * oversampling + xx) as f64,
                                                                    (y * oversampling + yy) as f64,
                                                                    &self.scene.camera),
                                                 },
                                                0.0)
                        }
                    }
                    color * (1.0 / ((oversampling * oversampling) as f64))
                };
                let c = color.to_pixel_color();
                buffer[(x + (height - y - 1) * width) as usize] =
                    (c.r as u32) << 16 | (c.g as u32) << 8 | (c.b as u32);
            }
        }
    }
}

