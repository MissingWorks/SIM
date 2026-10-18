# المعمارية (Architecture)

> مستقلة عن الخطة المختارة في معظمها: **المحرك (Core) مشترك**، وقشرة الواجهة قابلة للتبديل.

## مبادئ معمارية (تُحسم الخلافات بها)

1. **المسار الحرج لا يتوقف أبدًا:** خيط مستمع الحافظة لا يقرأ ولا يكتب ولا ينتظر — يرسل إشارة فقط. كل العمل في خيط العامل.
2. **الواجهة غبية:** JS يعرض ويستدعي أوامر — صفر منطق أعمال في الواجهة.
3. **مصدر واحد للحقيقة:** الإعدادات والبيانات في Core فقط؛ الواجهة تعكسها.
4. **الفشل الجزئي لا يسقط التطبيق:** كل مسار فشل له سياسة (إعادة محاولة / تدهور رشيق / تسجيل) — لا `unwrap` في مسار التشغيل.

## نظرة عامة

```
┌───────────────────────────────────────────────────────────────┐
│                         sim-core                          │
│   (Rust خالص — بلا واجهة — قابل للاختبار بالكامل عبر CLI)      │
│                                                                │
│  clipboard/  ── مستمع WM_CLIPBOARDUPDATE + قراءة/كتابة         │
│  store/      ── SQLite: CRUD + بحث + استبقاء (Retention)       │
│  config      ── TOML: تحميل/تحقق/حفظ ذري                       │
│  image_util  ── DIB/PNG → PNG + مصغرات                          │
│  paste       ── SendInput + حفظ/استرجاع النافذة السابقة        │
│  events      ── ناقل أحداث (crossbeam)                         │
└─────────────────────────────┬─────────────────────────────────┘
                              │ EngineHandle API
        ┌─────────────────────┼──────────────────────┐
        ▼                     ▼                      ▼
 sim-desktop      sim-egui          sim-cli
 (Tauri — Plan A)      (بديل — Plan B)        (تطوير/اختبار)
```

## خريطة الكريتات والملفات (البنية النهائية المستهدفة)

```
code/
├─ Cargo.toml                        # workspace + shared [workspace.dependencies]
├─ rust-toolchain.toml               # تثبيت إصدار Rust المستقر
├─ crates/
│  ├─ sim-core/
│  │  └─ src/
│  │     ├─ lib.rs                   # يصدّر Engine, Config, Clip, Event...
│  │     ├─ error.rs                 # CoreError (thiserror) + تصنيف الأخطاء
│  │     ├─ model.rs                 # Clip, ClipKind, ClipId, PasteMode, ListQuery...
│  │     ├─ config.rs                # Config (serde) + defaults + load/save ذري + validate
│  │     ├─ events.rs                # enum Event + EventBus (crossbeam)
│  │     ├─ engine.rs                # Engine::start + EngineHandle + actor dispatch
│  │     ├─ clipboard/
│  │     │  ├─ mod.rs
│  │     │  ├─ listener.rs           # نافذة رسائل فقط + AddClipboardFormatListener
│  │     │  ├─ reader.rs             # أولوية التنسيقات + retry + فلاتر الاستبعاد
│  │     │  └─ writer.rs             # كتابة نص/صورة/ملفات للحافظة
│  │     ├─ store/
│  │     │  ├─ mod.rs                # StoreActor (خيط واحد يملك الاتصال)
│  │     │  ├─ schema.rs             # DDL + migrations عبر user_version
│  │     │  ├─ queries.rs            # دوال CRUD/بحث/استبقاء
│  │     │  └─ blobs.rs              # ملفات الصور الكبيرة (مجلدات مقسمة)
│  │     ├─ image_util.rs            # PNG/DIB → الموحّد PNG + thumbnail
│  │     ├─ paste.rs                 # تسلسل اللصق التلقائي + الحالات الراجعة
│  │     └─ platform/
│  │        ├─ mod.rs
│  │        ├─ window.rs             # مساعد نافذة الرسائل (HWND_MESSAGE)
│  │        ├─ process.rs            # PID/اسم العملية من HWND (للاستبعاد)
│  │        └─ autostart.rs          # مفتاح Run كاحتياطي مستقل
│  ├─ sim-cli/
│  │  └─ src/main.rs                 # list/watch/pause/config/bench — للاختبار بلا واجهة
│  └─ sim-desktop/
│     ├─ tauri.conf.json             # إعداد النوافذ (الجدول أدناه)
│     ├─ capabilities/default.json   # صلاحيات الإضافات
│     ├─ icons/                      # أيقونات التطبيق والـ Tray
│     ├─ src/
│     │  ├─ main.rs                  # Tauri builder: الإضافات + setup
│     │  ├─ state.rs                 # AppState { engine: EngineHandle }
│     │  ├─ commands.rs              # #[tauri::command] رقيقة فوق EngineHandle
│     │  ├─ tray.rs                  # بناء الأيقونة + القائمة + toggle اللوحة
│     │  ├─ hotkeys.rs               # تسجيل الاختصارات من الإعدادات + كشف التعارض
│     │  └─ bridge.rs                # تحويل أحداث Core → أحداث Tauri للواجهة
│     └─ ui/                         # الواجهة (بدون إطار عمل)
│        ├─ panel/    (index.html, app.js, style.css)
│        ├─ settings/ (index.html, app.js, style.css)
│        ├─ i18n/     (en.json, ar.json, i18n.js)
│        └─ shared/   (theme.css, ipc.js)
```

