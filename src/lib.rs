
pub fn parabola_variable(focal: f64) -> f64 {
    // y = b x^2
    let b = 1.0 / (4.0 * focal);
    println!("p {}, b {}", focal, b);
    return b;
}

