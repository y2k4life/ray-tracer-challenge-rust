use crate::{float_cmp, shapes::Shape, Computations, Ray, World, EPSILON};
use std::cmp::Ordering;

/// Aggregate of the distance, `t`, from a [`Ray`]'s origin and the object that was
/// intersected by a [`Ray`] at the distance `t`.
#[derive(Debug)]
pub struct Intersection<'a> {
    /// Distance from the origin of a [`Ray`] to the intersection.
    pub t: f64,
    /// The object intersected by a ray.
    pub object: &'a dyn Shape,
    /// `u` property
    pub u: Option<f64>,
    /// `v` property
    pub v: Option<f64>,
}

impl<'a> Intersection<'a> {
    /// Constructs a new `Intersection` with the give distance, `t`, from the origin
    /// of a [`Ray`] to the `object` intersected.
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
        Intersection {
            t,
            object,
            u: None,
            v: None,
        }
    }

    /// Constructs a new `Intersection` with the give distance from the origin
    /// of a [`Ray`] to the intersection, the `t` value and the object
    /// intersected. Along with a `u` and `v` properties.
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
    pub fn intersection_with_uv(t: f64, object: &dyn Shape, u: f64, v: f64) -> Intersection {
        Intersection {
            t,
            object,
            u: Some(u),
            v: Some(v),
        }
    }

    /// Compute information related to an `Intersection` returning the
    /// information as [`Computations].
    pub fn prepare_computations<'h>(
        &'h self,
        r: Ray,
        xs: &[Intersection],
        w: Option<&World>,
    ) -> Computations<'h> {
        let point = r.position(self.t);
        let mut normalv = self.object.normal_at(point, Some(self), w);
        let mut inside = false;
        if normalv.dot(-r.direction) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        let over_point = point + normalv * EPSILON;
        let under_point = point - normalv * EPSILON;

        let reflectv = r.direction.reflect(normalv);

        let mut n1 = 0.0;
        let mut n2 = 0.0;
        let mut container: Vec<&dyn Shape> = Vec::new();
        for i in xs {
            if i == self {
                if container.is_empty() {
                    n1 = 1.0;
                } else if let Some(object) = container.last() {
                    n1 = match w {
                        Some(w) => w.get_object_material(*object).refractive_index,
                        None => object.material().refractive_index,
                    }
                }
            }

            if container.contains(&i.object) {
                container = container.into_iter().filter(|o| *o != i.object).collect();
            } else {
                container.push(i.object);
            }

            if i == self {
                if container.is_empty() {
                    n2 = 1.0;
                } else if let Some(object) = container.last() {
                    n2 = match w {
                        Some(w) => w.get_object_material(*object).refractive_index,
                        None => object.material().refractive_index,
                    }
                }

                break;
            }
        }

        Computations {
            t: self.t,
            object: self.object,
            point,
            over_point,
            under_point,
            eyev: -r.direction,
            normalv,
            inside,
            reflectv,
            n1,
            n2,
        }
    }
}

