#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, WindowEvent,
};
use tauri_plugin_positioner::{on_tray_event, Position, WindowExt};

struct SpikeState {
    show_requested: Mutex<Option<Instant>>,
    startup_at: Instant,
}

fn log_line(msg: &str) {
    use std::io::Write;
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let line = format!("[{ms}] {msg}");
    let _ = writeln!(std::io::stdout(), "{line}");
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("spike2-log.txt")
    {
        let _ = writeln!(f, "{line}");
    }
}

fn create_panel(app: &AppHandle) -> tauri::Result<WebviewWindow> {
    let win = WebviewWindowBuilder::new(app, "panel", WebviewUrl::App("index.html".into()))
        .title("Spike2 Panel")
        .inner_size(360.0, 480.0)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .visible(false)
        .build()?;
    let w = win.clone();
    win.on_window_event(move |event| {
        if let WindowEvent::Focused(false) = event {
            let _ = w.hide();
            log_line("panel hidden (blur)");
        }
    });
    Ok(win)
}

fn show_panel(app: &AppHandle, win: &WebviewWindow) {
    {
        let st = app.state::<SpikeState>();
        *st.show_requested.lock().unwrap() = Some(Instant::now());
    }
    if let Err(e) = win.move_window(Position::TrayCenter) {
        log_line(&format!("move_window failed: {e}"));
    }
    let t = Instant::now();
    let _ = win.show();
    let _ = win.set_focus();
    log_line(&format!(
        "show()+set_focus() returned in {:.2} ms",
        t.elapsed().as_secs_f64() * 1000.0
    ));
    let _ = app.emit_to("panel", "spike:show", ());
}

fn toggle_panel(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("panel") {
        if win.is_visible().unwrap_or(false) {
            let _ = win.hide();
        } else {
            show_panel(app, &win);
        }
    } else {
        let t = Instant::now();
        match create_panel(app) {
            Ok(win) => {
                log_line(&format!(
                    "panel webview built (demand) in {:.1} ms",
                    t.elapsed().as_secs_f64() * 1000.0
                ));
                show_panel(app, &win);
            }
            Err(e) => log_line(&format!("panel build failed: {e}")),
        }
    }
}

#[tauri::command]
fn spike_painted(app: AppHandle, nav_ms: f64, phase: String) {
    let st = app.state::<SpikeState>();
    let since_show = st
        .show_requested
        .lock()
        .unwrap()
        .map(|t| t.elapsed().as_secs_f64() * 1000.0);
    let since_start = st.startup_at.elapsed().as_secs_f64() * 1000.0;
    match since_show {
        Some(ms) => log_line(&format!(
            "RESULT phase={phase} painted {ms:.1} ms after show request (js nav_ms={nav_ms:.1})"
        )),
        None => log_line(&format!(
            "paint with no pending show (initial) after startup {since_start:.1} ms (js nav_ms={nav_ms:.1})"
        )),
    }
}

fn main() {
    let mode = std::env::var("SPIKE_MODE").unwrap_or_else(|_| "prewarm".into());
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .manage(SpikeState {
            show_requested: Mutex::new(None),
            startup_at: Instant::now(),
        })
        .invoke_handler(tauri::generate_handler![spike_painted])
        .setup(move |app| {
            log_line(&format!("spike2 started, mode={mode}"));
            if mode == "prewarm" {
                match create_panel(app.handle()) {
                    Ok(_) => log_line("prewarm: panel webview built hidden"),
                    Err(e) => log_line(&format!("prewarm build failed: {e}")),
                }
            }
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit])?;
            TrayIconBuilder::with_id("main")
                .icon(Image::from_bytes(include_bytes!("../icons/tray.png"))?)
                .tooltip("SIM Spike-2 Tray")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    if event.id.as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    on_tray_event(tray.app_handle(), &event);
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        toggle_panel(tray.app_handle());
                    }
                })
                .build(app)?;
            log_line("tray icon built");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running spike2");
}
