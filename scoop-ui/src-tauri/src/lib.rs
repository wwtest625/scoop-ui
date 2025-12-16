use serde::{Deserialize, Serialize};
use std::process::Command;
use std::os::windows::process::CommandExt;
use tauri::Emitter;

mod install;

#[derive(Serialize, Deserialize, Clone)]
struct ScoopBucket {
    name: String,
    source: String,
    updated: u64,
}

#[derive(Serialize, Deserialize, Clone)]
struct DiscoverApp {
    name: String,
    description: String,
    version: String,
    homepage: String,
    bucket: String,
    icon: String,
}

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

#[derive(Serialize, Deserialize, Clone)]
struct UpdatesCheckedPayload {
    updatable_apps: Vec<String>,
}

#[tauri::command]
fn get_buckets() -> Result<Vec<ScoopBucket>, String> {
    use std::path::PathBuf;
    
    let buckets_dir = std::env::var_os("USERPROFILE")
        .map(|home| PathBuf::from(home).join("scoop").join("buckets"))
        .ok_or("Failed to get USERPROFILE")?;

    if !buckets_dir.exists() {
        return Ok(Vec::new());
    }

    let mut buckets = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(&buckets_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                
                // Try to find the git remote origin URL if possible, otherwise just use local path or empty
                // Reading .git/config is one way, but simple approach logic:
                // Check if .git exists
                let git_config = entry.path().join(".git").join("config");
                let mut source = "Local".to_string();
                
                if git_config.exists() {
                     if let Ok(config) = std::fs::read_to_string(git_config) {
                         // Simple parse for [remote "origin"] url = ...
                         // This is hacky but fast. 
                         // Or we can just leave source empty or "Git".
                         // Let's try to extract url = ...
                         let lines: Vec<&str> = config.lines().collect();
                         let mut in_origin = false;
                         for line in lines {
                             let trimmed = line.trim();
                             if trimmed == "[remote \"origin\"]" {
                                 in_origin = true;
                             } else if in_origin && trimmed.starts_with("url =") {
                                 source = trimmed.trim_start_matches("url =").trim().to_string();
                                 break;
                             } else if in_origin && trimmed.starts_with("[") {
                                 in_origin = false;
                             }
                         }
                     }
                }
                
                // Get updated time (last modified of the folder or .git/FETCH_HEAD?)
                let updated = if let Ok(metadata) = std::fs::metadata(&entry.path()) {
                    metadata.modified()
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs())
                        .unwrap_or(0)
                } else {
                    0
                };
                
                buckets.push(ScoopBucket {
                    name,
                    source,
                    updated,
                });
            }
        }
    }
    
    Ok(buckets)
}

#[tauri::command]
fn add_bucket(name: String, url: Option<String>) -> Result<String, String> {
    let mut args = vec!["-NoProfile", "-Command", "scoop", "bucket", "add", &name];
    if let Some(ref u) = url {
        args.push(u);
    }

    let output = Command::new("powershell")
        .args(&args)
        .creation_flags(0x08000000)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tauri::command]
fn remove_bucket(name: String) -> Result<String, String> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "scoop", "bucket", "rm", &name])
        .creation_flags(0x08000000)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tauri::command]
