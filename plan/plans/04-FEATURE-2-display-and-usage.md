# الميزة الثانية (مخططة) — التحكم بإضاءة الشاشات + تتبع وقت التطبيقات

> **الوضع:** مخططة — تُنفَّذ كالمراحل **9–11** بعد إكمال v1 (المراحل 0–8). لا يُبدأ فيها قبل ذلك.
> **العلاقة بالخطط:** المحرك (Core) مشترك مع أي خطة منصة؛ تفاصيل الواجهة هنا تفترض Plan A (Tauri v2) وفروق Plan B محدودة ومذكورة حيث تلزم. لا تكرار لهذه الخطة في A/B/C.
> **قرار المستخدم (2026-10-18):** الميزة كأقسام إضافية في **نفس التطبيق** — قسم في نافذة الإعدادات (من قائمة ابدأ) + قائمة فرعية مقابلة في Tray.

**اسم القسمين في الواجهة:** `Displays` (السطوع) و `Activity` (تتبع الاستخدام).

---

## 1. الأساس السوقي (لماذا هذه الميزة)

| الفجوة | الدليل (التفاصيل في `../SOURCES.md` قسم 8) |
|---|---|
| ويندوز لا يدعم سطوع الشاشات **الخارجية** إطلاقًا | Twinkle Tray حرفيًا: "Windows is capable of adjusting the backlight... it doesn't support external monitors natively" |
| ويندوز بلا Screen Time / تتبع وقت التطبيقات | MakeUseOf عن Scolect: "Windows doesn't actually offer a way to measure how productive you've been" |
| لماذا نحن | نفس التطبيق الواحد: محلي، خفيف، بلا اشتراك ولا سحابة — امتداد مباشر لفلسفة الحزمة (GOAL) وبلا منافسة SaaS |

---

## 2. المتطلبات الوظيفية (نطاق v2)

### 2.1 قسم Displays — سطوع الشاشات الخارجية

| # | المتطلب |
|---|---|
| D1 | اكتشاف كل الشاشات الموصولة تلقائيًا؛ عرض حالة كل شاشة: مدعومة (DDC/CI) / غير مدعومة **مع سبب مقروء** |
| D2 | شريحة سطوع مستقلة لكل شاشة مدعومة + خيار "Adjust all together" |
| D3 | Flyout سطوع سريع من Tray بنمط شريط صوت ويندوز (شرائح + نسبة رقمية) |
| D4 | قائمة Tray الفرعية Displays: Presets (0 / 25 / 50 / 75 / 100٪) + "Brightness…" (يفتح الـ Flyout) |
| D5 | إعادة تعداد تلقائية عند: توصيل/فصل شاشة، تغيير إعدادات العرض، الرجوع من السبات |
| D6 | خطوة ضبط (افتراضي 10٪) + كتابة مكبوتة أثناء السحب (لا رشّ VCP — الأحدث يفوز) |
| D7 | إعادة تطبيق آخر سطوع معروف عند إعادة التوصيل (يُحفظ آخر قيمة لكل شاشة أثناء الجلسة) |

**خارج نطاق v2 (Backlog):** سطوع حسب الوقت/الخمول (Time of Day)، Profiles لكل تطبيق، اختصارات سطوع عامة، تباين/مدخلات/طاقة (أكواد VCP إضافية)، الشاشة الداخلية عبر WMI، HDR.

### 2.2 قسم Activity — تتبع وقت التطبيقات

