use rustic_ray::{
    patterns::*, shapes::Cone, shapes::Cylinder, shapes::Plane, Camera, Color, Colors, Point,
    PointLight, Transformation, Vector, World,
};
use std::{f64::consts::PI, fs::File, io::Write, path::Path};

fn main() {
    let mut world = World::new();

    let mut pattern = Checkers::new(Colors::WHITE, Colors::BLACK);
    pattern.transform = Transformation::new()
        .scale(0.1, 0.1, 0.1)
        .rotate_y(0.174)
        .translate(10.0, 0.0, 10.0)
        .build();

    let mut floor = Plane::new();
    floor.material.pattern = Some(Box::new(pattern));
    floor.material.reflective = 0.1;
    world.add_object(Box::new(floor));

    let mut ceiling = Plane::new();
    ceiling.transform = Transformation::new().translate(0.0, 10.0, 0.0).build();
    ceiling.material.reflective = 0.1;
    ceiling.material.pattern = Some(Box::new(pattern));
    world.add_object(Box::new(ceiling));

    let mut checkers = Checkers::new(Colors::WHITE, Colors::BLACK);
    checkers.transform = Transformation::new().translate(10.0, 0.0, 10.0).build();

    let mut left_wall = Plane::new();
    left_wall.transform = Transformation::new()
        .rotate_x(PI / 2.0)
        .rotate_y(-PI / 4.0)
        .translate(0.0, 0.0, 10.0)
        .build();
    left_wall.material.pattern = Some(Box::new(checkers));
    world.add_object(Box::new(left_wall));

    let mut right_wall = Plane::new();
    right_wall.transform = Transformation::new()
        .rotate_x(PI / 2.0)
        .rotate_y(PI / 4.0)
        .translate(10.0, 0.0, 0.0)
        .build();
    right_wall.material.pattern = Some(Box::new(checkers));
    world.add_object(Box::new(right_wall));

    let mut middle = Cone::new();
    middle.maximum = 1.0;
    middle.minimum = -1.0;
    middle.closed = false;
    middle.transform = Transformation::new().translate(-0.5, 1.0, 0.5).build();
    middle.material.transparency = 1.0;
    middle.material.refractive_index = 1.5;
    middle.material.ambient = 0.1;
    middle.material.diffuse = 0.05;
    world.add_object(Box::new(middle));

    let mut middle_back = Cylinder::new();
    middle_back.maximum = 1.0;
    middle_back.minimum = -1.0;
    middle_back.closed = true;
    middle_back.transform = Transformation::new()
        .scale(0.25, 0.25, 0.25)
        .translate(-0.5, 1.0, -1.0)
        .build();
    middle_back.material.color = Colors::BLACK;
    world.add_object(Box::new(middle_back));

    world.add_object(Box::new(get_ring(1.5, 0.5)));
    world.add_object(Box::new(get_ring(1.25, 1.0)));
    world.add_object(Box::new(get_ring(1.0, 1.5)));
    world.add_object(Box::new(get_ring(0.75, 2.0)));
    let mut middle_ring = get_ring(0.50, 2.5);
    middle_ring.closed = true;
    world.add_object(Box::new(middle_ring));

    let mut left = Cylinder::new();
    left.maximum = 1.0;
    left.minimum = -1.0;
    left.closed = true;
    left.transform = Transformation::new()
        .scale(0.33, 0.33, 0.33)
        .translate(-1.5, 0.33, -1.0)
        .build();
    left.material.color = Color::new(0.0, 0.6, 0.0);
    left.material.ambient = 0.8;
    left.material.reflective = 0.6;
    left.material.refractive_index = 2.417;
    world.add_object(Box::new(left));

    world.light = Some(PointLight::new(
        Point::new(10.0, 3.5, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(1920, 1080, PI / 3.0);

    camera.transform = Transformation::view_transform(
        Point::new(0.0, 2.0, -6.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let path = Path::new("chapter_13.ppm");
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

pub fn get_ring(s1: f64, s2: f64) -> Cylinder {
    let mut ring = Cylinder::new();
    ring.maximum = 0.25;
    ring.minimum = -0.25;
    ring.closed = false;
    ring.transform = Transformation::new()
        .scale(s1, s2, s1)
        .translate(1.75, 0.0, -1.25)
        .build();
    ring.material.color = Color::new(0.8, 0.0, 0.2);
    ring
}