fn search_apps(query: String) -> Result<Vec<SearchResult>, String> {
    // `scoop search <query>`
    // Output is text table. Needs parsing.
    // '7zip 23.01 main'
    
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &format!("scoop search {}", query)])
        .creation_flags(0x08000000)
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
fn update_app(app_handle: tauri::AppHandle, app_name: String) -> Result<String, String> {
    let app_handle_clone = app_handle.clone();
    let app_name_clone = app_name.clone();
    
    // 发送开始更新事件
    let _ = app_handle.emit("update-progress", install::InstallProgress {
        app_name: app_name.clone(),
        status: "starting".to_string(),
        progress: 0.0,
        message: format!("开始更新 {}...", app_name),
    });
    
    // 使用线程在后台执行更新
    std::thread::spawn(move || {
        // 模拟进度更新 (5秒内从0到100)
        let total_steps = 50;
        let step_duration = std::time::Duration::from_millis(100);
        
        for i in 1..=total_steps {
            std::thread::sleep(step_duration);
            let progress = i as f32 / total_steps as f32;
            let message = if progress < 0.4 {
                format!("正在检查 {} 的更新...", app_name_clone)
            } else if progress < 0.7 {
                format!("正在下载 {} 的新版本...", app_name_clone)
            } else {
                format!("正在安装 {} 的更新...", app_name_clone)
            };
            
            let _ = app_handle_clone.emit("update-progress", install::InstallProgress {
                app_name: app_name_clone.clone(),
                status: "updating".to_string(),
                progress,
                message,
            });
        }
        
        let output = Command::new("powershell")
            .args(["-NoProfile", "-Command", &format!("scoop update {}", app_name_clone)])
            .creation_flags(0x08000000)
            .output();
        
        match output {
            Ok(output) => {
                if !output.status.success() {
                    let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
                    let _ = app_handle_clone.emit("update-progress", install::InstallProgress {
                        app_name: app_name_clone.clone(),
                        status: "error".to_string(),
                        progress: 0.0,
                        message: error_msg,
                    });
                } else {
                    // 发送完成事件
                    let _ = app_handle_clone.emit("update-progress", install::InstallProgress {
                        app_name: app_name_clone.clone(),
                        status: "completed".to_string(),
                        progress: 1.0,
                        message: format!("{} 更新成功!", app_name_clone),
                    });
                }
            }
            Err(e) => {
                let _ = app_handle_clone.emit("update-progress", install::InstallProgress {
                    app_name: app_name_clone.clone(),
                    status: "error".to_string(),
                    progress: 0.0,
                    message: e.to_string(),
                });
            }
        }
    });
    
    Ok("更新已开始".to_string())
}

#[tauri::command]
fn update_all_apps(app_handle: tauri::AppHandle) -> Result<String, String> {
    let app_handle_clone = app_handle.clone();
    
    // 发送开始批量更新事件
    let _ = app_handle.emit("update-all-progress", install::InstallProgress {
        app_name: "all".to_string(),
        status: "starting".to_string(),
        progress: 0.0,
        message: "开始批量更新所有应用...".to_string(),
    });
    
    // 使用线程在后台执行批量更新
    std::thread::spawn(move || {
        // 模拟进度更新 (5秒内从0到100)
        let total_steps = 50;
        let step_duration = std::time::Duration::from_millis(100);
        
        for i in 1..=total_steps {
            std::thread::sleep(step_duration);
            let progress = i as f32 / total_steps as f32;
            let message = if progress < 0.3 {
                "正在检查可更新的应用...".to_string()
            } else if progress < 0.6 {
                "正在下载更新...".to_string()
            } else if progress < 0.9 {
                "正在安装更新...".to_string()
            } else {
                "即将完成...".to_string()
            };
            
            let _ = app_handle_clone.emit("update-all-progress", install::InstallProgress {
                app_name: "all".to_string(),
                status: "updating".to_string(),
                progress,
                message,
            });
        }
        
        let output = Command::new("powershell")
            .args(["-NoProfile", "-Command", "scoop update *"])
            .creation_flags(0x08000000)
            .output();
        
        match output {
            Ok(output) => {
                if !output.status.success() {
                    let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
                    let _ = app_handle_clone.emit("update-all-progress", install::InstallProgress {
                        app_name: "all".to_string(),
                        status: "error".to_string(),
                        progress: 0.0,
                        message: error_msg,
                    });
                } else {
                    // 发送完成事件
                    let _ = app_handle_clone.emit("update-all-progress", install::InstallProgress {
                        app_name: "all".to_string(),
                        status: "completed".to_string(),
                        progress: 1.0,
                        message: "所有应用更新完成!".to_string(),
                    });
                }
            }
            Err(e) => {
                let _ = app_handle_clone.emit("update-all-progress", install::InstallProgress {
                    app_name: "all".to_string(),
                    status: "error".to_string(),
                    progress: 0.0,
                    message: e.to_string(),
                });
            }
        }
    });
    
    Ok("批量更新已开始".to_string())
}

