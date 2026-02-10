#![windows_subsystem = "windows"]
//! CatShare - Fast & Easy File Sharing Application
//!
//! A cross-platform file sharing application built with Rust and egui
//!
//! Features:
//! - Automatic name generation based on IP (Adjective + Noun format)
//! - ID-based peer discovery using IP last octet
//! - High-performance LZ4 compression
//! - Multi-threaded file transfer
//! - Support for 10 languages with easy i18n extension

mod config;
mod crypto;
mod i18n;
mod network;
mod transfer;
mod ui;

use eframe::egui;
use ui::CatShareApp;

// Add image crate for icon loading
use image;

fn main() -> eframe::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    tracing::info!("Starting CatShare...");

    // Configure native options for a high-end feel
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 660.0])
            .with_min_inner_size([420.0, 580.0])
            .with_icon(load_icon()),
        centered: true,
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "CatShare - File Sharing",
        native_options,
        Box::new(|cc| {
            // Configure fonts for multi-language support
            configure_fonts(&cc.egui_ctx);

            // Apply custom high-end theme with configurable palette
            let config = crate::config::Config::load().unwrap_or_else(|_| crate::config::Config::default());
            ui::setup_custom_theme(&cc.egui_ctx, &config.get_app_color_theme());

            Ok(Box::new(CatShareApp::new()))
        }),
    )
}

/// Configure fonts to support multiple languages
fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Load multiple fonts for comprehensive language support
    let font_configs = load_system_fonts();

    for (font_name, font_data) in font_configs {
        fonts
            .font_data
            .insert(font_name.clone(), egui::FontData::from_owned(font_data));

        // Add to proportional font family
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .push(font_name.clone());

        // Also add for monospace as fallback
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push(font_name);
    }

    ctx.set_fonts(fonts);
}

/// Load multiple system fonts for comprehensive language support
fn load_system_fonts() -> Vec<(String, Vec<u8>)> {
    use font_kit::family_name::FamilyName;
    use font_kit::properties::Properties;
    use font_kit::source::SystemSource;

    let source = SystemSource::new();
    let mut loaded_fonts = Vec::new();

    // Font categories for different language groups, prioritized by modern UI standards
    let font_groups = [
        // Modern System UI & Latin - Focus on high readability and "speed" aesthetics
        vec![
            "Inter",
            "Segoe UI Variable Display",
            "Segoe UI",
            "San Francisco",
            "SF Pro Display",
            "Roboto",
            "Ubuntu",
            "Helvetica Neue",
        ],
        // CJK fonts (Chinese, Japanese, Korean) - Prioritize UI-optimized variants
        vec![
            "Microsoft YaHei UI",
            "PingFang SC",
            "Hiragino Sans GB",
            "Noto Sans CJK SC",
            "Yu Gothic UI",
            "Meiryo UI",
            "Malgun Gothic",
            "WenQuanYi Micro Hei",
        ],
        // Arabic fonts
        vec![
            "Segoe UI",
            "Noto Sans Arabic",
            "Dubai",
            "Tahoma",
            "Simplified Arabic",
        ],
        // Devanagari/Hindi fonts
        vec!["Nirmala UI", "Noto Sans Devanagari", "Mangal"],
        // Universal fallback
        vec!["Segoe UI Symbol", "Arial Unicode MS", "DejaVu Sans"],
    ];

    // Track which fonts we've already loaded
    let mut loaded_names = std::collections::HashSet::new();

    for group in &font_groups {
        for font_name in group {
            if loaded_names.contains(*font_name) {
                continue;
            }

            if let Ok(handle) = source.select_best_match(
                &[FamilyName::Title(font_name.to_string())],
                &Properties::default(),
            ) {
                if let Ok(font) = handle.load() {
                    if let Some(data) = font.copy_font_data() {
                        tracing::info!("Loaded font: {}", font_name);
                        loaded_fonts.push((font_name.to_string(), (*data).clone()));
                        loaded_names.insert(*font_name);
                        // Load at most one font per group to avoid too many fallbacks
                        break;
                    }
                }
            }
        }
    }

    // If we didn't load any fonts, try generic sans-serif
    if loaded_fonts.is_empty() {
        if let Ok(handle) =
            source.select_best_match(&[FamilyName::SansSerif], &Properties::default())
        {
            if let Ok(font) = handle.load() {
                if let Some(data) = font.copy_font_data() {
                    tracing::info!("Loaded fallback sans-serif font");
                    loaded_fonts.push(("fallback_sans".to_string(), (*data).clone()));
                }
            }
        }
    }

    if loaded_fonts.is_empty() {
        tracing::warn!("Could not load any system fonts for multi-language support");
    } else {
        tracing::info!(
            "Loaded {} fonts for multi-language support",
            loaded_fonts.len()
        );
    }

    loaded_fonts
}

/// Load application icon
fn load_icon() -> egui::IconData {
    // Load icon from assets directory
    load_icon_from_path("assets/icon.png").unwrap_or_else(|_| {
        // Fallback to a simple colored square if icon loading fails
        let size = 32;
        let mut rgba = vec![0u8; size * size * 4];

        // Create a simple colored square as placeholder
        for y in 0..size {
            for x in 0..size {
                let idx = (y * size + x) * 4;
                // Softer Electric Blue color (matching speed-focused brand)
                rgba[idx] = 79; // R
                rgba[idx + 1] = 114; // G
                rgba[idx + 2] = 205; // B
                rgba[idx + 3] = 255; // A
            }
        }

        egui::IconData {
            rgba,
            width: size as u32,
            height: size as u32,
        }
    })
}

/// Load icon from a specified path
fn load_icon_from_path(path: &str) -> Result<egui::IconData, Box<dyn std::error::Error>> {
    use image::imageops::{self, FilterType};
    
    let image = image::open(path)?;
    let image = image.to_rgba8();
    let (width, height) = image.dimensions();
    
    // Ensure the icon is properly sized for display (typically 256x256 or 512x512)
    // Most systems expect square icons
    let target_size = 256u32; // Standard icon size
    
    // Resize the image maintaining aspect ratio and pad if needed
    let resized_image = if width != height {
        // If the image is not square, create a square canvas and center the image
        let max_dim = width.max(height);
        let scale_factor = (target_size as f32) / (max_dim as f32);
        let new_width = (width as f32 * scale_factor) as u32;
        let new_height = (height as f32 * scale_factor) as u32;
        
        // Resize the image
        let resized = imageops::resize(&image, new_width, new_height, FilterType::Lanczos3);
        
        // Create a square canvas
        let mut square_canvas = image::RgbaImage::new(target_size, target_size);
        
        // Calculate position to center the resized image
        let x_offset = (target_size - new_width) / 2;
        let y_offset = (target_size - new_height) / 2;
        
        // Paste the resized image onto the center of the square canvas
        imageops::overlay(&mut square_canvas, &resized, x_offset as i64, y_offset as i64);
        
        square_canvas
    } else {
        // If already square, just resize to target size
        imageops::resize(&image, target_size, target_size, FilterType::Lanczos3)
    };
    
    Ok(egui::IconData {
        rgba: resized_image.as_raw().clone(),
        width: target_size,
        height: target_size,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        // Basic smoke test
        let _app = CatShareApp::new();
    }
}
