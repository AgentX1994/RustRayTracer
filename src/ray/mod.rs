//! This module defines a struct representing a mathematical Ray


use vector3d::Vector3;

/// The struct representing a Ray
///
/// Contains a Vector3 for the Ray's starting position, and a
/// Vector3 for the direction of the Ray
#[derive(Debug, Default)]
pub struct Ray {
    pub pos: Vector3,
    pub dir: Vector3,
}

impl Ray {
    /// Creates a new Ray with the given starting position and direction
    ///
    /// # Example
    ///
    /// ```
    /// let ray_position = ray_tracer::vector3d::Vector3::new(1.0, 0.0, -1.0);
    /// let ray_direction = ray_tracer::vector3d::Vector3::new(0.0, 1.0, 2.0);
    /// let ray = ray_tracer::ray::Ray::new(ray_position, ray_direction);
    ///
    /// assert_eq!(ray.pos.x, 1.0);
    /// assert_eq!(ray.pos.y, 0.0);
    /// assert_eq!(ray.pos.z, -1.0);
    ///
    /// assert_eq!(ray.dir.x, 0.0);
    /// assert_eq!(ray.dir.y, 1.0);
    /// assert_eq!(ray.dir.z, 2.0);
    /// ```
    pub fn new(pos: Vector3, dir: Vector3) -> Ray {
        Ray { pos, dir }
    }
}