## نموذج الخيوط (Threading)

```
[Main Thread — Tauri Event Loop]
  ├─ Tray icon + قوائمها
  ├─ نافذتا WebView: panel (مخفية غالبًا) + settings
  └─ Global hotkeys (عبر الإضافة) — تُترجم لأوامر للـ Engine

[Listener Thread — نافذة رسائل فقط (HWND_MESSAGE)]
  ├─ AddClipboardFormatListener(hwnd)
  └─ WM_CLIPBOARDUPDATE → CaptureSignal { seq } → capture_tx
      (لا يفتح الحافظة أبدًا — فقط يمرر الإشارة)

[Worker Thread — يملك اتصال SQLite الوحيد (StoreActor)]
  ├─ يستقبل: CaptureSignal + Command{...}
  ├─ التقاط: يقرأ الحافظة → يطبّع → يدرج → استبقاء → Event
  ├─ أوامر: list/get/pin/delete/clear/move_slot/config/stats
  └─ يرسل الردود عبر oneshot channels (crossbeam) ويعمّم Events عبر broadcast
```

- **لماذا Actor؟** اتصال SQLite واحد بلا أقفال متشابكة؛ كل العمليات مرتبة؛ الاختبار أسهل.
- **أوامر Tauri:** كل `#[tauri::command]` ينفذ `tauri::async_runtime::spawn_blocking` يستدعي `EngineHandle` (الذي يرسل الأمر وينتظر الرد بزمن أقصى 2 ثانية) — لا تجميد للواجهة.
- **الأحداث إلى الواجهة:** `bridge.rs` يشترك في broadcast ويحوّله إلى `app.emit("clip-added" | "clips-changed" | "config-changed", ...)`.

## واجهة EngineHandle البرمجية (الشكل الملزم)

```rust
// sim-core/src/model.rs
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ClipId(pub i64);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClipKind { Text, Image, Files }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PasteMode { Auto, CopyOnly }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClearScope { All, Unpinned, Slot(u16) }

#[derive(Default, Clone)]
pub struct ListQuery {
    pub slot: Option<u16>,        // None = اللوحة الرئيسية فقط, Some = لوحة محددة
    pub kind: Option<ClipKind>,   // None = الكل
    pub pinned_only: bool,
    pub search: Option<String>,   // بحث نصي LIKE
    pub limit: u32,               // افتراضي 50، أقصى 500
    pub offset: u32,
}

// sim-core/src/engine.rs
pub struct EngineHandle { /* Arc<EngineInner> */ }

impl EngineHandle {
    pub fn start(config: Config) -> Result<Self, CoreError>;

    // اشتراك بالأحداث — القشرة تترجمها لواجهتها
    pub fn subscribe(&self) -> crossbeam_channel::Receiver<Event>;

    // استعلامات
    pub fn list(&self, q: ListQuery) -> Result<Vec<Clip>, CoreError>;
    pub fn get(&self, id: ClipId) -> Result<Clip, CoreError>;
    pub fn stats(&self) -> Result<DbStats, CoreError>;          // عدد عناصر/أحجام/أقدم عنصر

    // تعديلات
    pub fn pin(&self, id: ClipId, pinned: bool) -> Result<(), CoreError>;
    pub fn delete(&self, ids: &[ClipId]) -> Result<(), CoreError>;
    pub fn clear(&self, scope: ClearScope) -> Result<(), CoreError>;
    pub fn move_to_slot(&self, ids: &[ClipId], slot: u16) -> Result<(), CoreError>;

    // لصق
    pub fn note_previous_foreground(&self, hwnd: i64);          // تلتقطه القشرة عند فتح اللوحة
    pub fn paste(&self, id: ClipId, mode: PasteMode) -> Result<(), CoreError>;
    pub fn paste_slot(&self, slot: u16, mode: PasteMode) -> Result<(), CoreError>;

    // إعدادات وحالة
    pub fn config(&self) -> Config;
    pub fn set_config(&self, cfg: Config) -> Result<(), CoreError>;
    pub fn set_paused(&self, paused: bool) -> Result<(), CoreError>;

    // نسخ احتياطي (نستخدمها في مركز التحكم)
    pub fn export_backup(&self, dest: &std::path::Path) -> Result<(), CoreError>;
}
```

