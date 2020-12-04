use std::{f64::consts::PI, fs::File, io::Write, path::Path};

use rustic_ray::{
    shapes::Shape, shapes::Sphere, Canvas, Color, Intersection, Point, PointLight, Ray,
    Transformation,
};

fn main() {
    let mut shape = Sphere::new();
    shape.material.color = Color::new(1.0, 0.2, 1.0);
    draw_shape(&shape, "ch06_circle.ppm");

    // shrink it along the y axis
    shape.transform = Transformation::new().scale(1.0, 0.5, 1.0).build();
    draw_shape(&shape, "ch06_shrink_y.ppm");

    // shrink it along the x axis
    shape.transform = Transformation::new().scale(0.5, 1.0, 1.0).build();
    draw_shape(&shape, "ch06_shrink_x.ppm");

    // shrink it and rotate it!
    shape.transform = Transformation::new()
        .scale(0.5, 1.0, 1.0)
        .rotate_z(PI / 4.0)
        .build();
    draw_shape(&shape, "ch06_shrink_rotate.ppm");

    // shrink it and skew it!
    shape.transform = Transformation::new()
        .scale(0.5, 1.0, 1.0)
        .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        .build();
    draw_shape(&shape, "ch06_shrink_skew.ppm");
}

fn draw_shape(shape: &Sphere, file_name: &str) {
    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;

    let wall_size = 7.0;
    let canvas_pixels = 400;

    let pixel_size = wall_size / canvas_pixels as f64;

    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(r);

            if xs.is_some() {
                if let Some(hit) = Intersection::hit(&xs.unwrap()) {
                    let point = r.position(hit.t);
                    let normal = hit.object.normal_at(point, None);
                    let eye = -r.direction;

                    let color = hit
                        .object
                        .material()
                        .lighting(shape, light, point, eye, normal, false);

                    canvas.pixels[x][y] = color;
                }
            }
        }
    }

    write_file(file_name, canvas.canvas_to_ppm().as_bytes())
}

fn write_file(file_name: &str, ppm: &[u8]) {
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(ppm) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    };
}
