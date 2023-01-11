use std::any::Any;

use crate::{Intersection, Material, Matrix, Point, Ray, Vector, EPSILON, IDENTITY};
use uuid::Uuid;

use super::Shape;

#[derive(Debug)]
pub struct Triangle {
    id: Uuid,
    parent_id: Option<Uuid>,
    pub transform: Matrix,
    pub material: Material,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub n1: Option<Vector>,
    pub n2: Option<Vector>,
    pub n3: Option<Vector>,
    e1: Vector,
    e2: Vector,
    normal: Vector,
    smooth_triangle: bool,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Triangle {
            id: Uuid::new_v4(),
            parent_id: None,
            transform: IDENTITY,
            material: Material::new(),
            p1,
            p2,
            p3,
            n1: None,
            n2: None,
            n3: None,
            e1: p2 - p1,
            e2: p3 - p1,
            normal: (p3 - p1).cross(p2 - p1).normalize(),
            smooth_triangle: false,
        }
    }

    pub fn smooth_triangle(
        p1: Point,
        p2: Point,
        p3: Point,
        n1: Vector,
        n2: Vector,
        n3: Vector,
    ) -> Self {
        Triangle {
            id: Uuid::new_v4(),
            parent_id: None,
            transform: IDENTITY,
            material: Material::new(),
            p1,
            p2,
            p3,
            n1: Some(n1),
            n2: Some(n2),
            n3: Some(n3),
            e1: p2 - p1,
            e2: p3 - p1,
            normal: (p3 - p1).cross(p2 - p1).normalize(),
            smooth_triangle: true,
        }
    }
}

impl Shape for Triangle {
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
        let dir_cross_e2 = ray.direction.cross(self.e2);
        let det = self.e1.dot(dir_cross_e2);
        if det.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / det;
        let p1_to_origin = ray.origin - self.p1;
        let u = f * p1_to_origin.dot(dir_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let origin_cross_e1 = p1_to_origin.cross(self.e1);
        let v = f * ray.direction.dot(origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }

        let t = f * self.e2.dot(origin_cross_e1);
        Some(vec![Intersection::intersection_with_uv(t, self, u, v)])
    }

    fn local_normal_at(&self, _point: Point, hit: Option<&Intersection>) -> Vector {
        if self.smooth_triangle {
            let hit = hit.unwrap();

            self.n2.unwrap() * hit.u.unwrap()
                + self.n3.unwrap() * hit.v.unwrap()
                + self.n1.unwrap() * (1.0 - hit.u.unwrap() - hit.v.unwrap())
        } else {
            self.normal
        }
    }

    fn inherit_material(&self) -> bool {
        true
    }

    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Chapter 15 Triangles
    // Page 208
    #[test]
    fn constructing_a_triangle() {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(-1.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 0.0, 0.0);
        let t = Triangle::new(p1, p2, p3);

        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);
        assert_eq!(t.e1, Vector::new(-1.0, -1.0, 0.0));
        assert_eq!(t.e2, Vector::new(1.0, -1.0, 0.0));
        assert_eq!(t.normal, Vector::new(0.0, 0.0, -1.0));
    }

    // Chapter 15 Triangles
    // Page 209
    #[test]
    fn finding_the_normal_on_a_triangle() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );
        let n1 = t.local_normal_at(Point::new(0.0, 0.5, 0.0), None);
        let n2 = t.local_normal_at(Point::new(-0.5, 0.75, 0.0), None);
        let n3 = t.local_normal_at(Point::new(0.5, 0.25, 0.0), None);

        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
        assert_eq!(n3, t.normal);
    }

    // Chapter 15 Triangles
    // Page 210
    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );
        let r = Ray::new(Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 0.0));
        let xs = t.local_intersect(r);

        assert!(xs.is_none());
    }

    // Chapter 15 Triangles
    // Page 211
    #[test]
    fn a_ray_misses_the_p1_to_p3_edge() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );
        let r = Ray::new(Point::new(1.0, 1.0, -2.0), Vector::new(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r);

        assert!(xs.is_none());
    }

    // Chapter 15 Triangles
    // Page 211
    #[test]
    fn a_ray_misses_the_p1_to_p2_edge() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );
        let r = Ray::new(Point::new(-1.0, 1.0, -2.0), Vector::new(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r);

        assert!(xs.is_none());
    }

    // Chapter 15 Triangles
    // Page 211
    #[test]
    fn a_ray_misses_the_p2_to_p3_edge() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );
        let r = Ray::new(Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r);

        assert!(xs.is_none());
    }

    // Chapter 15 Triangles
    // Page 211
    #[test]
    fn a_ray_strikes_a_triangle() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );
        let r = Ray::new(Point::new(0.0, 0.5, -2.0), Vector::new(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r).unwrap();

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 2.0);
    }
}
