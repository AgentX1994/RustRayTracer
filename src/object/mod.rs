use super::material::Material;
use super::ray::Ray;
use super::vector3d::Vector3;

pub trait Object {
    /// Calculates if and where the given ray intersects with this object.
    ///
    /// This function calculates the intersection point, if any, of this object with the given ray.
    ///
    /// The ray is represented mathematically as _p + t*d_, where _p_ is the starting point of the ray,
    /// and _d_ is the direction vector.
    ///
    /// If the ray does intersect with this object, the return value will be a "Some" value with
    /// the smallest non negative _t_.
    ///
    /// If the ray does not intersect, then "None" is returned.
    fn ray_intersection(&self, r: &Ray) -> Option<f64>;

    /// Returns this object's Material
    fn material(&self) -> &Material;

    /// Get the position of this object
    fn position(&self) -> &Vector3;
}

pub mod sphere;
