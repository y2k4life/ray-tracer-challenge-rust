#[cfg(test)]
use super::Shape;
#[cfg(test)]
use crate::{Intersection, Material, Matrix, Point, Ray, Vector, IDENTITY};
#[cfg(test)]

#[derive(Debug)]
#[cfg(test)]
pub struct TestShape {
    id: u64,
    parent_id: Option<u64>,
    pub transform: Matrix,
    pub material: Material,
}

#[cfg(test)]
impl TestShape {
    pub fn new() -> TestShape {
        TestShape {
            id: crate::next_id(),
            parent_id: None,
            transform: IDENTITY,
            material: Material::new(),
        }
    }
}

#[cfg(test)]
impl Shape for TestShape {
    impl_shape_common!();

    fn local_intersect(&self, ray: Ray) -> Option<Vec<Intersection<'_>>> {
        let t = ray.origin.x
            + ray.origin.y
            + ray.origin.z
            + ray.direction.x
            + ray.direction.y
            + ray.direction.z;
        Some(vec![Intersection::new(t, self)])
    }

    fn local_normal_at(&self, point: Point, _hit: Option<&Intersection>) -> Vector {
        Vector::new(point.x, point.y, point.z)
    }
}

#[cfg(test)]
impl PartialEq for TestShape {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform && self.material == other.material
    }
}
