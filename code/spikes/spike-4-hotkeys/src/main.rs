use std::io::Write;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, MOD_ALT, MOD_CONTROL, MOD_SHIFT, MOD_WIN,
};
use windows::Win32::UI::WindowsAndMessaging::{PeekMessageW, MSG, PM_REMOVE, WM_HOTKEY};

const KEYS: [(&str, u32); 10] = [
    ("Win+Alt+V", 0x56),
    ("Win+Alt+1", 0x31),
    ("Win+Alt+2", 0x32),
    ("Win+Alt+3", 0x33),
    ("Win+Alt+4", 0x34),
    ("Win+Alt+5", 0x35),
    ("Win+Alt+6", 0x36),
    ("Win+Alt+7", 0x37),
    ("Win+Alt+8", 0x38),
    ("Win+Alt+9", 0x39),
];

struct Log {
    f: std::fs::File,
}

impl Log {
    fn new() -> Self {
        Log {
            f: std::fs::File::create("spike4-log.txt").expect("log"),
        }
    }
    fn line(&mut self, s: &str) {
        let ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        println!("[{ms}] {s}");
        let _ = writeln!(self.f, "[{ms}] {s}");
        let _ = self.f.flush();
    }
}

unsafe fn register_all(log: &mut Log, tag: &str) {
    for (i, (name, vk)) in KEYS.iter().enumerate() {
        let id = i as i32 + 1;
        match RegisterHotKey(None, id, MOD_WIN | MOD_ALT, *vk) {
            Ok(_) => log.line(&format!("[{tag}] REGISTER {name}: ok (id={id})")),
            Err(e) => log.line(&format!(
                "[{tag}] REGISTER {name}: FAILED code=0x{:x} {}",
                e.code().0,
                e.message()
            )),
        }
    }
}

unsafe fn unregister_all(log: &mut Log, tag: &str) {
    for (i, (name, _)) in KEYS.iter().enumerate() {
        let id = i as i32 + 1;
        let r = UnregisterHotKey(None, id);
        match r {
            Ok(_) => log.line(&format!("[{tag}] UNREGISTER {name}: ok")),
            Err(e) => log.line(&format!("[{tag}] UNREGISTER {name}: err {e}")),
        }
    }
}

unsafe fn wait_presses(log: &mut Log, tag: &str, seconds: u64) -> usize {
    let deadline = Instant::now() + Duration::from_secs(seconds);
    let start = Instant::now();
    let mut fired = [false; 10];
    let mut count = 0usize;
    let mut msg = MSG::default();
    while Instant::now() < deadline && count < KEYS.len() {
        while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
            if msg.message == WM_HOTKEY {
                let id = msg.wParam.0 as usize;
                if (1..=10).contains(&id) && !fired[id - 1] {
                    fired[id - 1] = true;
                    count += 1;
                    log.line(&format!(
                        "[{tag}] FIRED {} (id={id}) at {:.1}s",
                        KEYS[id - 1].0,
                        start.elapsed().as_secs_f64()
                    ));
                }
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    for (i, fired_one) in fired.iter().enumerate() {
        if !fired_one {
            log.line(&format!("[{tag}] NOT PRESSED: {}", KEYS[i].0));
        }
    }
    count
}

fn probe_alternatives() {
    let mut f = std::fs::File::create("spike4-probe.txt").expect("probe log");
    let mut write = |line: &str| {
        println!("{line}");
        let _ = writeln!(f, "{line}");
    };
    write("=== Spike-4 probe: alternative combos (registration only) ===");
    let variants: [(&str, windows::Win32::UI::Input::KeyboardAndMouse::HOT_KEY_MODIFIERS); 6] = [
        ("Ctrl+Alt", MOD_CONTROL | MOD_ALT),
        ("Win+Shift", MOD_WIN | MOD_SHIFT),
        ("Ctrl+Shift", MOD_CONTROL | MOD_SHIFT),
        ("Win+Alt+Shift", MOD_WIN | MOD_ALT | MOD_SHIFT),
        ("Ctrl+Win", MOD_CONTROL | MOD_WIN),
        ("Win+Ctrl+Alt", MOD_CONTROL | MOD_ALT | MOD_WIN),
    ];
    let mut id = 100i32;
    unsafe {
        for (name, mods) in variants {
            for digit in 1u32..=9 {
                let vk = 0x30 + digit;
                match RegisterHotKey(None, id, mods, vk) {
                    Ok(_) => {
                        write(&format!("{name}+{digit}: ok"));
                        let _ = UnregisterHotKey(None, id);
                    }
                    Err(e) => write(&format!("{name}+{digit}: FAILED 0x{:x}", e.code().0)),
                }
                id += 1;
            }
        }
        for (name, mods, vk) in [
            ("Ctrl+Alt+V", MOD_CONTROL | MOD_ALT, 0x56u32),
            ("Win+Shift+V", MOD_WIN | MOD_SHIFT, 0x56),
        ] {
            match RegisterHotKey(None, id, mods, vk) {
                Ok(_) => {
                    write(&format!("{name}: ok"));
                    let _ = UnregisterHotKey(None, id);
                }
                Err(e) => write(&format!("{name}: FAILED 0x{:x}", e.code().0)),
            }
            id += 1;
        }
    }
    write("probe done.");
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("probe") {
        probe_alternatives();
        return;
    }
    let mut log = Log::new();
    log.line("=== Spike-4: global hotkeys (SIM phase 0) ===");
    log.line("press Win+Alt+V then Win+Alt+1..9 — phase 1 waits 45s, phase 2 waits 20s after re-register");

    unsafe {
        register_all(&mut log, "phase1");
        log.line("phase1: waiting for key presses (45s max)...");
        let n1 = wait_presses(&mut log, "phase1", 45);
        log.line(&format!("phase1: {n1}/10 hotkeys fired"));
        log.line("");

        log.line("re-register cycle (settings change simulation)...");
        unregister_all(&mut log, "cycle");
        register_all(&mut log, "phase2");

        log.line("phase2: waiting for key presses (20s max) — press Win+Alt+V again...");
        let n2 = wait_presses(&mut log, "phase2", 20);
        log.line(&format!("phase2: {n2}/10 hotkeys fired"));

        unregister_all(&mut log, "cleanup");
    }

    log.line("done.");
    std::thread::sleep(Duration::from_secs(6));
}