impl Intersection<'_> {
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
    pub fn hit<'a>(xs: &'a [Intersection]) -> Option<&'a Intersection<'a>> {
        xs.iter().filter(|x| x.t >= 0.0).min()
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
    use crate::{
        float_eq, shapes::Plane, shapes::Sphere, shapes::Triangle, Point, Ray, Transformation,
        Vector, EPSILON,
    };

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
        let xs = vec![Intersection::new(4.0, &shape)];
        let comps = i.prepare_computations(r, &xs, None);

        assert_eq!(comps.t, 4.0);
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
        let xs = vec![Intersection::new(4.0, &shape)];
        let comps = i.prepare_computations(r, &xs, None);

        assert!(!comps.inside);
    }

    // Chapter 7 Making a Scene
    // Page 95
    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape);
        let xs = vec![Intersection::new(1.0, &shape)];
        let comps = i.prepare_computations(r, &xs, None);

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
        let xs = vec![Intersection::new(5.0, &shape)];
        let comps = i.prepare_computations(r, &xs, None);

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    // Chapter 11 Reflection and Refraction
    // Page 143
    #[test]
    fn precomputing_reflection_vector() {
        let shape = Plane::new();
        let r = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2_f64.sqrt(), &shape);
        let xs = vec![Intersection::new(2_f64.sqrt(), &shape)];
        let comps = i.prepare_computations(r, &xs, None);
        assert_eq!(
            comps.reflectv,
            Vector::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
        );
    }

    // Chapter 11 Reflection and Refraction
    // Page 152
    #[test]
    fn finding_n1_n2_at_various_intersections() {
        let mut a = Sphere::glass_sphere();
        a.transform = Transformation::new().scale(2.0, 2.0, 2.0).build();
        a.material.refractive_index = 1.5;
        let ia1 = Intersection::new(2.0, &a);
        let ia2 = Intersection::new(6.0, &a);

        let mut b = Sphere::glass_sphere();
        b.transform = Transformation::new().translate(0.0, 0.0, -0.25).build();
        b.material.refractive_index = 2.0;
        let ib1 = Intersection::new(2.75, &b);
        let ib2 = Intersection::new(4.75, &b);

        let mut c = Sphere::glass_sphere();
        c.transform = Transformation::new().translate(0.0, 0.0, 0.25).build();
        c.material.refractive_index = 2.5;
        let ic1 = Intersection::new(3.25, &c);
        let ic2 = Intersection::new(5.35, &c);

        let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::new(0.0, 0.0, 1.0));
        let xs = vec![ia1, ib1, ic1, ib2, ic2, ia2];

        let expected = vec![
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];

        for i in 0..5 {
            let comps = xs[i].prepare_computations(r, &xs, None);
            assert_eq!(expected[i].0, comps.n1);
            assert_eq!(expected[i].1, comps.n2);
        }
    }

    // Chapter 11 Reflection and Refraction
    // Page 154
    #[test]
    fn under_point_offset_below_surface() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut shape = Sphere::glass_sphere();
        shape.transform = Transformation::new().translate(0.0, 0.0, 1.0).build();
        let i = Intersection::new(5.0, &shape);
        let xs = vec![Intersection::new(5.0, &shape)];
        let comps = i.prepare_computations(r, &xs, None);

        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    // Chapter 11 Reflection and Refraction
    // Page 161
    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = Sphere::glass_sphere();
        let r = Ray::new(
            Point::new(0.0, 0.0, 2_f64.sqrt()),
            Vector::new(0.0, 1.0, 0.0),
        );
        let i = Intersection::new(2_f64.sqrt() / 2.0, &shape);
        let xs = vec![
            Intersection::new(-2_f64.sqrt() / 2.0, &shape),
            Intersection::new(2_f64.sqrt() / 2.0, &shape),
        ];

        let comps = i.prepare_computations(r, &xs, None);
        let reflectance = comps.schlick();

        assert!(float_eq(reflectance, 1.0));
    }

    // Chapter 11 Reflection and Refraction
    // Page 162
    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = Sphere::glass_sphere();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let i = Intersection::new(1.0, &shape);
        let xs = vec![
            Intersection::new(-1.0, &shape),
            Intersection::new(1.0, &shape),
        ];
        let comps = i.prepare_computations(r, &xs, None);
        let reflectance = comps.schlick();

        assert!(float_eq(reflectance, 0.04));
    }

    // Chapter 11 Reflection and Refraction
    // Page 163
    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let shape = Sphere::glass_sphere();
        let r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(1.8589, &shape);
        let xs = vec![Intersection::new(1.8589, &shape)];
        let comps = i.prepare_computations(r, &xs, None);
        let reflectance = comps.schlick();

        assert!(float_eq(reflectance, 0.48873));
    }

    // Chapter 15 Triangles
    // Page 221
    #[test]
    fn an_intersection_can_encapsulate_u_and_v() {
        let s = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        );

        let i = Intersection::intersection_with_uv(3.5, &s, 0.2, 0.4);

        assert_eq!(i.u.unwrap(), 0.2);
        assert_eq!(i.v.unwrap(), 0.4);
    }
}
