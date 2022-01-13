#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

//use tauri::{CustomMenuItem, Menu, Submenu};
use serde::{Deserialize, Serialize};

mod menu;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RequestBody {
  id: i32,
  name: String,
}

#[tauri::command]
fn my_custom_command() -> String {
  println!("I was invoked from JS!");
  String::from("This is some stuff")
}

#[tauri::command]
fn do_a_thing(body: RequestBody) -> String {
  println!("{:?}", body);
  format!("{:?}", body)
  // "message response".into()
}

fn main() {
  tauri::Builder::default()
    .menu(menu::get_menu())
    .on_menu_event(|event| {
      match event.menu_item_id() {
        "quit" => {
          std::process::exit(0);
        },
        _ => {
          println!("{:?}", event.menu_item_id());
        }
      }
    })
    .invoke_handler(tauri::generate_handler![my_custom_command, do_a_thing])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
