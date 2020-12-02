use crate::{float_cmp, shapes::Shape, Computations, Ray, EPSILON};
use std::cmp::Ordering;

/// Aggregate of the distance from a [`Ray`]'s origin and the object that was
/// intersected by a [`Ray`] at that distance.
#[derive(Debug)]
pub struct Intersection<'a> {
    /// Distance from the origin of a [`Ray`] to the intersection.
    pub t: f64,
    /// The object intersected by a ray.
    pub object: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    /// Constructs a new `Intersection` with the give distance from the origin
    /// of a [`Ray`] to the intersection, the `t` value and the object
    /// intersected.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Intersection, shapes::Shape, shapes::Sphere};
    ///
    /// let s = Sphere::new();
    /// let i = Intersection::new(3.5, &s);
    ///
    /// assert_eq!(i.t, 3.5);
    /// assert!(s.shape_eq(i.object));
    /// ```
    pub fn new(t: f64, object: &dyn Shape) -> Intersection {
        Intersection { t, object }
    }

    /// Identify which intersection from a list of intersections is visible
    /// from the ray's origin looking out in the direction of the ray. The `hit`
    /// is the intersection with the shortest distance from the origin going in
    /// a positive direction, the `t` value. A negative distance is behind the
    /// origin of the the ray and can't be seen. The shortest or lowest `t` value
    /// in a positive direction is the closest to the origin, the intersection(s)
    /// that are greater are behind the `hit` and can't be seen because the `hit`
    /// is blocking them.
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

    /// Compute information related to an `Intersection` returning the
    /// information as [`Computations].
    pub fn prepare_computations<'h>(&'h self, r: Ray) -> Computations<'h> {
        let point = r.position(self.t);
        let mut normalv = self.object.normal_at(point);
        let mut inside = false;
        if normalv.dot(-r.direction) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        let over_point = point + normalv * EPSILON;

        Computations {
            t: self.t,
            object: self.object,
            point,
            over_point,
            eyev: -r.direction,
            normalv,
            inside,
        }
    }
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Intersection) -> bool {
        self.t == other.t && self.object.shape_eq(other.object)
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
    use crate::{shapes::Sphere, Point, Ray, Transformation, Vector, EPSILON};

    // Chapter 5 Ray-Sphere Intersections
    // Page 63
    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert!(s.shape_eq(i.object));
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
        assert!(s.shape_eq(xs[0].object));
        assert!(s.shape_eq(xs[1].object));
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

    // Chapter 7 Making a Scene
    // Page 93
    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(r);

        assert_eq!(comps.t, i.t);
        assert!(shape.shape_eq(comps.object));
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    // Chapter 7 Making a Scene
    // Page 94
    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(r);

        assert!(!comps.inside);
    }

    // Chapter 7 Making a Scene
    // Page 95
    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape);
        let comps = i.prepare_computations(r);

        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    // Chapter 8 Shadows
    // Page 115
    #[test]
    fn the_hit_should_offset_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut shape = Sphere::new();
        shape.set_transform(Transformation::new().translate(0.0, 0.0, 1.0).build());
        let i = Intersection::new(5.0, &shape);
        let comps = i.prepare_computations(r);

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
