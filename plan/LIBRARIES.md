# المكتبات (Libraries)

> **قاعدة إلزامية (من المستخدم):** لا نكتب من الصفر شيئًا له بديل جاهز مجرّب.
> قبل كتابة أي ميزة: ابحث هنا أولًا، إن لم تجدها ابحث في crates.io وأضفها هنا قبل الاستخدام.

**قواعد القبول:**
1. ترخيص متساهل فقط (MIT / Apache-2.0 / BSD / BSL-1.0) — ممنوع GPL/AGPL.
2. مشروع حي (آخر تحديث خلال سنة) أو رسمي (مايكروسوفت / tauri-apps).
3. تُسجَّل هنا: الاسم، الغرض، الـ APIs التي سنستخدمها، البديل، الحالة.
4. النسخ الدقيقة تُثبت وقت التنفيذ في `Cargo.toml` (لا نحفظ أرقام نسخ في الخطة كي لا تتقادم).
5. الإنفاذ الآلي: `cargo-deny` (تراخيص) + `cargo-audit` (ثغرات) — لا إصدار بدونهما.

**مفاتيح الحالة:** [مرشح] / [معتمد] / [يُحسم بالفحص] (راجع UNVERIFIED)

---

## 1. القشرة والواجهة (Plan A — Tauri)

### 1.1 `tauri` (v2) [معتمد]
- **الغرض:** إطار التطبيق: إدارة النوافذ WebView، الأوامر (`#[tauri::command]`)، الأحداث (`emit`)، دورة الحياة، التغليف.
- **الـ APIs التي سنستخدمها:** `Builder`, `AppHandle`, `WebviewWindow`, `Manager::get_webview_window`, `Emitter::emit`, `WindowEvent::Focused`, إعداد `tauri.conf.json`، ميزة `tray-icon` مدمجة (`TrayIconBuilder`, `TrayIconEvent::Click`, `MenuBuilder`).
- **البديل:** egui (Plan B) / WinUI 3 (Plan C — مرفوض).
- **الترخيص:** Apache-2.0 / MIT (متساهل — مؤكد عمليًا بالاستخدام الواسع).
- **ملاحظات/مخاطر:** يجرّ WebView2 (مثبت مسبقًا على Win11، وعلى Win10 يحتاج Bootstrapper في المُثبّت — بند U17). استهلاك ذاكرة أعلى من واجهة أصلية (مقبول مقابل سرعة التطوير ودعم RTL).

### 1.2 `tauri-plugin-positioner` [معتمد — راجع U04/U16]
- **الغرض:** تموضع نافذة اللوحة قرب أيقونة الـ Tray (المشكلة المعمارية الأولى في نمط menu-bar app).
- **API:** `WindowExt::move_window(Position::TrayCenter)` + `tauri_plugin_positioner::on_tray_event(app, &event)` (إلزامي في معالج الـ Tray لتخزين مستطيل الأيقونة) + ميزة Cargo `tray-icon` لتفعيل متغيرات `Tray*`.
- **البديل:** حساب الإحداثيات يدويًا من `TrayIconEvent::rect` (نحتفظ به كـ Fallback في Spike-2 لو سيئ على ويندوز).
- **الترخيص:** MIT / Apache-2.0 (رسمي من tauri-apps).

### 1.3 `tauri-plugin-global-shortcut` [معتمد]
- **الغرض:** الاختصارات العامة (فتح اللوحة، لصق من لوحة i).
- **API:** `Builder::new().with_shortcuts([...]).with_handler(|app, shortcut, event| ...)`، `ShortcutState::{Pressed,Released}`, `Shortcut::matches(Modifiers, Code)`, `global_shortcut().register/unregister`, و`isRegistered()` (كشف تعارض الحجز — D11).
- **البديل:** `global-hotkey` crate مباشرة.
- **الترخيص:** MIT / Apache-2.0 (رسمي).

### 1.4 `tauri-plugin-autostart` [معتمد]
- **الغرض:** التشغيل التلقائي عند الإقلاع (مفتاح Run).
- **البديل:** `winreg` يدويًا (نحتفظ به كمسار احتياطي في Core لأن نفس المنطق سنحتاجه لأدوات الحزمة لاحقًا).
- **الترخيص:** MIT / Apache-2.0 (رسمي).

### 1.5 `tauri-plugin-single-instance` [معتمد]
- **الغرض:** نسخة واحدة فقط؛ تشغيل ثانٍ يمرر وسائطه (مثل `--settings`) للنسخة القائمة التي تفتح نافذة الإعدادات.
- **الترخيص:** MIT / Apache-2.0 (رسمي).

