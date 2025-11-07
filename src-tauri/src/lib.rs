use serde::{Deserialize, Serialize};
// src-tauri/src/lib.rs
use tauri::{AppHandle, Manager};

use std::process::Command;
use std::path::PathBuf;
use tauri::Emitter;
use rmp_serde::from_slice;
use std::thread;
use reqwest;
use tiny_http::{Server, Response, Request};
use futures_util::{StreamExt};


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Person {
    person_guid : String,
    first_name: Option<String>,
    last_name: Option<String>,
    referral_status_id:  Option<i32>,
    person_status_id:  Option<i32>,
    zone_name:  Option<String>,
    area_name:  Option<String>,
    find_id:  Option<i32>
}


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
    which::which("ollama").is_ok()
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

#[tauri::command]
fn open_in_chrome(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(&["/C", "start", "", &url])
        .spawn()
        .map_err(|e| e.to_string())?;

    #[cfg(target_os = "macos")]
    Command::new("open")
        .arg(&url)
        .spawn()
        .map_err(|e| e.to_string())?;

    #[cfg(target_os = "linux")]
    Command::new("xdg-open")
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

    for (i, (model_name, file_path)) in models.iter().enumerate() {
        let modelfile = app_handle
            .path()
            .resolve(file_path, tauri::path::BaseDirectory::Resource)
            .map_err(|_| format!("Modelfile {} not found in bundle", file_path))?;

        if !modelfile.exists() {
            return Err(format!("Modelfile not found at {}", modelfile.display()));
        }

        // Build command
        let mut cmd = tokio::process::Command::new(&ollama_path);
        cmd.arg("create").arg(model_name).arg("-f").arg(modelfile);

        // Attach stdout/stderr only for the first model
        if i == 0 {
            cmd.stdout(std::process::Stdio::piped())
               .stderr(std::process::Stdio::piped());
        }

        let mut child = cmd.spawn().map_err(|e| e.to_string())?;

        // Progress handling only for the first model
        if i == 0 {
            let mut tasks = vec![];

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
        } else {
            // Silent install for other models
            let status = child.wait().await.map_err(|e| e.to_string())?;
            if !status.success() {
                return Err(format!("Installation of {} failed", model_name));
            }
        }
    }

    // Let UI know everything finished
    let _ = app_handle.emit("install-complete", ());

    Ok(())
}

#[tauri::command]
async fn generate( app_handle: tauri::AppHandle, prompt: String, model: String, id: i32) -> Result<(), String> {
    println!("command recieved for generate");
    let handle = app_handle.clone();
    let body = serde_json::json!({
        "model": model,
        "prompt": prompt,
        "stream": true
    });

    let client = reqwest::Client::new();
    let res = client.post("http://localhost:11434/api/generate")
        .body(prompt)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| e.to_string())?;
        let text = String::from_utf8_lossy(&chunk);
        for line in text.lines() {
            if line.trim().is_empty() {
                continue;
            }

            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(done) = json.get("done").and_then(|d| d.as_bool()) {
                    if done {
                        // Stream finished
                        break;
                    }
                }

                if let Some(resp) = json.get("response").and_then(|r| r.as_str()) {
                    // Here you can emit the response to the frontend
                    handle.emit(&format!("chunk-{}", id), resp).map_err(|e| e.to_string())?;
                }
            }
        }
    }

     handle
        .emit(&format!("finish-{}", id), ())
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn download_extension() -> Result<String, String> {
    use std::fs;
    use std::fs::File;
    use reqwest::blocking::get;
    use zip::ZipArchive;

    // 1. Download the zipball
    let url = "https://github.com/ElderJackJones/CharlesExtension/zipball/main";
    let response = get(url).map_err(|e| e.to_string())?;
    let bytes = response.bytes().map_err(|e| e.to_string())?;

    // 2. Get user's Downloads directory
    let downloads_dir = dirs::download_dir().ok_or("Couldn't find Downloads folder")?;
    let zip_path = downloads_dir.join("CharlesExtension.zip");
    let extract_temp = downloads_dir.join("CharlesExtension_temp");
    let final_dir = downloads_dir.join("CharlesExtension");

    // Clean up any old files
    let _ = fs::remove_file(&zip_path);
    let _ = fs::remove_dir_all(&extract_temp);
    let _ = fs::remove_dir_all(&final_dir);

    // 3. Save zip file
    fs::write(&zip_path, &bytes).map_err(|e| e.to_string())?;

    // 4. Extract zip into temp directory
    let file = File::open(&zip_path).map_err(|e| e.to_string())?;
    let mut zip = ZipArchive::new(file).map_err(|e| e.to_string())?;
    fs::create_dir_all(&extract_temp).map_err(|e| e.to_string())?;
    zip.extract(&extract_temp).map_err(|e| e.to_string())?;

    // 5. Find the extracted folder (GitHub adds a single top-level folder)
    let inner_folder = fs::read_dir(&extract_temp)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .find(|entry| entry.path().is_dir())
        .map(|entry| entry.path())
        .ok_or("No folder found in ZIP")?;

    // 6. Move contents to final "CharlesExtension" folder
    fs::create_dir_all(&final_dir).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(&inner_folder).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let dest = final_dir.join(entry.file_name());
        if entry.path().is_dir() {
            fs::rename(entry.path(), &dest)
                .or_else(|_| fs_extra::dir::copy(&entry.path(), &final_dir, &fs_extra::dir::CopyOptions::new())
                .map(|_| ()))
                .map_err(|e| e.to_string())?;
        } else {
            fs::copy(entry.path(), &dest).map_err(|e| e.to_string())?;
        }
    }

    // 7. Cleanup
    let _ = fs::remove_file(&zip_path);
    let _ = fs::remove_dir_all(&extract_temp);

    Ok(final_dir.to_string_lossy().into_owned())
}

#[tauri::command]
fn start_server() -> Result<(), String> {
    thread::spawn(|| {
            let server = Server::http("127.0.0.1:51234").expect("Could not bind to port 51234");
            println!("tiny-http listening on http://127.0.0.1:51234");

            for request in server.incoming_requests() {
                handle_request(request);
            }
        });

    Ok(())
}

fn handle_request(mut request: Request) {
    if request.url() == "/receive" && request.method().as_str() == "POST" {
        let mut body = Vec::new();
        if let Err(e) = request.as_reader().read_to_end(&mut body) {
            eprintln!("Error reading body: {e}");
            let _ = request.respond(Response::from_string("Bad Request").with_status_code(400));
            return;
        }

        match from_slice::<Vec<Person>>(&body) {
            Ok(persons) => {
                println!("✅ Received {} persons!", persons.len());
                let _ = request.respond(Response::from_string("OK").with_status_code(200));
            }
            Err(e) => {
                eprintln!("❌ MsgPack decode error: {e}");
                let _ = request.respond(Response::from_string("Invalid MsgPack").with_status_code(400));
            }
        }
    } else {
        let _ = request.respond(Response::from_string("Not Found").with_status_code(404));
    }
}
// Entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            open_messenger_window,
            close_messenger_window,
            is_messenger_window_open,
            check_ollama_installed,
            open_in_chrome,
            install_models,
            generate,
            download_extension,
            start_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



// TODO: get all session data and replicate in the fetch request