#[tauri::command]
fn update_scoop(app_handle: tauri::AppHandle) -> Result<String, String> {
    // 发送开始更新事件
    let _ = app_handle.emit("scoop-update-progress", install::InstallProgress {
        app_name: "scoop".to_string(),
        status: "starting".to_string(),
        progress: 0.0,
        message: "开始更新 Scoop 和 Buckets...".to_string(),
    });
    
    // 使用线程在后台执行更新
    std::thread::spawn(move || {
        // 模拟进度更新
        for i in 1..=3 {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let progress = i as f32 / 10.0;
            let _ = app_handle.emit("scoop-update-progress", install::InstallProgress {
                app_name: "scoop".to_string(),
                status: "updating".to_string(),
                progress,
                message: "正在更新 Scoop 和 Buckets...".to_string(),
            });
        }
        
        // 执行实际的 scoop update 命令
        let output = Command::new("powershell")
            .args(["-NoProfile", "-Command", "scoop update"])
            .creation_flags(0x08000000)
            .output();
        
        match output {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    let _ = app_handle.emit("scoop-update-progress", install::InstallProgress {
                        app_name: "scoop".to_string(),
                        status: "completed".to_string(),
                        progress: 1.0,
                        message: format!("Scoop 和 Buckets 更新完成!\n{}", stdout),
                    });
                } else {
                    let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
                    let _ = app_handle.emit("scoop-update-progress", install::InstallProgress {
                        app_name: "scoop".to_string(),
                        status: "error".to_string(),
                        progress: 0.0,
                        message: format!("更新失败: {}", error_msg),
                    });
                }
            }
            Err(e) => {
                let _ = app_handle.emit("scoop-update-progress", install::InstallProgress {
                    app_name: "scoop".to_string(),
                    status: "error".to_string(),
                    progress: 0.0,
                    message: e.to_string(),
                });
            }
        }
    });
    
    Ok("Scoop 更新已开始".to_string())
}

#[derive(Serialize, Deserialize)]
struct AppDetail {
    name: String,
    version: String,
    description: String,
    homepage: String,
    license: String,
    bucket: String,
    notes: Vec<String>,
    bin: Vec<String>,
    depends: Option<Vec<String>>,
    suggest: Option<serde_json::Value>,
}

// Helper function to get list of apps with updates available
fn get_updatable_apps() -> std::collections::HashSet<String> {
    let mut updatable = std::collections::HashSet::new();
    
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "scoop status"])
        .creation_flags(0x08000000)
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
    use std::path::PathBuf;
    
    let apps_dir = std::env::var_os("USERPROFILE")
        .map(|home| PathBuf::from(home).join("scoop").join("apps"))
        .ok_or("Failed to get USERPROFILE")?;

    if !apps_dir.exists() {
        return Ok(Vec::new());
    }

    // Optimization: Skip synchronous update check to avoid UI freeze
    // Updates will be checked asynchronously via check_updates_async command
    // let updatable_apps = get_updatable_apps(); 
    
    let mut result = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(&apps_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name == "scoop" { continue; } // Optional: skip scoop itself?
                
                let current_path = entry.path().join("current");
                if !current_path.exists() { continue; }
                
                // Read manifest.json for version/desc
                let mut version = String::new();
                let mut description = String::new();
                
                let manifest_path = current_path.join("manifest.json");
                if let Ok(content) = std::fs::read_to_string(&manifest_path) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        version = json.get("version").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        description = json.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    }
                }
                
                // Read install.json for bucket
                let mut bucket = "unknown".to_string();
                let install_path = current_path.join("install.json");
                if let Ok(content) = std::fs::read_to_string(&install_path) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        bucket = json.get("bucket").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
                    }
                }
                
                // Check failed install? Usually if 'current' exists, it's fine.
                
                 result.push(ScoopApp {
                    name: name.clone(),
                    version,
                    bucket,
                    description,
                    updated: 0, 
                    has_update: false,  // Will be updated asynchronously
                    install_size: 0,
                });
            }
        }
    }

    Ok(result)
}

#[tauri::command]
fn get_app_sizes(app_names: Vec<String>) -> Result<std::collections::HashMap<String, u64>, String> {
    let mut sizes = std::collections::HashMap::new();
    
    for app_name in app_names {
        let size = get_app_install_size(&app_name);
        sizes.insert(app_name, size);
    }
    
    Ok(sizes)
}

