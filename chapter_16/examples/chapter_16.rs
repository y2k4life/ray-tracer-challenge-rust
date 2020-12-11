use rustic_ray::{
    patterns::Checkers,
    shapes::CsgOperation,
    shapes::Plane,
    shapes::Shape,
    shapes::CSG,
    shapes::{Cube, Sphere},
    Camera, Color, Colors, Point, PointLight, Transformation, Vector, World,
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
    world.add_object(Box::new(floor));

    let mut middle = Sphere::new();
    middle.material.color = Color::new_rgb(255, 242, 0);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    // world.add_object(Box::new(middle));

    let mut cube = Cube::new();
    cube.material.color = Color::new_rgb(255, 242, 0);
    cube.transform = Transformation::new()
        .scale(0.55, 0.55, 1.5)
        .rotate_z(PI / 4.0)
        .translate(0.65, 0.0, 0.0)
        .build();

    let mut cube_ball = CSG::new(CsgOperation::Difference, Box::new(middle), Box::new(cube));
    cube_ball.transform = Transformation::new()
        .scale(1.5, 1.5, 1.5)
        .rotate_y(PI / 8.0)
        .translate(-0.75, 1.5, 2.0)
        .build();
    world.add_object(Box::new(cube_ball));

    let mut right = Sphere::new();
    right.set_transform(
        Transformation::new()
            .scale(1.5, 1.5, 1.5)
            .translate(1.5, 0.5, -1.0)
            .build(),
    );
    right.material.color = Color::new(0.1, 0.15, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    right.material.transparency = 0.8;

    let mut left = Sphere::new();
    left.set_transform(Transformation::new().translate(-0.5, 0.5, -1.0).build());
    left.material.color = Color::new(0.1, 0.1, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    left.material.transparency = 0.8;

    let mut group = CSG::new(CsgOperation::Union, Box::new(left), Box::new(right));
    group.transform = Transformation::new().translate(-0.25, 0.5, 0.0).build();
    world.add_object(Box::new(group));

    let mut right1 = Sphere::new();
    right1.set_transform(
        Transformation::new()
            .scale(0.5, 0.5, 0.5)
            .translate(-1.75, 0.5, -3.0)
            .build(),
    );
    right1.material.color = Color::new(0.1, 0.15, 0.1);
    right1.material.diffuse = 0.7;
    right1.material.specular = 0.3;
    right1.material.transparency = 0.8;
    world.add_object(Box::new(right1));

    let mut left1 = Sphere::new();
    left1.set_transform(
        Transformation::new()
            .scale(0.5, 0.5, 0.5)
            .translate(-2.5, 0.5, -3.0)
            .build(),
    );
    left1.material.color = Color::new(0.1, 0.1, 0.1);
    left1.material.diffuse = 0.7;
    left1.material.specular = 0.3;
    left1.material.transparency = 0.8;
    world.add_object(Box::new(left1));

    world.light = Some(PointLight::new(
        Point::new(-8.0, 10.0, -6.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(400, 400, PI / 3.0);

    camera.transform = Transformation::view_transform(
        Point::new(0.0, 1.5, -8.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let path = Path::new("chapter_16.ppm");
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
