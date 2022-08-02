fn main() {
    println!("Hello, Rust!");
    let max:f32 = std::f32::MAX;
    let min:f32 = std::f32::MIN;
    println!("max f32 - {}", max);
    println!("min f32 - {}", min);
    let max:f64 = std::f64::MAX;
    let min:f64 = std::f64::MIN;
    println!("max f64 - {}", max);
    println!("min f64 - {}", min);
    decorate_printing();
}

fn decorate_printing() {
    println!("Bye Rust!");
}
