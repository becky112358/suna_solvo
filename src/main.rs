use std::f64::consts::FRAC_PI_2;

const LOWER_RADIUS: f64 = 0.0;
const HEATING_RADIUS: f64 = 0.4;
const UPPER_RADIUS: f64 = 0.6;
const HEATING_HEIGHT: f64 = 0.2;
const N_PANELS: i32 = 12;

// This function is used to print numbers to 2 decimal places.
// Probably there is a better way to do that?!
fn truncate(long: f64) -> f64 {
    let bigger = long * 100.0;
    let integer = bigger.round() as i32;
    integer as f64 / 100.0
}

fn length(b: f64, x: f64) -> f64 {
    let u_squared = 1.0 + (4.0 * b * b * x * x);
    let u = u_squared.sqrt();

    if u < 1.0 {
        println!("***** u is less than 1 *****");
    }

    (1.0 / (4.0 * b)) * (u.acosh() + ((u.acosh().sinh()) * u))
}

fn main() {

    // p is the heating height above zero
    let p = ((HEATING_RADIUS * HEATING_RADIUS) - (LOWER_RADIUS * LOWER_RADIUS))
          / (4.0 * HEATING_HEIGHT);

    // y = b x^2
    let b = 1.0 / (4.0 * p);

    println!("p {}, b {}", p, b);


    let angle_sum = (N_PANELS as f64 - 2.0) * FRAC_PI_2;
    let half_angle = angle_sum / (2.0 * N_PANELS as f64);

    println!("half angle {}", half_angle);


    let invisible_length = length(b, LOWER_RADIUS);


    let increment_size = (UPPER_RADIUS - LOWER_RADIUS) / 200.0;
    let mut x = LOWER_RADIUS;


    while x <= UPPER_RADIUS {
        let t = length(b, x) - invisible_length;

        let w = (2.0 * x) / (half_angle.tan());

        let y = b * x * x;

        println!(
      "x {:>width$}   length {:>width$}   width {:>width$}   height {:>width$}",
            truncate(x), truncate(t), truncate(w), truncate(y), width=6);

        x += increment_size;
    }
}

