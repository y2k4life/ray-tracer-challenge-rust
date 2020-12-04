#[cfg(test)]
use super::Shape;
#[cfg(test)]
use crate::{Intersection, Material, Matrix, Point, Ray, Vector, IDENTITY};
#[cfg(test)]
use uuid::Uuid;

#[derive(Debug)]
#[cfg(test)]
pub struct TestShape {
    id: Uuid,
    parent_id: Option<Uuid>,
    pub transform: Matrix,
    pub material: Material,
}

#[cfg(test)]
impl TestShape {
    pub fn new() -> TestShape {
        TestShape {
            id: Uuid::new_v4(),
            parent_id: None,
            transform: IDENTITY,
            material: Material::new(),
        }
    }
}

#[cfg(test)]
impl Shape for TestShape {
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
        let t = ray.origin.x
            + ray.origin.y
            + ray.origin.z
            + ray.direction.x
            + ray.direction.y
            + ray.direction.z;
        Some(vec![Intersection::new(t, self)])
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        Vector::new(point.x, point.y, point.z)
    }
}

#[cfg(test)]
impl PartialEq for TestShape {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform && self.material == other.material
    }
}
