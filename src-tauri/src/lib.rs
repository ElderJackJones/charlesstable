// src-tauri/src/lib.rs
use tauri::{AppHandle, Manager};

use std::process::Command;
use std::path::PathBuf;
use tauri::Emitter;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, COOKIE, HeaderName};
use reqwest;
use futures_util::StreamExt;


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
async fn get_people( app_handle: tauri::AppHandle, userobj: String) -> Result<(), String> {
    // Take in the user obj and use it to fetch a bunch of people info. Process it then feed it back up to the frontend.
    use serde::{Serialize, Deserialize};
    use base64::{Engine as _, engine::general_purpose};

    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct User {
        org_name: String,
        first_name: String,
        last_name: String,
        missionary_title: String,
        person_guid: String,
        token: String,
        cookies: String 
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Payload {
        lds_account_id: String,
        cmis_id: u64,
        mission_id: i32,
        missionary_id: u64,
        org_id: u64,

        #[serde(default)]
        full_token: String
    }

    let current_user: User = serde_json::from_str(&userobj).unwrap();
    
    let token_parts: Vec<&str> = current_user.token.split('.').collect();
    let token_bytes = general_purpose::URL_SAFE_NO_PAD.decode(token_parts[1]).unwrap();
    let token_parsed: Payload = serde_json::from_slice(&token_bytes).unwrap();
    let token_final = Payload {
        full_token : current_user.token.clone(),
        ..token_parsed
    };
    println!("{:?}", &token_final);
    // Reqwest full person list and then process it

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token_final.full_token)).map_err(|e| e.to_string())?);
    headers.insert(COOKIE, HeaderValue::from_str(&current_user.cookies).map_err(|e| e.to_string())?);
    headers.insert(HeaderName::from_static("origin"), HeaderValue::from_static("https://referralmanager.churchofjesuschrist.org"));
    headers.insert(HeaderName::from_static("referer"), HeaderValue::from_static("https://referralmanager.churchofjesuschrist.org/"));
    headers.insert(HeaderName::from_static("accept"), HeaderValue::from_static("application/json, text/plain, */*"));
    headers.insert(HeaderName::from_static("accept-language"), HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(HeaderName::from_static("connection"), HeaderValue::from_static("keep-alive"));
    headers.insert(HeaderName::from_static("user-agent"), HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64)"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| e.to_string())?;

    let url = &format!("https://referralmanager.churchofjesuschrist.org/services/people/mission/{}?includeDroppedPersons=true",
        token_final.mission_id);
    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    app_handle.emit("people_list", text)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn start_bridge(app_handle: tauri::AppHandle) -> Result<u16, String> {
    use warp::Filter;
    use warp::http::Method;
    use std::net::TcpListener;

    let health = warp::get()
        .and(warp::path("health"))
        .map(|| warp::reply::json(&serde_json::json!({ "status": "ok" })));

    let app_handle_clone = app_handle.clone();
    let auth = warp::post()
    .and(warp::path("auth"))
    .and(warp::body::bytes())
    .map(move |body: bytes::Bytes| {
        let data: String = String::from_utf8(body.to_vec()).unwrap_or_default(); // owned String
        println!("📩 Received from extension: {}", data);
        let _ = app_handle_clone.emit("user_auth", data);
        warp::reply::with_status("OK".to_string(), warp::http::StatusCode::OK) // owned String
    });

    
   let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET, Method::POST])
        .allow_headers(vec![
            "Content-Type",
            "Accept",
            "Origin",
            "Sec-Fetch-Mode",
            "Sec-Fetch-Site",
        ])
        .build();

    let routes = health.or(auth)
    .with(cors).boxed();

    // Try ports 53102–53105
    for port in 53102..=53105 {
        if TcpListener::bind(("127.0.0.1", port)).is_ok() {
            println!("✅ Bridge active on port {}", port);

            let routes = routes.clone(); // warp filters are cheap to clone
            tauri::async_runtime::spawn(async move {
                warp::serve(routes)
                    .run(([127, 0, 0, 1], port))
                    .await;
            });

            return Ok(port);
        }
    }

    Err("No free port found in range 53102–53105".into())
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
            get_people,
            start_bridge
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// TODO: get all session data and replicate in the fetch request