// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::decode;
use image::io::Reader as ImageReader;
use serde::Serialize;
use std::convert::TryFrom;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::api::path::{app_data_dir, resolve_path, BaseDirectory};
use tauri::{App, AppHandle, Manager};
use tiff::decoder::{Decoder, DecodingResult};

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

#[derive(Serialize)]
struct LandscapeData {
    width: usize,
    height: usize,
    // data: Vec<u8>,
    pixel_data: Vec<Vec<PixelData>>,
}

#[derive(Serialize)]
struct PixelData {
    height_value: f32,
    position: [f32; 3],
    tex_coords: [f32; 2],
}

fn read_tiff_heightmap(landscape_path: &str) -> (usize, usize, Vec<Vec<PixelData>>) {
    let file = File::open(landscape_path).expect("Couldn't open tif file");
    let mut decoder = Decoder::new(file).expect("Couldn't decode tif file");

    let (width, height) = decoder.dimensions().expect("Couldn't get tif dimensions");

    let width = usize::try_from(width).unwrap();
    let height = usize::try_from(height).unwrap();

    let image = match decoder
        .read_image()
        .expect("Couldn't read image data from tif")
    {
        DecodingResult::F32(vec) => vec,
        _ => return (0, 0, Vec::new()),
    };

    let mut pixel_data = Vec::new();
    let scale = 1.0;

    for y in 0..height {
        let mut row = Vec::new();
        for x in 0..width {
            let idx = (y * width + x) as usize;
            let height_value = image[idx] * scale;

            let position = [
                x as f32 / width as f32 * 2.0 - 1.0,
                height_value,
                y as f32 / height as f32 * 2.0 - 1.0,
            ];
            let tex_coords = [x as f32 / width as f32, y as f32 / height as f32];

            row.push(PixelData {
                height_value,
                position,
                tex_coords,
            });
        }
        pixel_data.push(row);
    }

    (width, height, pixel_data)
}

#[tauri::command]
fn get_landscape_pixels(
    state: tauri::State<'_, AppState>,
    projectId: String,
    landscapeFilename: String,
) -> LandscapeData {
    let handle = &state.handle;
    let config = handle.config();
    let package_info = handle.package_info();
    let env = handle.env();

    let sync_dir = PathBuf::from("C:/Users/alext/CommonOSFiles");
    let landscapes_dir = sync_dir.join(format!("midpoint/projects/{}/landscapes", projectId));
    let landscape_path = landscapes_dir.join(landscapeFilename);

    let (width, height, pixel_data) = read_tiff_heightmap(
        landscape_path
            .to_str()
            .expect("Couldn't form landscape string"),
    );

    LandscapeData {
        width,
        height,
        // data: heightmap.to_vec(),
        pixel_data,
    }
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
fn save_texture(
    state: tauri::State<'_, AppState>,
    projectId: String,
    textureBase64: String,
    textureFilename: String,
) -> String {
    let handle = &state.handle;
    let config = handle.config();
    let package_info = handle.package_info();
    let env = handle.env();

    let sync_dir = PathBuf::from("C:/Users/alext/CommonOSFiles");
    let textures_dir = sync_dir.join(format!("midpoint/projects/{}/textures", projectId));

    // Check if the concepts directory exists, create if it doesn't
    if !Path::new(&textures_dir).exists() {
        fs::create_dir_all(&textures_dir).expect("Couldn't create textures directory");
    }

    let texture_path = textures_dir.join(textureFilename);

    // Strip the "data:image/png;base64," prefix
    let base64_data = textureBase64
        .strip_prefix("data:image/png;base64,")
        .ok_or("Invalid base64 image string")
        .expect("Couldn't get base64 string");

    // Decode the base64 string
    let image_data = decode(base64_data)
        .map_err(|e| format!("Couldn't decode base64 string: {}", e))
        .expect("Couldn't decode base64 string");

    // Save the decoded image data to a file
    fs::write(texture_path, image_data)
        .map_err(|e| format!("Couldn't save texture file: {}", e))
        .expect("Couldn't save texture file");

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

#[tauri::command]
fn save_landscape(
    state: tauri::State<'_, AppState>,
    projectId: String,
    landscapeBase64: String,
    landscapeFilename: String,
) -> String {
    let handle = &state.handle;
    let config = handle.config();
    let package_info = handle.package_info();
    let env = handle.env();

    let sync_dir = PathBuf::from("C:/Users/alext/CommonOSFiles");
    let landscapes_dir = sync_dir.join(format!("midpoint/projects/{}/landscapes", projectId));

    // Check if the concepts directory exists, create if it doesn't
    if !Path::new(&landscapes_dir).exists() {
        fs::create_dir_all(&landscapes_dir).expect("Couldn't create landscapes directory");
    }

    let landscape_path = landscapes_dir.join(landscapeFilename);

    // Strip the "data:image/png;base64," prefix
    // let base64_data = landscapeBase64
    //     .strip_prefix("data:image/tiff;base64,")
    //     .ok_or("Invalid base64 landscape string")
    //     .expect("Couldn't get base64 string for landscape");
    let base64_data = landscapeBase64;

    // Decode the base64 string
    let landscape_data = decode(base64_data)
        .map_err(|e| format!("Couldn't decode base64 string for landscape: {}", e))
        .expect("Couldn't decode base64 string for landscape");

    // Save the decoded image data to a file
    fs::write(landscape_path, landscape_data)
        .map_err(|e| format!("Couldn't save landscape file: {}", e))
        .expect("Couldn't save landscape file");

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
        .invoke_handler(tauri::generate_handler![
            read_token,
            create_project,
            save_concept,
            save_model,
            read_model,
            get_landscape_pixels,
            save_landscape,
            save_texture,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