| # | المتطلب |
|---|---|
| A1 | تسجيل زمن الاستخدام لكل تطبيق في المقدمة (اسم العملية فقط — **بلا عناوين نوافذ** ولا محتوى، افتراضيًا) |
| A2 | لوحة Today: إجمالي اليوم + تطبيقات مرتبة + أشرطة نسب |
| A3 | مدى آخر 7 أيام (أعمدة يومية) من التجميع اليومي |
| A4 | تصنيف يدوي: Productive / Neutral / Leisure + مؤشر إنتاجية بسيط (نسبة وقت العمل) |
| A5 | إخفاء تطبيقات من العرض + استبعاد تطبيقات من التتبع + إيقاف مؤقت شامل |
| A6 | الخمول: إيقاف العد بعد `idle_seconds` (افتراضي 300 ث)؛ القفل والنوم فواصل جلسات |
| A7 | كل البيانات محلية؛ حذف البيانات من الإعدادات؛ صفر شبكة (نفس قواعد v1 الملزمة) |

**خارج نطاق v2 (Backlog):** حدود وتنبيهات زمنية (Limits/Alerts)، حجب تطبيقات، تصدير تقارير، تتبع دومينات المتصفح (يحتاج تعقيد إضافة متصفح — ليس من قيمتنا الآن)، Pomodoro، الوضع الخاص.

---

## 3. كيف تُطبَّق تقنيًا (بناءً على البحث — ملزم)

### 3.1 السطوع — DDC/CI عبر أكواد VCP

ويندوز لا يعرض واجهة، لكنه يفتح البروتوكول عبر **Monitor Configuration API** (موثق في SOURCES قسم 8):

```
EnumDisplayMonitors → GetNumberOfPhysicalMonitorsFromHMONITOR → GetPhysicalMonitorsFromHMONITOR
→ GetCapabilitiesStringLength + CapabilitiesRequestAndCapabilitiesReply   (ما تدعمه الشاشة فعلًا)
→ GetVCPFeatureAndVCPFeatureReply(0x10)  /  SetVCPFeature(0x10, value)    (قراءة/كتابة السطوع)
```

