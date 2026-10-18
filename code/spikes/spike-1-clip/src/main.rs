use std::io::Write;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use windows::core::w;
use windows::Win32::Foundation::{HANDLE, HWND};
use windows::Win32::System::DataExchange::{
    CloseClipboard, EmptyClipboard, EnumClipboardFormats, GetClipboardFormatNameW,
    GetClipboardOwner, GetClipboardSequenceNumber, OpenClipboard, RegisterClipboardFormatW,
    SetClipboardData,
};
use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
use windows::Win32::System::Ole::CF_UNICODETEXT;
use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;

const MARKER: &str = "SIMSpikeMarker";

fn log(msg: &str) {
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let line = format!("[{ms}] {msg}");
    let _ = writeln!(std::io::stdout(), "{line}");
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("spike1-report.txt")
    {
        let _ = writeln!(f, "{line}");
    }
}

fn fmt_name(id: u32) -> String {
    let mut buf = [0u16; 256];
    let n = unsafe { GetClipboardFormatNameW(id, &mut buf) };
    if n > 0 {
        String::from_utf16_lossy(&buf[..n as usize])
    } else {
        match id {
            1 => "CF_TEXT".into(),
            2 => "CF_BITMAP".into(),
            3 => "CF_METAFILEPICT".into(),
            7 => "CF_OEMTEXT".into(),
            8 => "CF_DIB".into(),
            13 => "CF_UNICODETEXT".into(),
            14 => "CF_ENHMETAFILE".into(),
            15 => "CF_HDROP".into(),
            16 => "CF_LOCALE".into(),
            17 => "CF_DIBV5".into(),
            _ => format!("CF_#{id}"),
        }
    }
}

fn enum_formats() -> Result<Vec<(u32, String)>, String> {
    unsafe {
        OpenClipboard(None).map_err(|e| format!("OpenClipboard: {e}"))?;
        let mut out = Vec::new();
        let mut f = EnumClipboardFormats(0);
        while f != 0 {
            out.push((f, fmt_name(f)));
            f = EnumClipboardFormats(f);
        }
        let _ = CloseClipboard();
        Ok(out)
    }
}

fn owner_info() -> (Option<usize>, Option<u32>) {
    unsafe {
        let h: HWND = match GetClipboardOwner() {
            Ok(h) => h,
            Err(_) => return (None, None),
        };
        if h.0.is_null() {
            return (None, None);
        }
        let mut pid = 0u32;
        let _ = GetWindowThreadProcessId(h, Some(&mut pid));
        (Some(h.0 as usize), Some(pid))
    }
}

fn count_arabic(s: &str) -> usize {
    s.chars()
        .filter(|c| {
            let u = *c as u32;
            (0x0600..=0x06FF).contains(&u)
                || (0x0750..=0x077F).contains(&u)
                || (0xFB50..=0xFDFF).contains(&u)
                || (0xFE70..=0xFEFF).contains(&u)
        })
        .count()
}

fn count_emoji(s: &str) -> usize {
    s.chars()
        .filter(|c| {
            let u = *c as u32;
            (0x1F000..=0x1FAFF).contains(&u) || (0x2600..=0x27BF).contains(&u)
        })
        .count()
}

