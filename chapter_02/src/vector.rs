use crate::float_eq;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// An element that has magnitude and direction that is commonly
/// represented by a directed line segment whose length represents the
/// magnitude and whose orientation in space represents the direction.
///
/// It is assumed a `Vector` has an initial point at the origin and has
/// three floating point numbers ([`f64`]) representing the terminal point.
#[derive(Debug, Copy, Clone)]
pub struct Vector {
    /// The distance the terminal point of the vector is from the origin
    /// measured along the X axis.
    pub x: f64,
    /// The distance the terminal point of the vector is from the origin
    /// measured along the Y axis.
    pub y: f64,
    /// The distance the terminal point of teh vector is from the origin
    /// measured along the Z axis.
    pub z: f64,
}

impl Vector {
    /// Creates a new [`Vector`] with a terminal point created from three
    /// [`f64`] numbers
    ///
    /// # Example
    /// ```
    /// use rustic_ray::Vector;
    ///
    /// let v = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!(1.0, v.x);
    /// assert_eq!(2.0, v.y);
    /// assert_eq!(3.0, v.z);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    /// Computes the length or the magnitude of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::Vector;
    ///
    /// let v = Vector::new(1.0, 2.0, 3.0);
    ///
    /// assert_eq!(v.magnitude(), 14_f64.sqrt());
    /// ```
    pub fn magnitude(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Computes a unit vector of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::Vector;
    ///
    /// let v = Vector::new(4.0, 0.0, 0.0);
    ///
    /// assert_eq!(v.normalize(), Vector::new(1.0, 0.0, 0.0));
    /// ```
    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    /// Computes a single number of `self` and a [`Vector`]. The formula is
    /// the sum of the products of corresponding coordinates between `self`
    /// and a [`Vector`].
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::Vector;
    ///
    /// let a = Vector::new(1.0, 2.0, 3.0);
    /// let b = Vector::new(2.0, 3.0, 4.0);
    ///
    /// assert_eq!(a.dot(b), 20.0);
    /// ```
    pub fn dot(self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Computes a [`Vector`] that is perpendicular to both `self` and a [`Vector`].
    ///
    /// # Example
    /// ```
    /// use rustic_ray::Vector;
    ///
    /// let v1 = Vector::new(1.0, 2.0, 3.0);
    /// let v2 = Vector::new(2.0, 3.0, 4.0);
    ///
    /// assert_eq!(v1.cross(v2), Vector::new(-1.0, 2.0, -1.0));
    /// assert_eq!(v2.cross(v1), Vector::new(1.0, -2.0, 1.0));
    /// ```
    pub fn cross(self, b: Vector) -> Vector {
        Vector {
            x: self.y * b.z - self.z * b.y,
            y: self.z * b.x - self.x * b.z,
            z: self.x * b.y - self.y * b.x,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y) && float_eq(self.z, other.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Chapter 1 Tuples, Points, and Vectors
    // page 4
    #[test]
    fn create_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, v.x);
        assert_eq!(2.0, v.y);
        assert_eq!(3.0, v.z);
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 6
    #[test]
    fn add_two_vectors_returns_a_vector() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);

        assert_eq!(v1 + v2, Vector::new(1.0, 1.0, 6.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 7
    #[test]
    fn subtracting_two_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, Vector::new(-2.0, -4.0, -6.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 7
    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Vector::new(0.0, 0.0, 0.0);
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(zero - v, Vector::new(-1.0, 2.0, -3.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 7
    #[test]
    fn negating_vector() {
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(-v, Vector::new(-1.0, 2.0, -3.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 8
    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(3.5 * v, Vector::new(3.5, -7.0, 10.5));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 8
    #[test]
    fn multiplying_vector_by_fraction() {
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(0.5 * v, Vector::new(0.5, -1.0, 1.5));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 8
    #[test]
    fn divide_vector_by_float() {
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(v / 2.0, Vector::new(0.5, -1.0, 1.5));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 8
    #[test]
    fn compute_magnitude_for_vector_1_0_0() {
        let v = Vector::new(1.0, 0.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 9
    #[test]
    fn compute_magnitude_for_vector_0_1_0() {
        let v = Vector::new(0.0, 1.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 9
    #[test]
    fn compute_magnitude_for_vector_0_0_1() {
        let v = Vector::new(0.0, 0.0, 1.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 9
    #[test]
    fn compute_magnitude_for_vector_1_2_3() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v.magnitude(), 14_f64.sqrt());
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 9
    #[test]
    fn compute_magnitude_for_vector_neg_1_2_3() {
        let v = Vector::new(-1.0, -2.0, -3.0);

        assert_eq!(v.magnitude(), 14_f64.sqrt());
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 10
    #[test]
    fn normalizing_vector_4_0_0_gives_vector_1_0_0() {
        let v = Vector::new(4.0, 0.0, 0.0);

        assert_eq!(v.normalize(), Vector::new(1.0, 0.0, 0.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 10
    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v.normalize(), Vector::new(0.26726, 0.53452, 0.80178));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 10
    #[test]
    fn the_magnitude_of_a_normalized_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let norm = v.normalize();

        assert_eq!(norm.magnitude(), 1.0);
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 10
    #[test]
    fn the_dot_product_of_two_vectors() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(a.dot(b), 20.0);
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 11
    #[test]
    fn the_cross_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v1.cross(v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(v1), Vector::new(1.0, -2.0, 1.0));
    }
}
