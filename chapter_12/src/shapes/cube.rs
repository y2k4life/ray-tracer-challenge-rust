use uuid::Uuid;
use super::Shape;
use crate::{IDENTITY, Intersection, Material, Matrix, Point, Ray, Vector, float_cmp};

#[derive(Debug)]
pub struct Cube {
    id: Uuid,
    pub parent_id: Option<Uuid>,
    pub transform: Matrix,
    pub material: Material,
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            id: Uuid::new_v4(),
            parent_id: None,
            transform: IDENTITY,
            material: Material::new(),
        }
    }

    fn check_axis(&self, origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let tmin = tmin_numerator / direction;
        let tmax = tmax_numerator / direction;

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }
}

impl Shape for Cube {
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

    fn local_intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let (xtmin, xtmax) = self.check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = self.check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = self.check_axis(ray.origin.z, ray.direction.z);

        let min_values = [xtmin, ytmin, ztmin];
        let tmin = min_values.iter().max_by(|x, y| float_cmp(**x, **y));
        let max_values = [xtmax, ytmax, ztmax];
        let tmax = max_values.iter().min_by(|x, y| float_cmp(**x, **y));

        let tmin = *tmin.unwrap();
        let tmax = *tmax.unwrap();

        if tmin > tmax {
            None
        } else {
            Some(vec![
                Intersection::new(tmin, self),
                Intersection::new(tmax, self),
            ])
        }
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        let max_values = [point.x.abs(), point.y.abs(), point.z.abs()];
        let maxc = max_values.iter().max_by(|x, y| float_cmp(**x, **y));

        let maxc = *maxc.unwrap();

        if maxc == point.x.abs() {
            Vector::new(point.x, 0.0, 0.0)
        } else if maxc == point.y.abs() {
            Vector::new(0.0, point.y, 0.0)
        } else {
            Vector::new(0.0, 0.0, point.z)
        }
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform && self.material == other.material
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Chapter 12 Cubes
    // Page 168
    #[test]
    fn a_ray_intersects_a_cube() {
        let c = Cube::new();
        let data = vec![
            (
                Point::new(5.0, 0.5, 0.0),
                Vector::new(-1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(-5.0, 0.5, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 5.0, 0.0),
                Vector::new(0.0, -1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, -5.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 0.0, 5.0),
                Vector::new(0.0, 0.0, -1.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.5, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                Point::new(0.0, 0.5, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                -1.0,
                1.0,
            ),
        ];
        for rec in data {
            let r = Ray::new(rec.0, rec.1);
            let xs = c.local_intersect(r).unwrap();
            assert_eq!(2, xs.len());
            assert_eq!(xs[0].t, rec.2);
            assert_eq!(xs[1].t, rec.3);
        }
    }

    // Chapter 12 Cubes
    // Page 172
    #[test]
    fn a_ray_misses_a_cube() {
        let c = Cube::new();
        let data = vec![
            (
                Point::new(-2.0, 0.0, 0.0),
                Vector::new(0.2673, 0.5345, 0.8018),
            ),
            (
                Point::new(0.0, -2.0, 0.0),
                Vector::new(0.8018, 0.2673, 0.5345),
            ),
            (
                Point::new(0.0, 0.0, -2.0),
                Vector::new(0.5345, 0.8018, 0.2673),
            ),
            (Point::new(2.0, 0.0, 2.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(0.0, 2.0, 2.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(2.0, 2.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for rec in data {
            let r = Ray::new(rec.0, rec.1);
            let xs = c.local_intersect(r);
            assert_eq!(None, xs);
        }
    }

    // Chapter 12 Cubes
    // Page 173 & 174
    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let c = Cube::new();
        let data = vec![
            (Point::new(1.0, 0.5, -0.8), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(-1.0, -0.2, 0.9), Vector::new(-1.0, 0.0, 0.0)),
            (Point::new(-0.4, 1.0, -0.1), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.3, -1.0, -0.7), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(-0.6, 0.3, 1.0), Vector::new(0.0, 0.0, 1.0)),
            (Point::new(0.4, 0.4, -1.0), Vector::new(0.0, 0.0, -1.0)),
            (Point::new(1.0, 1.0, 1.0), Vector::new(1.0, 0.0, 0.0)),
            (Point::new(-1.0, -1.0, -1.0), Vector::new(-1.0, 0.0, 0.0)),
        ];
        for rec in data {
            let p = rec.0;
            let normal = c.local_normal_at(p);
            assert_eq!(rec.1, normal);
        }
    }
}
