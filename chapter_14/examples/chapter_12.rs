use rustic_ray::{
    patterns::Checkers, patterns::Stripe, shapes::Cube, Camera, Color, Colors, Point, PointLight,
    Transformation, Vector, World,
};
use std::{f64::consts::PI, fs::File, io::Write, path::Path};

fn main() {
    let mut world = World::new();

    let mut checkers = Checkers::new(Colors::WHITE, Colors::BLACK);
    checkers.transform = Transformation::new().scale(0.15, 0.15, 0.15).build();

    let mut stripes1 = Stripe::new(Color::from_u8(161, 64, 5), Color::from_u8(145, 41, 3));
    stripes1.transform = Transformation::new()
        .scale(0.05, 0.05, 0.05)
        .rotate_y(PI / 2.0)
        .build();

    let mut stripes2 = Stripe::new(Color::from_u8(161, 64, 5), Color::from_u8(145, 41, 3));
    stripes2.transform = Transformation::new().scale(0.05, 0.05, 0.05).build();

    let mut floor = Cube::new();
    floor.transform = Transformation::new()
        .scale(5.0, 0.1, 5.0)
        .translate(0.0, -0.1, 0.0)
        .build();
    floor.material.pattern = Some(Box::new(checkers));
    world.add_object(Box::new(floor));

    let mut right_wall = Cube::new();
    right_wall.transform = Transformation::new()
        .scale(0.1, 4.0, 5.0)
        .translate(-5.1, 4.0, 0.0)
        .build();
    right_wall.material.pattern = Some(Box::new(stripes1));
    world.add_object(Box::new(right_wall));

    let mut left_wall = Cube::new();
    left_wall.transform = Transformation::new()
        .scale(0.1, 4.0, 5.0)
        .translate(5.1, 4.0, 0.0)
        .build();
    left_wall.material.pattern = Some(Box::new(stripes1));
    world.add_object(Box::new(left_wall));

    let mut back_wall = Cube::new();
    back_wall.transform = Transformation::new()
        .scale(5.0, 4.0, 0.1)
        .translate(0.0, 4.0, 5.1)
        .build();
    back_wall.material.pattern = Some(Box::new(stripes2));
    world.add_object(Box::new(back_wall));

    let mut painting = Cube::new();
    painting.transform = Transformation::new()
        .scale(1.0, 2.0, 0.1)
        .translate(-1.5, 4.0, 4.9)
        .build();
    painting.material.color = Color::new(0.1, 1.0, 0.1);
    world.add_object(Box::new(painting));

    let mut painting2 = Cube::new();
    painting2.transform = Transformation::new()
        .scale(1.75, 0.5, 0.1)
        .translate(1.5, 4.0, 4.9)
        .build();
    painting2.material.color = Color::new(1.0, 0.3, 0.3);
    world.add_object(Box::new(painting2));

    let mut painting3 = Cube::new();
    painting3.transform = Transformation::new()
        .scale(1.75, 0.5, 0.1)
        .translate(1.5, 2.75, 4.9)
        .build();
    painting3.material.color = Color::new(0.0, 0.3, 1.0);
    world.add_object(Box::new(painting3));

    let mut mirror = Cube::new();
    mirror.transform = Transformation::new()
        .scale(0.01, 2.0, 4.0)
        .translate(5.0, 3.0, 0.0)
        .build();
    mirror.material.reflective = 1.0;
    mirror.material.refractive_index = 1.458;
    world.add_object(Box::new(mirror));

    let mut table_top = Cube::new();
    table_top.transform = Transformation::new()
        .scale(2.5, 0.1, 3.0)
        .translate(0.5, 1.25, 0.0)
        .build();
    table_top.material.pattern = Some(Box::new(stripes1));
    table_top.material.reflective = 0.02;
    table_top.material.refractive_index = 3.45;
    world.add_object(Box::new(table_top));

    let mut leg1 = Cube::new();
    leg1.transform = Transformation::new()
        .scale(0.1, 0.65, 0.1)
        .translate(-1.9, 0.65, -2.9)
        .build();
    leg1.material.color = Color::from_u8(161, 64, 5);
    world.add_object(Box::new(leg1));

    let mut leg2 = Cube::new();
    leg2.transform = Transformation::new()
        .scale(0.1, 0.65, 0.1)
        .translate(2.9, 0.65, -2.9)
        .build();
    leg2.material.color = Color::from_u8(161, 64, 5);
    world.add_object(Box::new(leg2));

    let mut leg3 = Cube::new();
    leg3.transform = Transformation::new()
        .scale(0.1, 0.65, 0.1)
        .translate(2.9, 0.65, 2.9)
        .build();
    leg3.material.color = Color::from_u8(161, 64, 5);
    world.add_object(Box::new(leg3));

    let mut leg4 = Cube::new();
    leg4.transform = Transformation::new()
        .scale(0.1, 0.65, 0.1)
        .translate(-1.9, 0.65, 2.9)
        .build();
    leg4.material.color = Color::from_u8(161, 64, 5);
    world.add_object(Box::new(leg4));

    let mut glass_block = Cube::new();
    glass_block.transform = Transformation::new()
        .scale(0.1, 1.0, 1.0)
        .translate(-0.75, 2.35, -1.0)
        .build();
    glass_block.material.color = Color::from_u8(211, 102, 151);
    glass_block.material.transparency = 1.0;
    world.add_object(Box::new(glass_block));

    let mut block1 = Cube::new();
    block1.transform = Transformation::new()
        .scale(0.1, 0.1, 0.1)
        .translate(0.5, 1.45, -2.0)
        .build();
    block1.material.color = Color::from_u8(213, 14, 151);
    world.add_object(Box::new(block1));

    let mut block3 = Cube::new();
    block3.transform = Transformation::new()
        .scale(0.2, 0.2, 0.2)
        .translate(1.75, 1.55, -1.0)
        .build();
    block3.material.color = Color::from_u8(10, 234, 36);
    world.add_object(Box::new(block3));

    let mut block3 = Cube::new();
    block3.transform = Transformation::new()
        .scale(0.55, 0.5, 1.75)
        .translate(0.2, 1.55, 0.05)
        .build();
    block3.material.reflective = 0.6;
    block3.material.refractive_index = 1.31;
    block3.material.ambient = 0.025;
    block3.material.diffuse = 0.25;
    block3.material.color = Color::from_u8(237, 234, 36);
    world.add_object(Box::new(block3));

    world.light = Some(PointLight::new(
        Point::new(3.0, 11.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(614, 614, PI / 3.0);

    camera.transform = Transformation::view_transform(
        Point::new(-4.0, 2.5, -4.8),
        Point::new(0.90, 1.25, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let path = Path::new("chapter_12.ppm");
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
