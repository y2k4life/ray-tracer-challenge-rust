use crate::{patterns::Stripe, shapes::Shape, Color, Point, PointLight, Vector};

/// Encapsulates the attributes from the Phong reflection model.
///
/// *Ambient reflection* is background light, or light reflected from other
/// objects in the environment. The Phong model treats this as ta constant
/// coloring all points on the surface equally.
///
/// *Diffuse reflection* is light reflected form a matte surface. It depends  
/// only on the angle between the light source and the surface normal.
///
/// *Specular reflection* is the reflection of the light source itself and
/// results in what is called a *specular highlight* - the bright spot on a
/// curved surface. It depends on only on the angle between the flection vector
/// and the eye vector and is controlled by a parameter that is called
/// *shininess*. The higher the shininess, the smaller and tighter the specular
/// highlight.
///
/// Buck, Jamis "The Ray Tracer Challenge" (84)
#[derive(Debug, PartialEq)]
pub struct Material {
    /// Color of the material.
    pub color: Color,
    /// Background light, or light reflected from other objects in the environment.
    pub ambient: f64,
    /// Light reflected form a matte surface.
    pub diffuse: f64,
    /// Reflection of the light source itself and results in what is called
    /// a *specular highlight* - the bright spot on a curved surface. Default
    /// value is 200.0.
    pub specular: f64,
    /// Controlled *specular highlight*. The higher the shininess, the smaller
    /// and tighter the specular highlight.
    pub shininess: f64,

    pub pattern: Option<Stripe>,
}

impl Material {
    /// Create a default material with
    /// ```text
    /// Color: red: 1.0, green: 1.0, blue 1.0
    /// ambient:     0.1
    /// diffuse:     0.9,
    /// specular:    0.9,
    /// shininess: 200.0,
    /// ```
    ///
    /// # Example
    /// ```
    /// use rustic_ray::{Color, Material};
    ///
    /// let m = Material::new();
    ///
    /// assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
    /// assert_eq!(m.ambient, 0.1);
    /// assert_eq!(m.diffuse, 0.9);
    /// assert_eq!(m.specular, 0.9);
    /// assert_eq!(m.shininess, 200.0);
    /// ```
    pub fn new() -> Self {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
        }
    }

    /// Add together the material's ambient, diffuse, and specular components,
    /// weighted by the angels between the different vectors.
    pub fn lighting(
        &self,
        object: &dyn Shape,
        light: PointLight,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        in_shadow: bool,
    ) -> Color {
        let color = match self.pattern {
            Some(p) => p.stripe_at_object(object, point),
            None => self.color,
        };
        // combine the surface color with the light's color/intensity
        let effective_color = color * light.intensity;

        // find the direction to the light source
        let lightv = (light.position - point).normalize();

        // compute the ambient contribution
        let ambient = effective_color * self.ambient;

        // light_dot_normal represents the cosine of the the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let diffuse: Color;
        let specular: Color;
        let light_dot_normal = lightv.dot(normalv);
        if light_dot_normal < 0.0 || in_shadow {
            diffuse = Color::new(0.0, 0.0, 0.0);
            specular = Color::new(0.0, 0.0, 0.0);
        } else {
            // compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of teh the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye <= 0.0 {
                specular = Color::new(0.0, 0.0, 0.0);
            } else {
                // Compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        // Add teh three contributions together to get the final shading
        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{shapes::Sphere, Colors, Point, PointLight, Vector};

    use super::*;

    /// Chapter 6 Light and Shading
    /// Page 85
    #[test]
    fn the_default_material() {
        let m = Material::new();

        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    /// Chapter 6 Light and Shading
    /// Page 86
    #[test]
    fn lighting_with_the_eye_between_light_and_the_surface() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let results = m.lighting(&Sphere::new(), light, position, eyev, normalv, false);

        assert_eq!(results, Color::new(1.9, 1.9, 1.9));
    }

    /// Chapter 6 Light and Shading
    /// Page 86
    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_degree() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let results = m.lighting(&Sphere::new(), light, position, eyev, normalv, false);

        assert_eq!(results, Color::new(1.0, 1.0, 1.0));
    }

    /// Chapter 6 Light and Shading
    /// Page 87
    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let results = m.lighting(&Sphere::new(), light, position, eyev, normalv, false);

        assert_eq!(results, Color::new(0.7364, 0.7364, 0.7364));
    }

    /// Chapter 6 Light and Shading
    /// Page 87
    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let results = m.lighting(&Sphere::new(), light, position, eyev, normalv, false);

        assert_eq!(results, Color::new(1.6364, 1.6364, 1.6364));
    }

    /// Chapter 6 Light and Shading
    /// Page 88
    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let results = m.lighting(&Sphere::new(), light, position, eyev, normalv, false);

        assert_eq!(results, Color::new(0.1, 0.1, 0.1));
    }

    // Chapter 8 Shadows
    // Page 110
    #[test]
    fn lighting_with_surface_in_shadow() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let result = m.lighting(&Sphere::new(), light, position, eyev, normalv, in_shadow);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    // Chapter 10 Patterns
    // Page 129
    #[test]
    fn lighting_with_a_pattern_applied() {
        let mut m = Material::new();

        m.pattern = Some(Stripe::new(Colors::WHITE, Colors::BLACK));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let c1 = m.lighting(
            &Sphere::new(),
            light,
            Point::new(0.9, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );
        let c2 = m.lighting(
            &Sphere::new(),
            light,
            Point::new(1.1, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );
        assert_eq!(c1, Colors::WHITE);
        assert_eq!(c2, Colors::BLACK);
    }
}
