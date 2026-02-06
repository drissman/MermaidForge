// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct ConversionResult {
    success: bool,
    output_path: Option<String>,
    error: Option<String>,
}

#[tauri::command]
async fn convert_mermaid(
    mermaid_code: String,
    format: String,
) -> Result<ConversionResult, String> {
    println!("Conversion demandée : format={}", format);

    if format != "svg" && format != "png" {
        return Ok(ConversionResult {
            success: false,
            output_path: None,
            error: Some("Format non supporté. Utilisez 'svg' ou 'png'.".to_string()),
        });
    }

    let app_data_dir = dirs::data_dir()
        .ok_or("Impossible de trouver le dossier AppData")?;
    
    let output_dir = app_data_dir.join("MermaidForge").join("outputs");
    fs::create_dir_all(&output_dir)
        .map_err(|e| format!("Erreur création dossier : {}", e))?;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let filename = format!("diagram_{}.{}", timestamp, format);
    let output_path = output_dir.join(&filename);

    let result = call_mermaid_api(&mermaid_code, &format, &output_path).await;

    match result {
        Ok(_) => Ok(ConversionResult {
            success: true,
            output_path: Some(output_path.to_string_lossy().to_string()),
            error: None,
        }),
        Err(e) => Ok(ConversionResult {
            success: false,
            output_path: None,
            error: Some(e),
        }),
    }
}

async fn call_mermaid_api(
    mermaid_code: &str,
    format: &str,
    output_path: &PathBuf,
) -> Result<(), String> {
    use base64::{Engine as _, engine::general_purpose};
    let encoded = general_purpose::STANDARD.encode(mermaid_code);
    
    let url = match format {
        "svg" => format!("https://mermaid.ink/svg/{}", encoded),
        "png" => format!("https://mermaid.ink/img/{}", encoded),
        _ => return Err("Format invalide".to_string()),
    };

    println!("Appel API : {}", url);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Erreur requête HTTP : {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Erreur API : {}", response.status()));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Erreur lecture réponse : {}", e))?;

    fs::write(output_path, &bytes)
        .map_err(|e| format!("Erreur écriture fichier : {}", e))?;

    println!("Fichier sauvegardé : {:?}", output_path);
    Ok(())
}

#[tauri::command]
async fn read_file_as_base64(path: String) -> Result<String, String> {
    let bytes = fs::read(&path)
        .map_err(|e| format!("Erreur lecture fichier : {}", e))?;
    
    use base64::{Engine as _, engine::general_purpose};
    Ok(general_purpose::STANDARD.encode(&bytes))
}

#[tauri::command]
async fn open_output_folder() -> Result<(), String> {
    let app_data_dir = dirs::data_dir()
        .ok_or("Impossible de trouver le dossier AppData")?;
    
    let output_dir = app_data_dir.join("MermaidForge").join("outputs");
    
    fs::create_dir_all(&output_dir)
        .map_err(|e| format!("Erreur création dossier : {}", e))?;

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(output_dir.to_string_lossy().to_string())
            .spawn()
            .map_err(|e| format!("Erreur ouverture explorateur : {}", e))?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            convert_mermaid,
            read_file_as_base64,
            open_output_folder
        ])
        .run(tauri::generate_context!())
        .expect("Erreur lors du lancement de l'application");
}
