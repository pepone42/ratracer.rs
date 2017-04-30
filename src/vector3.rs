use std::f64;
use std::ops::{Mul, Add, Sub, BitXor};

#[derive(Debug,Clone,PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn norm(&self) -> Vector3 {
        let len = self.len();
        let div = if len == 0.0 {
            f64::INFINITY
        } else {
            1.0 / len
        };
        self * div
    }
}

impl<'b> Mul<f64> for &'b Vector3 {
    type Output = Vector3;

    fn mul(self, a: f64) -> Vector3 {
        Vector3 {
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
        }
    }
}

impl<'a> Mul<&'a Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, v: &'a Vector3) -> Vector3 {
        Vector3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}
// Cross product
impl<'a, 'b> Mul<&'a Vector3> for &'b Vector3 {
    type Output = Vector3;

    fn mul(self, v: &'a Vector3) -> Vector3 {
        Vector3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }
}

impl<'a, 'b> Add<&'a Vector3> for &'b Vector3 {
    type Output = Vector3;
    fn add(self, v: &'a Vector3) -> Vector3 {
        Vector3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl<'a, 'b> Sub<&'a Vector3> for &'b Vector3 {
    type Output = Vector3;
    fn sub(self, v: &'a Vector3) -> Vector3 {
        Vector3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl<'a, 'b> BitXor<&'a Vector3> for &'b Vector3 {
    type Output = f64;
    fn bitxor(self, v: &'a Vector3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

#[test]
fn scalar_mul_vector() {
    assert_eq!(Vector3::new(2.0, 4.0, 6.0),
               2.0 * &Vector3::new(1.0, 2.0, 3.0));
}
#[test]
fn vector_mul_scalar() {
    assert_eq!(Vector3::new(2.0, 4.0, 6.0),
               &Vector3::new(1.0, 2.0, 3.0) * 2.0);
}
#[test]
fn cross() {
    assert_eq!(Vector3::new(-4.0, 8.0, -4.0),
               &Vector3::new(1.0, 2.0, 3.0) * &Vector3::new(3.0, 2.0, 1.0));
}
#[test]
fn add() {
    assert_eq!(Vector3::new(4.0, 4.0, 4.0),
               &Vector3::new(1.0, 2.0, 3.0) + &Vector3::new(3.0, 2.0, 1.0));
}
#[test]
fn sub() {
    assert_eq!(Vector3::new(-2.0, 0.0, 2.0),
               &Vector3::new(1.0, 2.0, 3.0) - &Vector3::new(3.0, 2.0, 1.0));
}
#[test]
fn dot() {
    assert_eq!(10.0,
               &Vector3::new(1.0, 2.0, 3.0) ^ &Vector3::new(3.0, 2.0, 1.0));
}
#[test]
fn len() {
    assert_eq!((14f64).sqrt(),Vector3::new(1.0, 2.0, 3.0).len());
}
#[test]
fn norm() {
    assert_eq!(Vector3::new(2.0,0.0,0.0).norm(), Vector3::new(1.0,0.0,0.0));
    assert_eq!(Vector3::new(0.0,30.0,0.0).norm(), Vector3::new(0.0,1.0,0.0));
    assert_eq!(Vector3::new(0.0,0.0,64.0).norm(), Vector3::new(0.0,0.0,1.0));
}