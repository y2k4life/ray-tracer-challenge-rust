use super::Shape;
#[allow(unused_imports)]
use crate::Transformation;
use crate::{Intersection, Material, Matrix, Point, Ray, Vector, IDENTITY};
use uuid::Uuid;

/// A sphere is a three-dimensional solid figure which is perfectly round in
/// shape and every point on its surface is equidistant from the point
/// of the origin.
#[derive(Debug, PartialEq)]
pub struct Sphere {
    id: Uuid,
    /// [`Transformation`] matrix used to manipulate the `Sphere`
    pub transform: Matrix,
    /// [`Material`] describing the look of the `Sphere`
    pub material: Material,
}

impl Sphere {
    /// Create a new `Sphere`.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: IDENTITY,
            material: Material::new(),
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

impl Shape for Sphere {
    fn id(&self) -> Uuid {
        self.id
    }

    fn transform(&self) -> Matrix {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_intersect(&self, r: Ray) -> Option<Vec<Intersection>> {
        let mut xs: Vec<Intersection> = Vec::new();

        let sphere_to_ray = r.origin - Point::new(0.0, 0.0, 0.0);
        let a = r.direction.dot(r.direction);

        let b = 2.0 * r.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            xs.push(Intersection::new(t1, self));
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            xs.push(Intersection::new(t2, self));
        }

        if !xs.is_empty() {
            Some(xs)
        } else {
            None
        }
    }

    fn local_normal_at(&self, object_point: Point) -> Vector {
        object_point - Point::new(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Transformation, Vector};
    use std::f64::consts::PI;

    // Chapter 5 Ray-Sphere Intersections
    // Page 59
    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.local_intersect(r).expect("Expected hit, found none!");

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
        let xs = s.local_intersect(r).expect("Expected hit, found none!");

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
        let xs = s.local_intersect(r);

        assert!(xs.is_none());
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 61
    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.local_intersect(r).expect("Expected hit, found none!");

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
        let xs = s.local_intersect(r).expect("Expected hit, found none!");

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 69
    // removed in Chapter 9
    // a_sphere_default_transformation

    // Chapter 5 Ray-Sphere Intersections
    // Page 69
    // removed in Chapter 9
    // changing_a_sphere_transformation

    // Chapter 5 Ray-Sphere Intersections
    // Page 69 & 70
    // removed in Chapter 9
    // intersecting_a_scaled_sphere_with_a_ray()

    // Chapter 5 Ray-Sphere Intersections
    // Page 70
    // Removed in Chapter 9
    /// intersecting_a_translated_sphere_with_a_ray()

    // Chapter 6 Light and Shading
    // Page 78
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(Point::new(1.0, 0.0, 0.0));

        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    // Chapter 6 Light and Shading
    // Page 78
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(Point::new(0.0, 1.0, 0.0));

        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    // Chapter 6 Light and Shading
    // Page 78
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(Point::new(0.0, 0.0, 1.0));

        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    // Chapter 6 Light and Shading
    // Page 78
    #[test]
    fn the_normal_on_sphere_at_point_non_axial_point() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
        ));

        assert_eq!(
            n,
            Vector::new(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0)
        );
    }

    // Chapter 6 Light and Shading
    // Page 78
    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
        ));

        assert_eq!(n, n.normalize());
    }

    // Chapter 6 Light and Shading
    // Page 80
    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Sphere::new();
        s.transform = Transformation::new().translate(0.0, 1.0, 0.0).build();
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(Vector::new(0.0, 0.70711, -0.70711), n);
    }

    // Chapter 6 Light and Shading
    // Page 80
    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut s = Sphere::new();
        s.transform = Transformation::new()
            .rotate_z(PI / 5.0)
            .scale(1.0, 0.5, 1.0)
            .build();
        let n = s.normal_at(Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));

        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    // Chapter 6 Light and Shading
    // Page 85
    // Removed in Chapter 9
    // a_sphere_has_a_default_material()

    // Chapter 6 Light and Shading
    // Page 85
    // Removed in Chapter 9
    // a_sphere_may_be_assigned_material() {
}