### 1.6 `tauri-plugin-opener` [مرشح]
- **الغرض:** فتح مجلد blobs / ملف التصدير / رابط "عن التطبيق" من الواجهة.
- **البديل:** `ShellExecuteW` عبر windows-rs.

### 1.7 الواجهة الأمامية: **HTML/CSS/JS بسيط + `@tauri-apps/api`** [معتمد]
- **الغرض:** اللوحة والإعدادات. `invoke()` للأوامر و`listen()` للأحداث.
- **مرفوض:** React/Vue/Svelte — وزن وصيانة زائدة لواجهة بهذا الحجم. القاعدة: لو تضخمت الواجهة لاحقًا نعيد النظر هنا فقط (لا يمس Core).
- **i18n:** ملفات JSON + دالة صغيرة (`i18n.js`) — بدون مكتبات (D12).
- **قوائم طويلة:** نبني عرضًا افتراضيًا بسيطًا (Virtual list) أو `content-visibility: auto` — لا مكتبة.

---

## 2. محرك ويندوز (Core)

### 2.1 `windows` (windows-rs) [معتمد]
- **الغرض:** كل استدعاءات Win32 التي لا تغطيها المكتبات الأخص.
- **ميزات Cargo المطلوبة (نثبتها بدقة في المرحلة 1):**
  - `Win32_System_DataExchange` — GetClipboardData/EnumClipboardFormats/RegisterClipboardFormat/GetClipboardOwner/GetClipboardSequenceNumber
  - `Win32_UI_WindowsAndMessaging` — إنشاء نافذة الرسائل، WM_CLIPBOARDUPDATE، AddClipboardFormatListener، GetForegroundWindow/SetForegroundWindow، نصوص النوافذ
  - `Win32_UI_Input_KeyboardAndMouse` — SendInput و VK_CONTROL/VK_V
  - `Win32_Foundation` — أنواع أساسية (HWND, HGLOBAL)
  - `Win32_Graphics_Gdi` — BITMAPINFOHEADER/BITMAPV5HEADER لفك DIB
  - `Win32_System_Threading` — GetWindowThreadProcessId (مالك الحافظة)
  - `Win32_System_ProcessStatus` + `Win32_System_Threading` — اسم عملية المالك (استبعاد التطبيقات)
  - `Win32_System_Registry` — مسار autostart الاحتياطي
- **البديل:** `windows-sys` (أخفض مستوى، بلا أغلفة آمنة) — نستخدمه فقط لو اضطررنا.
- **الترخيص:** MIT / Apache-2.0 (مايكروسوفت رسميًا).

### 2.2 `clipboard-win` [يُحسم بالفحص — المرشح الأول D4]
- **الغرض:** طبقة قراءة/كتابة الحافظة بتنسيقاتها — نتفادى كتابة هذا الملف يدويًا.
- **الـ APIs الموثقة التي سنستخدمها:**
  - `raw::get_owner() -> Option<NonNull<c_void>>` — منع حلقة نسخنا + استبعاد التطبيقات.
  - `raw::get_string() -> SysResult<usize>` (UTF-8) — النصوص.
  - `raw::get_file_list(&mut Vec<String>)` — CF_HDROP (موثقة + أمثلة في المصدر).
  - `raw::get_bitmap()` — يعيد BITMAPINFO + بيانات RGB (كافية لبناء صورة).
  - `formats::{CF_UNICODETEXT, CF_DIB, CF_DIBV5, CF_HDROP}` — الأولى في الأولوية.
  - `raw::set / set_string` — الكتابة عند اللصق.
- **البديل:** `arboard` (أبسط لكن بلا مالك/خام).
- **الترخيص:** BSL-1.0 (طبقًا للتوثيق — **يُتحقق نهائيًا في U13 قبل الإصدار**).
- **ملاحظة Spike-1:** يُختبر جنبًا إلى جنب مع arboard على مصفوفة: نص عربي/إيموجي، صورة متصفح، Snipping Tool، Excel (عرض مؤجل)، ملفات من المستكشف، ونسخ من Bitwarden.

### 2.3 `arboard` [يُحسم بالفحص — بديل/مكمل]
- **الغرض:** الاحتياط ومصدر فكرة أعلام الكتابة؛ يدعم `exclude_from_monitoring/cloud/history` عند الكتابة (مرئية في كوده) وكتابة قوائم ملفات.
- **البديل:** clipboard-win.
- **الترخيص:** MIT / Apache-2.0.
- **ملاحظة:** لا يعطي مالك الحافظة ولا DIB خام بنفس السهولة → لا يصلح قائدًا وحده، يصلح مساعدًا/احتياطًا.

