use std::f64::consts::PI;

const LOWER_RADIUS_M: f64 = 0.0;
const UPPER_RADIUS_M: f64 = 0.6;
// Heating height above zero.
// Note that if the lower radius is not zero, then the parabola will start
// above zero.
const HEATING_HEIGHT_M: f64 = 0.2;
const N_PANELS: i32 = 12;

fn main() {
    let b = parabola_variable(HEATING_HEIGHT_M);

    let half_angle = half_angle_of_regular_n_sided_polygon(N_PANELS);

    let invisible_length = length(b, LOWER_RADIUS_M);

    iterate_over_x(
        LOWER_RADIUS_M,
        UPPER_RADIUS_M,
        b,
        half_angle,
        invisible_length);
}

fn parabola_variable(focus_point: f64) -> f64 {
    // y = b x^2
    let b = 1.0 / (4.0 * focus_point);
    println!("p {}, b {}", focus_point, b);
    return b;
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
        let t = length(b, x) - invisible_length;

        let w = (2.0 * x) / (half_angle.tan());

        let y = b * x * x;

        println!("x {:>width$.ndp$}   length {:>width$.ndp$}   width {:>width$.ndp$}   height {:>width$.ndp$}",
            x, t, w, y, width=6, ndp=3);

        x += increment_size;
    }
}

fn length(b: f64, x: f64) -> f64 {
    let u_squared = 1.0 + (4.0 * b * b * x * x);
    let u = u_squared.sqrt();

    if u < 1.0 {
        println!("***** u is less than 1 *****");
    }

    (1.0 / (4.0 * b)) * (u.acosh() + ((u.acosh().sinh()) * u))
}

