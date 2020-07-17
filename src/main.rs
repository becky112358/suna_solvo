use std::f64::consts::PI;

const LOWER_RADIUS_M: f64 = 0.0;
const UPPER_RADIUS_M: f64 = 0.6;
// Heating height above zero.
// Note that if the lower radius is not zero, then the parabola will start
// above zero.
const HEATING_HEIGHT_M: f64 = 0.2;
const N_PANELS: i32 = 12;

fn main() {
    // y = b x^2
    let b = 1.0 / (4.0 * HEATING_HEIGHT_M);
    println!("p {}, b {}", HEATING_HEIGHT_M, b);


    let angle = ((N_PANELS - 2) as f64 / (N_PANELS as f64)) * PI;
    let half_angle = angle / 2.0;
    println!("half angle {}", half_angle);


    let invisible_length = length(b, LOWER_RADIUS_M);


    let increment_size = (UPPER_RADIUS_M - LOWER_RADIUS_M) / 40.0;
    let mut x = LOWER_RADIUS_M;

    while x <= UPPER_RADIUS_M {
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