### 2.4 `blake3` [معتمد]
- **الغرض:** بصمة محتوى 32 بايت (hex) — منع التكرار + تسمية ملفات blobs (Content addressing).
- **API:** `blake3::Hasher::new()/update()/finalize().to_hex()`.
- **البديل:** `sha2` (أبطأ بنفس التطبيق)، xxhash (أسرع لكن أضعف — تُستخدم فقط لو أثبت القياس أنها ضرورية).
- **الترخيص:** CC0-1.0 / Apache-2.0 (متساهل — يسمح الاستخدام التجاري بلا قيود).

### 2.5 `image` [معتمد]
- **الغرض:** فك/ترميز الصور + توليد المصغرات.
- **الميزات المطلوبة:** `png` (ترميز موحد + مصغرات). JPEG اختياري لو أردنا حفظ الأصل لاحقًا.
- **API:** `image::imageops::thumbnail(&img, w, h)`, `DynamicImage::write_to(Cursor, ImageFormat::Png)`.
- **تحويل DIB:** المكتبة لا تفك DIB خام؛ الحل المتبع (كود صغير ~20 سطرًا بإضافة BITMAPFILEHEADER لإنتاج BMP صالح) ثم الفك بـ `image::load_from_memory` — هذا "لصق تنسيقات" لا "إعادة اختراع": فك الضغط والإنتاج على المكتبة.
- **البديل:** `png` crate مباشرة (نستخدمه ضمنيًا عبر image) — لا بديل أوفر أحسن.
- **الترخيص:** MIT / Apache-2.0.

---

## 3. التخزين والإعدادات

### 3.1 `rusqlite` [معتمد]
- **الغرض:** كل التخزين: عناصر، فهارس، استعلامات، معاملات.
- **الميزات:** `bundled` (يضم SQLite مبنية مع المشروع — صفر متطلبات على جهاز المستخدم) + `blob_open` لقراءة الـ BLOB بلا نسخ زائد (تحسين لاحق).
- **API:** `Connection::open_with_flags`, `execute_batch` للـ PRAGMA/DDL, `prepare_cached` للاستعلامات المتكررة, `Transaction`.
- **البديل:** `sqlx` (async أشمل — مرفوض: تعقيد غير مبرر لتطبيق محلي بأمر واحد)، `redb/sled` (بلا SQL/فهارس نصية — مرفوض).
- **الترخيص:** MIT.
- **قرارات مرتبطة:** D5 (عتبة 96KB، PRAGMAs كاملة هناك).

### 3.2 `serde` + `serde_json` [معتمد]
- **الغرض:** تسلسل الإعدادات وقوائم الملفات ورسائل IPC الداخلية.
- **الترخيص:** MIT / Apache-2.0.

### 3.3 `toml` [معتمد]
- **الغرض:** ملف الإعدادات (D6). API: `toml::from_str` / `toml::to_string_pretty`.
- **البديل:** JSON/YAML — مرفوض: TOML أفضل لملف تُعدله يدويًا عند الضرورة.

### 3.4 `dirs` [معتمد]
- **الغرض:** مسارات `config_dir()/data_dir()` القياسية → `%APPDATA%\SIM`.
- **البديل:** بناء المسار يدويًا من `%APPDATA%` (نتفاداه لأن dirs مصانة ومختبرة).

---

## 4. بنية تحتية (Infrastructure)

| المكتبة | الغرض والـ API | البديل | الترخيص | الحالة |
|---|---|---|---|---|
| `crossbeam-channel` | ناقل الأحداث والأوامر بين الخيوط. `bounded` للـ capture (256 — لا تراكم)، `unbounded` للأوامر، `select!` | `std::mpsc` (أضعف: بلا select/timeout) | MIT/Apache-2.0 | معتمد |
| `tracing` + `tracing-subscriber` + `tracing-appender` | سجلات دوّارة يوميًا، 7 ملفات، فلترة بمتغير بيئة. **لا محتوى حافظة أبدًا** | `log` + `env_logger` | MIT | معتمد |
| `thiserror` (في core) | تعريف `CoreError` الغني بتصنيف الأخطاء | كتب يدوية | MIT/Apache-2.0 | معتمد |
| `anyhow` (في القشرة/CLI) | تمرير الأخطاء بسياق في طبقات التطبيق | thiserror فقط | MIT/Apache-2.0 | معتمد |
| `winreg` | autostart الاحتياطي + قراءات ريجستري مستقبلية للحزمة | استدعاءات windows-rs خام | MIT | معتمد |

---

## 5. أدوات تطوير (لا تدخل الإصدار النهائي)

