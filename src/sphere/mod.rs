//! This module defines a struct representing a mathematical Sphere

use ray::Ray;
use vector3d::Vector3;

/// The struct representing a sphere
///
/// Contains a Vector3 for the sphere's position, and a
/// f64 radius
#[derive(Debug, Default)]
pub struct Sphere {
    pub pos: Vector3,
    pub radius: f64,
}

impl Sphere {
    /// Creates a new sphere with the given position and radius
    ///
    /// # Example
    ///
    /// ```
    /// let pos = ray_tracer::vector3d::Vector3::new(1.0, 1.0, 1.0);
    /// let sphere = ray_tracer::sphere::Sphere::new(pos, 3.0);
    ///
    /// assert_eq!(sphere.pos.x, 1.0);
    /// assert_eq!(sphere.pos.y, 1.0);
    /// assert_eq!(sphere.pos.z, 1.0);
    /// assert_eq!(sphere.radius, 3.0);
    /// ```
    pub fn new(pos: Vector3, radius: f64) -> Sphere {
        Sphere { pos, radius }
    }

    /// Calculates if and where the given ray intersects with this sphere.
    ///
    /// This function calculates the intersection points, if any, of this sphere with the given ray.
    ///
    /// The ray is represented mathematically as _p + t*d_, where _p_ is the starting point of the ray,
    /// and _d_ is the direction vector.
    ///
    /// If the ray does intersect with this sphere, the return value will be a "Some" value with two
    /// floats, representing two values of _t_ in the above equation. The two floats are equal if
    /// there is only one intersection point.
    ///
    /// If the ray does not intersect, then "None" is returned.
    /// # Example
    ///
    /// ```
    /// let sphere_position = ray_tracer::vector3d::Vector3::new(2.0, 0.0, 0.0);
    /// let sphere = ray_tracer::sphere::Sphere::new(sphere_position, 1.0);
    ///
    /// let ray1_position = ray_tracer::vector3d::Vector3::new(0.0, 0.0, 0.0);
    /// let ray1_direction = ray_tracer::vector3d::Vector3::new(1.0, 0.0, 0.0);
    /// let ray1 = ray_tracer::ray::Ray::new(ray1_position, ray1_direction);
    ///
    /// let ray1_intersection = sphere.ray_intersection(&ray1);
    ///
    /// assert!(ray1_intersection.is_some());
    /// assert_eq!(ray1_intersection.unwrap(), (3.0, 1.0));
    ///
    /// let ray2_position = ray_tracer::vector3d::Vector3::new(0.0, 1.0, 0.0);
    /// let ray2_direction = ray_tracer::vector3d::Vector3::new(1.0, 0.0, 0.0);
    /// let ray2 = ray_tracer::ray::Ray::new(ray2_position, ray2_direction);
    ///
    /// let ray2_intersection = sphere.ray_intersection(&ray2);
    ///
    /// assert!(ray2_intersection.is_some());
    /// assert_eq!(ray2_intersection.unwrap(), (2.0, 2.0));
    ///
    /// let ray3_position = ray_tracer::vector3d::Vector3::new(0.0, 0.0, 0.0);
    /// let ray3_direction = ray_tracer::vector3d::Vector3::new(0.0, 1.0, 0.0);
    /// let ray3 = ray_tracer::ray::Ray::new(ray3_position, ray3_direction);
    ///
    /// let ray3_intersection = sphere.ray_intersection(&ray3);
    ///
    /// assert!(ray3_intersection.is_none());
    /// ```
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
