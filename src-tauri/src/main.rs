// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::decode;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
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

#[tauri::command]
fn save_concept(
    state: tauri::State<'_, AppState>,
    projectId: String,
    conceptBase64: String,
    conceptFilename: String,
) -> String {
    let handle = &state.handle;
    let config = handle.config();
    let package_info = handle.package_info();
    let env = handle.env();

    let sync_dir = PathBuf::from("C:/Users/alext/CommonOSFiles");
    let concepts_dir = sync_dir.join(format!("midpoint/projects/{}/concepts", projectId));

    // Check if the concepts directory exists, create if it doesn't
    if !Path::new(&concepts_dir).exists() {
        fs::create_dir_all(&concepts_dir).expect("Couldn't create concepts directory");
    }

    let concept_path = concepts_dir.join(conceptFilename);

    // Strip the "data:image/png;base64," prefix
    let base64_data = conceptBase64
        .strip_prefix("data:image/png;base64,")
        .ok_or("Invalid base64 image string")
        .expect("Couldn't get base64 string");

    // Decode the base64 string
    let image_data = decode(base64_data)
        .map_err(|e| format!("Couldn't decode base64 string: {}", e))
        .expect("Couldn't decode base64 string");

    // Save the decoded image data to a file
    fs::write(concept_path, image_data)
        .map_err(|e| format!("Couldn't save concept file: {}", e))
        .expect("Couldn't save concept file");

    "success".to_string()
}

#[tauri::command]
fn save_model(
    state: tauri::State<'_, AppState>,
    projectId: String,
    modelBase64: String,
    modelFilename: String,
) -> String {
    let handle = &state.handle;
    let config = handle.config();
    let package_info = handle.package_info();
    let env = handle.env();

    let sync_dir = PathBuf::from("C:/Users/alext/CommonOSFiles");
    let models_dir = sync_dir.join(format!("midpoint/projects/{}/models", projectId));

    // Check if the concepts directory exists, create if it doesn't
    if !Path::new(&models_dir).exists() {
        fs::create_dir_all(&models_dir).expect("Couldn't create models directory");
    }

    let model_path = models_dir.join(modelFilename);

    // Strip the "data:image/png;base64," prefix
    let base64_data = modelBase64
        .strip_prefix("data:model/gltf-binary;base64,")
        .ok_or("Invalid base64 model string")
        .expect("Couldn't get base64 string for model");

    // Decode the base64 string
    let model_data = decode(base64_data)
        .map_err(|e| format!("Couldn't decode base64 string for model: {}", e))
        .expect("Couldn't decode base64 string for model");

    // Save the decoded image data to a file
    fs::write(model_path, model_data)
        .map_err(|e| format!("Couldn't save model file: {}", e))
        .expect("Couldn't save model file");

    "success".to_string()
}

#[tauri::command]
async fn read_model(
    state: tauri::State<'_, AppState>,
    projectId: String,
    modelFilename: String,
) -> Result<Vec<u8>, String> {
    let handle = &state.handle;
    let config = handle.config();
    let package_info = handle.package_info();
    let env = handle.env();

    let sync_dir = PathBuf::from("C:/Users/alext/CommonOSFiles");
    let model_path = sync_dir.join(format!(
        "midpoint/projects/{}/models/{}",
        projectId, modelFilename
    ));

    let mut file = File::open(&model_path).map_err(|e| format!("Failed to open model: {}", e))?;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|e| format!("Failed to read model: {}", e))?;

    Ok(bytes)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let config = handle.config();

            app.manage(AppState { handle });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            read_token,
            create_project,
            save_concept,
            save_model,
            read_model,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
