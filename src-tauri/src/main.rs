use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};

#[derive(Serialize, Deserialize)]
struct ConversionResult {
    success: bool,
    output_path: Option<String>,
    error: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct BatchResult {
    total: i32,
    converted: i32,
    failed: i32,
    files: Vec<String>,
}

#[tauri::command]
fn convert_mermaid(mermaid_code: String, format: String, scale: Option<f32>, width: Option<u32>, height: Option<u32>, bg_color: Option<String>) -> ConversionResult {
    let clean_code = mermaid_code.trim_start_matches('\u{feff}').trim();
    let encoded = general_purpose::STANDARD.encode(clean_code);

    let base_url = match format.as_str() {
        "svg" => format!("https://mermaid.ink/svg/{}", encoded),
        "png" => format!("https://mermaid.ink/img/{}", encoded),
        _ => return ConversionResult {
            success: false,
            output_path: None,
            error: Some("Format invalide".to_string()),
        },
    };

    let mut params: Vec<String> = Vec::new();
    let has_custom_width = width.is_some();
    let has_custom_height = height.is_some();

    if let Some(w) = width {
        params.push(format!("width={}", w));
    }
    if let Some(h) = height {
        params.push(format!("height={}", h));
    }
    // scale requires width or height to be set; if neither was provided, use a default width
    if let Some(s) = scale {
        let s = s.clamp(1.0, 3.0);
        if s > 1.0 {
            if !has_custom_width && !has_custom_height {
                params.push("width=800".to_string());
            }
            params.push(format!("scale={}", s));
        }
    }
    if let Some(bg) = bg_color {
        if !bg.is_empty() {
            params.push(format!("bgColor=!{}", bg.trim_start_matches('#')));
        }
    }

    let url = if params.is_empty() {
        base_url
    } else {
        format!("{}?{}", base_url, params.join("&"))
    };

    match reqwest::blocking::get(&url) {
        Ok(response) => {
            if response.status().is_success() {
                match response.bytes() {
                    Ok(bytes) => {
                        // Fix SVG rendering on Linux (Qt/Gwenview doesn't apply CSS <style>):
                        // 1. Arrows: stroke="none" makes lines invisible
                        // 2. Text: no inline fill attribute, relies on CSS → invisible in Qt
                        // 3. Font fallback for systems without Microsoft fonts
                        let output_bytes = if format == "svg" {
                            let svg_content = String::from_utf8_lossy(&bytes).to_string();
                            let fixed = svg_content
                                // Fix invisible arrows
                                .replace("stroke=\"none\" marker-end", "stroke=\"#333\" marker-end")
                                .replace("stroke=\"none\" marker-start", "stroke=\"#333\" marker-start")
                                // Fix invisible text: add inline fill to all <text> elements
                                .replace("<text ", "<text fill=\"#333\" ")
                                // Add Linux-compatible font fallbacks
                                .replace(
                                    "\"trebuchet ms\",verdana,arial,sans-serif",
                                    "\"Trebuchet MS\",\"DejaVu Sans\",\"Liberation Sans\",verdana,arial,sans-serif"
                                );
                            fixed.into_bytes()
                        } else {
                            bytes.to_vec()
                        };

                        let app_dir = dirs::data_local_dir()
                            .unwrap_or_else(|| std::path::PathBuf::from("."))
                            .join("MermaidForge")
                            .join("outputs");

                        if let Err(e) = fs::create_dir_all(&app_dir) {
                            return ConversionResult {
                                success: false,
                                output_path: None,
                                error: Some(format!("Erreur création dossier: {}", e)),
                            };
                        }

                        let timestamp = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        let filename = format!("diagram_{}.{}", timestamp, format);
                        let output_path = app_dir.join(&filename);

                        match fs::write(&output_path, &output_bytes) {
                            Ok(_) => ConversionResult {
                                success: true,
                                output_path: Some(output_path.to_string_lossy().to_string()),
                                error: None,
                            },
                            Err(e) => ConversionResult {
                                success: false,
                                output_path: None,
                                error: Some(format!("Erreur écriture: {}", e)),
                            },
                        }
                    }
                    Err(e) => ConversionResult {
                        success: false,
                        output_path: None,
                        error: Some(format!("Erreur lecture réponse: {}", e)),
                    },
                }
            } else {
                ConversionResult {
                    success: false,
                    output_path: None,
                    error: Some(format!("HTTP {}", response.status())),
                }
            }
        }
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Erreur réseau: {}", e)),
        },
    }
}

#[tauri::command]
fn batch_convert_folder(folder_path: String, format: String, scale: Option<f32>, width: Option<u32>, height: Option<u32>, bg_color: Option<String>) -> BatchResult {
    let mut total = 0;
    let mut converted = 0;
    let mut failed = 0;
    let mut files = Vec::new();

    let path = Path::new(&folder_path);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();

            if let Some(ext) = file_path.extension() {
                if ext == "mmd" {
                    total += 1;

                    if let Ok(content) = fs::read_to_string(&file_path) {
                        let result = convert_mermaid(content, format.clone(), scale, width, height, bg_color.clone());

                        if result.success {
                            converted += 1;
                            if let Some(output) = result.output_path {
                                files.push(output);
                            }
                        } else {
                            failed += 1;
                        }
                    } else {
                        failed += 1;
                    }
                }
            }
        }
    }

    BatchResult {
        total,
        converted,
        failed,
        files,
    }
}

#[tauri::command]
fn read_file_as_base64(path: String) -> Result<String, String> {
    match fs::read(&path) {
        Ok(bytes) => Ok(general_purpose::STANDARD.encode(&bytes)),
        Err(e) => Err(format!("Erreur lecture fichier: {}", e)),
    }
}

#[tauri::command]
fn open_output_folder() -> Result<(), String> {
    let app_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("MermaidForge")
        .join("outputs");

    if let Err(e) = fs::create_dir_all(&app_dir) {
        return Err(format!("Erreur création dossier: {}", e));
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&app_dir)
            .spawn()
            .map_err(|e| format!("Erreur ouverture: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        // Try dolphin (KDE) first, then nautilus (GNOME), fallback to xdg-open
        let result = std::process::Command::new("dolphin")
            .arg(&app_dir)
            .spawn()
            .or_else(|_| std::process::Command::new("nautilus").arg(&app_dir).spawn())
            .or_else(|_| std::process::Command::new("xdg-open").arg(&app_dir).spawn());
        result.map_err(|e| format!("Erreur ouverture: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&app_dir)
            .spawn()
            .map_err(|e| format!("Erreur ouverture: {}", e))?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            convert_mermaid,
            batch_convert_folder,
            read_file_as_base64,
            open_output_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}