use std::f64;
use std::ops::{Mul, Add, Sub};

#[derive(Debug,Clone,PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
    pub fn white() -> Self {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }
    pub fn grey() -> Self {
        Color {
            r: 0.5,
            g: 0.5,
            b: 0.5,
        }
    }
    pub fn black() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
    pub fn clamp(&self) -> Color {
        let cl = |a| if a > 1.0 { 1.0 } else { a };
        Color {
            r: cl(self.r),
            g: cl(self.g),
            b: cl(self.b),
        }
    }
    pub fn to_pixel_color(&self) -> Color {
        let c = self.clamp();
        Color {
            r: (c.r * 255.0).floor(),
            g: (c.g * 255.0).floor(),
            b: (c.b * 255.0).floor(),
        }
    }
}

impl<'b> Mul<f64> for &'b Color {
    type Output = Color;
    fn mul(self, a: f64) -> Color {
        Color {
            r: self.r * a,
            g: self.g * a,
            b: self.b * a,
        }
    }
}

impl<'a> Mul<&'a Color> for f64 {
    type Output = Color;
    fn mul(self, c: &'a Color) -> Color {
        Color {
            r: self * c.r,
            g: self * c.g,
            b: self * c.b,
        }
    }
}

impl<'a, 'b> Add<&'a Color> for &'b Color {
    type Output = Color;
    fn add(self, c: &'a Color) -> Color {
        Color {
            r: self.r + c.r,
            g: self.g + c.g,
            b: self.b + c.b,
        }
    }
}


impl<'a, 'b> Mul<&'a Color> for &'b Color {
    type Output = Color;
    fn mul(self, c: &'a Color) -> Color {
        Color {
            r: self.r * c.r,
            g: self.g * c.g,
            b: self.b * c.b,
        }
    }
}

impl<'a, 'b> Sub<&'a Color> for &'b Color {
    type Output = Color;
    fn sub(self, c: &'a Color) -> Color {
        Color {
            r: self.r - c.r,
            g: self.g - c.g,
            b: self.b - c.b,
        }
    }
}

#[test]
fn scalar_mul() {
    assert_eq!(&Color::new(0.5, 0.5, 0.5) * 2.0, Color::new(1.0, 1.0, 1.0));
    assert_eq!(2.0 * &Color::new(0.5, 0.5, 0.5), Color::new(1.0, 1.0, 1.0));
}
#[test]
fn add() {
    assert_eq!(Color::new(1.0, 2.0, 4.0),
               &Color::new(0.5, 1.0, 1.0) + &Color::new(0.5, 1.0, 3.0));
}
#[test]
fn sub() {
    assert_eq!(Color::new(1.0, 2.0, 0.0),
               &Color::new(1.5, 3.0, 3.0) - &Color::new(0.5, 1.0, 3.0));
}
#[test]
fn clamp() {
    assert_eq!(Color::new(0.3, 0.4, 0.6), Color::new(0.3, 0.4, 0.6).clamp());
    assert_eq!(Color::new(1.0, 1.0, 1.0), Color::new(1.3, 1.4, 1.6).clamp());
}
#[test]
fn to_pixel_color() {
    assert_eq!(Color::new(1.0, 1.0, 1.0).to_pixel_color(),
               Color::new(255.0, 255.0, 255.0));
    assert_eq!(Color::new(1.6, 1.3, 1.1).to_pixel_color(),
               Color::new(255.0, 255.0, 255.0));
    assert_eq!(Color::new(0.5, 1.0, 0.0).to_pixel_color(),
               Color::new(127.0, 255.0, 0.0));

}