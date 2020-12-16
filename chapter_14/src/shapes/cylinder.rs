use super::Shape;
#[allow(unused_imports)]
use crate::Transformation;
use crate::{float_eq, Intersection, Material, Matrix, Point, Ray, Vector, EPSILON, IDENTITY};
use std::f64::{INFINITY, NEG_INFINITY};
use uuid::Uuid;

/// A solid geometric figure with straight parallel sides and a circular or oval
/// cross section.
///
/// A cylinder has a default radius of 1 unit and are infinity in both `+y` and
/// `-y`. Cylinders can be truncated in either `y` direction or both. They can
/// also be opened at each end or closed. By default they are open.
#[derive(Debug)]
pub struct Cylinder {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    /// [`Transformation`] matrix used to manipulate the `Cylinder`
    pub transform: Matrix,
    /// [`Material`] describing the look of the `Cylinder`
    pub material: Material,
    /// Maximum length along the y-axis defined in object space
    pub maximum: f64,
    /// Minimum length along the y-axis defined in object space
    pub minimum: f64,
    /// Determine is the ends of the cylinder are open or close
    pub closed: bool,
}

impl Cylinder {
    /// Create a new `Cylinder`.
    pub fn new() -> Cylinder {
        Cylinder {
            id: Uuid::new_v4(),
            parent_id: None,
            transform: IDENTITY,
            material: Material::new(),
            minimum: NEG_INFINITY,
            maximum: INFINITY,
            closed: false,
        }
    }

    fn check_cap(&self, ray: Ray, t: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        x.powi(2) + z.powi(2) <= 1.0
    }

    fn intersect_caps(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let mut xs: Vec<Intersection> = Vec::new();

        if !self.closed || float_eq(ray.direction.y, 0.0) {
            return None;
        }

        let t = (self.minimum - ray.origin.y) / ray.direction.y;
        if self.check_cap(ray, t) {
            xs.push(Intersection::new(t, self));
        }

        let t = (self.maximum - ray.origin.y) / ray.direction.y;
        if self.check_cap(ray, t) {
            xs.push(Intersection::new(t, self));
        }

        if xs.is_empty() {
            None
        } else {
            Some(xs)
        }
    }
}

impl Shape for Cylinder {
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

    fn local_intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

        if float_eq(a, 0.0) {
            return self.intersect_caps(ray);
        }

        let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;

        let disc = b.powi(2) - 4.0 * a * c;

        if disc < 0.0 {
            return None;
        }

        let mut t = (
            (-b - disc.sqrt()) / (2.0 * a),
            (-b + disc.sqrt()) / (2.0 * a),
        );

        if t.0 > t.1 {
            t = (t.1, t.0);
        }

        let mut xs: Vec<Intersection> = Vec::new();

