
use vector3d::Vector3;

#[derive(Debug, Default)]
pub struct Ray {
    pub pos: Vector3,
    pub dir: Vector3,
}

impl Ray {
    pub fn new(pos: Vector3, dir: Vector3) -> Ray {
        Ray { pos, dir }
    }
}
