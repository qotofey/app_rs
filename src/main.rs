fn main() {
    print_main_menu_screen();
    print_training_set_screen();
    print_config_screen();
}

const TRAINING_SET_SCREEN_CODE: u8 = 1;
const CONFIG_SCREEN_CODE: u8 = 2;
const EXIT_CODE: u8 = 0;

fn print_main_menu_screen() {
    println!("[ Главный экран ]");
    println!("Для перехода к экрану работы с обучающей выборкой введите: {}", TRAINING_SET_SCREEN_CODE);
    println!("Для перехода к экрау настроек введите: {}", CONFIG_SCREEN_CODE);
    println!("Для выхода из программы введите: {}", EXIT_CODE);
}

fn print_training_set_screen() {
    println!("[ Экран работы с обучающей выборкой ]");
}

fn print_config_screen() {
    println!("[ Экран настроек ]");
}
