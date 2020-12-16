use crate::{Intersection, Material, Matrix, Point, Ray, Vector, World};
use std::fmt;
use uuid::Uuid;

/// Trait with common functionality for types that describe an object or
/// a graphical primitive. Abstraction of the implementation for a particular
/// shape.
pub trait Shape: 'static + fmt::Debug {
    /// Get the unique identifier for an object.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{shapes::Shape, shapes::Sphere};
    ///
    /// let mut s = Sphere::new();
    ///
    /// assert_eq!(s.id().get_version_num(), 4);
    /// ```
    fn id(&self) -> Uuid;

    /// Get parent id of an `object`
    fn parent_id(&self) -> Option<Uuid>;

    /// Set parent id of an `object`
    fn set_parent_id(&mut self, id: Uuid);

    /// Test if `other` is equal to `self` by comparing their `id`'s.
    fn shape_eq(&self, other: &dyn Shape) -> bool {
        self.id() == other.id()
    }

    /// If the object is a container then get child with `id`.
    fn get_object_by_id(&self, _id: Uuid) -> Option<&dyn Shape> {
        None
    }

    /// Gets the [`Transformation`] [`Matrix`] for an object
    ///
    /// Example
    /// ```
    /// use rustic_ray::{Transformation, shapes::Shape, shapes::Sphere};
    ///
    /// let mut s = Sphere::new();
    /// s.set_transform(Transformation::new().translate(2.0, 3.0, 4.0).build());
    ///
    /// assert_eq!(
    ///     s.transform(),
    ///     Transformation::new().translate(2.0, 3.0, 4.0).build()
    /// );
    /// ```
    fn transform(&self) -> Matrix;

    /// Sets the [`Transformation`] [`Matrix`] for an object
    ///
    /// Example
    /// ```
    /// use rustic_ray::{Transformation, shapes::Shape, shapes::Sphere};
    ///
    /// let mut s = Sphere::new();
    /// s.set_transform(Transformation::new().translate(2.0, 3.0, 4.0).build());
    ///
    /// assert_eq!(
    ///     s.transform(),
    ///     Transformation::new().translate(2.0, 3.0, 4.0).build()
    /// );
    /// ```
    fn set_transform(&mut self, transform: Matrix);

    /// Gets the [`Material`] for an object
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{shapes::Shape, shapes::Sphere};
    ///
    /// let mut s = Sphere::new();
    /// let m = s.material();
    ///
    /// assert_eq!(m.ambient, 0.1);
    /// ```
    fn material(&self) -> &Material;

    /// Gets the [`Material`] as mutable for an object
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{shapes::Shape, shapes::Sphere};
    ///
    /// let mut s = Sphere::new();
    /// s.material_mut().ambient = 1.0;
    ///
    /// assert_eq!(s.material().ambient, 1.0);
    /// ```
    fn material_mut(&mut self) -> &mut Material;

    /// Sets the [`Material`] as mutable for an object
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{Material, shapes::Shape, shapes::Sphere};
    ///
    /// let mut s = Sphere::new();
    /// let mut m = Material::new();
    /// m.ambient = 1.0;
    /// s.set_material(m);
    ///
    /// assert_eq!(s.material().ambient, 1.0);
    /// ```
    fn set_material(&mut self, material: Material);

    /// Specific implementation of how a shape test if the given [`Ray`] intersects
    /// with `self`. Returns a list of [`Intersection`]s between the [`Ray`]
    /// and `self`, the object. Each intersection has the distances, `t`, from the
    /// origin of the [`Ray`] and the shape intersected, `self`. If there are
    /// no intersections then [`None`] is returned. The implementation is called
    /// from the `intersect` function.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{Intersection, Point, shapes::Shape, shapes::Sphere, Ray, Vector};
    ///
    /// let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    /// let s = Sphere::new();
    /// let xs = s.local_intersect(r).expect("Expected hit, found none!");
    ///
    /// assert_eq!(2, xs.len());
    /// assert_eq!(xs[0].t, 4.0);
    /// assert_eq!(xs[1].t, 6.0,);
    /// ```
    fn local_intersect(&self, ray: Ray) -> Option<Vec<Intersection>>;

    /// Specific implementation of a shape to Calculate how the vector that points
    /// perpendicular to a surface at a give point
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{Point, shapes::Shape, shapes::Sphere, Vector};
    ///
    /// let s = Sphere::new();
    /// let n = s.local_normal_at(Point::new(1.0, 0.0, 0.0));
    ///
    /// assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    /// ```
    fn local_normal_at(&self, point: Point) -> Vector;

    /// Test if the given [`Ray`] intersects with `self`. Returns
    /// [`Some`]`(`[`Vec`]`<`[`Intersection`]`>)` which is a list of
    /// intersection(s) between the [`Ray`] and `self`. Each intersection
    /// has the position of the [`Ray`] the intersection occurs at, `t` and
    /// `self` as the object intersected. If there are no intersections
    /// then [`None`] is returned. The implementation to determine if the ray
    /// intersects an object is computed in the `local_intersect`. The default
    /// behavior in `intersect` is to transform the ray from *world space* to
    /// *object space* then call `local_intersect` which determines if and the
    /// ray intersects with the shape.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Point, Ray, shapes::Shape, shapes::Sphere, Vector};
    ///
    /// let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    /// let s = Sphere::new();
    /// let xs = s.intersect(r).unwrap();
    ///
    /// assert_eq!(2, xs.len());
    /// assert_eq!(xs[0].t, 4.0);
    /// assert_eq!(xs[1].t, 6.0,);
    /// ```
    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let local_ray = ray.transform(self.transform().inverse());
        self.local_intersect(local_ray)
    }

    /// Calculates the normal of an object for the give point by performing the
    /// following
    ///
    /// 1. Convert the `point` from a world space to a local space.
    /// 2. Call the implementation of `local_normal_at` for the object to
    /// calculate the normal.
    /// 3. Convert the local space normal to a world space normal
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{Point, shapes::Shape, shapes::Sphere, Vector};
    ///
    /// let s = Sphere::new();
    /// let n = s.normal_at(Point::new(1.0, 0.0, 0.0), None);
    ///
    /// assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    /// ```
    fn normal_at(&self, point: Point, w: Option<&World>) -> Vector {
        match w {
            Some(w) => {
                let local_point = self.world_to_object(point, w);
                let local_normal = self.local_normal_at(local_point);
                self.normal_to_world(local_normal, w)
            }
            None => {
                let local_point = self.transform().inverse() * point;
                let local_normal = self.local_normal_at(local_point);
                (self.transform().inverse().transpose() * local_normal).normalize()
            }
        }
    }

    fn world_to_object(&self, point: Point, w: &World) -> Point {
        let object_point = match self.parent_id() {
            Some(id) => {
                let parent = w.get_object_by_id(id).expect("Shape not found!");
                parent.world_to_object(point, w)
            }
            None => point,
        };

        self.transform().inverse() * object_point
    }

    fn normal_to_world(&self, normal: Vector, w: &World) -> Vector {
        let world_normal = (self.transform().inverse().transpose() * normal).normalize();

        match self.parent_id() {
            Some(id) => {
                let parent = w.get_object_by_id(id).expect("Shape not found!");
                parent.normal_to_world(world_normal, w)
            }
            None => world_normal,
        }
    }
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::{
        shapes::Group,
        shapes::{Sphere, TestShape},
        Transformation, IDENTITY,
    };

    // Chapter 9 Planes
    // page 119
    #[test]
    fn the_default_transformation() {
        let s = TestShape::new();

        assert_eq!(s.transform(), IDENTITY);
    }

    // Chapter 9 Planes
    // page 119
    #[test]
    fn assigning_a_transformation() {
        let mut s = TestShape::new();
        s.set_transform(Transformation::new().translate(2.0, 3.0, 4.0).build());

        assert_eq!(
            s.transform(),
            Transformation::new().translate(2.0, 3.0, 4.0).build()
        );
    }

    // Chapter 9 Planes
    // page 119
    #[test]
    fn the_default_material() {
        let s = TestShape::new();
        let m = s.material();

        assert_eq!(*m, Material::new());
    }

    // Chapter 9 Planes
    // page 119
    #[test]
    fn assigning_a_material() {
        let mut s = TestShape::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.set_material(m);

        assert_eq!(s.material().ambient, 1.0);
    }

    // Chapter 9 Planes
    // page 120
    #[test]
    fn intersecting_a_scaled_shape_with_ray() {
        // This test would require a mutable reference
        // therefore the local_intersect will create an Intersection with
        // the sum of the ray origin value and the ray's direction be the t value.
        // The object intersected will be s value.
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = TestShape::new();
        s.set_transform(Transformation::new().scale(2.0, 2.0, 2.0).build());
        let xs = s.intersect(r).expect("No intersections");

        assert_eq!(xs[0].t, -2.0);
        assert!(s.shape_eq(xs[0].object));
    }

    // Chapter 9 Plane
    // Page 120
    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        // This test would require a mutable reference
        // therefore the local_intersect will create an Intersection with
        // the sum of the ray origin value and the ray's direction be the t value.
        // The object intersected will be s value.
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = TestShape::new();
        s.set_transform(Transformation::new().translate(5.0, 0.0, 0.0).build());
        let xs = s.intersect(r).expect("Not intersections!");

        assert_eq!(xs[0].t, -9.0);
        assert!(s.shape_eq(xs[0].object));
    }

    // Chapter 9 Planes
    // Page 121
    #[test]
    fn computing_normal_on_translated_shape() {
        let mut s = TestShape::new();
        s.set_transform(Transformation::new().translate(0.0, 1.0, 0.0).build());
        let n = s.normal_at(Point::new(0.0, 1.7071, -0.70711), None);

        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    // Chapter 9 Planes
    // Page 121
    #[test]
    fn computer_the_normal_on_a_transformed_shape() {
        let mut s = TestShape::new();
        let m = Transformation::new().scale(1.0, 0.5, 1.0).build();
        s.set_transform(m);
        let n = s.normal_at(
            Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0),
            None,
        );

        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    // Chapter 14 Groups
    // Page 195
    #[test]
    fn a_shape_has_a_parent_attribute() {
        let s = TestShape::new();

        assert!(s.parent_id().is_none());
    }

    // Chapter 14 Groups
    // Page 198
    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let mut w = World::new();

        let mut g1 = Group::new();
        g1.transform = Transformation::new().rotate_y(PI / 2.0).build();

        let mut g2 = Group::new();
        g2.transform = Transformation::new().scale(2.0, 2.0, 2.0).build();

        let mut s = Sphere::new();
        let s_id = s.id();
        s.transform = Transformation::new().translate(5.0, 0.0, 0.0).build();

        g2.add_object(Box::new(s));
        g1.add_object(Box::new(g2));
        w.add_object(Box::new(g1));

        let s = w.get_object_by_id(s_id).unwrap();

        let p = s.world_to_object(Point::new(-2.0, 0.0, -10.0), &w);
        assert_eq!(p, Point::new(0.0, 0.0, -1.0));
    }

    // Chapter 14 Groups
    // Page 198 & 199
    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let mut g1 = Group::new();
        g1.transform = Transformation::new().rotate_y(PI / 2.0).build();

        let mut g2 = Group::new();
        g2.transform = Transformation::new().scale(1.0, 2.0, 3.0).build();

        let mut s = Sphere::new();
        s.transform = Transformation::new().translate(5.0, 0.0, 0.0).build();
        let s_id = s.id();

        let mut w = World::new();

        g2.add_object(Box::new(s));
        g1.add_object(Box::new(g2));
        w.add_object(Box::new(g1));

        let s = w.get_object_by_id(s_id).unwrap();

        let p = s.normal_to_world(
            Vector::new(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0),
            &w,
        );

        assert_eq!(p, Vector::new(0.2857, 0.4286, -0.8571));
    }

    // Chapter 14 Groups
    // Page 199
    #[test]
    fn finding_the_normal_on_a_child_object() {
        let mut g1 = Group::new();
        g1.transform = Transformation::new().rotate_y(PI / 2.0).build();

        let mut g2 = Group::new();
        g2.transform = Transformation::new().scale(1.0, 2.0, 3.0).build();

        let mut s = Sphere::new();
        s.transform = Transformation::new().translate(5.0, 0.0, 0.0).build();
        let s_id = s.id();

        let mut w = World::new();

        g2.add_object(Box::new(s));
        g1.add_object(Box::new(g2));
        w.add_object(Box::new(g1));

        let s = w.get_object_by_id(s_id).unwrap();

        let p = s.normal_at(Point::new(1.7321, 1.1547, -5.5774), Some(&w));

        assert_eq!(p, Vector::new(0.2857, 0.4286, -0.8571));
    }
}
