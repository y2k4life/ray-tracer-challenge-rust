use super::Shape;
#[allow(unused_imports)]
use crate::Transformation;
use crate::{float_eq, Intersection, Material, Matrix, Point, Ray, Vector, EPSILON, IDENTITY};
use std::f64::{INFINITY, NEG_INFINITY};
use uuid::Uuid;

/// Not a cone in the natural sense but a double-napped code. Two cones
/// "nose to nose", with one cone balanced perfectly on the other.
///
/// A `Cone` has a default radius of 1 unit and are infinity in both `+y` and
/// `-y`. `Cone` can be truncated in either `y` direction or both. They can
/// also be opened at each end or closed. By default they are open.
#[derive(Debug)]
pub struct Cone {
    id: Uuid,
    parent_id: Option<Uuid>,
    /// [`Transformation`] matrix used to manipulate the `Cone`
    pub transform: Matrix,
    /// [`Material`] describing the look of the `Cone`
    pub material: Material,
    /// Maximum length along the y-axis defined in object space
    pub maximum: f64,
    /// Minimum length along the y-axis defined in object space
    pub minimum: f64,
    /// Determine is the ends of the `Cone` are open or close
    pub closed: bool,
}

impl Cone {
    pub fn new() -> Cone {
        Cone {
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
        let y = ray.origin.y + t * ray.direction.y;

        x.powi(2) + z.powi(2) <= y.abs()
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

impl Shape for Cone {
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
        let mut xs: Vec<Intersection> = Vec::new();

        let a = ray.direction.x.powi(2) - ray.direction.y.powi(2) + ray.direction.z.powi(2);

        let b = 2.0 * ray.origin.x * ray.direction.x - 2.0 * ray.origin.y * ray.direction.y
            + 2.0 * ray.origin.z * ray.direction.z;

        let c = ray.origin.x.powi(2) - ray.origin.y.powi(2) + ray.origin.z.powi(2);

        if float_eq(a, 0.0) && float_eq(b, 0.0) {
            return None;
        }

        if float_eq(a, 0.0) && b != 0.0 {
            xs.push(Intersection::new(-c / (2.0 * b), self));
        }

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

        let y0 = ray.origin.y + t.0 * ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(Intersection::new(t.0, self));
        }

        let y1 = ray.origin.y + t.1 * ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(Intersection::new(t.1, self))
        }

        match self.intersect_caps(ray) {
            Some(cxs) => {
                for i in cxs {
                    xs.push(i)
                }
            }
            _ => (),
        }

        if xs.is_empty() {
            None
        } else {
            Some(xs)
        }
    }

    fn local_normal_at(&self, point: Point, _hit: Option<&Intersection>) -> Vector {
        let dist = point.x.powi(2) + point.z.powi(2);

        if dist < 1.0 && point.y >= self.maximum - EPSILON {
            Vector::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y <= self.minimum + EPSILON {
            Vector::new(0.0, -1.0, 0.0)
        } else if point.y > 0.0 {
            let y = -((point.x.powi(2) + point.z.powi(2)).sqrt());
            Vector::new(point.x, y, point.z)
        } else {
            let y = (point.x.powi(2) + point.z.powi(2)).sqrt();
            Vector::new(point.x, y, point.z)
        }
    }
}

impl PartialEq for Cone {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform && self.material == other.material
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{shapes::Shape, Point, Ray, Vector};

    // Chapter 13 Cylinders
    // Page 189
    #[test]
    fn a_ray_intersects_a_cone_with_a_ray() {
        let c = Cone::new();
        let data = vec![
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(1.0, 1.0, 1.0),
                8.66025,
                8.66025,
            ),
            (
                Point::new(1.0, 1.0, -5.0),
                Vector::new(-0.5, -1.0, 1.0),
                4.55006,
                49.44994,
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
    // Page 190
    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let c = Cone::new();
        let direction = Vector::new(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(Point::new(0.0, 0.0, -1.0), direction);
        let xs = c.local_intersect(r).unwrap();
        assert_eq!(1, xs.len());
        assert!(float_eq(xs[0].t, 0.35355));
    }

    // Chapter 13 Cylinders
    // Page 190
    #[test]
    fn intersecting_a_cone_end_caps() {
        let mut c = Cone::new();
        c.minimum = -0.5;
        c.maximum = 0.5;
        c.closed = true;
        let data = vec![
            (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0), 0),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 2),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 0.0), 4),
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
    // Page 190 & 191
    #[test]
    pub fn computing_the_normal_vector_on_a_cone() {
        let cone = Cone::new();
        let data = vec![
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)),
            (
                Point::new(1.0, 1.0, 1.0),
                Vector::new(1.0, -2_f64.sqrt(), 1.0),
            ),
            (Point::new(-1.0, -1.0, 0.0), Vector::new(-1.0, 1.0, 0.0)),
        ];
        for rec in data {
            let n = cone.local_normal_at(rec.0, None);
            assert_eq!(n, rec.1);
        }
    }
}
