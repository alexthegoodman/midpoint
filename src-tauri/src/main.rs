// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::api::path::{app_data_dir, resolve_path, BaseDirectory};
use tauri::{App, AppHandle, Manager};

struct AppState {
    handle: AppHandle,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn read_token(state: tauri::State<'_, AppState>) -> String {
    let handle = &state.handle;
    let config = handle.config();
    let package_info = handle.package_info();
    let env = handle.env();

    read_auth_token(&config)
}

fn read_auth_token(config: &Arc<tauri::Config>) -> String {
    println!("read_auth_token");

    let app_data_path = app_data_dir(config)
        .ok_or("Failed to get AppData directory (1)")
        .expect("Failed to get AppData directory (2)");
    let app_data_path = app_data_path
        .parent()
        .expect("Failed to get AppData directory (3)")
        .join("com.common.commonosfiles");
    let read_path = app_data_path.join("auth");

    // pull String content from read_path
    let auth_data =
        String::from_utf8_lossy(&std::fs::read(read_path).unwrap_or_default()).to_string();

    auth_data
}

#[tauri::command]
fn create_project(state: tauri::State<'_, AppState>, projectId: String) -> String {
    let handle = &state.handle;
    let config = handle.config();
    let package_info = handle.package_info();
    let env = handle.env();

    let sync_dir = PathBuf::from("C:/Users/alext/CommonOSFiles");
    let project_dir = PathBuf::from(format!(
        "C:/Users/alext/CommonOSFiles/midpoint/projects/{}",
        projectId
    ));

    // create project folder(s) within sync folder: /CommonOSFiles/midpoint/projects/project_id/
    fs::create_dir_all(project_dir).expect("Couldn't create project directory");

    "success".to_string()
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let config = handle.config();

            app.manage(AppState { handle });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read_token, create_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
