use super::Shape;
#[allow(unused_imports)]
use crate::Transformation;
use crate::{Intersection, Material, Matrix, Point, Ray, Vector, EPSILON, IDENTITY};
use uuid::Uuid;

/// A perfectly flat surface that extends infinitely in two dimensions.
///
/// The place extends infinitely far in both teh `x` and `z` dimensions passing
/// through the origin.
#[derive(Debug)]
pub struct Plane {
    id: Uuid,
    parent_id: Option<Uuid>,
    /// [`Transformation`] matrix used to manipulate the `Plane`
    pub transform: Matrix,
    /// [`Material`] describing the look of the `Plane`
    pub material: Material,
}

impl Plane {
    /// Create a new plane.
    pub fn new() -> Self {
        Plane {
            id: Uuid::new_v4(),
            parent_id: None,
            transform: IDENTITY,
            material: Material::new(),
        }
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}

impl Shape for Plane {
    fn id(&self) -> Uuid {
        self.id
    }

    fn parent_id(&self) -> Option<Uuid> {
        self.parent_id
    }

    fn set_parent_id(&mut self, id: Uuid) {
        self.parent_id = Some(id);
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

    fn local_intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        if ray.direction.y.abs() < EPSILON {
            return None;
        }

        let t = -ray.origin.y / ray.direction.y;
        Some(vec![Intersection::new(t, self)])
    }

    fn local_normal_at(&self, _point: Point, _hit: Option<&Intersection>) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Chapter 9 Planes
    // Page 122
    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.local_normal_at(Point::new(0.0, 0.0, 0.0), None);
        let n2 = p.local_normal_at(Point::new(0.0, 0.0, 0.0), None);
        let n3 = p.local_normal_at(Point::new(0.0, 0.0, 0.0), None);

        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }

    // Chapter 9 Planes
    // Page 123
    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = p.local_intersect(r);

        assert_eq!(xs, None);
    }

    // Chapter 9 Planes
    // Page 123
    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = p.local_intersect(r);

        assert_eq!(xs, None);
    }

    // Chapter 9 Planes
    // Page 123
    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let xs = p.local_intersect(r).expect("No intersections");

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert!(p.shape_eq(xs[0].object));
    }

    // Chapter 9 Planes
    // Page 123
    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let xs = p.local_intersect(r).expect("No intersections");

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert!(p.shape_eq(xs[0].object));
    }
}
