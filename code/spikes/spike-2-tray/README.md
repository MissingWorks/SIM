# Spike-2 — Tray + prewarm (المرحلة 0)

**الغرض:** قياس زمن إظهار لوحة Tray (on-demand مقابل prewarm) والإجابة عن U04/U05/U16.
**النتيجة الكاملة:** `../../../plan/plans/phase0-results.md` (قسم Spike-2) — مكتمل 2026-10-18.
**القرار:** `prewarm_panel = true` (فتح 33.6–60.9ms بدل ~750ms بارد، بتكلفة +13.2MB).

## المُكدَّس

- Tauri v2 (2.12.x) + `tauri-plugin-positioner` 2.4 + نافذة بلا إطار + إخفاء عند Blur.

## التشغيل

1. `.\make_icon.ps1` — يولّد `icons/icon.ico` (متعدد الأحجام) من `assets/icons/logo-dark.png`.
2. `cargo build --release`
3. `run-prewarm.cmd` أو `run-demand.cmd` — يشغّل الوضع المقصود (سجل في `spike2-log.txt` داخل المجلد).
4. نقرة على أيقونة الصينية: تفتح/تغلق اللوحة. يمين → Quit.

## الأدلة المحفوظة

- `prewarm-log.txt` و `demand-log.txt` — سجلات الجلستين المقاستين على جهاز المستخدم.

> مجلد تجارب مؤقت — لا يدخل الإصدار النهائي (يُحذف أو يؤرشف عند إغلاق المرحلة 0).
