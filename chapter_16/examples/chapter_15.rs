use rustic_ray::{
    patterns::Checkers, shapes::Plane, Camera, Color, Colors, ObjFile, Point, PointLight,
    Transformation, Vector, World,
};
use std::{f64::consts::PI, fs::File, io::Read, io::Write, path::Path};

fn main() -> std::io::Result<()> {
    let w = &mut World::new();

    let mut pattern = Checkers::new(Colors::WHITE, Colors::BLACK);
    pattern.transform = Transformation::new().rotate_y(0.174).build();

    let mut floor = Plane::new();
    floor.material.pattern = Some(Box::new(pattern));
    floor.material.reflective = 0.4;
    w.add_object(Box::new(floor));

    let mut file = File::open("tea_pot.obj")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut tea_pot = ObjFile::parse(&contents);
    tea_pot.transform = Transformation::new()
        .scale(0.25, 0.25, 0.25)
        .rotate_x(-PI / 2.0)
        .rotate_y(PI / 8.0)
        .translate(-0.5, 0.0, 0.0)
        .build();

    tea_pot.material.color = Color::from_u8(192, 192, 192);
    tea_pot.material.reflective = 0.7;
    w.add_object(Box::new(tea_pot));

    w.light = Some(PointLight::new(
        Point::new(1.0, 6.5, -2.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let w = &*w;

    let mut camera = Camera::new(400, 400, PI / 3.0);

    camera.transform = Transformation::view_transform(
        Point::new(0.0, 3.5, -7.85),
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

    Ok(())
}
