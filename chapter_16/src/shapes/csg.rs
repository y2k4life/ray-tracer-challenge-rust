use super::Shape;
use crate::{Intersection, Material, Matrix, Point, Ray, Vector, IDENTITY};
use uuid::Uuid;

#[derive(Debug)]
pub struct CSG {
    id: Uuid,
    parent_id: Option<Uuid>,
    left: Box<dyn Shape>,
    right: Box<dyn Shape>,
    operation: CsgOperation,
    pub transform: Matrix,
    pub material: Material,
}

#[derive(Debug, Copy, Clone)]
pub enum CsgOperation {
    Union,
    Intersection,
    Difference,
}

impl CSG {
    pub fn new(
        operation: CsgOperation,
        mut left: Box<dyn Shape>,
        mut right: Box<dyn Shape>,
    ) -> Self {
        let id = Uuid::new_v4();
        left.set_parent_id(id);
        right.set_parent_id(id);
        CSG {
            id,
            parent_id: None,
            left,
            right,
            operation,
            transform: IDENTITY,
            material: Material::new(),
        }
    }

    pub fn filter_intersections<'a>(&'a self, xs: &'a [Intersection]) -> Vec<Intersection> {
        let mut inl = false;
        let mut inr = false;

        let mut results: Vec<Intersection> = Vec::new();

        for i in xs {
            let lhit =
                self.left.id() == i.object.id() || self.left.contains_object_by_id(i.object.id());

            if CSG::intersection_allowed(self.operation, lhit, inl, inr) {
                let c = i.clone();
                results.push(Intersection::new(c.t, c.object));
            }

            if lhit {
                inl = !inl;
            } else {
                inr = !inr;
            }
        }

        results
    }

    pub fn intersection_allowed(operation: CsgOperation, lhit: bool, inl: bool, inr: bool) -> bool {
        match operation {
            CsgOperation::Union => (lhit && !inr) || (!lhit && !inl),
            CsgOperation::Intersection => (lhit && inr) || (!lhit && inl),
            CsgOperation::Difference => (lhit && !inr) || (!lhit && inl),
        }
    }
}

impl Shape for CSG {
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

    fn contains_object_by_id(&self, id: Uuid) -> bool {
        self.left.id() == id || self.right.id() == id
    }

    fn local_intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let mut xs: Vec<Intersection> = Vec::new();

        if let Some(left_xs) = self.left.intersect(ray) {
            for i in left_xs {
                xs.push(i);
            }
        }

        if let Some(right_xs) = self.right.intersect(ray) {
            for i in right_xs {
                xs.push(i);
            }
        }

