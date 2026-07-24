use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use tauri::{
    AppHandle, Manager,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{TrayIconBuilder, TrayIconEvent},
    webview::DownloadEvent,
    WebviewWindowBuilder,
};

#[derive(Default, Clone, Serialize)]
struct DownloadRecord {
    file_name: String,
    path: String,
    url: String,
    success: bool,
}

#[derive(Default, Clone)]
struct DownloadHistory(Arc<Mutex<Vec<DownloadRecord>>>);

impl DownloadHistory {
    fn push(&self, record: DownloadRecord) {
        let mut history = self.0.lock().unwrap();
        history.push(record);
        if history.len() > 100 {
            let remove = history.len() - 100;
            history.drain(..remove);
        }
    }

    fn snapshot(&self) -> Vec<DownloadRecord> {
        self.0.lock().unwrap().clone()
    }
}

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        if window.is_minimized().unwrap_or(false) {
            let _ = window.unminimize();
        }
        let _ = window.set_focus();
    }
}

fn sanitize_filename(input: &str) -> String {
    let fallback = "download";
    let name = input.rsplit('/').find(|part| !part.is_empty()).unwrap_or(fallback);
    let mut cleaned: String = name.chars().filter_map(|c| match c {
        '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => None,
        _ => Some(c),
    }).collect();

    if cleaned.trim().is_empty() {
        cleaned = fallback.to_string();
    }
    if cleaned.len() > 120 {
        cleaned.truncate(120);
    }
    if cleaned.ends_with('.') {
        cleaned.pop();
    }
    cleaned
}

fn unique_download_path(dir: &Path, file_name: &str) -> PathBuf {
    let path = dir.join(file_name);
    if !path.exists() {
        return path;
    }

    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or_default();
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("download");
    let mut counter = 1;

    loop {
        let candidate = if extension.is_empty() {
            dir.join(format!("{} ({counter})", stem))
        } else {
            dir.join(format!("{} ({counter}).{}", stem, extension))
        };
        if !candidate.exists() {
            return candidate;
        }
        counter += 1;
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_download_history(history: tauri::State<'_, DownloadHistory>) -> Vec<DownloadRecord> {
    history.snapshot()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let download_history = DownloadHistory::default();

    tauri::Builder::default()
        .manage(download_history.clone())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_main_window(app);
        }))
        .setup(move |app| {
            let icon = app.default_window_icon().cloned().unwrap();
            let app_handle = app.handle().clone();
            let menu_app_handle = app_handle.clone();
            let tray_app_handle = app_handle.clone();
            let history_for_downloads = download_history.clone();

            let whatsapp_url = tauri::utils::config::WebviewUrl::External("https://web.whatsapp.com".parse().unwrap());
            WebviewWindowBuilder::new(app, "main", whatsapp_url)
                .title("Whatsapp")
                .inner_size(1280.0, 800.0)
                .min_inner_size(900.0, 600.0)
                .resizable(true)
                .fullscreen(false)
                .maximized(false)
                .decorations(true)
                .visible(true)
                .center()
                .devtools(false)
                .disable_drag_drop_handler()
                .shadow(true)
                .transparent(false)
                .on_download(move |_webview, event| {
                    match event {
                        DownloadEvent::Finished { url, path, success } => {
                            let downloaded_path = path.clone().unwrap_or_default();
                            let record = DownloadRecord {
                                file_name: downloaded_path.file_name().and_then(|name| name.to_str()).unwrap_or("download").to_string(),
                                path: downloaded_path.to_string_lossy().into_owned(),
                                url: url.to_string(),
                                success,
                            };
                            history_for_downloads.push(record);
                        }
                        _ => {}
                    }
                    true
                })
                .build()?;

            let open_item = MenuItemBuilder::with_id("open", "Open").build(app)?;
            let exit_item = MenuItemBuilder::with_id("exit", "Exit").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&open_item)
                .item(&exit_item)
                .build()?;

            TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .on_menu_event(move |_tray, event| {
                    match event.id.as_ref() {
                        "open" => show_main_window(&menu_app_handle),
                        "exit" => menu_app_handle.exit(0),
                        _ => {}
                    }
                })
                .on_tray_icon_event(move |_tray, event| {
                    if let TrayIconEvent::DoubleClick { .. } = event {
                        show_main_window(&tray_app_handle);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![greet, get_download_history])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