```rust
// sim-core/src/events.rs
#[derive(Clone, Debug)]
pub enum Event {
    ClipAdded { id: ClipId },        // عنصر جديد (تحديث فوري للواجهة إن مفتوحة)
    ClipsRemoved { count: usize },   // نتيجة حذف/استبقاء
    ClipsChanged,                    // تغيير عام (إعادة استعلام)
    ConfigChanged,
    CapturePaused { paused: bool },
    PasteFallback { reason: String },// فشل اللصق التلقائي → نسخ فقط + سبب
}
```

## مخطط الإعدادات (config.toml)

```toml
# %APPDATA%\SIM\config.toml  —  يُدار من مركز التحكم فقط
schema_version = 1

[general]
language = "en"                # "en" | "ar"
launch_at_login = true
paste_mode = "auto"            # "auto" | "copy_only"
prewarm_panel = true           # إنشاء نافذة اللوحة مخفية عند الإقلاع (من Spike-2)
paste_restore_delay_ms = 80    # الانتظار بعد إعادة التركيز قبل SendInput

[history]
max_items = 1000               # إجمالي العناصر (غير المثبتة تُحذف الأقدم أولًا)
max_total_bytes = 524288000    # 500MB سقف تخزين
max_item_bytes = 20971520      # 20MB — أكبر من كذا يُتجاهل
blob_threshold_bytes = 98304   # 96KB — أقل=داخل القاعدة، أكثر=ملف
keep_pinned = true             # المثبت لا يُحذف أبدًا بالإخلاء

[slots]
count = 5                      # 0..=20
names = ["A", "B", "C", "D", "E"]

[hotkeys]
open_panel = "Win+Alt+V"
paste_slot_1 = "Win+Alt+1"
# ... حتى paste_slot_9

[privacy]
paused = false                 # إيقاف الالتقاط مؤقتًا
clear_on_exit = false
excluded_apps = []             # ["keepass.exe", ...] بأسماء العمليات (lowercase)
honor_system_exclusions = true # أعلام ويندوز الرسمية — لا تُعطّل إلا على مسؤولية المستخدم

[ui]
theme = "system"               # system | light | dark
panel_width = 380
panel_max_height = 520
hide_panel_on_blur = true
```

## مخطط قاعدة البيانات (DDL)

```sql
-- migrations عبر PRAGMA user_version: v0 → v1 = DDL أدناه
CREATE TABLE IF NOT EXISTS clips (
  id            INTEGER PRIMARY KEY,
  kind          TEXT    NOT NULL CHECK (kind IN ('text','image','files')),
  text          TEXT,                -- kind=text  : النص الكامل
  preview       TEXT    NOT NULL,    -- سطر معاينة محسوب عند الإدراج (بحث + عرض)
  blob          BLOB,                -- صور أكبر من thumb وأصغر من العتبة
  blob_path     TEXT,                -- صور فوق العتبة: "blobs/ab/{hash}.png"
  width         INTEGER,             -- أبعاد الصورة
  height        INTEGER,
  files_json    TEXT,                -- kind=files : مصفوفة JSON للمسارات
  size_bytes    INTEGER NOT NULL,
  hash          TEXT    NOT NULL,    -- blake3 (hex) للمحتوى المطبع
  pinned        INTEGER NOT NULL DEFAULT 0,
  slot          INTEGER NOT NULL DEFAULT 0,   -- 0 = الرئيسية
  created_at    INTEGER NOT NULL,    -- unix millis
  last_used_at  INTEGER
);

CREATE TABLE IF NOT EXISTS meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);
-- meta: schema_version, created_at, install_id

CREATE INDEX IF NOT EXISTS clips_created_idx ON clips(created_at DESC);
CREATE INDEX IF NOT EXISTS clips_slot_idx    ON clips(slot, created_at DESC);
CREATE INDEX IF NOT EXISTS clips_kind_idx    ON clips(kind, created_at DESC);
CREATE INDEX IF NOT EXISTS clips_hash_idx    ON clips(hash);
CREATE INDEX IF NOT EXISTS clips_pinned_idx  ON clips(pinned) WHERE pinned = 1;
```

