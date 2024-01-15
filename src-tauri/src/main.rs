// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


const BASIC_SETUP_WINDOW_WIDTH: f64 = 350.0;  
const BASIC_SETUP_WINDOW_HEIGHT: f64 = 140.0;  
const SETUP_WINDOW_WIDTH_WITH_PROGRAMMES: f64 = 720.0;  
const SETUP_WINDOW_HEIGHT_WITH_LIST: f64 = 500.0;  
const STUDYING_WINDOW_WIDTH: f64 = 150.0;  
const STUDYING_WINDOW_HEIGHT: f64 = 150.0;  
const STUDYING_WINDOW_HEIGHT_SMALL: f64 = 18.0;  


fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_store::Builder::default().build())
    .invoke_handler(tauri::generate_handler![
      tauri_on_start, 
      tauri_window_state_change,
      tauri_study_ended
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command] // i.e. on ui intial load
fn tauri_on_start(window: tauri::Window) {
  set_window_size(window.clone(), false, false, false);

  use window_shadows::set_shadow;
  set_shadow(&window, true).unwrap(); 
  
  window.set_always_on_top(true).unwrap();

  use tauri_plugin_positioner::{WindowExt, Position}; 
  let _ = window.move_window(Position::Center);
}

#[tauri::command]
fn tauri_window_state_change(window: tauri::Window, studying: bool, list: bool, programmes: bool) {
  set_window_size(window, studying, list, programmes);
}

#[tauri::command]
fn tauri_study_ended(window: tauri::Window) {
  use tauri_plugin_positioner::{WindowExt, Position};
  let _ = window.move_window(Position::Center);
}

fn set_window_size(window: tauri::Window, is_studying: bool, study_list_showing: bool, programmes_showing: bool) {
  let mut width: f64 = 100.0;
  let mut height: f64 = 100.0;

  if is_studying {
    match window.current_monitor() {
        Ok(Some(monitor)) => {
          width = monitor.size().width as f64;
          height = STUDYING_WINDOW_HEIGHT_SMALL;

          use tauri::Position;
          let position = Position::Logical(tauri::LogicalPosition::new(0.0, 0.0));
          window.set_position(position).unwrap();
        },
      Ok(None) => { 
          println!("No monitor found");
      },
      Err(e) => { 
          println!("An error occurred: {:?}", e);
      },
    }

  } else {
    width = get_width(is_studying, programmes_showing);
    height = get_height(is_studying, study_list_showing, programmes_showing);
  }
  
  window.set_size(tauri::Size::Logical(tauri::LogicalSize {
    width,
    height,
  })).unwrap();
 
  use window_shadows::set_shadow;
  set_shadow(&window, !is_studying).unwrap(); 
} 

fn get_width(is_studying: bool, programmes_showing: bool) -> f64 {
  if is_studying {
    return STUDYING_WINDOW_WIDTH; 
  }

  if programmes_showing {
    return SETUP_WINDOW_WIDTH_WITH_PROGRAMMES; 
  }

  return BASIC_SETUP_WINDOW_WIDTH; 
}

fn get_height(is_studying: bool, study_list_showing: bool, programmes_showing: bool) -> f64 {
  if is_studying {
    return STUDYING_WINDOW_HEIGHT; 
  }

  if study_list_showing || programmes_showing {
    return SETUP_WINDOW_HEIGHT_WITH_LIST;
  }

  return BASIC_SETUP_WINDOW_HEIGHT;
} 

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct Data {
  key: String,
  value: String,
}