#[tauri::command]
fn check_updates_async(app_handle: tauri::AppHandle) -> Result<String, String> {
    // Spawn a background thread to check for updates asynchronously
    std::thread::spawn(move || {
        // Get list of updatable apps (this is slow, 2-5 seconds)
        let updatable_apps = get_updatable_apps();
        
        // Convert HashSet to Vec for serialization
        let updatable_list: Vec<String> = updatable_apps.into_iter().collect();
        
        // Emit event to frontend with the list of updatable apps
        let _ = app_handle.emit("updates-checked", UpdatesCheckedPayload {
            updatable_apps: updatable_list,
        });
    });
    
    Ok("Update check started in background".to_string())
}



#[tauri::command]
fn get_random_apps(count: usize) -> Result<Vec<DiscoverApp>, String> {
    use rand::seq::SliceRandom;
    use std::path::PathBuf;
    
    // Material icon pool for random selection
    let icons = vec![
        "terminal", "code", "folder", "settings", "extension",
        "download", "cloud", "storage", "database", "security",
        "language", "build", "bug_report", "dashboard", "devices",
        "edit", "favorite", "grade", "help", "home",
        "info", "lock", "palette", "play_circle", "power",
        "search", "star", "sync", "videocam", "volume_up",
        "wifi", "work", "apps", "archive", "bookmark",
    ];
    
    // Get scoop buckets directory
    let buckets_dir = std::env::var_os("USERPROFILE")
        .map(|home| PathBuf::from(home).join("scoop").join("buckets"))
        .ok_or("Failed to get USERPROFILE")?;
    
    if !buckets_dir.exists() {
        return Err("Scoop buckets directory not found".to_string());
    }
    
    // Collect all manifest files from all buckets
    let mut manifest_files = Vec::new();
    
    if let Ok(bucket_entries) = std::fs::read_dir(&buckets_dir) {
        for bucket_entry in bucket_entries.flatten() {
            if bucket_entry.path().is_dir() {
                let bucket_name = bucket_entry.file_name().to_string_lossy().to_string();
                let bucket_dir = bucket_entry.path().join("bucket");
                
                if bucket_dir.exists() {
                    if let Ok(manifest_entries) = std::fs::read_dir(&bucket_dir) {
                        for manifest_entry in manifest_entries.flatten() {
                            let path = manifest_entry.path();
                            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                                manifest_files.push((path, bucket_name.clone()));
                            }
                        }
                    }
                }
            }
        }
    }
    
    if manifest_files.is_empty() {
        return Err("No manifest files found".to_string());
    }
    
    // Randomly select manifests
    let mut rng = rand::thread_rng();
    manifest_files.shuffle(&mut rng);
    
    let selected_count = count.min(manifest_files.len());
    let mut apps = Vec::new();
    
    for (manifest_path, bucket_name) in manifest_files.iter().take(selected_count) {
        // Parse manifest JSON
        if let Ok(content) = std::fs::read_to_string(manifest_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                let name = manifest_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                let description = json.get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                
                let version = json.get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                
                let homepage = json.get("homepage")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                
                // Randomly select an icon
                let icon = icons.choose(&mut rng)
                    .unwrap_or(&"apps")
                    .to_string();
                
                apps.push(DiscoverApp {
                    name,
                    description,
                    version,
                    homepage,
                    bucket: bucket_name.clone(),
                    icon,
                });
            }
        }
    }
    
    Ok(apps)
}

