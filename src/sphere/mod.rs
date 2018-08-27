
use ray::Ray;
use vector3d::Vector3;

#[derive(Debug, Default)]
pub struct Sphere {
    pub pos: Vector3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(pos: Vector3, radius: f64) -> Sphere {
        Sphere { pos, radius }
    }

    pub fn ray_intersection(&self, r: &Ray) -> Option<(f64, f64)> {
        let o_sub_c = r.pos.sub(&self.pos);
        let len_sq_o_sub_c = o_sub_c.dot(&o_sub_c);
        let dir_dot_o_sub_c = r.dir.dot(&o_sub_c);
        let dir_dot_o_sub_c_sq = dir_dot_o_sub_c.powi(2);
        let radius_sq = self.radius.powi(2);
        let discrimant = dir_dot_o_sub_c_sq - len_sq_o_sub_c + radius_sq;

        match discrimant {
            x if x < 0.0 => None,
            x if x == 0.0 => Some((-dir_dot_o_sub_c, -dir_dot_o_sub_c)),
            x => Some((-dir_dot_o_sub_c + x.sqrt(), -dir_dot_o_sub_c - x.sqrt())),
        }
    }
}
