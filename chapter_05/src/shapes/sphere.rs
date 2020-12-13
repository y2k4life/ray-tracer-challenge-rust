use uuid::Uuid;

use crate::{Intersection, Matrix, Point, Ray, IDENTITY};

/// A sphere is a three-dimensional solid figure which is perfectly round in
/// shape and every point on its surface is equidistant from the point  
/// of the origin.
#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub id: Uuid,
    pub transform: Matrix,
}

impl Sphere {
    /// Create a new `Sphere`.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: IDENTITY,
        }
    }

    /// Test if the given [`Ray`] intersects with `self`. Returns
    /// [`Some`]`(`[`Vec`]`<`[`Intersection`]`>)` which is a list of
    /// intersection(s) between the [`Ray`] and `self`. Each intersection
    /// has the position of the [`Ray`] the intersection occurs at and the
    /// `Sphere` as the object intersected. If there are no intersections
    /// then [`None`] is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Point, Ray, shapes::Sphere, Vector};
    ///
    /// let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    /// let s = Sphere::new();
    /// let xs = s.intersect(r).expect("Expected hit, found none!");
    ///
    /// assert_eq!(2, xs.len());
    /// assert_eq!(xs[0].t, 4.0);
    /// assert_eq!(xs[1].t, 6.0,);
    /// ```
    pub fn intersect(&self, r: Ray) -> Option<Vec<Intersection>> {
        let mut xs: Vec<Intersection> = Vec::new();

        let r = r.transform(self.transform.inverse());

        let sphere_to_ray = r.origin - Point::new(0.0, 0.0, 0.0);

        let a = r.direction.dot(r.direction);
        let b = 2.0 * r.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        xs.push(Intersection::new(t1, self));

        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        xs.push(Intersection::new(t2, self));

        Some(xs)
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Transformation, Vector, IDENTITY};

    use super::*;

    // Chapter 5 Ray-Sphere Intersections
    // Page 59
    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).expect("Expected hit, found none!");

        assert_eq!(2, xs.len());
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0,);
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 60
    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).expect("Expected hit, found none!");

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0,);
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 60
    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert!(xs.is_none());
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 61
    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).expect("Expected hit, found none!");

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 62
    #[test]
    fn a_sphere_behind_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).expect("Expected hit, found none!");

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 69
    #[test]
    pub fn a_sphere_default_transformation() {
        let s = Sphere::new();

        assert!(s.transform == IDENTITY)
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 69
    #[test]
    pub fn changing_a_sphere_transformation() {
        let mut s = Sphere::new();
        let t = Transformation::new().translate(2.0, 3.0, 4.0).build();
        s.transform = t;

        assert!(s.transform == t);
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 69 & 70
    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = Transformation::new().scale(2.0, 2.0, 2.0).build();
        let xs = s.intersect(r).expect("Expected hit, found none!");

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0)
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 70
    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = Transformation::new().translate(5.0, 0.0, 0.0).build();
        let xs = s.intersect(r);

        assert!(xs.is_none());
    }
}
