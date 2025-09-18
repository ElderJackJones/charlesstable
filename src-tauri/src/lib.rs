// src-tauri/src/lib.rs
use tauri::{AppHandle, Manager};

use std::process::Command;
use std::path::PathBuf;
use tauri::Emitter;


fn find_ollama_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let home = std::env::var("USERPROFILE").ok()?;
        let candidates = [
            format!(r"{home}\AppData\Local\Programs\Ollama\ollama.exe"),
        ];
        for c in candidates {
            let p = PathBuf::from(c);
            if p.exists() {
                return Some(p);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let candidates = ["/usr/local/bin/ollama", "/opt/homebrew/bin/ollama"];
        for c in candidates {
            let p = PathBuf::from(c);
            if p.exists() {
                return Some(p);
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let candidates = ["/usr/bin/ollama", "/usr/local/bin/ollama"];
        for c in candidates {
            let p = PathBuf::from(c);
            if p.exists() {
                return Some(p);
            }
        }
    }

    None
}

#[tauri::command]
fn check_ollama_installed() -> bool {
    // First attempt: use PATH as-is
    if try_ollama_version("ollama") {
        return true;
    }

    // Otherwise, search common install paths
    if let Some(ollama_path) = find_ollama_path() {
        if let Some(parent) = ollama_path.parent() {
            let parent_str = parent.to_string_lossy().to_string();
            let mut new_path = std::env::var("PATH").unwrap_or_default();

            // Prepend ollama's directory to PATH
            new_path = format!("{}{}{}", parent_str, std::path::MAIN_SEPARATOR, new_path);
            std::env::set_var("PATH", new_path);
        }

        // Retry using the absolute path
        return try_ollama_version(ollama_path.to_string_lossy().as_ref());
    }

    false
}

fn try_ollama_version(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}


#[tauri::command]
async fn open_messenger_window(app: AppHandle) -> Result<(), String> {
    // Check if window already exists
    if let Some(window) = app.get_webview_window("messenger") {
        // If it exists, just focus it
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    // Create new messenger window using the correct v2 API
    let messenger_window = tauri::WebviewWindowBuilder::new(
        &app,
        "messenger",
        tauri::WebviewUrl::External("https://messenger.com".parse().unwrap())
    )
    .title("Facebook Messenger")
    .inner_size(1000.0, 700.0)
    .min_inner_size(400.0, 300.0)
    .resizable(true)
    .center()
    .build()
    .map_err(|e| e.to_string())?;

    // Optional: Add window event listeners
    let _window_clone = messenger_window.clone();
    messenger_window.on_window_event(move |event| {
        match event {
            tauri::WindowEvent::CloseRequested { .. } => {
                println!("Messenger window close requested");
            },
            tauri::WindowEvent::Focused(focused) => {
                if *focused {
                    println!("Messenger window focused");
                } else {
                    println!("Messenger window unfocused"); 
                }
            },
            _ => {}
        }
    });

    Ok(())
}

#[tauri::command]
async fn close_messenger_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("messenger") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn is_messenger_window_open(app: AppHandle) -> Result<bool, String> {
    Ok(app.get_webview_window("messenger").is_some())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            open_messenger_window,
            close_messenger_window,
            is_messenger_window_open,
           check_ollama_installed,
           open_in_chrome,
           install_models,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn open_in_chrome(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(&["/C", "start", "chrome", &url])
        .spawn()
        .map_err(|e| e.to_string())?;

    #[cfg(target_os = "macos")]
    Command::new("open")
        .arg("-a")
        .arg("Google Chrome")
        .arg(&url)
        .spawn()
        .map_err(|e| e.to_string())?;

    #[cfg(target_os = "linux")]
    Command::new("google-chrome")
        .arg(&url)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn install_models(app_handle: tauri::AppHandle) -> Result<(), String> {
    use tokio::io::{AsyncBufReadExt, BufReader};
    use regex::Regex;

    // 1. Find Ollama
    let ollama_path = find_ollama_path().ok_or("Ollama not found")?;

    // 2. Map model names to their Modelfiles
    let models = vec![
        ("charlesJest", "resources/jestFile"),
        ("charlesSage", "resources/sageFile"),
        ("charlesExpert", "resources/expertFile"),
    ];

    let re = Regex::new(r"(\d+)%").unwrap();
    let handle = app_handle.clone();

    for (model_name, file_path) in models {
        let modelfile = app_handle
            .path()
            .resolve(file_path, tauri::path::BaseDirectory::Resource)
            .map_err(|_| format!("Modelfile {} not found in bundle", file_path))?;

        if !modelfile.exists() {
            return Err(format!("Modelfile not found at {}", modelfile.display()));
        }

        let mut child = tokio::process::Command::new(&ollama_path)
            .arg("create")
            .arg(model_name)
            .arg("-f")
            .arg(modelfile)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

        let mut tasks = vec![];

        // Stream stdout
        if let Some(stdout) = child.stdout.take() {
            let handle = handle.clone();
            let re = re.clone();
            tasks.push(tauri::async_runtime::spawn(async move {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();
                let mut last_percent = 0;
                while let Ok(Some(line)) = lines.next_line().await {
                    if let Some(caps) = re.captures(&line) {
                        if let Ok(percent) = caps[1].parse::<u8>() {
                            if percent > last_percent {
                                last_percent = percent;
                                let _ = handle.emit("install-progress", percent);
                            }
                        }
                    }
                }
                if last_percent < 100 {
                    let _ = handle.emit("install-progress", 100);
                }
            }));
        }

        // Stream stderr
        if let Some(stderr) = child.stderr.take() {
            let handle = handle.clone();
            let re = re.clone();
            tasks.push(tauri::async_runtime::spawn(async move {
                let reader = BufReader::new(stderr);
                let mut lines = reader.lines();
                let mut last_percent = 0;
                while let Ok(Some(line)) = lines.next_line().await {
                    if let Some(caps) = re.captures(&line) {
                        if let Ok(percent) = caps[1].parse::<u8>() {
                            if percent > last_percent {
                                last_percent = percent;
                                let _ = handle.emit("install-progress", percent);
                            }
                        }
                    }
                }
                if last_percent < 100 {
                    let _ = handle.emit("install-progress", 100);
                }
            }));
        }

        let status = child.wait().await.map_err(|e| e.to_string())?;
        for t in tasks {
            let _ = t.await;
        }

        if !status.success() {
            return Err(format!("Installation of {} failed", model_name));
        }
    }

    Ok(())
}
