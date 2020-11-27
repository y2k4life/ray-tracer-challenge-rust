use crate::{Matrix, Point, Vector};

/// A line which starts at a point and goes off in a particular
/// direction to infinity.
///
/// A ray will have a starting ([`Point`]) called the origin and a ([`Vector`])
/// describing the direction of the ray.
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    // The origin of the ray
    pub origin: Point,
    // The direction of the ray
    pub direction: Vector,
}

impl Ray {
    /// Create a Ray for the given origin and direction.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Point, Ray, Vector};
    ///
    /// let origin = Point::new(1.0, 2.0, 3.0);
    /// let direction = Vector::new(4.0, 5.0, 6.0);
    /// let r = Ray::new(origin, direction);
    ///
    /// assert_eq!(origin, r.origin);
    /// assert_eq!(direction, r.direction);
    /// ```
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    /// Find the position that lie any distance `t` along te ray.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Point, Ray, Vector};
    ///
    /// let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
    ///
    /// assert_eq!(r.position(0.0), Point::new(2.0, 3.0, 4.0));
    /// assert_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
    /// assert_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
    /// assert_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    /// ```
    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn transform(&self, transformation: Matrix) -> Ray {
        Ray::new(
            transformation * self.origin,
            transformation * self.direction,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, Transformation, Vector};

    // Chapter 5 Ray-Sphere Intersections
    // Page 58
    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 58
    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 69
    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Transformation::new().translate(3.0, 4.0, 5.0).build();
        let r2 = r.transform(m);

        assert_eq!(r2.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Vector::new(0.0, 1.0, 0.0));
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 69
    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Transformation::new().scale(2.0, 3.0, 4.0).build();
        let r2 = r.transform(m);

        assert_eq!(r2.origin, Point::new(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Vector::new(0.0, 3.0, 0.0));
    }
}