fn preview(s: &str) -> String {
    let p: String = s.chars().take(60).collect();
    let mut out = String::new();
    for c in p.chars() {
        match c {
            '\r' => out.push_str("\\r"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    if s.chars().count() > 60 {
        out.push('…');
    }
    out
}

fn report_event(seq: u32) {
    log(&format!("=== EVENT seq={seq} ==="));
    let (owner_hwnd, owner_pid) = owner_info();
    let us = std::process::id();
    log(&format!(
        "owner: hwnd={owner_hwnd:?} pid={owner_pid:?} is_us={}",
        owner_pid == Some(us)
    ));

    match enum_formats() {
        Ok(formats) => {
            let names: Vec<String> = formats.iter().map(|(id, n)| format!("{n}({id})")).collect();
            log(&format!("formats({}): {}", names.len(), names.join(", ")));
            log(&format!(
                "marker_present={}",
                names.iter().any(|n| n.starts_with(MARKER))
            ));
        }
        Err(e) => log(&format!("formats: ERROR {e}")),
    }

    let t0 = Instant::now();
    match clipboard_win::get_clipboard::<String, _>(clipboard_win::formats::Unicode) {
        Ok(s) => log(&format!(
            "win text: ok {:.2}ms len={} arabic={} emoji={} preview={:?}",
            t0.elapsed().as_secs_f64() * 1000.0,
            s.chars().count(),
            count_arabic(&s),
            count_emoji(&s),
            preview(&s)
        )),
        Err(e) => log(&format!(
            "win text: err {:.2}ms {e}",
            t0.elapsed().as_secs_f64() * 1000.0
        )),
    }

    let t0 = Instant::now();
    match clipboard_win::get_clipboard::<Vec<u8>, _>(clipboard_win::formats::RawData(8)) {
        Ok(b) => log(&format!(
            "win dib: ok {:.2}ms bytes={}",
            t0.elapsed().as_secs_f64() * 1000.0,
            b.len()
        )),
        Err(e) => log(&format!(
            "win dib: err {:.2}ms {e}",
            t0.elapsed().as_secs_f64() * 1000.0
        )),
    }

    let t0 = Instant::now();
    match clipboard_win::get_clipboard::<Vec<String>, _>(clipboard_win::formats::FileList) {
        Ok(f) => log(&format!(
            "win files: ok {:.2}ms count={} {:?}",
            t0.elapsed().as_secs_f64() * 1000.0,
            f.len(),
            f
        )),
        Err(e) => log(&format!(
            "win files: err {:.2}ms {e}",
            t0.elapsed().as_secs_f64() * 1000.0
        )),
    }

    match arboard::Clipboard::new() {
        Ok(mut cb) => {
            let t0 = Instant::now();
            match cb.get_text() {
                Ok(s) => log(&format!(
                    "ar text: ok {:.2}ms len={} arabic={} emoji={}",
                    t0.elapsed().as_secs_f64() * 1000.0,
                    s.chars().count(),
                    count_arabic(&s),
                    count_emoji(&s)
                )),
                Err(e) => log(&format!(
                    "ar text: err {:.2}ms {e}",
                    t0.elapsed().as_secs_f64() * 1000.0
                )),
            }
            let t0 = Instant::now();
            match cb.get_image() {
                Ok(img) => log(&format!(
                    "ar image: ok {:.2}ms {}x{} bytes={}",
                    t0.elapsed().as_secs_f64() * 1000.0,
                    img.width,
                    img.height,
                    img.bytes.len()
                )),
                Err(e) => log(&format!(
                    "ar image: err {:.2}ms {e}",
                    t0.elapsed().as_secs_f64() * 1000.0
                )),
            }
        }
        Err(e) => log(&format!("ar init err: {e}")),
    }
    log("=== END EVENT ===");
}

fn watch_loop() {
    log(&format!("watch started (pid={}) polling every 50ms", std::process::id()));
    let mut last_seq = unsafe { GetClipboardSequenceNumber() };
    log(&format!("baseline seq={last_seq}"));
    loop {
        std::thread::sleep(Duration::from_millis(50));
        let seq = unsafe { GetClipboardSequenceNumber() };
        if seq != last_seq {
            last_seq = seq;
            report_event(seq);
        }
    }
}

fn copy_plain(text: &str, label: &str) {
    match clipboard_win::set_clipboard(clipboard_win::formats::Unicode, text) {
        Ok(_) => log(&format!(
            "copied ({label}): len={} preview={:?}",
            text.chars().count(),
            preview(text)
        )),
        Err(e) => log(&format!("copy failed ({label}): {e}")),
    }
}

unsafe fn copy_marked(text: &str) -> Result<(), String> {
    OpenClipboard(None).map_err(|e| e.to_string())?;
    let _ = EmptyClipboard();

    let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let bytes = wide.len() * 2;
    let h = GlobalAlloc(GMEM_MOVEABLE, bytes).map_err(|e| e.to_string())?;
    let p = GlobalLock(h) as *mut u16;
    std::ptr::copy_nonoverlapping(wide.as_ptr(), p, wide.len());
    let _ = GlobalUnlock(h);
    SetClipboardData(CF_UNICODETEXT.0 as u32, Some(HANDLE(h.0))).map_err(|e| e.to_string())?;

    let fmt = RegisterClipboardFormatW(w!("SIMSpikeMarker"));
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("watch") => watch_loop(),
        Some("copy") => {
            let case = args.get(2).map(|s| s.as_str()).unwrap_or("text-en");
            match case {
                "text-en" => copy_plain("Hello World 12345 — English baseline", "text-en"),
                "text-ar" => copy_plain("مرحبا بالعالم — اختبار عربي ١٢٣", "text-ar"),
                "text-emoji" => copy_plain("test 🚀📋✅😀 end", "text-emoji"),
                "marked" => match unsafe { copy_marked("marked copy from spike (self-detection test)") } {
                    Ok(_) => log("copied (marked): text + SIMSpikeMarker format"),
                    Err(e) => log(&format!("copy failed (marked): {e}")),
                },
                other => log(&format!("unknown copy case: {other}")),
            }
        }
        _ => {
            println!("usage: spike1-clip watch | copy <text-en|text-ar|text-emoji|marked>");
        }
    }
}
