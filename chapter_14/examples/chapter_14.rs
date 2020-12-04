use rustic_ray::{
    patterns::Checkers, shapes::Cylinder, shapes::Group, shapes::Plane, shapes::Sphere, Camera,
    Color, Colors, Point, PointLight, Transformation, Vector, World,
};
use std::{f64::consts::PI, fs::File, io::Write, path::Path};

fn main() {
    let w = &mut World::new();

    let mut pattern = Checkers::new(Colors::WHITE, Colors::BLACK);
    pattern.transform = Transformation::new()
        .scale(0.1, 0.1, 0.1)
        .rotate_y(0.174)
        .translate(10.0, 0.0, 10.0)
        .build();

    let mut floor = Plane::new();
    floor.material.pattern = Some(Box::new(pattern));
    w.add_object(Box::new(floor));

    let row1 = hex_row();
    w.add_object(Box::new(row1));

    let mut row3 = hex_row();
    row3.transform = Transformation::new().translate(0.0, 0.0, -1.0).build();
    w.add_object(Box::new(row3));

    let mut row5 = hex_row();
    row5.transform = Transformation::new().translate(0.0, 0.0, -2.0).build();
    w.add_object(Box::new(row5));

    let mut row7 = hex_row();
    row7.transform = Transformation::new().translate(0.0, 0.0, -3.0).build();
    w.add_object(Box::new(row7));

    w.light = Some(PointLight::new(
        Point::new(8.0, 3.5, -8.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let w = &*w;

    let mut camera = Camera::new(400, 400, PI / 2.5);

    camera.transform = Transformation::view_transform(
        Point::new(0.0, 2.5, -4.5),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&w);

    let path = Path::new("chapter_14.ppm");
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

pub fn hex_row() -> Group {
    let mut h1 = hex_in_hex();
    h1.transform = Transformation::new().translate(-1.0, 1.0, 0.0).build();

    let mut h2 = hex_in_hex();
    h2.transform = Transformation::new().translate(0.0, 1.0, 0.0).build();

    let mut h3 = hex_in_hex();
    h3.transform = Transformation::new().translate(1.0, 1.0, 0.0).build();

    let mut row = Group::new();
    row.add_object(Box::new(h1));
    row.add_object(Box::new(h2));
    row.add_object(Box::new(h3));
    row
}

pub fn hex_in_hex() -> Group {
    let mut hex = hexagon();
    hex.transform = Transformation::new()
        .scale(0.5, 0.5, 0.5)
        .rotate_x(-PI / 0.08726646)
        .build();

    let mut hex2 = hexagon();
    hex2.transform = Transformation::new()
        .scale(0.25, 0.25, 0.25)
        .rotate_x(-PI / 6.0)
        .build();
    hex2.material.color = Color::new(0.0, 1.0, 0.0);

    let mut hex_in_hex = Group::new();
    hex_in_hex.add_object(Box::new(hex));
    hex_in_hex.add_object(Box::new(hex2));

    hex_in_hex
}

pub fn hexagon() -> Group {
    let mut hex = Group::new();

    for n in 0..6 {
        let mut side = hexagon_side();
        side.transform = Transformation::new().rotate_y(n as f64 * PI / 3.0).build();

        hex.add_object(Box::new(side));
    }

    hex
}

fn hexagon_corner() -> Sphere {
    let mut corner = Sphere::new();
    corner.transform = Transformation::new()
        .scale(0.25, 0.25, 0.25)
        .translate(0.0, 0.0, -1.0)
        .build();

    corner.material.color = Color::new(1.0, 0.0, 0.0);

    corner
}

pub fn hexagon_edge() -> Cylinder {
    let mut edge = Cylinder::new();
    edge.minimum = 0.0;
    edge.maximum = 1.0;
    edge.transform = Transformation::new()
        .scale(0.25, 1.0, 0.25)
        .rotate_z(-PI / 2.0)
        .rotate_y(-PI / 6.0)
        .translate(0.0, 0.0, -1.0)
        .build();
    edge.material.color = Color::new(1.0, 0.0, 0.0);

    edge
}

pub fn hexagon_side() -> Group {
    let mut side = Group::new();

    let corner = hexagon_corner();
    let hexagon_edge = hexagon_edge();

    side.add_object(Box::new(corner));
    side.add_object(Box::new(hexagon_edge));

    side
}
