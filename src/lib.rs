
/// The equation of a parabola is y = b x^2.
/// The relationship between b and the focal of the parabola is
/// b = 1.0 / (4.0 * focal)
pub fn parabola_variable(focal: f64) -> f64 {
    // y = b x^2
    let b = 1.0 / (4.0 * focal);
    println!("p {}, b {}", focal, b);
    return b;
}

