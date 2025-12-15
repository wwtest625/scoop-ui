use serde::{Deserialize, Serialize};
use std::process::Command;
use tauri::Emitter;

#[derive(Serialize, Deserialize)]
struct ScoopApp {
    name: String,
    version: String,
    description: String,
    bucket: String,
    updated: i64,           // Unix timestamp in milliseconds
    has_update: bool,       // Whether an update is available
    install_size: u64,      // Size in bytes
}

#[derive(Serialize, Deserialize)]
struct SearchResult {
    name: String,
    version: String,
    description: String,
    bucket: String,
}

// Helper function to get list of apps with updates available
fn get_updatable_apps() -> std::collections::HashSet<String> {
    let mut updatable = std::collections::HashSet::new();
    
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "scoop status"])
        .output();
    
    if let Ok(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Parse scoop status output
            // Format is a table with columns: Name, Installed Version, Available Version, Info
            // Example: "redis            8.0.2             8.4.0101"
            let mut in_table = false;
            for line in stdout.lines() {
                let line = line.trim();
                
                // Skip header and separator lines
                if line.starts_with("Name") || line.starts_with("----") {
                    in_table = true;
                    continue;
                }
                
                // Skip warnings and empty lines
                if line.is_empty() || line.starts_with("WARN") || line.starts_with("Scoop") {
                    continue;
                }
                
                // If we're in the table and the line has content, extract the app name
                if in_table && !line.is_empty() {
                    // First word is the app name
                    if let Some(name) = line.split_whitespace().next() {
                        updatable.insert(name.to_string());
                    }
                }
            }
        }
    }
    
    updatable
}

// Helper function to calculate directory size
fn get_dir_size(path: &std::path::Path) -> u64 {
    let mut size = 0u64;
    
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    size += metadata.len();
                } else if metadata.is_dir() {
                    size += get_dir_size(&entry.path());
                }
            }
        }
    }
    
    size
}

// Helper function to get app install size
fn get_app_install_size(app_name: &str) -> u64 {
    // Scoop apps are installed in ~/scoop/apps/<app_name>/current
    if let Some(home) = std::env::var_os("USERPROFILE") {
        let app_path = std::path::PathBuf::from(home)
            .join("scoop")
            .join("apps")
            .join(app_name)
            .join("current");
        
        if app_path.exists() {
            return get_dir_size(&app_path);
        }
    }
    
    0
}

// Helper function to parse .NET date format from scoop export
fn parse_dotnet_date(date_str: &str) -> i64 {
    // Format: "/Date(1744960672608)/"
    if let Some(timestamp_str) = date_str.strip_prefix("/Date(").and_then(|s| s.strip_suffix(")/")) {
        timestamp_str.parse::<i64>().unwrap_or(0)
    } else {
        0
    }
}


#[tauri::command]
fn get_installed_apps() -> Result<Vec<ScoopApp>, String> {
    // Use `scoop list` which is faster than `scoop export`
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "scoop list"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Get list of updatable apps (only call once)
    let updatable_apps = get_updatable_apps();
    
    let mut result = Vec::new();
    let mut in_table = false;
    
    for line in stdout.lines() {
        let line = line.trim();
        
        // Skip header and find separator
        if line.starts_with("Name") || line.starts_with("----") {
            in_table = true;
            continue;
        }
        
        // Skip empty lines and headers
        if line.is_empty() || line.starts_with("Installed apps:") {
            continue;
        }
        
        // Parse table rows
        if in_table {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let name = parts[0].to_string();
                let version = parts[1].to_string();
                let bucket = parts[2].to_string();
                
                // Skip if "Install failed" in info
                let info = parts.get(5..).map(|s| s.join(" ")).unwrap_or_default();
                if info.contains("Install failed") {
                    continue;
                }
                
                result.push(ScoopApp {
                    name: name.clone(),
                    version,
                    bucket,
                    description: "".to_string(),
                    updated: 0, // scoop list doesn't provide easy timestamp parsing
                    has_update: updatable_apps.contains(&name),
                    install_size: 0,
                });
            }
        }
    }

    Ok(result)
}

#[tauri::command]
fn search_apps(query: String) -> Result<Vec<SearchResult>, String> {
    // `scoop search <query>`
    // Output is text table. Needs parsing.
    // '7zip 23.01 main'
    
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &format!("scoop search {}", query)])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    let mut results = Vec::new();
    let lines: Vec<&str> = stdout.lines().collect();
    
    // Skip header usually... 
    // Simple parsing logic:
    // '  7zip (23.01) <main>'
    
    for line in lines {
        let line = line.trim();
        if line.is_empty() || line.starts_with("'main' bucket:") || line.starts_with("Results from") {
            continue;
        }
        
        // Very basic parser: split by whitespace
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 1 {
            results.push(SearchResult {
                name: parts[0].to_string(),
                version: if parts.len() > 1 { parts[1].replace("(", "").replace(")", "") } else { "".to_string() },
                bucket: if parts.len() > 2 { parts.last().unwrap_or(&"").replace("<", "").replace(">", "") } else { "".to_string() },
                description: "".to_string(),
            });
        }
    }

    Ok(results)
}

#[tauri::command]
fn update_app(app_name: String) -> Result<String, String> {
    // Execute `scoop update <app_name>`
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &format!("scoop update {}", app_name)])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    if !output.status.success() {
        return Err(format!("Failed to update {}: {}", app_name, stderr));
    }

    Ok(stdout.to_string())
}

#[tauri::command]
fn update_all_apps() -> Result<String, String> {
    // Execute `scoop update *`
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "scoop update *"])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    if !output.status.success() {
        return Err(format!("Failed to update apps: {}", stderr));
    }

    Ok(stdout.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_installed_apps, search_apps, update_app, update_all_apps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
