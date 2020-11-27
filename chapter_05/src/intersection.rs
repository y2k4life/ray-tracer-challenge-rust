use crate::float_cmp;
use crate::shapes::Sphere;
use std::cmp::Ordering;

/// Aggregate the position of an intersection and the object that was
/// intersected.
#[derive(Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    /// Create a new `intersection` for the give position and object.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Intersection, shapes::Sphere};
    ///
    /// let s = Sphere::new();
    /// let i = Intersection::new(3.5, &s);
    ///
    /// assert_eq!(i.t, 3.5);
    /// assert_eq!(*i.object, s);
    /// ```
    pub fn new(t: f64, object: &Sphere) -> Intersection {
        Intersection { t, object }
    }

    /// Identify which intersection of all teh intersections are visible from
    /// the ray's origin. The `hit` is the intersection with the lowest
    /// nonnegative `t` value.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Intersection, shapes::Sphere};
    ///
    /// let s = Sphere::new();
    /// let i1 = Intersection::new(1.0, &s);
    /// let i2 = Intersection::new(2.0, &s);
    /// let xs = vec![i2, i1];
    /// let i = Intersection::hit(&xs).expect("Intersection did not hit!");
    ///
    /// assert_eq!(*i, xs[1]);
    /// ```
    pub fn hit(xs: &'a [Intersection]) -> Option<&'a Intersection<'a>> {
        xs.iter().filter(|x| x.t >= 0.0).min()
    }
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Intersection) -> bool {
        self.t == other.t && self.object == other.object
    }
}

impl PartialOrd for Intersection<'_> {
    fn partial_cmp(&self, other: &Intersection) -> Option<Ordering> {
        Some(float_cmp(self.t, other.t))
    }
}

impl Eq for Intersection<'_> {}

impl Ord for Intersection<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        float_cmp(self.t, other.t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, Ray, Vector};

    // Chapter 5 Ray-Sphere Intersections
    // Page 63
    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert_eq!(*i.object, s);
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 64
    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let xs = vec![Intersection::new(1.0, &s), Intersection::new(2.0, &s)];

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 64
    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).expect("No intersections!");

        assert_eq!(xs.len(), 2);
        assert_eq!(*xs[0].object, s);
        assert_eq!(*xs[1].object, s);
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 65
    #[test]
    fn the_hit_when_all_intersections_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i2, i1];
        let i = Intersection::hit(&xs).expect("Intersection did not hit!");

        assert_eq!(*i, xs[1]);
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 65
    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i2, i1];
        let i = Intersection::hit(&xs).expect("Intersection did not hit!");

        assert_eq!(*i, xs[0]);
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 65
    #[test]
    fn the_hit_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i2, i1];
        let i = Intersection::hit(&xs);

        assert!(i.is_none());
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 66
    #[test]
    fn the_hit_is_always_lowest_nonnegative_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2, i3, i4];
        let i = Intersection::hit(&xs).expect("Expected hit intersection");

        assert_eq!(*i, xs[3]);
    }
}