        let y0 = ray.origin.y + t.0 * ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(Intersection::new(t.0, self));
        }

        let y1 = ray.origin.y + t.1 * ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(Intersection::new(t.1, self))
        }

        if let Some(cxs) = self.intersect_caps(ray) {
            for i in cxs {
                xs.push(i)
            }
        }

        if xs.is_empty() {
            None
        } else {
            Some(xs)
        }
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        let dist = point.x.powi(2) + point.z.powi(2);

        if dist < 1.0 && point.y >= self.maximum - EPSILON {
            Vector::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y <= self.minimum + EPSILON {
            Vector::new(0.0, -1.0, 0.0)
        } else {
            Vector::new(point.x, 0.0, point.z)
        }
    }
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform && self.material == other.material
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{shapes::Shape, Point, Ray, Vector};
    use std::f64::{INFINITY, NEG_INFINITY};

    // Chapter 13 Cylinders
    // Page 178 & 179
    #[test]
    pub fn a_ray_misses_a_cylinder() {
        let cyl = Cylinder::new();
        let data = vec![
            (Point::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0)),
        ];

        for rec in data {
            let direction = rec.1;
            let r = Ray::new(rec.0, direction.normalize());
            let xs = cyl.local_intersect(r);
            assert_eq!(None, xs);
        }
    }

    // Chapter 13 Cylinders
    // Page 180
    #[test]
    fn a_ray_intersects_a_cylinder() {
        let c = Cylinder::new();
        let data = vec![
            (
                Point::new(1.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 0.0, -5.0),
                Vector::new(0.1, 1.0, 1.0),
                6.80798,
                7.08872,
            ),
        ];

        for rec in data {
            let direction = rec.1;
            let r = Ray::new(rec.0, direction.normalize());
            let xs = c.local_intersect(r).unwrap();
            assert_eq!(2, xs.len());
            assert!(float_eq(xs[0].t, rec.2));
            assert!(float_eq(xs[1].t, rec.3));
        }
    }

    // Chapter 13 Cylinders
    // Page 181
    #[test]
    fn normal_vector_on_a_cylinder() {
        let c = Cylinder::new();
        let data = vec![
            (Point::new(1.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(0.0, 5.0, -1.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(0.0, -2.0, 1.0), Vector::new(0.0, 0.0, 1.0)),
            (Point::new(-1.0, 1.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
        ];

        for rec in data {
            let p = rec.0;
            let normal = c.local_normal_at(p);
            assert_eq!(rec.1, normal);
        }
    }

    // Chapter 13 Cylinders
    // Page 182
    #[test]
    pub fn the_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = Cylinder::new();

        assert_eq!(cyl.minimum, NEG_INFINITY);
        assert_eq!(cyl.maximum, INFINITY);
    }

    // Chapter 13 Cylinders
    // Page 182 & 183
    #[test]
    pub fn intersecting_a_constrained_cylinder() {
        let mut c = Cylinder::new();
        c.minimum = 1.0;
        c.maximum = 2.0;
        let data = vec![
            (Point::new(0.0, 1.5, 0.0), Vector::new(0.1, 1.0, 0.0), 0),
            (Point::new(0.0, 3.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 1.5, -2.0), Vector::new(0.0, 0.0, 1.0), 2),
        ];

        for rec in data {
            let direction = rec.1;
            let r = Ray::new(rec.0, direction.normalize());
            match c.local_intersect(r) {
                Some(xs) => assert_eq!(rec.2, xs.len()),
                None => assert_eq!(rec.2, 0),
            }
        }
    }

    // Chapter 13 Cylinders
    // Page 185
    #[test]
    pub fn the_default_closed_value_for_a_cylinder() {
        let cyl = Cylinder::new();

        assert_eq!(false, cyl.closed)
    }

    // Chapter 13 Cylinders
    // Page 185
    #[test]
    pub fn intersecting_the_caps_of_a_closed_cylinder() {
        let mut c = Cylinder::new();
        c.minimum = 1.0;
        c.maximum = 2.0;
        c.closed = true;
        let data = vec![
            (Point::new(0.0, 3.0, 0.0), Vector::new(0.0, -1.0, 0.0), 2),
            (Point::new(0.0, 3.0, -2.0), Vector::new(0.0, -1.0, 2.0), 2),
            (Point::new(0.0, 4.0, -2.0), Vector::new(0.0, -1.0, 1.0), 2),
            (Point::new(0.0, 0.0, -2.0), Vector::new(0.0, 1.0, 2.0), 2),
            (Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 1.0), 2),
        ];

        for rec in data {
            let direction = rec.1;
            let r = Ray::new(rec.0, direction.normalize());
            let xs = c.local_intersect(r).unwrap();
            assert_eq!(rec.2, xs.len());
        }
    }

    // Chapter 13
    // Page 187
    #[test]
    pub fn the_normal_vector_on_a_cylinder_end_caps() {
        let mut c = Cylinder::new();
        c.minimum = 1.0;
        c.maximum = 2.0;
        c.closed = true;
        let data = vec![
            (Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.5, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.0, 1.0, 0.5), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.0, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.5, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.0, 2.0, 0.5), Vector::new(0.0, 1.0, 0.0)),
        ];

        for rec in data {
            let n = c.local_normal_at(rec.0);
            assert_eq!(rec.1, n);
        }
    }
}
