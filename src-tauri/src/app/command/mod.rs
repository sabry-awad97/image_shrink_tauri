use image::{GenericImageView, ImageFormat};
use std::{fs, path::Path};
use tauri::api::path;

/// Minimizes an image at the given path with the specified quality and saves it to the user's home directory.
#[tauri::command]
pub fn minimize_image(image_path: &Path, quality: u8) -> Result<(), String> {
    let img = image::open(image_path).map_err(|e| format!("Failed to open image: {}", e))?;

    let (width, height) = img.dimensions();

    let factor = (quality as f32 / 100.0).sqrt();
    let new_width = (width as f32 * factor) as u32;
    let new_height = (height as f32 * factor) as u32;

    let resized_img =
        img.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3);

    let file_extension = image_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .unwrap_or_else(|| "jpg".to_string());

    let img_format = match file_extension.as_str() {
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        _ => ImageFormat::Png,
    };

    let file_name = image_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("Invalid file path")?;

    let output_dir = path::home_dir()
        .ok_or_else(|| "Failed to get home directory".to_string())?
        .join("imageshrink");

    fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;
    let output_path = output_dir.join(file_name);
    resized_img
        .save_with_format(output_path, img_format)
        .map_err(|e| format!("Failed to write image to file: {}", e))?;

    Ok(())
}
