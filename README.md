# SIM — Something Is Missing

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/icons/logo-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="assets/icons/logo-light.svg">
    <img src="assets/icons/logo-light.svg" alt="SIM" width="110">
  </picture>
</p>

حزمة أدوات لويندوز مبنية حول فكرتين:

1. **مركز تحكم** من قائمة ابدأ — للتحكم بإعدادات ويندوز والميزات (يُفتح نادرًا، للضبط).
2. **لوحة سريعة** من صينية النظام (Tray) — لأدوات الاستخدام اليومي.

**أول أداة (v1): مدير حافظة (Clipboard Manager) احترافي** — نصوص + صور + ملفات، سعة قابلة للتحديد، لوحات متعددة، بحث، خصوصية أولًا. **الواجهة إنجليزية أساسًا والعربية خيار ثانٍ** (قرار D12).

**الميزة الثانية (مخططة بعد v1):** قسمان إضافيان في نفس التطبيق — `Displays` (سطوع الشاشات الخارجية — بنمط Twinkle Tray) و `Activity` (تتبع وقت التطبيقات — بنمط Scolect). الخطة: `plan/plans/04-FEATURE-2-display-and-usage.md`.

- **اللغة:** Rust (محسوم — التفاصيل في `plan/DECISIONS.md`)
- **الخطة المعتمدة:** Plan A — Rust + Tauri v2 (`plan/plans/01-PLAN-A-rust-tauri.md`)
- **الحالة الحالية:** المرحلة 0 (التحقق والتصميمات) — راجع `plan/STATUS.md`

## بنية المشروع

```
SIM\
├── plan\                     ← كل التخطيط والمتابعة (Markdown فقط)
│   ├── GOAL.md               ← الهدف والرؤية والمتطلبات
│   ├── DECISIONS.md          ← القرارات التقنية وسببها
│   ├── ARCHITECTURE.md       ← المعمارية والبيانات والخيوط
│   ├── STATUS.md             ← الحالة الحالية (يُحدَّث كل مرحلة)
│   ├── UNVERIFIED.md         ← ما لم يُثبت خلوه من المشاكل (يُحدَّث كل مرحلة)
│   ├── SOURCES.md            ← المصادر والأبحاث
│   ├── LIBRARIES.md          ← المكتبات المستخدمة (APIs محددة وبدائل ومرفوضات)
│   └── plans\
│       ├── README.md             ← ترتيب الخطط ومبرراته
│       ├── phase0-results.md     ← سجل أدلة تجارب المرحلة 0 (يُملأ عند التنفيذ)
│       ├── 01-PLAN-A-rust-tauri.md    ← المعتمدة (المراحل 0–8 لـ v1 + 9–11 للميزة الثانية)
│       ├── 02-PLAN-B-rust-egui.md     ← بديلة
│       ├── 03-PLAN-C-cpp-win32.md     ← للعلم
│       └── 04-FEATURE-2-display-and-usage.md  ← الميزة الثانية: Displays + Activity
├── code\                     ← الكود المصدري (Cargo workspace — يُبنى في المرحلة 1)
├── assets\                   ← الأيقونات والصور والخطوط
├── dist\                     ← النسخ الجاهزة للتوزيع (installer / portable)
└── .gitignore
```

## ترتيب القراءة للمشاركين الجدد

1. `plan/STATUS.md` — أين وصلنا الآن
2. `plan/GOAL.md` — ماذا نبني ولمن
3. `plan/DECISIONS.md` + `plan/ARCHITECTURE.md` — كيف نبني
4. `plan/plans/01-PLAN-A-rust-tauri.md` — خطة التنفيذ بالمراحل
5. `plan/UNVERIFIED.md` — ما الذي يجب الحذر منه في كل مرحلة
6. `plan/plans/04-FEATURE-2-display-and-usage.md` — الميزة الثانية (تُنفذ بعد v1)

## قواعد العمل (إلزامية)

- لا نكتب من الصفر ما له مكتبة جاهزة مجرّبة — راجع `LIBRARIES.md` قبل أي كود.
- كل مكتبة جديدة تُسجل في `LIBRARIES.md`، وكل قرار في `DECISIONS.md`.
- نهاية كل مرحلة: تحديث `STATUS.md` + `UNVERIFIED.md`.
- لا شيء يعتبر "سليمًا" إلا بدليل تجربة موثق.
