use std::{f64::consts::PI, fs::File, io::Write, path::Path};

use rustic_ray::{shapes::Sphere, Canvas, Color, Point, Ray, Transformation};

fn main() {
    let mut shape = Sphere::new();
    draw_shape(&shape, "ch05_circle.ppm");

    // shrink it along the y axis
    shape.transform = Transformation::new().scale(1.0, 0.5, 1.0).build();
    draw_shape(&shape, "ch05_shrink_y.ppm");

    // shrink it along the x axis
    shape.transform = Transformation::new().scale(0.5, 1.0, 1.0).build();
    draw_shape(&shape, "ch05_shrink_x.ppm");

    // shrink it and rotate it!
    shape.transform = Transformation::new()
        .scale(0.5, 1.0, 1.0)
        .rotate_z(PI / 4.0)
        .build();
    draw_shape(&shape, "ch05_shrink_rotate.ppm");

    // shrink it and skew it!
    shape.transform = Transformation::new()
        .scale(0.5, 1.0, 1.0)
        .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        .build();
    draw_shape(&shape, "ch05_shrink_skew.ppm");
}

fn draw_shape(shape: &Sphere, file_name: &str) {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;

    let wall_size = 7.0;
    let canvas_pixels = 400;

    let pixel_size = wall_size / canvas_pixels as f64;

    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(r);

            if xs.is_some() {
                canvas.write_pixel(x, y, color)
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
