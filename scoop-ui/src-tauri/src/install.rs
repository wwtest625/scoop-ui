use serde::{Deserialize, Serialize};
use std::process::Command;
use std::os::windows::process::CommandExt;
use tauri::{AppHandle, Emitter};

#[derive(Clone, Serialize)]
pub struct InstallProgress {
    pub app_name: String,
    pub status: String,
    pub progress: f32,
    pub message: String,
}

#[derive(Clone, Serialize)]
pub struct DependencyInfo {
    pub app_name: String,
    pub dependencies: Vec<String>,
}

/// 检测应用的依赖项
#[tauri::command]
pub fn check_dependencies(app_name: String, bucket: Option<String>) -> Result<Vec<String>, String> {
    use std::path::PathBuf;
    
    // 构建 manifest 文件路径
    let manifest_path = if let Some(bucket_name) = bucket {
        std::env::var_os("USERPROFILE")
            .map(|home| {
                PathBuf::from(home)
                    .join("scoop")
                    .join("buckets")
                    .join(&bucket_name)
                    .join("bucket")
                    .join(format!("{}.json", app_name))
            })
            .ok_or("Failed to get USERPROFILE")?
    } else {
        // 如果没有指定 bucket,尝试在所有 bucket 中查找
        let buckets_dir = std::env::var_os("USERPROFILE")
            .map(|home| PathBuf::from(home).join("scoop").join("buckets"))
            .ok_or("Failed to get USERPROFILE")?;
        
        let mut found_path = None;
        if let Ok(entries) = std::fs::read_dir(&buckets_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let test_path = entry.path()
                        .join("bucket")
                        .join(format!("{}.json", app_name));
                    if test_path.exists() {
                        found_path = Some(test_path);
                        break;
                    }
                }
            }
        }
        
        found_path.ok_or(format!("Manifest not found for {}", app_name))?
    };
    
    if !manifest_path.exists() {
        return Ok(Vec::new());
    }
    
    // 读取并解析 manifest
    let content = std::fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read manifest: {}", e))?;
    
    let json: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse manifest: {}", e))?;
    
    // 提取依赖
    let dependencies = match json.get("depends") {
        Some(serde_json::Value::String(s)) => vec![s.clone()],
        Some(serde_json::Value::Array(arr)) => {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect()
        }
        _ => Vec::new(),
    };
    
    Ok(dependencies)
}

/// 安装应用
#[tauri::command]
pub fn install_app(
    app_handle: AppHandle,
    app_name: String,
) -> Result<String, String> {
    let app_handle_clone = app_handle.clone();
    let app_name_clone = app_name.clone();
    
    // 发送开始安装事件
    let _ = app_handle.emit("install-progress", InstallProgress {
        app_name: app_name.clone(),
        status: "starting".to_string(),
        progress: 0.0,
        message: format!("开始安装 {}...", app_name),
    });
    
    // 使用线程在后台执行安装
    std::thread::spawn(move || {
        // 模拟进度更新 (5秒内从0到100)
        let total_steps = 50;
        let step_duration = std::time::Duration::from_millis(100); // 100ms per step
        
        for i in 1..=total_steps {
            std::thread::sleep(step_duration);
            let progress = i as f32 / total_steps as f32;
            let message = if progress < 0.3 {
                format!("正在下载 {}...", app_name_clone)
            } else if progress < 0.6 {
                format!("正在解压 {}...", app_name_clone)
            } else if progress < 0.9 {
                format!("正在配置 {}...", app_name_clone)
            } else {
                format!("即将完成 {}...", app_name_clone)
            };
            
            let _ = app_handle_clone.emit("install-progress", InstallProgress {
                app_name: app_name_clone.clone(),
                status: "installing".to_string(),
                progress,
                message,
            });
        }
        
        // 执行实际的安装命令
        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!("scoop install {}", app_name_clone),
            ])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .output();
        
        match output {
            Ok(output) => {
                if !output.status.success() {
                    let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
                    let _ = app_handle_clone.emit("install-progress", InstallProgress {
                        app_name: app_name_clone.clone(),
                        status: "error".to_string(),
                        progress: 0.0,
                        message: error_msg,
                    });
                } else {
                    // 发送完成事件
                    let _ = app_handle_clone.emit("install-progress", InstallProgress {
                        app_name: app_name_clone.clone(),
                        status: "completed".to_string(),
                        progress: 1.0,
                        message: format!("{} 安装成功!", app_name_clone),
                    });
                }
            }
            Err(e) => {
                let _ = app_handle_clone.emit("install-progress", InstallProgress {
                    app_name: app_name_clone.clone(),
                    status: "error".to_string(),
                    progress: 0.0,
                    message: e.to_string(),
                });
            }
        }
    });
    
    Ok("安装已开始".to_string())
}

/// 卸载应用
#[tauri::command]
pub fn uninstall_app(
    app_handle: AppHandle,
    app_name: String,
) -> Result<String, String> {
    let app_handle_clone = app_handle.clone();
    let app_name_clone = app_name.clone();
    
    // 发送开始卸载事件
    let _ = app_handle.emit("uninstall-progress", InstallProgress {
        app_name: app_name.clone(),
        status: "starting".to_string(),
        progress: 0.0,
        message: format!("开始卸载 {}...", app_name),
    });
    
    std::thread::spawn(move || {
        // 模拟进度更新 (5秒内从0到100)
        let total_steps = 50;
        let step_duration = std::time::Duration::from_millis(100);
        
        for i in 1..=total_steps {
            std::thread::sleep(step_duration);
            let progress = i as f32 / total_steps as f32;
            let message = if progress < 0.5 {
                format!("正在清理 {} 的文件...", app_name_clone)
            } else {
                format!("正在移除 {} 的配置...", app_name_clone)
            };
            
            let _ = app_handle_clone.emit("uninstall-progress", InstallProgress {
                app_name: app_name_clone.clone(),
                status: "uninstalling".to_string(),
                progress,
                message,
            });
        }
        
        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!("scoop uninstall {}", app_name_clone),
            ])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .output();
        
        match output {
            Ok(output) => {
                if !output.status.success() {
                    let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
                    let _ = app_handle_clone.emit("uninstall-progress", InstallProgress {
                        app_name: app_name_clone.clone(),
                        status: "error".to_string(),
                        progress: 0.0,
                        message: error_msg,
                    });
                } else {
                    // 发送完成事件
                    let _ = app_handle_clone.emit("uninstall-progress", InstallProgress {
                        app_name: app_name_clone.clone(),
                        status: "completed".to_string(),
                        progress: 1.0,
                        message: format!("{} 卸载成功!", app_name_clone),
                    });
                }
            }
            Err(e) => {
                let _ = app_handle_clone.emit("uninstall-progress", InstallProgress {
                    app_name: app_name_clone.clone(),
                    status: "error".to_string(),
                    progress: 0.0,
                    message: e.to_string(),
                });
            }
        }
    });
    
    Ok("卸载已开始".to_string())
}

/// 检查应用是否已安装
#[tauri::command]
pub fn is_app_installed(app_name: String) -> Result<bool, String> {
    use std::path::PathBuf;
    
    let app_path = std::env::var_os("USERPROFILE")
        .map(|home| {
            PathBuf::from(home)
                .join("scoop")
                .join("apps")
                .join(&app_name)
                .join("current")
        })
        .ok_or("Failed to get USERPROFILE")?;
    
    Ok(app_path.exists())
}
