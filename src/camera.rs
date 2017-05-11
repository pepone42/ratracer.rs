use vector3::Vector3;

#[derive(Debug)]
pub struct Camera {
    pub forward: Vector3,
    pub right: Vector3,
    pub up: Vector3,
    pub pos: Vector3,
    pub look_at: Vector3,
}

impl Camera {
    pub fn new(pos: Vector3, look_at: Vector3) -> Self {
        let down = Vector3::new(0.0, -1.0, 0.0);
        let forward = (look_at - pos).norm();
        let right = (forward * down).norm() * 1.5;
        let up = (forward * right).norm() * 1.5;
        Camera {
            look_at,
            pos,
            forward,
            right,
            up,
        }
    }
}
