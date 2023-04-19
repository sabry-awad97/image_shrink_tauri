// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};

use image::{io::Reader, GenericImageView, ImageFormat, ImageResult};
use tauri::api::path;

mod app;
use app::setup;

/// Resizes an image at the given path with the specified quality.
fn resize_image(image_path: &Path, quality: u8) -> ImageResult<Vec<u8>> {
    let file_extension = image_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .unwrap_or_else(|| "jpg".to_string());

    let img_format = match file_extension.as_str() {
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        _ => ImageFormat::Png,
    };

    // Load the image from the file path
    let img = Reader::open(image_path)?.decode()?;
    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    let factor = (quality as f32 / 100.0).sqrt();
    let new_width = (width as f32 * factor) as u32;
    let new_height = (height as f32 * factor) as u32;

    let resized_img = img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);
    // Encode the resized image as a JPEG with the specified quality
    let mut output = Cursor::new(Vec::new());

    resized_img.write_to(&mut output, img_format)?;

    Ok(output.into_inner())
}

/// Minimizes an image at the given path with the specified quality and saves it to the user's home directory.
#[tauri::command]
fn minimize_image(image_path: &Path, quality: u8) -> Result<(), String> {
    let file_name = image_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "Invalid file path".to_string())?;

    let img = resize_image(image_path, quality).map_err(|e| e.to_string())?;
    let output_dir =
        PathBuf::from(path::home_dir().ok_or_else(|| "Failed to get home directory".to_string())?)
            .join("imageshrink");
    std::fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;
    let output_path = output_dir.join(file_name);
    let mut output = std::fs::File::create(&output_path).map_err(|e| e.to_string())?;
    output.write_all(&img).map_err(|e| e.to_string())?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(setup::init)
        .invoke_handler(tauri::generate_handler![minimize_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
