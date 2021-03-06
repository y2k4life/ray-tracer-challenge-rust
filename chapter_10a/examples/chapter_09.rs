use rustic_ray::{
    shapes::Plane, shapes::Shape, shapes::Sphere, Camera, Color, Point, PointLight, Transformation,
    Vector, World,
};
use std::{f64::consts::PI, fs::File, io::Write, path::Path};

fn main() {
    let mut world = World::new();

    let mut floor = Plane::new();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;
    world.add_object(Box::new(floor));

    let mut middle = Sphere::new();
    middle.transform = Transformation::new().translate(-0.5, 1.0, 0.5).build();
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    world.add_object(Box::new(middle));

    let mut right = Sphere::new();
    right.set_transform(
        Transformation::new()
            .scale(0.5, 0.5, 0.5)
            .translate(1.5, 0.5, -0.5)
            .build(),
    );
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    world.add_object(Box::new(right));

    let mut left = Sphere::new();
    left.set_transform(
        Transformation::new()
            .scale(0.33, 0.33, 0.33)
            .translate(-1.5, 0.33, -0.75)
            .build(),
    );
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.add_object(Box::new(left));

    world.light = Some(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(400, 400, PI / 3.0);

    camera.transform = Transformation::view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let path = Path::new("chapter_09.ppm");
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
