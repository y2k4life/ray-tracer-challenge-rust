use rustic_ray::{shapes::Sphere, Camera, Color, Point, PointLight, Transformation, Vector, World};
use std::{f64::consts::PI, fs::File, io::Write, path::Path};

fn main() {
    let mut world = World::new();

    let mut floor = Sphere::new();
    floor.transform = Transformation::new().scale(10.0, 0.01, 10.0).build();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;
    world.add_object(floor);

    let mut left_wall = Sphere::new();
    left_wall.transform = Transformation::new()
        .scale(10.0, 0.01, 10.0)
        .rotate_x(PI / 2.0)
        .rotate_y(-PI / 4.0)
        .translate(0.0, 0.0, 5.0)
        .build();
    left_wall.material.color = Color::new(1.0, 0.9, 0.9);
    left_wall.material.specular = 0.0;
    world.add_object(left_wall);

    let mut right_wall = Sphere::new();
    right_wall.transform = Transformation::new()
        .scale(10.0, 0.01, 10.0)
        .rotate_x(PI / 2.0)
        .rotate_y(PI / 4.0)
        .translate(0.0, 0.0, 5.0)
        .build();
    right_wall.material.color = Color::new(1.0, 0.9, 0.9);
    right_wall.material.specular = 0.0;
    world.add_object(right_wall);

    let mut middle = Sphere::new();
    middle.transform = Transformation::new().translate(-0.5, 1.0, 0.5).build();
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    world.add_object(middle);

    let mut right = Sphere::new();
    right.transform = Transformation::new()
        .scale(0.5, 0.5, 0.5)
        .translate(1.5, 0.5, -0.5)
        .build();
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    world.add_object(right);

    let mut left = Sphere::new();
    left.transform = Transformation::new()
        .scale(0.33, 0.33, 0.33)
        .translate(-1.5, 0.33, -0.75)
        .build();
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.add_object(left);

    world.light = Some(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(1920, 1080, PI / 3.0);

    camera.transform = Transformation::view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let path = Path::new("balls.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let ppm = canvas.canvas_to_ppm();
    match file.write_all(ppm.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    };
}
