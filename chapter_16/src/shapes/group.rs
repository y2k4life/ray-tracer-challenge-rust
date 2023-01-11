use std::any::Any;

use super::Shape;
use crate::{Intersection, Material, Matrix, Point, Ray, Vector, IDENTITY};
use uuid::Uuid;

#[derive(Debug)]
pub struct Group {
    id: Uuid,
    parent_id: Option<Uuid>,
    pub transform: Matrix,
    pub material: Material,
    pub objects: Vec<Box<dyn Shape>>,
    pub inherit_material: bool,
}

impl Group {
    pub fn new() -> Group {
        Group {
            id: Uuid::new_v4(),
            parent_id: None,
            transform: IDENTITY,
            material: Material::new(),
            objects: Vec::new(),
            inherit_material: false,
        }
    }

    pub fn add_object(&mut self, mut shape: Box<dyn Shape>) {
        shape.set_parent_id(self.id);
        self.objects.push(shape);
    }

    pub fn get_object(&self, index: usize) -> Option<&dyn Shape> {
        match self.objects.get(index) {
            Some(o) => Some(o.as_ref()),
            None => None,
        }
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}

impl Shape for Group {
    fn id(&self) -> Uuid {
        self.id
    }

    fn parent_id(&self) -> Option<Uuid> {
        self.parent_id
    }

    fn set_parent_id(&mut self, id: Uuid) {
        self.parent_id = Some(id);
    }

    fn shape_eq(&self, other: &dyn Shape) -> bool {
        self.id == other.id()
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

    fn get_object_by_id(&self, id: Uuid) -> Option<&dyn Shape> {
        let mut shape = None;
        for s in &self.objects {
            if s.id() == id {
                shape = Some(s.as_ref());
                break;
            }
            if let Some(c) = s.get_object_by_id(id) {
                shape = Some(c);
                break;
            }
        }

        shape
    }

    fn contains_object_by_id(&self, id: Uuid) -> bool {
        let mut contains = false;
        for s in &self.objects {
            if s.id() == id {
                contains = true;
                break;
            }
            if s.get_object_by_id(id).is_some() {
                contains = true;
                break;
            }
        }

        contains
    }

    fn local_intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let mut xs: Vec<Intersection> = Vec::new();

        for o in &self.objects {
            if let Some(oxs) = o.intersect(ray) {
                for ox in oxs {
                    xs.push(ox);
                }
            }
        }

        if xs.is_empty() {
            None
        } else {
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Some(xs)
        }
    }

    fn local_normal_at(&self, _point: Point, _hit: Option<&Intersection>) -> Vector {
        panic!("Should not be called!")
    }

    fn inherit_material(&self) -> bool {
        self.inherit_material
    }

    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shapes::{Sphere, TestShape},
        Transformation,
    };

    use super::*;

    // Chapter 14 Groups
    // Page 195
    #[test]
    fn creating_a_new_group() {
        let g = Group::new();

        assert!(g.objects.is_empty());
        assert_eq!(g.transform, IDENTITY);
    }

    // Chapter 14 Groups
    // Page 195
    #[test]
    fn adding_a_child_to_a_group() {
        let mut g = Group::new();
        let mut s = TestShape::new();
        s.set_parent_id(g.id);
        g.add_object(Box::new(s));

        assert!(!g.objects.is_empty());
        assert_eq!(g.objects[0].parent_id().unwrap(), g.id());
    }

    // Chapter 14 Groups
    // Page 196
    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let g = Group::new();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = g.local_intersect(r);

        assert!(xs.is_none());
    }

    // Chapter 14 Groups
    // Page 196
    #[test]
    fn intersecting_a_ray_with_a_none_empty_group() {
        let mut g = Group::new();

        let s1 = Sphere::new();
        let s1_id = s1.id();

        let mut s2 = Sphere::new();
        s2.transform = Transformation::new().translate(0.0, 0.0, -3.0).build();
        let s2_id = s2.id();

        let mut s3 = Sphere::new();
        s3.transform = Transformation::new().translate(5.0, 0.0, 0.0).build();

        g.add_object(Box::new(s1));
        g.add_object(Box::new(s2));
        g.add_object(Box::new(s3));

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = g.intersect(r).unwrap();

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object.id(), s2_id);
        assert_eq!(xs[1].object.id(), s2_id);
        assert_eq!(xs[2].object.id(), s1_id);
        assert_eq!(xs[3].object.id(), s1_id);
    }

    // Chapter 14 Groups
    // Page 197
    #[test]
    fn intersecting_a_transformed_group() {
        let mut g = Group::new();
        g.transform = Transformation::new().scale(2.0, 2.0, 2.0).build();

        let mut s = Sphere::new();
        s.transform = Transformation::new().translate(5.0, 0.0, 0.0).build();

        g.add_object(Box::new(s));

        let r = Ray::new(Point::new(10.0, 0.0, -10.0), Vector::new(0.0, 0.0, 1.0));

        let xs = g.intersect(r).unwrap();
        assert_eq!(xs.len(), 2);
    }
}