**لا نخزن المصغرة في عمود منفصل؟** مبدئيًا thumb تُخزن داخل `blob` (PNG صغير — أقل من العتبة دائمًا). الواجهة تعرض `thumb`/`blob` مباشرة عبر `asset://` أو base64 مؤقت.
> قرار مفتوح صغير: تسليم الصور للواجهة (base64 vs custom protocol vs ملف مؤقت) — يُحسم في المرحلة 4 حسب `convertFileSrc` في Tauri.

## سياسة الاستبقاء (Retention) — الخوارزمية

```
عند إدراج عنصر جديد (نفس المعاملة):
  1. INSERT
  2. while count(clips) > max_items:
        احذف أقدم صف غير مثبت (slot=0 أولًا ثم الأقدم عمومًا)
  3. while حجم_التخزين > max_total_bytes:
        احذف أقدم صف غير مثبت
  4. سجّل الصفوف المحذوفة (ids + blob_paths)
بعد commit:
  5. احذف ملفات blobs المحذوفة (best-effort — فشل الحذف يُسجل فقط)
  6. إن حُذف شيء → Event::ClipsRemoved، ثم Event::ClipAdded دائمًا
  7. كل 50 عملية حذف: PRAGMA incremental_vacuum
```

**حساب الحجم:** مجموع (حجم ملف db + wal) + مجموع أحجام ملفات blobs. يُحدَّث في ذاكرة العامل بعد كل عملية (لا مسح للمجلد كل مرة).

## خط أنابيب الالتقاط (خطوة بخطوة)

```
1) WM_CLIPBOARDUPDATE يستقبل في خيط المستمع
   → يقرأ GetClipboardSequenceNumber() ويرسل CaptureSignal{seq} للعامل عبر channel (ميكروثوانٍ)
   → إن كان seq مساويًا لآخر رقم مُعالج → تجاهل صامت (منع الازدواج + رسائل الأدوات النظامية الزائفة — D21/U27)

2) العامل يستقبل الإشارة:
   a. إن paused أو التطبيق المالك في excluded_apps (يُحدد لاحقًا بـ GetClipboardOwner→PID→exe)
      → تجاهل
   b. فتح الحافظة بإعادة محاولة: حتى 10 محاولات × 10ms (أقصى ~100ms؛ تُرفع حتى 10×100ms بنمط .NET إن أثبت Spike-6 — وعند الفشل يُسجَّل GetOpenClipboardWindow للتشخيص)
   c. بعد الفتح: Owner == HWND نافذتنا؟ → تجاهل (منع حلقة نسخنا)
   d. فحص أعلام الاستبعاد (أي واحد يكفي للتجاهل):
        • وجود التنسيق المسجل "ExcludeClipboardContentFromMonitorProcessing"
        • "CanIncludeInClipboardHistory" بقيمة 0
        • وجود "Clipboard Viewer Ignore" (علم قديم — CopyQ يحترمه)
        • honor_system_exclusions=false → تخطى هذه الخطوة (من إعدادات المستخدم)
   e. قراءة التنسيقات بالأولوية:
        PNG (مسجل) → CF_DIBV5 → CF_DIB → CF_UNICODETEXT → CF_HDROP → فشل (تجاهل بهدوء)
   f. بصمة التسلسل: تكرر seq تغيّر أثناء القراءة؟ → أعد المحاولة مرة واحدة
   g. تطبيع + بصمة blake3:
        نص: توحيد CRLF→LF للبصمة فقط (النص يخزن كما هو) — عربي طبيعي بلا معالجة
        صورة: DIB → BMP بإضافة BITMAPFILEHEADER (14 بايت) → فك بـ image crate → PNG موحد
        ملفات: مسارات كما هي + معاينة بأسماء الملفات
   h. مكرر؟ = نفس البصمة لآخر عنصر (أو ضمن نافذة 800ms) → حدّث last_used_at فقط واخرج
   i. Insert + استبقاء (القسم السابق) + Events
```

## تسلسل اللصق (Paste Subsystem)

