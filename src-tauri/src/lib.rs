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
use std::collections::{HashMap};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering}
    }
};



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
    find_id:  Option<i32>,
    referral_assigned_date: Option<i64>
}

#[derive(Debug, Default, Serialize)]
struct Payload {
    map: HashMap<String, HashMap<String, Vec<String>>>,
}

impl Payload {
    fn insert(&mut self, key1: &str, key2: &str, value: &str) {
        self.map
            .entry(key1.to_string())       // auto-create top level
            .or_default()
            .entry(key2.to_string())       // auto-create sub level
            .or_default()
            .push(value.to_string());      // add value to existing vec
    }
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
async fn generate( app_handle: tauri::AppHandle, prompt: String, model: String) -> Result<(), String> {
    println!("command recieved for generate");
    let handle = app_handle.clone();
    let body = serde_json::json!({
        "model": model,
        "prompt": prompt,
        "stream": false,
        "keep_alive": "10m"
    });

    let client = reqwest::Client::new();
    let res = client.post("http://localhost:11434/api/generate")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let response_json : serde_json::Value = res
    .json()
    .await
    .map_err(|e| e.to_string())?;

    if let Some(response_text) = response_json.get("response").and_then(|r| r.as_str()) {
            handle.emit("response", response_text).map_err(|e| e.to_string())?;
    }    

    Ok(())
}

#[tauri::command]
fn start_server( app_handle: tauri::AppHandle) -> Result<(), String> {
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_flag = shutdown.clone();
    let handle = app_handle.clone();

    thread::spawn(move || {
            let server = Server::http("127.0.0.1:51234").expect("Could not bind to port 51234");
            println!("tiny-http listening on http://127.0.0.1:51234");

            for request in server.incoming_requests() {
                if shutdown_flag.load(Ordering::SeqCst) {
                    println!("shutting down server");
                    break
                }
                if handle_request(&handle, request, &shutdown_flag) {
                    println!("successful recieve! Shutting down server");
                    break
                }
            }

            println!("Cleaning threads...")
        });

    Ok(())
}

fn handle_request(app: &AppHandle, mut request: Request, shutdown_flag: &AtomicBool) -> bool {
    if request.url() == "/receive" && request.method().as_str() == "POST" {
        let mut body = Vec::new();
        if let Err(e) = request.as_reader().read_to_end(&mut body) {
            eprintln!("Error reading body: {e}");
            let _ = request.respond(Response::from_string("Bad Request").with_status_code(400));
            return false;
        }

        match from_slice::<Vec<Person>>(&body) {
            Ok(persons) => {
                let names = process_people(persons);
                println!("✅ Received {:?}", names);
                let _ = request.respond(Response::from_string("OK").with_status_code(200));
                shutdown_flag.store(true, Ordering::SeqCst);
                let _ = app.emit("payload", &names);

                return true;
            }
            Err(e) => {
                eprintln!("❌ MsgPack decode error: {e}");
                let _ = request.respond(Response::from_string("Invalid MsgPack").with_status_code(400));
                return false;
            }
        }
    } else {
        let _ = request.respond(Response::from_string("Not Found").with_status_code(404));
        false
    }
}

// Helper functions for processing
fn is_green_or_yellow(person: &Person) -> bool {
    matches!(person.person_status_id, Some(1 | 2 | 3 | 4))
}

fn unattempted(person: &Person) -> bool {
    matches!(person.referral_status_id, Some(10))
}

fn unattempted_or_unsuccessful(person: &Person) -> bool {
    matches!(person.referral_status_id, Some(20) | Some(10))
}

fn process_people(people: Vec<Person>) -> Payload {
    let mut data = Payload::default();
    
    for mut p in people.into_iter() {
        if should_include(&p) {
            let zone = p.zone_name.take().unwrap_or_default(); // String
            let area = p.area_name.take().unwrap_or_default(); // String
            let name = cleaned_name(&p);
            data.insert(&zone, &area, &name);

        }
    }

    data
}

fn should_include(person: &Person) -> bool {
    use std::time::{SystemTime, Duration, UNIX_EPOCH};

    let zone_ok = person.zone_name.as_ref().map_or(false, |z| !z.trim().is_empty());
    let area_ok = person.area_name.as_ref().map_or(false, |a| !a.trim().is_empty());

    if !(zone_ok && area_ok) {
        return false;
    }

    let green_or_yellow: bool = is_green_or_yellow(person);
    let is_unattempted_or_unsuccessful = unattempted_or_unsuccessful(person);
    let now = SystemTime::now();
    let two_weeks = Duration::from_secs(14 * 24 * 60 * 60);

    let less_than_two_weeks = if let Some(ts) = person.referral_assigned_date {
        let person_time = UNIX_EPOCH + Duration::from_millis(ts as u64);
        matches!(now.duration_since(person_time), Ok(diff) if diff <= two_weeks)
    } else {
        false
    };

    less_than_two_weeks && green_or_yellow && is_unattempted_or_unsuccessful
}

fn cleaned_name(person: &Person) -> String {
    let mut name = format!(
        "{}{}",
        person.first_name.as_deref().unwrap_or(""),
        person.last_name.as_deref().unwrap_or("")
    );
    if unattempted(person) {
        name.push_str("❗");
    }
    name
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
            start_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



// TODO: get all session data and replicate in the fetch request