        if xs.len() > 0 {
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let mut inl = false;
            let mut inr = false;

            let mut results: Vec<Intersection> = Vec::new();

            for i in xs {
                let lhit = self.left.id() == i.object.id()
                    || self.left.contains_object_by_id(i.object.id());

                if CSG::intersection_allowed(self.operation, lhit, inl, inr) {
                    results.push(Intersection::new(i.t, i.object));
                }

                if lhit {
                    inl = !inl;
                } else {
                    inr = !inr;
                }
            }

            Some(results)
        } else {
            None
        }
    }

    fn local_normal_at(&self, point: Point, _hit: Option<&Intersection>) -> Vector {
        Vector::new(point.x, point.y, point.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        shapes::{Cube, Sphere},
        Intersection, Transformation,
    };

    // Chapter 16 Constructive Solid Geometry (CSG)
    // Page 230
    #[test]
    fn csg_is_created_with_an_operation_and_two_shapes() {
        let s1 = Sphere::new();
        let s1_id = s1.id();
        let s2 = Cube::new();
        let s2_id = s2.id();
        let c = CSG::new(CsgOperation::Union, Box::new(s1), Box::new(s2));

        assert!(matches!(c.operation, CsgOperation::Union));
        assert_eq!(c.left.id(), s1_id);
        assert_eq!(c.right.id(), s2_id);
        assert_eq!(c.left.parent_id().unwrap(), c.id);
        assert_eq!(c.right.parent_id().unwrap(), c.id);
    }

    // Chapter 16 Constructive Solid Geometry (CSG)
    // Page 231 & 232 & 233
    #[test]
    fn evaluating_the_rule_for_a_csg_operation() {
        let examples = vec![
            (CsgOperation::Union, true, true, true, false),
            (CsgOperation::Union, true, true, false, true),
            (CsgOperation::Union, true, false, true, false),
            (CsgOperation::Union, true, false, false, true),
            (CsgOperation::Union, false, true, true, false),
            (CsgOperation::Union, false, true, false, false),
            (CsgOperation::Union, false, false, true, true),
            (CsgOperation::Union, false, false, false, true),
            (CsgOperation::Intersection, true, true, true, true),
            (CsgOperation::Intersection, true, true, false, false),
            (CsgOperation::Intersection, true, false, true, true),
            (CsgOperation::Intersection, true, false, false, false),
            (CsgOperation::Intersection, false, true, true, true),
            (CsgOperation::Intersection, false, true, false, true),
            (CsgOperation::Intersection, false, false, true, false),
            (CsgOperation::Intersection, false, false, false, false),
            (CsgOperation::Difference, true, true, true, false),
            (CsgOperation::Difference, true, true, false, true),
            (CsgOperation::Difference, true, false, true, false),
            (CsgOperation::Difference, true, false, false, true),
            (CsgOperation::Difference, false, true, true, true),
            (CsgOperation::Difference, false, true, false, true),
            (CsgOperation::Difference, false, false, true, false),
            (CsgOperation::Difference, false, false, false, false),
        ];

        for e in examples {
            let results = CSG::intersection_allowed(e.0, e.1, e.2, e.3);

            assert_eq!(results, e.4);
        }
    }

    // Chapter 16 Constructive Solid Geometry (CSG)
    // Page 234
    #[test]
    fn filtering_a_list_of_intersections() {
        let examples = vec![
            (CsgOperation::Union, 0, 3),
            (CsgOperation::Intersection, 1, 2),
            (CsgOperation::Difference, 0, 1),
        ];
        for e in examples {
            let s1 = Sphere::new();
            let s2 = Cube::new();
            let c = CSG::new(e.0, Box::new(s1), Box::new(s2));
            let xs = vec![
                Intersection::new(1.0, c.left.as_ref()),
                Intersection::new(2.0, c.right.as_ref()),
                Intersection::new(3.0, c.left.as_ref()),
                Intersection::new(4.0, c.right.as_ref()),
            ];
            let results = c.filter_intersections(&xs);

            assert_eq!(results.len(), 2);
            assert_eq!(results[0].t, xs[e.1].t);
            assert_eq!(results[0].object.id(), xs[e.1].object.id());
            assert_eq!(results[1].t, xs[e.2].t);
            assert_eq!(results[1].object.id(), xs[e.2].object.id());
        }
    }

    // Chapter 16 Constructive Solid Geometry (CSG)
    // Page 234
    #[test]
    fn a_ray_misses_a_csg_object() {
        let c = CSG::new(
            CsgOperation::Union,
            Box::new(Sphere::new()),
            Box::new(Cube::new()),
        );
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = c.local_intersect(r);

        assert!(xs.is_none());
    }

    // Chapter 16 Constructive Solid Geometry (CSG)
    // Page 234
    #[test]
    fn a_ray_hits_a_csg_object() {
        let s1 = Sphere::new();
        let s1_id = s1.id();
        let mut s2 = Sphere::new();
        s2.transform = Transformation::new().translate(0.0, 0.0, 0.5).build();
        let s2_id = s2.id();
        let c = CSG::new(CsgOperation::Union, Box::new(s1), Box::new(s2));
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = c.local_intersect(r).unwrap();

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[0].object.id(), s1_id);
        assert_eq!(xs[1].t, 6.5);
        assert_eq!(xs[1].object.id(), s2_id);
    }
}
