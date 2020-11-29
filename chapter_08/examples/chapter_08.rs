use rustic_ray::{shapes::Sphere, Camera, Color, Point, PointLight, Transformation, Vector, World};
use std::{f64::consts::PI, fs::File, io::Write, path::Path};

fn main() {
    let mut world = World::new();

    let mut right_wall = Sphere::new();
    right_wall.transform = Transformation::new()
        .scale(40.0, 40.0, 40.0)
        .rotate_x(PI / 2.0)
        .rotate_y(PI)
        .translate(0.0, 0.0, 10.0)
        .build();
    right_wall.material.color = Color::new(1.0, 0.9, 0.9);
    right_wall.material.specular = 0.0;
    world.add_object(right_wall);

    let mut wrist = Sphere::new();
    wrist.transform = Transformation::new()
        .scale(0.5, 0.5, 0.5)
        .translate(-1.25, 1.5, -2.0).build();
    wrist.material.color = Color::new(0.1, 1.0, 0.5);
    wrist.material.diffuse = 0.7;
    wrist.material.specular = 0.3;
    world.add_object(wrist);

    let mut palm = Sphere::new();
    palm.transform = Transformation::new()
        .scale(0.75, 0.75, 0.75)
        .translate(-0.5, 2.0, -1.75).build();
    palm.material.color = Color::new(0.1, 1.0, 0.5);
    palm.material.diffuse = 0.7;
    palm.material.specular = 0.3;
    world.add_object(palm);

    let mut finger1 = Sphere::new();
    finger1.transform = Transformation::new()
        .scale(1.0, 0.2, 0.1)
        .translate(0.0, 2.0, -2.5)
        .build();
    finger1.material.color = Color::new(1.0, 0.8, 0.1);
    finger1.material.diffuse = 0.7;
    finger1.material.specular = 0.3;
    world.add_object(finger1);

    let mut finger2 = Sphere::new();
    finger2.transform = Transformation::new()
        .scale(1.0, 0.2, 0.1)
        .translate(0.0, 1.80, -2.5)
        .build();
    finger2.material.color = Color::new(1.0, 0.8, 0.1);
    finger2.material.diffuse = 0.7;
    finger2.material.specular = 0.3;
    world.add_object(finger2);

    let mut finger3 = Sphere::new();
    finger3.transform = Transformation::new()
        .scale(0.75, 0.2, 0.1)
        .translate(0.0, 1.60, -2.5)
        .build();
    finger3.material.color = Color::new(1.0, 0.8, 0.1);
    finger3.material.diffuse = 0.7;
    finger3.material.specular = 0.3;
    world.add_object(finger3);

    let mut finger4 = Sphere::new();
    finger4.transform = Transformation::new()
        .scale(0.75, 0.2, 0.1)
        .rotate_z(PI / 2.0)
        .translate(-0.75, 2.40, -2.5)
        .build();
    finger4.material.color = Color::new(1.0, 0.8, 0.1);
    finger4.material.diffuse = 0.7;
    finger4.material.specular = 0.3;
    world.add_object(finger4);

    world.light = Some(PointLight::new(
        Point::new(-2.0, 1.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(640, 480, PI / 2.5);

    camera.transform = Transformation::view_transform(
        Point::new(1.25, 1.0, -6.0),
        Point::new(0.2, 1.0, 0.0),
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
