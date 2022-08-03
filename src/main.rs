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
    print_menu();
}

const HOME_SCREEN_CODE: u8 = 1;
const EXIT_CODE: u8 = 0;

fn decorate_printing() {
    println!("Bye Rust!");
}

fn print_menu() {
    println!("Для возврата на главную страницу введите: {}", HOME_SCREEN_CODE);
    println!("Для выхода из программы введите: {}", EXIT_CODE);
}
