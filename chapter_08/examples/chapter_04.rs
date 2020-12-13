use std::{f64::consts::PI, fs::File, io::Write, path::Path};

use rustic_ray::{Canvas, Color, Point, Transformation};

fn main() {
    let canvas = &mut Canvas::new(200, 200);
    canvas.write_pixel(100, 100, Color::new(1.0, 0.0, 0.0));

    let mut hour = 1.0;
    loop {
        let xy = clock_hour(hour);

        canvas.write_pixel(xy.0, xy.1, Color::new(1.0, 0.0, 0.0));
        hour += 1.0;

        if hour > 12.0 {
            break;
        }
    }

    let path = Path::new("clock.ppm");
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

fn clock_hour(hour: f64) -> (usize, usize) {
    let r = Transformation::new().rotate_y(hour * PI / 6.0).build();
    let hour_point = r * Point::new(0.0, 0.0, 1.0);

    let radius = (3.0 / 8.0) * 200.0;

    let x = (100.0 + (hour_point.x * radius)) as usize;
    let y = (100.0 + (hour_point.z * radius)) as usize;
    (x, y)
}
