//! This module defines a struct representing a 3-Dimensional Vector

/// The struct representing a 3-Dimensional Vector, with x, y, and z components
#[derive(Clone, Debug, Default)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    /// Creates a new vector with the given components.
    ///
    /// # Example
    ///
    /// ```
    /// let my_vec = ray_tracer::vector3d::Vector3::new(1.0, 2.0, 3.0);
    ///
    /// assert_eq!(my_vec.x, 1.0);
    /// assert_eq!(my_vec.y, 2.0);
    /// assert_eq!(my_vec.z, 3.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Returns the length of the Vector3.
    ///
    /// # Example
    ///
    /// ```
    /// let my_vec = ray_tracer::vector3d::Vector3::new(1.0, 1.0, 1.0);
    /// assert_eq!(my_vec.length(), (3.0f64).sqrt())
    /// ```
    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    /// Turns this vector into a unit vector.
    ///
    /// # Example
    ///
    /// ```
    /// let mut my_vec = ray_tracer::vector3d::Vector3::new(2.0, 0.0, 0.0);
    /// my_vec.normalize();
    /// assert_eq!(my_vec.length(), 1.0f64);
    /// ```
    pub fn normalize(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    /// Returns a unit vector pointing in the same direction as this vector.
    ///
    /// # Example
    ///
    /// ```
    /// let my_vec = ray_tracer::vector3d::Vector3::new(1.0, 2.0, 3.0);
    ///
    /// let my_unit_vec = my_vec.into_unit();
    ///
    /// assert_eq!(my_unit_vec.length(), 1.0)
    /// ```
    pub fn into_unit(&self) -> Vector3 {
        let length = self.length();

        Vector3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    /// Returns the result of adding this vector and the given vector as a new vector.
    ///
    /// # Example
    ///
    /// ```
    /// let my_vec1 = ray_tracer::vector3d::Vector3::new(1.0, 2.0, 3.0);
    /// let my_vec2 = ray_tracer::vector3d::Vector3::new(3.0, 2.0, 1.0);
    ///
    /// let my_vec3 = my_vec1.add(&my_vec2);
    ///
    /// assert_eq!(my_vec3.x, 4.0);
    /// assert_eq!(my_vec3.y, 4.0);
    /// assert_eq!(my_vec3.z, 4.0);
    /// ```
    pub fn add(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    /// Returns the result of subtracting the given vector from this vector as a new vector.
    ///
    /// # Example
    ///
    /// ```
    /// let my_vec1 = ray_tracer::vector3d::Vector3::new(1.0, 2.0, 3.0);
    /// let my_vec2 = ray_tracer::vector3d::Vector3::new(3.0, 2.0, 1.0);
    ///
    /// let my_vec3 = my_vec1.sub(&my_vec2);
    ///
    /// assert_eq!(my_vec3.x, -2.0);
    /// assert_eq!(my_vec3.y, 0.0);
    /// assert_eq!(my_vec3.z, 2.0);
    /// ```
    pub fn sub(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }

    /// Returns the result of multiplying this vector by the given scalar a new vector.
    ///
    /// # Example
    ///
    /// ```
    /// let my_vec1 = ray_tracer::vector3d::Vector3::new(1.0, 2.0, 3.0);
    ///
    /// let my_vec3 = my_vec1.mul(2.0);
    ///
    /// assert_eq!(my_vec3.x, 2.0);
    /// assert_eq!(my_vec3.y, 4.0);
    /// assert_eq!(my_vec3.z, 6.0);
    /// ```
    pub fn mul(&self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }

    /// Returns the dot product of this vector and the given vector
    ///
    /// The dot product is defined as follows, using v1 as this vector and v2 as the other vector:
    ///  v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    ///
    /// # Example
    ///
    /// ```
    /// let my_vec1 = ray_tracer::vector3d::Vector3::new(1.0, 2.0, 3.0);
    /// let my_vec2 = ray_tracer::vector3d::Vector3::new(2.0, 4.0, 6.0);
    ///
    /// assert_eq!(my_vec1.dot(&my_vec2), 2.0 + 8.0 + 18.0);
    /// ```
    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }
}