- **كل نداء يستهلك ~40ms (رقم رسمي من مايكروسوفت)** ⇒ إلزامي: خيط مستقل + Debounce "الأحدث يفوز" + عدم لمس خيط الواجهة أبدًا.
- **سلسلة بدائل الكود** إن لم تتوفر 0x10 (نفس ما يفعله Twinkle Tray): `0x13` → `0x6B` → ثم `0x12` (تباين) كملاذ أخير — تُطبق وتُوثق في نتائج F2-1.
- **تطبيع:** القيمة الخام حسب min/max الفعليين للموديل ← 0..100 للواجهة (لا نفترض مدى موحدًا).
- **حالات فشل معروفة نوثقها في واجهة "لماذا شاشتي غير مدعومة؟":** DDC/CI مفصول من OSD الشاشة؛ دوكات USB/Thunderbolt/سيرفس غالبًا تقص DDC؛ AMD Radeon "Custom Colors" يعطب؛ NVIDIA Surround / AMD Eyefinity غير متوافقتين؛ أغلب التلفزيونات لا تدعم؛ كابلات/منافذ — تغيير الكابل يحل أحيانًا.
- **قيود إلزامية مستفادة من فشل المنافسين (دروس Twinkle Tray — SOURCES §9):**
  - **عزل الفشل:** شاشة واحدة فاشلة (دوك USB / كارت استخراج DDC) **لا تُسقط التعداد كله** — قيد تصميمي نقيض مشكلة #1238/#1258؛ كل شاشة تُعالج بمعزل ونتيجتها تظهر بجانبها.
  - **فلترة مقابض NULL** من `GetPhysicalMonitorsFromHMONITOR` (حل PowerToys الموثق) + `DestroyPhysicalMonitors` بعد كل تعداد (صفر تسريب مقابض).
  - **لا كاش مقابض دائم:** HMONITOR يُبطل بمجرد خروج الشاشة من سطح المكتب ⇒ إعادة تعداد عند `WM_DISPLAYCHANGE` و `DEVICECHANGE` ودائمًا عند الطلب.
  - **الأخطاء متوقعة:** `ERROR_GRAPHICS_I2C_ERROR_TRANSMITTING_DATA` وأمثاله = "غير متاحة الآن" برسالة مفهومة — لا انهيار ولا إخفاء صامت.
  - **لا صلاحيات مسؤول** (D7): بعض البيئات تحتاجها (تقرير #1180) — نوثق الحد في صفحة الدعم بدل رفع الصلاحيات.
- **مرجع تنفيذي حيّ:** مايكروسوفت أضافت وحدة **PowerDisplay** إلى PowerToys (مفتوحة المصدر، MIT) — نفس المسار DDC/CI؛ نستأنس بمعالجتها للمقابض NULL، ووجودها يرفع سقف الجودة المتوقع منا.
- **التنفيذ:** كريت `ddc-hi` (MIT) يلف كل ما سبق (عبر `ddc-winapi` + خيار NVAPI). **مسار السقوط المعتمد إن أثبتت F2-1 قصورًا:** غلاف رقيق فوق دوال windows-rs أعلاه مباشرة (D16).
- **التخزين:** لا نحتاج قاعدة للسطوع — الشاشات تحفظ قيمتها بنفسها؛ نحتفظ آخر القيم في الذاكرة فقط لإعادة التوصيل داخل الجلسة (D7).

### 3.2 تتبع الاستخدام — خطاف نافذة المقدمة + الخمول

**الالتقاط (نمط Raymond Chen الموثق في SOURCES):**

```
خيط مخصص بحلقة رسائل
SetWinEventHook(EVENT_SYSTEM_FOREGROUND, EVENT_SYSTEM_FOREGROUND,
                None, callback, 0, 0, WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS)
```

- النظام يرسل الحدث عند **كل** تغير لنافذة المقدمة (حتى داخل نفس العملية) ⇒ لا Polling للنوافذ.
- من `hwnd` الحدث: `GetWindowThreadProcessId` → `OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION)` → `QueryFullProcessImageNameW` → اسم العملية.
- **الخمول:** `GetLastInputInfo` من مؤقت كل 30 ث (أرخص من خطافات الإدخال — موثق).
- **فواصل الجلسات:** قفل/فتح الجلسة (`WTSRegisterSessionNotification` + WM_WTSSESSION_CHANGE) والنوم = إغلاق الجلسة الحالية بأمان.
- **حالات دقيقة يجب معالجتها (بنود U21):** تطبيقات UWP تُستضاف داخل `ApplicationFrameHost.exe` ⇒ ننسب للنافذة الحقيقية (نمط معالجة Scolect نفسه)؛ التطبيقات المرفوعة تعمل (LIMITED_INFORMATION مصمم لذلك)؛ الألعاب Fullscreen-DX قد تقطع الأحداث ⇒ إغلاق جلسة عند انقطاع الحدث ثم استئناف.
- **التجميع:** جلسة = (process, start, end, idle_ms). تُغلق عند تبديل التطبيق/خمول/قفل؛ تُحدَّث `usage_daily` عند الإغلاق + Flush دوري كل 60 ث (أقصى خسارة عند انقطاع كهرباء = 60 ث).

---

## 4. الأقسام في الواجهتين (طلب المستخدم الحرفي)

| المكان | القسم | المحتوى |
|---|---|---|
| نافذة الإعدادات (Start Menu) | Clipboard | (موجود من v1) |
| نافذة الإعدادات | **Displays** | شريحة لكل شاشة + Adjust all + الخطوة + تفعيل Flyout + سبب عدم دعم أي شاشة |
| نافذة الإعدادات | **Activity** | Today + آخر 7 أيام + التصنيفات + المخفي + المستبعد + إيقاف التتبع + حذف البيانات |
| نافذة الإعدادات | General / About | (من v1 — لا تغيير هيكلي) |
| قائمة Tray | **Clipboard** (فرعية) | كما v1 (فتح اللوحة، Pause، إلخ) |
| قائمة Tray | **Displays** (فرعية) | Presets + Brightness… + Settings ← Displays |
| قائمة Tray | **Activity** (فرعية) | سطر عرض "Today: 6h 12m" (عنصر معطّل يُحدَّث) + Open Activity + Pause tracking + Settings ← Activity |

---

## 5. إضافات المعمارية (ملزمة — تُدمج في `../ARCHITECTURE.md` عند بدء المرحلة 9)

**وحدات Core جديدة (بنفس نمط `clipboard/`):**

```
features/display/   — display_worker: تعداد + قراءة/كتابة VCP (خيط وحيد حصري، الأحدث يفوز)
                       + الاستماع لتغيرات الأجهزة (DEVICECHANGE) وإعادة التعداد
features/usage/     — usage_worker: خيط رسائل + الخطاف + مؤقت الخمول + إدارة الجلسات + flush
```

**جداول جديدة (نفس قاعدة v1 — عبر StoreActor):**

```sql
CREATE TABLE IF NOT EXISTS usage_apps (
  process_name TEXT PRIMARY KEY,
  category     TEXT NOT NULL DEFAULT 'neutral',   -- productive|neutral|leisure
  hidden       INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS usage_sessions (
  id           INTEGER PRIMARY KEY,
  process_name TEXT    NOT NULL,
  started_at   INTEGER NOT NULL,                  -- unix millis
  ended_at     INTEGER NOT NULL,
  idle_ms      INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS usage_daily (
  day          TEXT NOT NULL,                     -- 'YYYY-MM-DD' محلي
  process_name TEXT NOT NULL,
  active_ms    INTEGER NOT NULL,
  PRIMARY KEY (day, process_name)
);
CREATE INDEX IF NOT EXISTS usage_sessions_start_idx ON usage_sessions(started_at);
```

- **الاستبقاء:** الجلسات الخام تُقلَّم بعد `retention_raw_days` (افتراضي 90) — التجميع اليومي صغير ويبقى.

**إعدادات جديدة (config.toml):**

```toml
[display]
step_pct = 10
adjust_all = false
flyout_enabled = true
write_debounce_ms = 80        # تجميع كتابات السحب

[usage]
enabled = true
idle_seconds = 300
excluded_apps = []
retention_raw_days = 90
```

**أحداث جديدة (enum Event نفسه):** `MonitorsChanged`، `BrightnessChanged { monitor_id, pct }`، `UsageChanged` (مع Throttling في bridge)، `UsagePaused { paused }`.

**إضافات EngineHandle:**

```rust
pub fn monitors(&self) -> Result<Vec<MonitorInfo>, CoreError>;   // { id, name, backend, supported, min, max, current }
pub fn refresh_monitors(&self) -> Result<(), CoreError>;
pub fn set_brightness(&self, id: &str, pct: u8) -> Result<(), CoreError>;
pub fn set_brightness_all(&self, pct: u8) -> Result<(), CoreError>;

pub fn usage_today(&self) -> Result<Vec<AppUsage>, CoreError>;    // { process, active_ms, category }
pub fn usage_days(&self, days: u8) -> Result<Vec<DayUsage>, CoreError>;
pub fn set_usage_paused(&self, paused: bool) -> Result<(), CoreError>;
pub fn set_app_meta(&self, process: &str, category: Option<&str>, hidden: Option<bool>) -> Result<(), CoreError>;
```

---

## 6. تجارب ما قبل المرحلة 9 (Spike-F2-*)

> ترقيم `F2-*` مستقل عمدًا — لا يتقاطع مع تجارب المنصة 1–8 ولا مع Spike-8 الخاص بـ Plan B.

| # | التجربة | الطريقة | معيار النجاح | يُغلق |
|---|---|---|---|---|
| F2-1 | DDC/CI على أجهزة المستخدم الفعلية | أداة Rust ~150 سطرًا بـ ddc-hi: تعداد → Capabilities → قراءة/كتابة VCP 0x10 على كل شاشة (مباشرة ومع أي دوك) + عدّاد تسريب مقابض (50 تعداد) + محاولة مسار دوك/كارت استخراج إن توفر | جدول لكل شاشة: مدعومة؟ زمن؟ شذوذ؟ + **الفاشلة لا تُسقط الناجحة (عزل)** + صفر مقابض مسربة + قرار نهائي: ddc-hi أم غلاف windows-rs | U19, U20, U29, D16 |
| F2-2 | Flyout الشرائح + Tray | نموذج نافذة بنمط اللوحة: سحب مع Debounce، Adjust-all، إعادة تعداد عند فصل/وصل + حارس النقر المزدوج للقوائم/الـ Flyout | إحساس ناعم بلا وميض، شاشة جديدة تظهر < 3 ث، الواجهة لا تتجمد أبدًا، قائمة Tray الفرعية تعمل مع عنصر "Today" المحدّث، نقرة سريعة لا تسبب إظهار/إخفاء متكررًا | U23, U24, U26, D18 |
| F2-3 | دقة التتبع | تشغيل ساعتي عمل حقيقيتين + سجل يدوي مرجعي: تنقلات كثيرة، تطبيق UWP، تطبيق مرفوع، فيديو/لعبة ملء الشاشة، قفل | تطابق ≥ 95٪ على التنقلات؛ UWP يُنسب للتطبيق الصحيح؛ انقطاع الحدث لا يُفسد الجلسة | U21, U22, D17 |
| F2-4 | التخزين واللوحة | توليد بيانات 90 يومًا (≈ 90×200 جلسة) ثم: الحجم الكلي + زمن استعلام Today و7 أيام والتصنيفات | استعلام Today < 50ms والحجم معقول (< 20MB) | U25 |

---

## 7. المراحل 9–11 (تُضاف لخريطة Plan A — خريطة واحدة لا تنقسم)

| المرحلة | الاسم | المحتوى الأساسي | معايير الخروج (DoD مختصرة) | التقدير |
|---|---|---|---|---|
| 9 | Displays | Spikes F2-1/F2-2 → features/display + أوامر Engine + قائمة Tray الفرعية + Flyout + قسم Displays في الإعدادات | كل شاشات الجهاز: الشريحة تغيّر السطوع فعلًا أو تظهر Unsupported بسبب واضح؛ وصل/فصل شاشة → تحديث < 3 ث؛ صفر تجميد؛ الإعدادات تُحفظ وتُطبق بعد إعادة التشغيل | 1–1.5 أسبوع |
| 10 | Activity | Spikes F2-3/F2-4 → features/usage + الجداول + الخطاف + لوحة Today/7 أيام + التصنيفات والإخفاء + قائمة Tray الفرعية | تطابق ≥ 95٪ مع سجل يدوي مرجعي؛ الخمول والقفل والسبات يعملون؛ 24 ساعة بلا تسريب أو فقد جلسات؛ استعلامات < 50ms | 1–1.5 أسبوع |
| 11 | تصليب وإصدار v2 | Soak 72 ساعة بالسطوع والتتبع معًا + حالات حدية (فصل شاشة أثناء السحب، فشل DDC مفاجئ، تغيير منطقة زمنية/تاريخ، DPI مختلط) + مراجعة RTL للشاشات الجديدة + تغليف v2 + تحديث الوثائق | يعيد نمط المرحلتين 7+8 لكن على v2؛ صفر بند حرج مفتوح جديد | 0.5–1 أسبوع |

**الترتيب الداخلي الملزم لكل مرحلة:**

**مرحلة 9 (Displays) — بهذا التسلسل:**
1. `features/display/display_worker`: تعداد + capabilities + قراءة/كتابة VCP + Debounce "الأحدث يفوز" (خيط وحيد حصري).
2. ربط أحداث الجهاز (`WM_DISPLAYCHANGE` + `DEVICECHANGE`) → إعادة تعداد + فلترة NULL + عزل الفاشلة.
3. أوامر Engine (`monitors`/`refresh_monitors`/`set_brightness`/`set_brightness_all`) + أحداث `MonitorsChanged`/`BrightnessChanged` + ترجمة bridge.
4. قائمة Tray الفرعية Displays (Presets / Brightness… / Settings) — فوق بنية القوائم المؤسسة في المرحلة 1.
5. Flyout نافذة بنمط اللوحة (شرائح + نسبة + Adjust all) مع حارس التركيز (D22).
6. قسم Displays في الإعدادات + صفحة تشخيص "لماذا شاشتي غير مدعومة؟" بأسباب U19/U29 المرصودة.
7. إعادة تشغيل F2-1/F2-2 على الكود الفعلي + إغلاق U19/U20/U23/U24/U26/U29.

**مرحلة 10 (Activity) — بهذا التسلسل:**
1. `features/usage/usage_worker`: خيط رسائل + خطاف المقدمة + مؤقت الخمول + جلسات في الذاكرة.
2. Migration جديد: `usage_apps`/`usage_sessions`/`usage_daily` + فهرس `started_at` (التزام سجل الـ migrations).
3. Flush كل 60 ث + إغلاق الجلسات عند: تبديل/خمول/قفل (WTS)/إغلاق نظيف + قص الخام حسب `retention_raw_days`.
4. أوامر Engine (`usage_today`/`usage_days`/`set_app_meta`/`set_usage_paused`) + أحداث + Throttle في bridge.
5. لوحة Activity: Today → آخر 7 أيام → التصنيفات/الإخفاء/الاستبعاد (حيّان).
6. قائمة Tray الفرعية Activity (سطر Today المحدَّث + Open + Pause).
7. إعادة تشغيل F2-3/F2-4 + إغلاق U21/U22/U25.

**مرحلة 11:** Soak 72h (الكل مفعّل) → الحالات الحدية → جولة RTL/LTR للصفحات الجديدة → تغليف v2 → تحديث الوثائق.

**ملاحظة ترتيب عامة:** تجارب F2-1/F2-2 مستقلتان عن محرك الحافظة — يجوز تنفيذهما في أي وقت بعد المرحلة 1 (قتل مبكر للمخاطر — D19)؛ والافتراضي يبقى بعد v1. ولا اعتماد بين المرحلتين 9 و10 (يمكن عكسهما) — التسلسل الحالي يبني نمط قسم الواجهة الجديد على الأبسط أولًا.

**الإجمالي المتوقع:** 3–4 أسابيع عمل مركّز بعد v1.

---

## 8. المخاطر → بنود U

| الخطر | البند |
|---|---|
| شاشات المستخدم نفسها قد لا تدعم DDC/CI (دوك/كابل/موديل) | U19 |
| `ddc-hi` شبه مجمدة (2021) | U20 |
| دقة نسبة UWP/المرفوع/الألعاب | U21 |
| أثقال الخطاف على يوم عمل كامل | U22 |
| تجميد التعداد عند Hotplug/سبات | U23 |
| إحساس السحب مع زمن VCP (~40ms) | U24 |
| نمو بيانات 90 يومًا | U25 |
| ديناميكيات قائمة Tray (فرعية + عداد يومي) | U26 |
| عزل شاشة فاشلة + مقابض NULL + إبطال HMONITOR بعد WM_DISPLAYCHANGE | U29 |

التفاصيل والمسؤوليات في `../UNVERIFIED.md` — قسم الميزة الثانية.

---

## 9. قواعد لا تتغير

- نفس قواعد v1 ملزمة هنا: صفر شبكة، صفر نصوص hardcoded، كل مكتبة تُسجل في LIBRARIES قبل الاستخدام، كل مرحلة تُحدّث STATUS و UNVERIFIED.
- أي فكرة خارج الجدولين (حدود، HDR، Profiles…) → Backlog المذكور أعلاه، لا تُنفذ قبل اكتمال v2.
