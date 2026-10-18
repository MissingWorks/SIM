#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WebviewUrl, WebviewWindowBuilder,
};
use windows::core::w;
use windows::Win32::Foundation::{CloseHandle, HANDLE, HWND};
use windows::Win32::System::DataExchange::{
    CloseClipboard, EmptyClipboard, OpenClipboard, RegisterClipboardFormatW, SetClipboardData,
};
use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
use windows::Win32::System::Ole::CF_UNICODETEXT;
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYBDINPUT,
    KEYEVENTF_KEYUP, MOD_ALT, MOD_WIN, VIRTUAL_KEY, VK_CONTROL, VK_MENU, VK_V,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetClassNameW, GetForegroundWindow, GetMessageW, GetWindowThreadProcessId, SetForegroundWindow,
    MSG, WM_HOTKEY,
};
use tauri_plugin_positioner::{Position, WindowExt};

#[derive(Clone)]
struct Target {
    hwnd: isize,
    pid: u32,
    name: String,
    at_ms: u128,
}

#[derive(Default)]
struct SpikeState {
    last_external: Mutex<Option<Target>>,
}

static COPY_COUNTER: AtomicU64 = AtomicU64::new(0);

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

fn log_line(msg: &str) {
    use std::io::Write;
    let line = format!("[{}] {msg}", now_ms());
    let _ = writeln!(std::io::stdout(), "{line}");
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("spike7-log.txt")
    {
        let _ = writeln!(f, "{line}");
    }
}

unsafe fn process_name_for_hwnd(hwnd: HWND) -> (u32, String) {
    let mut pid: u32 = 0;
    let _ = GetWindowThreadProcessId(hwnd, Some(&mut pid));
    if pid == 0 {
        return (0, "?".into());
    }
    let mut name = String::from("?");
    if let Ok(h) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
        let mut buf = [0u16; 1024];
        let mut len = buf.len() as u32;
        if QueryFullProcessImageNameW(h, PROCESS_NAME_WIN32, windows::core::PWSTR(buf.as_mut_ptr()), &mut len)
            .is_ok()
        {
            let full = String::from_utf16_lossy(&buf[..len as usize]);
            name = full.rsplit('\\').next().unwrap_or(&full).to_string();
        }
        let _ = CloseHandle(h);
    }
    (pid, name)
}

fn tracker_loop(app: AppHandle) {
    let own_pid = std::process::id();
    loop {
        std::thread::sleep(Duration::from_millis(200));
        unsafe {
            let fg = GetForegroundWindow();
            if fg.0.is_null() {
                continue;
            }
            let mut cls = [0u16; 256];
            let n = GetClassNameW(fg, &mut cls) as usize;
            let class = String::from_utf16_lossy(&cls[..n]);
            const SHELL: [&str; 5] = [
                "Shell_TrayWnd",
                "Shell_SecondaryTrayWnd",
                "NotifyIconOverflowWindow",
                "TopLevelWindowForOverflowXamlIsland",
                "Windows.UI.Core.CoreWindow",
            ];
            if SHELL.contains(&class.as_str()) {
                continue;
            }
            let (pid, name) = process_name_for_hwnd(fg);
            if pid == own_pid || pid == 0 {
                continue;
            }
            let st = app.state::<SpikeState>();
            let mut g = st.last_external.lock().unwrap();
            *g = Some(Target {
                hwnd: fg.0 as isize,
                pid,
                name,
                at_ms: now_ms(),
            });
        }
    }
}

unsafe fn set_clipboard_text(text: &str) -> Result<(), String> {
    OpenClipboard(None).map_err(|e| e.to_string())?;
    let _ = EmptyClipboard();

    let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let bytes = wide.len() * 2;
    let h = GlobalAlloc(GMEM_MOVEABLE, bytes).map_err(|e| e.to_string())?;
    let p = GlobalLock(h) as *mut u16;
    std::ptr::copy_nonoverlapping(wide.as_ptr(), p, wide.len());
    let _ = GlobalUnlock(h);
    SetClipboardData(CF_UNICODETEXT.0 as u32, Some(HANDLE(h.0))).map_err(|e| e.to_string())?;

    let fmt = RegisterClipboardFormatW(w!("SIMSpike7Marker"));
    if fmt != 0 {
        let hm = GlobalAlloc(GMEM_MOVEABLE, 1).map_err(|e| e.to_string())?;
        let pm = GlobalLock(hm) as *mut u8;
        *pm = 1;
        let _ = GlobalUnlock(hm);
        SetClipboardData(fmt, Some(HANDLE(hm.0))).map_err(|e| e.to_string())?;
    }

    let _ = CloseClipboard();
    Ok(())
}

fn key_input(vk: VIRTUAL_KEY, up: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: if up { KEYEVENTF_KEYUP } else { KEYBD_EVENT_FLAGS(0) },
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

unsafe fn send_keys(keys: &[(VIRTUAL_KEY, bool)]) {
    let inputs: Vec<INPUT> = keys.iter().map(|(vk, up)| key_input(*vk, *up)).collect();
    let _ = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
}

fn show_panel(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("panel") {
        let _ = win.move_window(Position::Center);
        let _ = win.show();
        let _ = win.set_focus();
        log_line("panel shown");
    }
}

fn hotkey_thread(app: AppHandle) {
    unsafe {
        match RegisterHotKey(None, 1, MOD_WIN | MOD_ALT, 0x56) {
            Ok(_) => log_line("hotkey Win+Alt+V registered"),
            Err(e) => {
                log_line(&format!("hotkey Win+Alt+V FAILED: {e} (use tray icon)"));
                return;
            }
        }
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            if msg.message == WM_HOTKEY {
                log_line("hotkey Win+Alt+V fired");
                show_panel(&app);
            }
        }
    }
}

