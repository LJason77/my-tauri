#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, Submenu};

pub fn get() -> Menu {
  let add = CustomMenuItem::new("add".to_string(), "Add");
  let file = Submenu::new("File".to_string(), Menu::new().add_item(add));
  Menu::new().add_submenu(file)
}

fn main() {
  tauri::Builder::default()
    .menu(get())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
