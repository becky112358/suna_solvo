use std::f64::consts::PI;

use suna_solvo::parabola_variable;

// Heating height above zero.
const HEATING_HEIGHT_M: f64 = 0.5;
const UPPER_RADIUS_M: f64 = 1.0;

pub fn method_cone_rings() {
    let b = parabola_variable(HEATING_HEIGHT_M);

    let mut x0 = 0.05;
    println!("Base circle radius {:.ndp$}", x0, ndp=3);

    while x0 < UPPER_RADIUS_M {
        let x1 = choose_x(b, x0);

        let y0 = parabola(b, x0);
        let y1 = parabola(b, x1);
        let length = hypotenuse_length_squared(x0, x1, y0, y1).sqrt();

        let x0_circumference = circumference(x0);
        let x1_circumference = circumference(x1);

        let sector_radius = (x0_circumference * length)
                          / (x1_circumference - x0_circumference);
        let sector_angle_radians = (x1_circumference - x0_circumference)
                                 / length;
        let sector_angle_degrees = (sector_angle_radians * 180.0) / PI;

        println!("Sector radius {:>w$.ndp$}  Length {:>w$.ndp$}  Sector angle {:>w$.ndp$}",
            sector_radius, length, sector_angle_degrees,
            w=5, ndp=3);

        x0 = x1;
    }
}

fn choose_x(b: f64, x0: f64) -> f64 {
    let focal_radius : f64 = 0.1;
    let focal_radius_squared = focal_radius.powf(2.0);
    let tolerance = 0.0015;

    let y0 = parabola(b, x0);

    let x_max = x0 + focal_radius;

    let mut x1 = x_max;
    let mut y1 = parabola(b, x1);

    let mut current_target_squared = hypotenuse_length_squared(x0, x1, y0, y1);

    while !(current_target_squared < (focal_radius_squared + tolerance)
        && current_target_squared > (focal_radius_squared - tolerance)) {
        if current_target_squared > (focal_radius_squared + tolerance) {
            x1 = x0 + ((x1 - x0) * 0.5);
        }
        else {
            x1 = x_max - ((x_max - x1) * 0.5);
        }

        y1 = parabola(b, x1);
        current_target_squared = hypotenuse_length_squared(x0, x1, y0, y1);
    }

    return x1;
}

fn circumference(radius: f64) -> f64 {
    2.0 * PI * radius
}

fn hypotenuse_length_squared(x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    (x1 - x0).powf(2.0) + (y1 - y0).powf(2.0)
}

fn parabola(b: f64, x: f64) -> f64 {
    b * x * x
}

