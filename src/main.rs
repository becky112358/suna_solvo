use std::f64::consts::PI;

const LOWER_RADIUS_M: f64 = 0.0;
const UPPER_RADIUS_M: f64 = 1.0;
// Heating height above zero.
// Note that if the lower radius is not zero, then the parabola will start
// above zero.
const HEATING_HEIGHT_M: f64 = 0.5;
const N_PANELS: i32 = 12;

fn main() {
    let b = parabola_variable(HEATING_HEIGHT_M);

    method_parabola_extended_to_a_plane(b);

    method_cone_rings(b);
}

fn parabola_variable(focal: f64) -> f64 {
    // y = b x^2
    let b = 1.0 / (4.0 * focal);
    println!("p {}, b {}", focal, b);
    return b;
}

fn method_parabola_extended_to_a_plane(b: f64) {
    let half_angle = half_angle_of_regular_n_sided_polygon(N_PANELS);

    let invisible_length = parabola_length(b, LOWER_RADIUS_M);

    iterate_over_x(
        LOWER_RADIUS_M,
        UPPER_RADIUS_M,
        b,
        half_angle,
        invisible_length);
}

fn half_angle_of_regular_n_sided_polygon(n_sides: i32) -> f64 {
    let angle = ((n_sides - 2) as f64 / (n_sides as f64)) * PI;
    let half_angle = angle / 2.0;
    println!("half angle {}", half_angle);
    return half_angle;
}

fn iterate_over_x(
    x_start: f64,
    x_finish: f64,
    b: f64,
    half_angle: f64,
    invisible_length: f64) {

    let increment_size = (x_finish - x_start) / 40.0;
    let mut x = x_start;

    while x <= x_finish {
        let t = parabola_length(b, x) - invisible_length;

        let w = (2.0 * x) / (half_angle.tan());

        let y = parabola(b, x);

        println!("x {:>width$.ndp$}   length {:>width$.ndp$}   width {:>width$.ndp$}   height {:>width$.ndp$}",
            x, t, w, y, width=6, ndp=3);

        x += increment_size;
    }
}

fn parabola_length(b: f64, x: f64) -> f64 {
    let u_squared = 1.0 + (4.0 * b * b * x * x);
    let u = u_squared.sqrt();

    if u < 1.0 {
        println!("***** u is less than 1 *****");
    }

    (1.0 / (4.0 * b)) * (u.acosh() + ((u.acosh().sinh()) * u))
}

fn method_cone_rings(b: f64) {
    let mut x0 = 0.05;
    println!("Base circle circumference {:.ndp$}", x0, ndp=3);

    while x0 < UPPER_RADIUS_M {
        let x1 = choose_x(b, x0);

        let y0 = parabola(b, x0);
        let y1 = parabola(b, x1);
        let length = hypotenuse_length_squared(x0, x1, y0, y1).sqrt();

        let x0_circumference = circumference(x0);
        let x1_circumference = circumference(x1);

        println!("Lower circumference {:>width$.ndp$}   Upper circumference {:>width$.ndp$}   Length {:>width$.ndp$}",
            x0_circumference, x1_circumference, length, width=6, ndp=3);

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

