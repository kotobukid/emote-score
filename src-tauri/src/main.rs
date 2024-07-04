// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod images;

use std::env;
use std::path::PathBuf;
use dotenv::dotenv;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() -> std::io::Result<()> {
    dotenv().ok();
    let file_dir = env::var("FILE_DIR").expect("FILE_DIR is not defined in .env file");
    let dir: PathBuf = env::current_dir().unwrap().join(file_dir);

    let files: Vec<String> = images::visit_dirs(&dir).unwrap();

    files.iter().for_each(|f| {
        println!("{}", f);
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