fn toggle_panel(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("panel") {
        if win.is_visible().unwrap_or(false) {
            let _ = win.hide();
            log_line("panel hidden by tray toggle");
        } else {
            show_panel(app);
        }
    }
}

#[tauri::command]
fn spike7_copy_only() -> String {
    let n = COPY_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    let text = format!("SPIKE7 #{n} — copy-only {}", now_ms());
    match unsafe { set_clipboard_text(&text) } {
        Ok(_) => {
            log_line(&format!("copy-only: clipboard set (marker) #{n}"));
            "Copied — press Ctrl+V in your app".into()
        }
        Err(e) => {
            log_line(&format!("copy-only failed: {e}"));
            format!("copy failed: {e}")
        }
    }
}

#[tauri::command]
fn spike7_paste(app: AppHandle, window: tauri::WebviewWindow, delay_ms: u64, use_alt: bool) -> String {
    let t0 = Instant::now();
    let mut r = String::new();
    let target = {
        let st = app.state::<SpikeState>();
        let t = st.last_external.lock().unwrap().clone();
        t
    };
    let Some(target) = target else {
        log_line("[paste] no target captured yet");
        return "No external target captured yet — focus the target app first, then open the panel from the tray.".into();
    };

    let n = COPY_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    let text = format!("SPIKE7 #{n} — line {} — خط تجربة لصق tashkeel", now_ms());
    if let Err(e) = unsafe { set_clipboard_text(&text) } {
        log_line(&format!("[paste] clipboard failed: {e}"));
        return format!("clipboard failed: {e}");
    }
    r.push_str(&format!("clipboard set (marker) at {:.2}ms\n", t0.elapsed().as_secs_f64() * 1000.0));
    let age = now_ms().saturating_sub(target.at_ms);
    r.push_str(&format!("target: {} (pid {}) captured {}ms ago\n", target.name, target.pid, age));

    let _ = window.hide();
    r.push_str(&format!("panel hidden at {:.1}ms; waiting {delay_ms}ms\n", t0.elapsed().as_secs_f64() * 1000.0));
    std::thread::sleep(Duration::from_millis(delay_ms));

    let hwnd = HWND(target.hwnd as *mut std::ffi::c_void);
    let direct_ok;
    let final_ok;
    unsafe {
        let _ = SetForegroundWindow(hwnd);
        std::thread::sleep(Duration::from_millis(15));
        direct_ok = GetForegroundWindow().0 == hwnd.0;
        r.push_str(&format!(
            "SetForegroundWindow direct -> fg_match={direct_ok} at {:.1}ms\n",
            t0.elapsed().as_secs_f64() * 1000.0
        ));
        let mut fg_ok = direct_ok;
        if !direct_ok && use_alt {
            send_keys(&[(VK_MENU, false), (VK_MENU, true)]);
            std::thread::sleep(Duration::from_millis(10));
            let _ = SetForegroundWindow(hwnd);
            std::thread::sleep(Duration::from_millis(15));
            fg_ok = GetForegroundWindow().0 == hwnd.0;
            r.push_str(&format!(
                "ALT trick -> fg_match={fg_ok} at {:.1}ms\n",
                t0.elapsed().as_secs_f64() * 1000.0
            ));
        }
        final_ok = fg_ok;
        let t_send = t0.elapsed().as_secs_f64() * 1000.0;
        send_keys(&[(VK_CONTROL, false), (VK_V, false), (VK_V, true), (VK_CONTROL, true)]);
        r.push_str(&format!("Ctrl+V sent at {t_send:.1}ms\n"));
        r.push_str(&format!("fg_ok={final_ok} direct={direct_ok} alt_used={}", use_alt && !direct_ok));
    }
    for line in r.lines() {
        log_line(&format!("[paste] {line}"));
    }
    log_line(&format!(
        "RESULT delay={delay_ms} alt={use_alt} direct={direct_ok} final_fg={final_ok}"
    ));
    r
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .manage(SpikeState::default())
        .invoke_handler(tauri::generate_handler![spike7_paste, spike7_copy_only])
        .setup(|app| {
            log_line("spike7 started");
            let _win = WebviewWindowBuilder::new(app, "panel", WebviewUrl::App("index.html".into()))
                .title("SPIKE-7 Paste Test")
                .inner_size(480.0, 430.0)
                .resizable(false)
                .visible(false)
                .build()?;
            show_panel(app.handle());
            log_line("panel built (visible)");

            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit])?;
            TrayIconBuilder::with_id("main")
                .icon(tauri::image::Image::from_bytes(include_bytes!("../icons/tray.png"))?)
                .tooltip("SIM Spike-7 Paste Test")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    if event.id.as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .on_tray_icon_event(|tray, event| {
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

            let handle = app.handle().clone();
            std::thread::spawn(move || tracker_loop(handle));
            let hk = app.handle().clone();
            std::thread::spawn(move || hotkey_thread(hk));
            log_line("tray built; target tracker running (200ms)");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running spike7");
}
