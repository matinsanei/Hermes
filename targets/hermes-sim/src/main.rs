use hermes_math::parse_linear;

fn main() {
    println!("Hermes simulator placeholder — small example using the hermes-math crate.");
    if let Some((a, b)) = parse_linear("2.0 * t + 1.0") {
        println!("Example linear formula: a={} b={}", a, b);
    }
}
