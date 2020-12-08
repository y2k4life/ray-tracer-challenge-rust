#[cfg(test)]
mod tests {
    use crate::{float_eq, shapes::Shape, shapes::Triangle, Intersection, Point, Ray, Vector};

    pub struct Background {}

    impl Background {
        pub const P1: Point = Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        pub const P2: Point = Point {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        };
        pub const P3: Point = Point {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        pub const N1: Vector = Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        pub const N2: Vector = Vector {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        };
        pub const N3: Vector = Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
    }

    // Chapter 15 Triangles
    // Page 221
    #[test]
    fn constructing_a_smooth_triangle() {
        let tri = Triangle::smooth_triangle(
            Background::P1,
            Background::P2,
            Background::P3,
            Background::N1,
            Background::N2,
            Background::N3,
        );

        assert_eq!(tri.p1, Background::P1);
        assert_eq!(tri.p2, Background::P2);
        assert_eq!(tri.p3, Background::P3);
        assert_eq!(tri.n1.unwrap(), Background::N1);
        assert_eq!(tri.n2.unwrap(), Background::N2);
        assert_eq!(tri.n3.unwrap(), Background::N3);
    }

    // Chapter 15 Triangles
    // Page 222
    #[test]
    fn an_intersection_with_a_smooth_triangle_stores_u_v() {
        let tri = Triangle::smooth_triangle(
            Background::P1,
            Background::P2,
            Background::P3,
            Background::N1,
            Background::N2,
            Background::N3,
        );
        let r = Ray::new(Point::new(-0.2, 0.3, -2.0), Vector::new(0.0, 0.0, 1.0));
        let xs = tri.local_intersect(r).unwrap();

        assert!(float_eq(xs[0].u.unwrap(), 0.45));
        assert!(float_eq(xs[0].v.unwrap(), 0.25));
    }

    // Chapter 15 Triangles
    // Page 222
    #[test]
    fn a_smooth_triangle_uses_u_and_v_to_interpolate_the_normal() {
        let tri = Triangle::smooth_triangle(
            Background::P1,
            Background::P2,
            Background::P3,
            Background::N1,
            Background::N2,
            Background::N3,
        );
        let i = Intersection::intersection_with_uv(1.0, &tri, 0.45, 0.25);
        let n = tri.normal_at(Point::new(0.0, 0.0, 0.0), Some(&i), None);

        assert_eq!(n, Vector::new(-0.5547, 0.83205, 0.0));
    }

    // Chapter 15 Triangles
    // Page 223
    #[test]
    fn preparing_the_normal_on_a_smooth_triangle() {
        let tri = Triangle::smooth_triangle(
            Background::P1,
            Background::P2,
            Background::P3,
            Background::N1,
            Background::N2,
            Background::N3,
        );
        let i = Intersection::intersection_with_uv(1.0, &tri, 0.45, 0.25);
        let r = Ray::new(Point::new(-0.2, 0.3, -2.0), Vector::new(0.0, 0.0, 1.0));
        let xs = vec![Intersection::intersection_with_uv(1.0, &tri, 0.45, 0.25)];
        let comps = i.prepare_computations(r, &xs, None);

        assert_eq!(comps.normalv, Vector::new(-0.5547, 0.83205, 0.0));
    }
}
