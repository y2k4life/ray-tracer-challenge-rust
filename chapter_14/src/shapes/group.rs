use super::Shape;
use crate::{Intersection, Material, Matrix, Point, Ray, Vector, IDENTITY};

#[derive(Debug)]
pub struct Group {
    id: u64,
    parent_id: Option<u64>,
    pub transform: Matrix,
    pub material: Material,
    pub objects: Vec<Box<dyn Shape>>,
}

impl Group {
    pub fn new() -> Group {
        Group {
            id: crate::next_id(),
            parent_id: None,
            transform: IDENTITY,
            material: Material::new(),
            objects: Vec::new(),
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

impl Shape for Group {
    impl_shape_common!();

    fn shape_eq(&self, other: &dyn Shape) -> bool {
        self.id == other.id()
    }

    fn get_object_by_id(&self, id: u64) -> Option<&dyn Shape> {
        self.objects.iter().find_map(|s| {
            if s.id() == id {
                Some(s.as_ref())
            } else {
                s.get_object_by_id(id)
            }
        })
    }

    fn local_intersect(&self, ray: Ray) -> Option<Vec<Intersection<'_>>> {
        let mut xs: Vec<Intersection> = Vec::new();

        for o in &self.objects {
            if let Some(oxs) = o.intersect(ray) {
                xs.extend(oxs);
            }
        }

        if xs.is_empty() {
            None
        } else {
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Some(xs)
        }
    }

    fn local_normal_at(&self, _point: Point) -> Vector {
        panic!("Should not be called!")
    }
}

impl Default for Group {
    fn default() -> Self {
        Group::new()
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