| الأداة | الغرض |
|---|---|
| `tauri-cli` | تشغيل/بناء وتغليف تطبيق Tauri |
| `cargo-watch` | إعادة بناء تلقائية أثناء التطوير |
| `cargo-nextest` (اختياري) | اختبارات أسرع (توازي أفضل) |
| `cargo-deny` | **إنفاذ قاعدة التراخيص (D8)** + كشف التكرارات + مصادر غير آمنة |
| `cargo-audit` | كشف الثغرات المعروفة في شجرة التبعيات |
| `clippy` (مدمج) | `cargo clippy -- -D warnings` بوابة جودة |

---

## 6. مرفوضات موثقة (لا نضيفها حتى لو شائعة)

| المكتبة | لماذا رُفضت |
|---|---|
| Electron / Node runtime | ضد متطلب "خفيف" و"بلا واجهة ويب ثقيلة" — Tauri يغطي الحاجة |
| أي ORM (diesel/sea-orm) | جدولان فقط — الاستعلامات المباشرة أوضح وأخف |
| `chrono` | نحتاج unix millis فقط — `std::time` كافٍ (نضيفه فقط لو احتجنا تنسيقات معقدة) |
| مكتبات شبكات (reqwest/tokio-net) في Core | التطبيق صفر شبكة — وجودها يخالف الوعد |

---

## 7. مؤجلة لأفق الحزمة (لا تُضاف الآن)

| الأفق | مكتبات مرشحة | ملاحظة |
|---|---|---|
| بحث ملفات (H2) | قراءة MFT عبر `windows-rs` + `io_uring`-like buffered reads | لا مكتبة ناضجة — استثناء مبرر من قاعدة "لا تكتب من الصفر" |
| محلّل مساحة (H2) | نفس أساس البحث + رسم Treemap بالواجهة | — |
| سطوع الشاشات | نُقلت إلى الميزة الثانية — انظر القسم 8 (المراحل 9–11) | — |
| معادل صوتي (H3) | APO عبر EqualizerAPO كتبعية خارجية | يحتاج دراسة ترخيص وتعبئة |
| إيماءات اللمس (H4) | Precision Touchpad APIs عبر windows-rs | — |

---

## 8. مكتبات الميزة الثانية (مخططة — المراحل 9–11)

### 8.1 `ddc-hi` [مرشح — يُحسم بـ F2-1]
- **الغرض:** التحكم DDC/CI عالي المستوى: تعداد الشاشات + قراءة/كتابة أكواد VCP (السطوع 0x10) بلا كتابة طبقة البروتوكول يدويًا.
- **الـ APIs المتوقعة:** `Display::enumerate()`، `display.update_capabilities()`، `display.info` (backend/id/manufacturer/model)، `display.handle.get_vcp_feature(0x10)` / `set_vcp_feature`.
- **يشمل داخليًا:** EnumDisplayMonitors + GetPhysicalMonitorsFromHMONITOR (وسيط ddc-winapi) + NVAPI (ميزة اختيارية).
- **الترخيص:** MIT.
- **الخطر المصنّف (→ U20):** آخر إصدار 0.4.1 (2021) — ناضج لكن شبه مجمد. **مسار السقوط المعتمد:** غلاف رقيق (~200 سطر) فوق Monitor Configuration API من windows-rs مباشرة (`GetVCPFeatureAndVCPFeatureReply` / `SetVCPFeature`) — تغليف مباشر لدوال النظام (ليس "إعادة اختراع")، يُبرَّر فقط إن أثبتت F2-1 قصورًا.

### 8.2 `windows` (windows-rs) — ميزات Cargo إضافية [معتمد]
- `Win32_Devices_Display` — دوال Monitor Configuration (مسار السقوط + الحالات الخاصة).
- `Win32_UI_Accessibility` — `SetWinEventHook` / `EVENT_SYSTEM_FOREGROUND` (تتبع المقدمة — D17).
- `Win32_System_RemoteDesktop` — `WTSRegisterSessionNotification` (فواصل القفل/الفتح).
- `Win32_UI_Input_KeyboardAndMouse` — `GetLastInputInfo` (موجودة أصلًا من v1).

### 8.3 مرشحات مرفوضة للميزة الثانية

| المرشح | لماذا رُفض |
|---|---|
| `active-win-pos-rs` (قراءة النافذة النشطة) | تعتمد Polling — أقل دقة من الخطاف وأعلى استهلاكًا؛ الخطاف المباشر عبر windows-rs أوضح وأخف (D17) |
| استدعاء أدوات جاهزة كعملية خارجية (ControlMyMonitor / winddcutil) | يخالف فلسفة التطبيق الواحد المكتفي ذاتيًا ويزيد سطح الأخطاء (تعبئة، مسارات، لغة إخراج) |
| WMI (`WmiMonitorBrightness`) للشاشة الداخلية | خارج نطاق v2 (يغطيها ويندوز نفسه) — Backlog |