```
فتح اللوحة:
  القشرة تلتقط GetForegroundWindow() قبل Show()  → engine.note_previous_foreground(hwnd)

اللصق (Enter أو نقرة):
  1. اقرأ العنصر من القاعدة
  2. اكتبه في الحافظة (نص / PNG / قائمة ملفات)
     - لا نضع أعلام استبعاد هنا: المحتوى مقصود لصقه في تطبيق آخر
     - نضع علامة ملكية: نافذتنا هي Owner → مستمعنا يتجاهل التغيير تلقائيًا (لا ازدواج)
  3. mode == CopyOnly → أكمل (إشعار اختياري "تم النسخ")
  4. mode == Auto:
       a. أخفِ اللوحة
       b. ضمان المقدمة — سلسلة D20 الملزمة:
            1) SetForegroundWindow(prev_hwnd)
            2) فشل؟ → خدعة ALT: SendInput(VK_MENU down/up) ثم إعادة محاولة مرة واحدة
            3) ما زال فشلًا؟ → Event::PasteFallback("target_not_focused") — نسخ فقط + إشعار
            ✗ AttachThreadInput ممنوع نهائيًا (تعليق موثق — Raymond Chen)
       c. انتظر paste_restore_delay_ms (افتراضي 80 — يُقاس في Spike-7)
       d. تحقق نهائي: GetForegroundWindow() != prev_hwnd؟ → PasteFallback كما في (b-3)
       e. SendInput(Ctrl down, V down, V up, Ctrl up)
       f. إن فشل SendInput (قيمة إرجاع 0) → Event::PasteFallback(reason)
  5. حدّث last_used_at
```

**قيد موثق:** هدف مرفوع الصلاحية (UIPI) → خطوات ضمان المقدمة/SendInput تفشل → نرجع لسلوك النسخ فقط + إشعار. لا نشغل التطبيق كمسؤول أبدًا.
**قيد موثق (Win10+):** الرفع المبرمج مسموح عمليًا لمن استلم آخر إدخال (حالتنا الغالبة — Enter داخل اللوحة)، لكن الحالات الحدّية والتوقيت تجعل خدعة ALT في (b-2) جزءًا إلزاميًا من السلسلة — D20/U28.

## نوافذ Tauri (tauri.conf.json)

| النافذة | الإعدادات |
|---|---|
| `panel` | 380×520، `decorations:false`، `transparent:true`، `alwaysOnTop:true`، `skipTaskbar:true`، `resizable:false`، `visible:false`، `focus:false` — تُظهرها القشرة بـ `move_window(Position::TrayCenter/TrayBottomCenter)` ثم `show()+set_focus()` |
| `settings` | 900×640، نوافذ عادية، تُنشأ عند أول فتح ثم تُخفى ولا تُحذف (فتح فوري لاحقًا) |

- **إخفاء اللوحة:** حدث `WindowEvent::Focused(false)` → `hide()` (قابل للتعطيل من الإعدادات `hide_panel_on_blur`).
- **التمرير الإلزامي للـ positioner:** `tauri_plugin_positioner::on_tray_event` يُستدعى دائمًا في معالج أيقونة الـ Tray قبل أي إظهار (وإلا يفشل `TrayCenter`).
- **prewarm:** إن كان `prewarm_panel=true` تُنشأ نافذة اللوحة مخفية في الإقلاع (قرار Spike-2 بمقايضة ذاكرة ~40-80MB مقابل فتح فوري).

## الاختصارات (Hotkeys)

| الوظيفة | الافتراضي | الفشل/التعارض |
|---|---|---|
| فتح/إغلاق اللوحة | Win+Alt+V | محجوز من تطبيق آخر → تحذير في الإعدادات + الأيقونة لا تزال تفتح اللوحة |
| لصق من لوحة i | Win+Alt+1..9 | نفس المعالجة |
| إيقاف الالتقاط | بدون افتراضي | — |

- التسجيل من `config.hotkeys` عند الإقلاع وبعد كل `set_config`.
- كشف التعارض: محاولة التسجيل → فشل → احتفظ بالقديم + `ConfigChanged` + علم `hotkey_conflict{}` تقرؤه الواجهة.

## الترجمة (i18n)

