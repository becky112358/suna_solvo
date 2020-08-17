mod method_cone_rings;
mod method_plane_vs;

// Heating height above zero.
// Note that if the lower radius is not zero, then the parabola will start
// above zero.
const HEATING_HEIGHT_M: f64 = 0.5;

fn main() {
    let b = parabola_variable(HEATING_HEIGHT_M);

    method_plane_vs::method_parabola_extended_to_a_plane(b);

    method_cone_rings::method_cone_rings(b);
}

fn parabola_variable(focal: f64) -> f64 {
    // y = b x^2
    let b = 1.0 / (4.0 * focal);
    println!("p {}, b {}", focal, b);
    return b;
}

