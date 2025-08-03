// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Stdio};

fn main() {
  tauri::Builder::default()
      .setup(|_app| {
          Command::new("../../backend/venv/Scripts/python.exe")
              .args(&["../../backend/app.py"])
              .stdout(Stdio::null())
              .stderr(Stdio::null())
              .spawn()
              .expect("Failed to start Python backend");
          Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