- `ui/i18n/en.json` (الافتراضي) و `ui/i18n/ar.json` — نفس المفاتيح إلزاميًا.
- `i18n.js` صغير: `t("panel.search_placeholder")` + تبديل `document.documentElement.dir`؛ `ar` → `rtl`.
- تغيير اللغة فوري بلا إعادة تشغيل (يعيد رسم النصوص من ملف الترجمة).
- القاعدة: لا نص واجهة مكتوب مباشرة في HTML/JS — مفاتيح فقط (فحص آلي بسيط في مرحلة 6: بحث `>نص<` في HTML المولّد).

## السجلات والأخطاء

- `logs/sim.log` دوّار يوميًا، الاحتفاظ بآخر 7 ملفات، مستوى Info افتراضي، Debug عبر متغير بيئة.
- **ممنوع كتابة محتوى أي عنصر حافظة في السجل** — فقط: نوع العنصر، حجمه، القرار الذي اتُخذ (stored/skipped-dup/skipped-excluded/skipped-big).
- تصنيف الأخطاء (`CoreError`):
  - `Transient` (حافظة مشغولة، قاعدة مقفولة لحظيًا) → إعادة محاولة محدودة.
  - `Degradable` (فشل لصق تلقائي) → استمرار + Event للمستخدم.
  - `Fatal` (تلف قاعدة/تعذر فتح الملف) → رسالة واضحة + وضع آمن: نقل الملف التالف وإعادة إنشاء جديدة (البيانات القديمة تُحفظ للمحاولة اليدوية).

## ميزانيات الأداء (تُقاس في المراحل 2 و 4 و 7)

| العملية | الميزانية | القياس |
|---|---|---|
| التقاط → إدراج (نص) | p95 < 10ms | سجل زمن داخل العامل |
| فتح اللوحة (مُهيأة) | < 150ms حتى أول رسم | قياس في Spike-2 |
| قائمة 50 من 10k | < 20ms | bench في CLI |
| بحث LIKE من 10k | < 50ms | bench في CLI |
| خمول (Tray + لوحة مُهيأة مخفية) | < 120MB RSS إجمالي (كل عمليات التطبيق) | Task Manager/psutil في Soak |
| إقلاع → جاهز | < 2s | قياس تلقائي في التطوير |

## الأمان والخصوصية (ملزم)

1. فحص أعلام الاستبعاد قبل أي تخزين (ثلاثة تنسيقات).
2. استبعاد التطبيقات حسب اسم العملية (المالك من `GetClipboardOwner` → PID → اسم exe).
3. **صفر شبكة:** `cargo-deny` + مراجعة شجرة تبعيات الإصدار للتأكد من عدم وجود مكتبة شبكات؛ يوثق ذلك كبند إغلاق في المرحلة 8.
4. السجلات نظيفة من المحتوى (قاعدة أعلاه).
5. قاعدة البيانات محلية تحت حساب المستخدم فقط؛ الاستثناء المعروف والمقبول (موثق): التشفير عند الخمول غير مفعل افتراضيًا (D10).

## قابلية التوسع للحزمة الكاملة (Tool Host)

- القشرة تُبنى من البداية كـ"مضيف أقسام": اليوم قسم واحد (Clipboard)، والأقسام القادمة تُضاف كـ:
  - قائمة فرعية في Tray + قسم في نافذة الإعدادات + أوامر Core + أحداث.
- **الميزة الثانية مؤكدة ومخططة** (سطوع الشاشات + تتبع وقت التطبيقات — `plans/04-FEATURE-2-display-and-usage.md`، المراحل 9–11):
  - **Core:** وحدات جديدة `features/display` و `features/usage` بنفس نمط `clipboard/` — لا تغيير في مبادئ الخيوط: كل I/O بطيء في خيطه العامل (VCP ≈ 40ms ⇒ خيط شاشات حصري + Debounce).
  - **القاعدة:** جداول أي ميزة جديدة ببادئة مجاله (`usage_*`) في نفس الـ DB ونفس سجل الـ migrations.
  - **الإعدادات:** قسم لكل ميزة (`[display]`, `[usage]`) — الأقسام الحالية (history/slots/privacy) تبقى كما هي ودلالتها الحافظة.
  - **الأحداث:** تُضاف Variants إلى `enum Event` نفسه، ويترجمها `bridge.rs` بأسماء مجال (`monitors-changed`, `usage-changed`) بجانب أسماء v1.
  - **الواجهة:** لا نمط نافذة جديد — Flyout السطوع يعيد استخدام نمط نافذة اللوحة (بلا إطار + positioner).
- لا تغييرات معمارية جذرية للتوسع القادم — إضافات بنفس النمط + بنود الجاهزية الصغيرة الموثقة في نهاية Plan A.
