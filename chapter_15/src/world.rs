use uuid::Uuid;

use crate::{
    shapes::Shape, shapes::Sphere, Color, Colors, Computations, Intersection, Material, Point,
    PointLight, Ray, Transformation,
};

/// A collection of all objects in a scene.
///
/// Routines for intersecting that world with a ray and computer the colors for
/// intersections.
#[derive(Debug)]
pub struct World {
    // Light source of the world.
    pub light: Option<PointLight>,
    objects: Vec<Box<dyn Shape>>,
}

impl World {
    /// Create a world with no objects and no lights.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::World;
    ///
    /// let w = World::new();
    ///
    /// assert!(w.light.is_none());
    /// ```
    pub fn new() -> Self {
        World {
            light: None,
            objects: Vec::new(),
        }
    }

    /// Add an `object` to the world `self`.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{shapes::Shape, shapes::Sphere, World};
    ///
    /// let mut w = World::new();
    /// let s = Sphere::new();
    /// let s_id = s.id();
    /// w.add_object(Box::new(s));
    /// let s = w.get_object(0).unwrap();
    ///
    /// assert_eq!(s.id(), s_id);
    /// ```
    pub fn add_object(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object);
    }

    /// Iterate over all of the objects added to the world. Intersecting each
    /// object with a ray and aggregating the intersections into a single
    /// collection. The collection is sorted.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{Intersection, Point, Ray, Vector, World};
    ///
    /// let w = World::default();
    /// let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    /// let xs = w.intersect_world(r).unwrap();
    ///
    /// assert_eq!(xs.len(), 4);
    /// assert_eq!(xs[0].t, 4.0);
    /// assert_eq!(xs[1].t, 4.5);
    /// assert_eq!(xs[2].t, 5.5);
    /// assert_eq!(xs[3].t, 6.0);
    pub fn intersect_world(&self, r: Ray) -> Option<Vec<Intersection>> {
        let mut xs: Vec<Intersection> = Vec::new();
        for o in &self.objects {
            if let Some(o_xs) = o.intersect(r) {
                for i in o_xs {
                    xs.push(i);
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

    /// Call the `lighting` function for the [`crate::Material`] of a `shape` intersected
    /// by a [`Ray`] to get the [`Color`] at that intersection.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{Color, Intersection, Point, Ray, Vector, World};
    ///
    /// let w = World::default();
    /// let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    /// let shape = w.get_object(0).unwrap();
    /// let i = Intersection::new(4.0, shape);
    /// let comps = i.prepare_computations(r, &vec![Intersection::new(4.0, shape)], Some(&w));
    /// let c = w.shade_hit(&comps, 5);
    ///
    /// assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    /// ```
    pub fn shade_hit(&self, comps: &Computations, remaining: usize) -> Color {
        let shadowed = self.is_shadow(comps.over_point);

        let material = self.get_object_material(comps.object);

        let surface = material.lighting(
            comps.object,
            self.light.expect("World has no light source"),
            comps.over_point,
            comps.eyev,
            comps.normalv,
            shadowed,
        );

        let reflected = self.reflected_color(comps, remaining);
        let refracted = self.refracted_color(comps, remaining);

        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = comps.schlick();
            surface + reflected * reflectance + refracted * (1.0 - reflectance)
        } else {
            surface + reflected + refracted
        }
    }

    /// Returns a [`Color`] for an intersection by doing the following
    ///
    /// 1. Find the [`Intersection`]s of a [`Ray`] by calling `intersect_world`.
    /// 2. Find the `hit` from the resulting intersections.
    /// 3. Return black if there are no intersections.
    /// 4. `prepare_computations` on the `hit` to get the [`Computations`] for
    /// the [`Intersection`].
    /// 5. Call `shade_hit` to get the color at the `hit`.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{Color, Point, Ray, Vector, World};
    ///
    /// let w = World::default();
    /// let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 1.0));
    /// let c = w.color_at(r, 5);
    ///
    /// assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    /// ```
    pub fn color_at(&self, r: Ray, remaining: usize) -> Color {
        match self.intersect_world(r) {
            Some(xs) => match Intersection::hit(&xs) {
                Some(i) => {
                    let comps = i.prepare_computations(r, &xs, Some(self));
                    self.shade_hit(&comps, remaining)
                }
                None => Colors::BLACK,
            },
            None => Colors::BLACK,
        }
    }

    /// Cast a ray, called a *shadow ray*, from the point of an intersection
    /// towards the light source. If an object intersects that *shadow ray* between
    /// the intersection point and the light source, then the point of intersection
    /// is considered to be in shadow, returning `true` otherwise
    /// return `false`.
    pub fn is_shadow(&self, point: Point) -> bool {
        let v = self.light.expect("No light in world!").position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point, direction);
        if let Some(intersections) = self.intersect_world(r) {
            if let Some(hit) = Intersection::hit(&intersections) {
                if hit.t < distance {
                    return true;
                }
            }
        }

        false
    }

    /// Create a new ray originating at the hit's location and pointing in the
    /// direction for the `reflectv`
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::{
    ///     shapes::Plane, Color, Intersection, Point, Ray, Transformation, Vector, World
    /// };
    ///
    /// let mut w = World::default();
    /// let mut shape = Plane::new();
    /// shape.material.reflective = 0.5;
    /// shape.transform = Transformation::new().translate(0.0, -1.0, 0.0).build();
    /// w.add_object(Box::new(shape));
    /// let r = Ray::new(
    ///     Point::new(0.0, 0.0, -3.0),
    ///     Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
    /// );
    /// let i = Intersection::new(2_f64.sqrt(), w.get_object(2).unwrap());
    /// let xs = vec![Intersection::new(2_f64.sqrt(), w.get_object(2).unwrap())];
    /// let comps = i.prepare_computations(r, &xs, None);
    /// let color = w.reflected_color(&comps, 1);
    ///
    /// assert_eq!(color, Color::new(0.190332, 0.237915, 0.1427492));
    /// ```
    pub fn reflected_color(&self, comps: &Computations, remaining: usize) -> Color {
        let material = self.get_object_material(comps.object);
        if material.reflective == 0.0 || remaining < 1 {
            Colors::BLACK
        } else {
            let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
            let color = self.color_at(reflect_ray, remaining - 1);
            color * material.reflective
        }
    }

    pub fn refracted_color(&self, comps: &Computations, remaining: usize) -> Color {
        let material = self.get_object_material(comps.object);
        if material.transparency == 0.0 || remaining == 0 {
            Colors::BLACK
        } else {
            let n_ratio = comps.n1 / comps.n2;
            let cos_i = comps.eyev.dot(comps.normalv);
            let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));

            if sin2_t > 1.0 {
                Colors::BLACK
            } else {
                let cos_t = (1.0 - sin2_t).sqrt();
                let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
                let refract_ray = Ray::new(comps.under_point, direction);
                self.color_at(refract_ray, remaining - 1) * material.transparency
            }
        }
    }

    /// Returns a reference to an `object` at the given index or `None`
    /// if index is out of range.
    pub fn get_object(&self, index: usize) -> Option<&dyn Shape> {
        match self.objects.get(index) {
            Some(o) => Some(o.as_ref()),
            None => None,
        }
    }

    /// Returns a mutable reference to an `object` at the given index or `None`
    /// if index is out of range.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{shapes::Sphere, World};
    ///
    /// let mut w = World::new();
    /// let s = Sphere::new();
    ///
    /// w.add_object(Box::new(s));
    /// let s = w.get_object_mut(0).unwrap();
    /// s.material_mut().diffuse = 2.0;
    ///
    /// assert_eq!(2.0, s.material().diffuse);
    /// ```
    pub fn get_object_mut(&mut self, index: usize) -> Option<&mut dyn Shape> {
        match self.objects.get_mut(index) {
            Some(o) => Some(o.as_mut()),
            None => None,
        }
    }

    pub fn get_object_by_id(&self, id: Uuid) -> Option<&dyn Shape> {
        for s in &self.objects {
            if s.id() == id {
                return Some(s.as_ref());
            }

            if let Some(c) = s.get_object_by_id(id) {
                return Some(c);
            }
        }

        None
    }

    pub fn get_object_material<'a>(&'a self, object: &'a dyn Shape) -> &'a Material {
        let mut root = object;
        loop {
            if root.inherit_material() {
                if let Some(id) = root.parent_id() {
                    root = self.get_object_by_id(id).unwrap();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        root.material()
    }
}

impl Default for World {
    fn default() -> Self {
        let mut w = World::new();

        w.light = Some(PointLight::new(
            Point::new(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));

        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        w.add_object(Box::new(s1));

        let mut s2 = Sphere::new();
        s2.set_transform(Transformation::new().scale(0.5, 0.5, 0.5).build());
        w.add_object(Box::new(s2));

        w
    }
}

#[cfg(test)]
mod tests {
    use crate::{patterns::TestPattern, shapes::Group, shapes::Plane, Material, Ray, Vector};

    use super::*;

    // Chapter 7 Making a Scene
    // Page 92
    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert!(w.objects.is_empty());
        assert!(w.light.is_none());
    }

    // Chapter 7 Making a Scene
    // Page 92
    #[test]
    fn the_default_world() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.set_transform(Transformation::new().scale(0.5, 0.5, 0.5).build());

        let w = World::default();

        assert_eq!(w.light.expect("There are not lights!"), light);
        assert_eq!(w.objects.len(), 2);
        assert_eq!(w.light.expect("No light source"), light);
        // Each object gets an ID therefore the id of the object created in
        // World::default() will not be the same. The transformation and material
        // should be.
        assert_eq!(w.objects[0].transform(), s1.transform());
        assert_eq!(*w.objects[0].material(), *s1.material());
        assert_eq!(w.objects[1].transform(), s2.transform());
        assert_eq!(*w.objects[1].material(), *s2.material());
    }

    // Chapter 7 Making a Scene
    // Page 92 & 93
    #[test]
    fn intersecting_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect_world(r).expect("No intersections found!");

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    // Chapter 7 Making a Scene
    // Page 95
    #[test]
    pub fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.get_object(0).expect("Object not found!");
        let i = Intersection::new(4.0, shape);
        let xs = vec![Intersection::new(4.0, shape)];
        let comps = i.prepare_computations(r, &xs, None);
        let c = w.shade_hit(&comps, 1);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    // Chapter 7 Making a Scene
    // Page 95
    #[test]
    pub fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.light = Some(PointLight::new(
            Point::new(0.0, 0.25, 0.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.get_object(1).expect("Object not found!");
        let i = Intersection::new(0.5, shape);
        let xs = vec![Intersection::new(0.5, shape)];
        let comps = i.prepare_computations(r, &xs, None);
        let c = w.shade_hit(&comps, 1);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    // Chapter 7 Making a Scene
    // Page 96
    #[test]
    pub fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 1.0));
        let c = w.color_at(r, 1);

        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    // Chapter 7 Making a Scene
    // Page 96
    #[test]
    pub fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(r, 1);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    // Chapter 7 Making a Scene
    // Page 96
    #[test]
    pub fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();
        {
            let outer = w.get_object_mut(0).expect("Object not found!");
            outer.material_mut().ambient = 1.0;
            let inner = w.get_object_mut(1).expect("Object not found!");
            inner.material_mut().ambient = 1.0;
        }
        let inner = w.get_object(1).expect("Object not found!");
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w.color_at(r, 1);

        assert_eq!(c, inner.material().color);
    }

    // Chapter 8 Shadows
    // Page 111
    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);

        assert!(!w.is_shadow(p));
    }

    // Chapter 8 Shadows
    // Page 112
    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = Point::new(10.0, -10.0, 10.0);

        assert!(w.is_shadow(p));
    }

    // Chapter 8 Shadows
    // Page 112
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Point::new(-20.0, 20.0, -20.0);

        assert!(!w.is_shadow(p));
    }

    // Chapter 8 Shadows
    // Page 112
    #[test]
    fn there_is_no_shadow_when_object_is_behind_the_point() {
        let w = World::default();
        let p = Point::new(-2.0, 2.0, -2.0);

        assert!(!w.is_shadow(p));
    }

    // Chapter 8 Shadows
    // Page 114
    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::new();
        w.light = Some(PointLight::new(
            Point::new(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));

        let s1 = Sphere::new();
        w.add_object(Box::new(s1));

        let mut s2 = Sphere::new();
        s2.set_transform(Transformation::new().translate(0.0, 0.0, 10.0).build());
        w.add_object(Box::new(s2));

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, w.get_object(1).unwrap());
        let xs = vec![Intersection::new(4.0, w.get_object(1).unwrap())];
        let comps = i.prepare_computations(r, &xs, None);
        let c = w.shade_hit(&comps, 1);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    // Chapter 11 Reflection and Refraction
    // Page 144
    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        w.get_object_mut(0).unwrap().material_mut().ambient = 1.0;
        let i = Intersection::new(1.0, w.get_object(1).unwrap());
        let xs = vec![Intersection::new(1.0, w.get_object(1).unwrap())];
        let comps = i.prepare_computations(r, &xs, None);
        let color = w.reflected_color(&comps, 5);

        assert_eq!(color, Colors::BLACK);
    }

    // Chapter 11 Reflection and Refraction
    // Page 144
    #[test]
    fn reflected_color_reflective_material() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.material.reflective = 0.5;
        shape.transform = Transformation::new().translate(0.0, -1.0, 0.0).build();
        w.add_object(Box::new(shape));
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2_f64.sqrt(), w.get_object(2).unwrap());
        let xs = vec![Intersection::new(2_f64.sqrt(), w.get_object(2).unwrap())];
        let comps = i.prepare_computations(r, &xs, None);
        let color = w.reflected_color(&comps, 1);

        assert_eq!(color, Color::new(0.190332, 0.237915, 0.1427492));
    }

    // Chapter 11 Reflection and Refraction
    // Page 145
    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.material.reflective = 0.5;
        shape.transform = Transformation::new().translate(0.0, -1.0, 0.0).build();
        w.add_object(Box::new(shape));
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2_f64.sqrt(), w.get_object(2).unwrap());
        let xs = vec![Intersection::new(2_f64.sqrt(), w.get_object(2).unwrap())];
        let comps = i.prepare_computations(r, &xs, None);
        let color = w.shade_hit(&comps, 1);

        assert_eq!(color, Color::new(0.87677, 0.92436, 0.82918));
    }

    // Chapter 11 Reflection and Refraction
    // Page 146
    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();
        w.light = Some(PointLight::new(
            Point::new(0.0, 0.0, 0.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let mut lower = Plane::new();
        lower.material.reflective = 1.0;
        lower.transform = Transformation::new().translate(0.0, -1.0, 0.0).build();
        w.add_object(Box::new(lower));
        let mut upper = Plane::new();
        upper.material.reflective = 1.0;
        upper.transform = Transformation::new().translate(0.0, 1.0, 0.0).build();
        w.add_object(Box::new(upper));
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        w.color_at(r, 1);
    }

    // Chapter 11 Reflection and Refraction
    // Page 147
    #[test]
    fn reflected_color_at_maximum_recursive_depth() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.material.reflective = 0.5;
        shape.transform = Transformation::new().translate(0.0, -1.0, 0.0).build();
        w.add_object(Box::new(shape));
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2_f64.sqrt(), w.get_object(1).unwrap());
        let xs = vec![Intersection::new(2_f64.sqrt(), w.get_object(1).unwrap())];
        let comps = i.prepare_computations(r, &xs, None);
        let color = w.reflected_color(&comps, 1);

        assert_eq!(color, Colors::BLACK);
    }

    // Chapter 11 Reflection and Refraction
    // Page 155
    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = World::default();
        let shape = w.get_object(0).unwrap();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, shape);
        let xs = vec![Intersection::new(4.0, shape), Intersection::new(6.0, shape)];
        let comps = i.prepare_computations(r, &xs, None);
        let c = w.refracted_color(&comps, 5);

        assert_eq!(c, Colors::BLACK);
    }

    // Chapter 11 Reflection and Refraction
    // Page 156
    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let w = &mut World::default();
        let mut m = Material::new();
        m.transparency = 1.0;
        m.refractive_index = 1.5;
        w.get_object_mut(0).unwrap().set_material(m);
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, w.get_object(0).unwrap());
        let xs = vec![
            Intersection::new(4.0, w.get_object(0).unwrap()),
            Intersection::new(6.0, w.get_object(0).unwrap()),
        ];
        let comps = i.prepare_computations(r, &xs, None);
        let c = w.refracted_color(&comps, 0);

        assert_eq!(c, Colors::BLACK);
    }

    // Chapter 11 Reflection and Refraction
    // Page 157
    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let mut w = World::default();
        let mut m = Material::new();
        m.transparency = 1.0;
        m.refractive_index = 1.5;
        w.get_object_mut(0).unwrap().set_material(m);
        let r = Ray::new(
            Point::new(0.0, 0.0, 2_f64.sqrt() / 2.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let i = Intersection::new(2_f64.sqrt() / 2.0, w.get_object(0).unwrap());
        let xs = vec![
            Intersection::new(-2_f64.sqrt() / 2.0, w.get_object(0).unwrap()),
            Intersection::new(2_f64.sqrt() / 2.0, w.get_object(0).unwrap()),
        ];
        let comps = i.prepare_computations(r, &xs, None);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Colors::BLACK);
    }

    // Chapter 11 Reflection and Refraction
    // Page 158
    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let mut w = World::default();

        let mut am = Material::new();
        am.ambient = 1.0;
        am.pattern = Some(Box::new(TestPattern::new()));
        w.get_object_mut(0).unwrap().set_material(am);

        let mut bm = Material::new();
        bm.transparency = 1.0;
        bm.refractive_index = 1.5;
        w.get_object_mut(1).unwrap().set_material(bm);

        let r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0));

        let xs = vec![
            Intersection::new(-0.9899, w.get_object(0).unwrap()),
            Intersection::new(-0.4899, w.get_object(1).unwrap()),
            Intersection::new(0.4899, w.get_object(1).unwrap()),
            Intersection::new(0.9899, w.get_object(0).unwrap()),
        ];

        let i = Intersection::new(0.4899, w.get_object(1).unwrap());
        let comps = i.prepare_computations(r, &xs, None);
        let c = w.refracted_color(&comps, 5);

        assert_eq!(c, Color::new(0.0, 0.99888, 0.04725));
    }

    // Chapter 11 Reflection and Refraction
    // Page 159
    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut w = World::default();

        let mut floor = Plane::new();
        floor.transform = Transformation::new().translate(0.0, -1.0, 0.0).build();
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.add_object(Box::new(floor));

        let mut ball = Sphere::new();
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.transform = Transformation::new().translate(0.0, -3.5, -0.5).build();
        w.add_object(Box::new(ball));

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        );

        let i = Intersection::new(2_f64.sqrt(), w.get_object(2).unwrap());
        let xs = vec![Intersection::new(2_f64.sqrt(), w.get_object(2).unwrap())];

        let comps = i.prepare_computations(r, &xs, None);
        let c = w.shade_hit(&comps, 5);

        assert_eq!(c, Color::new(0.93642, 0.68642, 0.68642));
    }

    // Chapter 11 Reflection and Refraction
    // Page 164
    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut w = World::default();

        let mut floor = Plane::new();
        floor.transform = Transformation::new().translate(0.0, -1.0, 0.0).build();
        floor.material.reflective = 0.5;
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.add_object(Box::new(floor));

        let mut ball = Sphere::new();
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.transform = Transformation::new().translate(0.0, -3.5, -0.5).build();
        w.add_object(Box::new(ball));

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        );

        let i = Intersection::new(2_f64.sqrt(), w.get_object(2).unwrap());
        let xs = vec![Intersection::new(2_f64.sqrt(), w.get_object(2).unwrap())];

        let comps = i.prepare_computations(r, &xs, None);
        let c = w.shade_hit(&comps, 5);
        assert_eq!(c, Color::new(0.93391, 0.69643, 0.69243));
    }

    #[test]
    fn get_material_from_top_group() {
        let mut w = World::new();

        let mut ball = Sphere::new();
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.inherit_material = true;
        let ball_id = ball.id();

        let mut g1 = Group::new();
        g1.material.color = Color::new(0.0, 1.0, 0.0);

        let mut g2 = Group::new();
        g2.material.color = Color::new(0.0, 0.0, 1.0);
        g2.inherit_material = true;

        g2.add_object(Box::new(ball));
        g1.add_object(Box::new(g2));
        w.add_object(Box::new(g1));

        let test_object = w.get_object_by_id(ball_id).unwrap();
        let m = w.get_object_material(test_object);

        assert_eq!(m.color, Color::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn get_material_from_2nd_group() {
        let mut w = World::new();

        let mut ball = Sphere::new();
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.inherit_material = true;
        let ball_id = ball.id();

        let mut g1 = Group::new();
        g1.material.color = Color::new(0.0, 1.0, 0.0);

        let mut g2 = Group::new();
        g2.material.color = Color::new(0.0, 0.0, 1.0);

        g2.add_object(Box::new(ball));
        g1.add_object(Box::new(g2));
        w.add_object(Box::new(g1));

        let test_object = w.get_object_by_id(ball_id).unwrap();
        let m = w.get_object_material(test_object);

        assert_eq!(m.color, Color::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn get_material_from_self() {
        let mut w = World::new();

        let mut ball = Sphere::new();
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        let ball_id = ball.id();

        let mut g1 = Group::new();
        g1.material.color = Color::new(0.0, 1.0, 0.0);

        let mut g2 = Group::new();
        g2.material.color = Color::new(0.0, 0.0, 1.0);

        g2.add_object(Box::new(ball));
        g1.add_object(Box::new(g2));
        w.add_object(Box::new(g1));

        let test_object = w.get_object_by_id(ball_id).unwrap();
        let m = w.get_object_material(test_object);

        assert_eq!(m.color, Color::new(1.0, 0.0, 0.0));
    }
}
