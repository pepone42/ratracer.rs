use vector3::Vector3;

#[derive(Debug,Clone,Copy)]
pub struct Ray {
    pub start: Vector3,
    pub dir: Vector3,
}