#[tauri::command]
fn get_app_detail(app_name: String, bucket: String) -> Result<AppDetail, String> {
    use std::path::PathBuf;
    
    // Construct path to manifest file
    let manifest_path = std::env::var_os("USERPROFILE")
        .map(|home| {
            PathBuf::from(home)
                .join("scoop")
                .join("buckets")
                .join(&bucket)
                .join("bucket")
                .join(format!("{}.json", app_name))
        })
        .ok_or("Failed to get USERPROFILE")?;
    
    if !manifest_path.exists() {
        return Err(format!("Manifest not found for {} in bucket {}", app_name, bucket));
    }
    
    // Read and parse manifest
    let content = std::fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read manifest: {}", e))?;
    
    let json: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse manifest: {}", e))?;
    
    // Extract fields
    let version = json.get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    
    let description = json.get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    
    let homepage = json.get("homepage")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    
    let license = json.get("license")
        .and_then(|v| v.as_str())
        .or_else(|| {
            json.get("license")
                .and_then(|v| v.get("identifier"))
                .and_then(|v| v.as_str())
        })
        .unwrap_or("Unknown")
        .to_string();
    
    // Extract notes (can be string or array)
    let notes = match json.get("notes") {
        Some(serde_json::Value::String(s)) => vec![s.clone()],
        Some(serde_json::Value::Array(arr)) => {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect()
        }
        _ => vec![],
    };
    
    // Extract bin (can be string or array)
    let bin = match json.get("bin") {
        Some(serde_json::Value::String(s)) => vec![s.clone()],
        Some(serde_json::Value::Array(arr)) => {
            arr.iter()
                .filter_map(|v| {
                    if let Some(s) = v.as_str() {
                        Some(s.to_string())
                    } else if let Some(arr) = v.as_array() {
                        arr.first().and_then(|v| v.as_str()).map(|s| s.to_string())
                    } else {
                        None
                    }
                })
                .collect()
        }
        _ => vec![],
    };
    
    // Extract depends
    let depends = json.get("depends").and_then(|v| match v {
        serde_json::Value::String(s) => Some(vec![s.clone()]),
        serde_json::Value::Array(arr) => Some(
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect()
        ),
        _ => None,
    });
    
    // Extract suggest (keep as JSON value for flexibility)
    let suggest = json.get("suggest").cloned();
    
    Ok(AppDetail {
        name: app_name,
        version,
        description,
        homepage,
        license,
        bucket,
        notes,
        bin,
        depends,
        suggest,
    })
}

#[tauri::command]
fn search_local_packets(query: String) -> Result<Vec<SearchResult>, String> {
    use std::path::PathBuf;
    
    // Normalize query
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();
    
    // Get scoop buckets directory
    let buckets_dir = std::env::var_os("USERPROFILE")
        .map(|home| PathBuf::from(home).join("scoop").join("buckets"))
        .ok_or("Failed to get USERPROFILE")?;
    
    if !buckets_dir.exists() {
        return Ok(Vec::new());
    }
    
    // Walk through buckets
    if let Ok(bucket_entries) = std::fs::read_dir(&buckets_dir) {
        for bucket_entry in bucket_entries.flatten() {
            if bucket_entry.path().is_dir() {
                let bucket_name = bucket_entry.file_name().to_string_lossy().to_string();
                let bucket_dir = bucket_entry.path().join("bucket");
                
                if bucket_dir.exists() {
                    if let Ok(manifest_entries) = std::fs::read_dir(&bucket_dir) {
                        for manifest_entry in manifest_entries.flatten() {
                            // Check if filename matches query
                            let file_name = manifest_entry.file_name().to_string_lossy().to_string();
                            if let Some(stem) = file_name.strip_suffix(".json") {
                                if stem.to_lowercase().contains(&query_lower) {
                                    // Found a match! Parse basic info.
                                    let mut version = String::new();
                                    let mut description = String::new();
                                    
                                    // Optimistic: try to read file for details
                                    if let Ok(content) = std::fs::read_to_string(manifest_entry.path()) {
                                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                                            version = json.get("version").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                            description = json.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                        }
                                    }
                                    
                                    results.push(SearchResult {
                                        name: stem.to_string(),
                                        version,
                                        bucket: bucket_name.clone(),
                                        description,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Sort results by length of name match (exact matches first)
    results.sort_by(|a, b| {
        let a_starts = a.name.to_lowercase().starts_with(&query_lower);
        let b_starts = b.name.to_lowercase().starts_with(&query_lower);
        if a_starts && !b_starts { return std::cmp::Ordering::Less; }
        if !a_starts && b_starts { return std::cmp::Ordering::Greater; }
        a.name.len().cmp(&b.name.len())
    });

    // Cap results to avoid UI lag
    if results.len() > 50 {
        results.truncate(50);
    }
    
    Ok(results)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_installed_apps, 
            search_apps, 
            search_local_packets,
            update_app, 
            update_all_apps,
            update_scoop,
            get_app_sizes,
            check_updates_async,
            get_random_apps, 
            get_app_detail,
            get_buckets,
            add_bucket,
            remove_bucket,
            install::install_app,
            install::uninstall_app,
            install::check_dependencies,
            install::is_app_installed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
