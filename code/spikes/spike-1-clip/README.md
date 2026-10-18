# Spike-1 — فحص مكتبة الحافظة (المرحلة 0)

**الغرض:** مصفوفة قراءة/كتابة الحافظة (`clipboard-win` مقابل `arboard` + windows-rs الخام) — يحسم U01 و D4 ويختبر آلية كشف النسخ الذاتي.
**الحالة:** ✅ مكتمل 2026-10-18 — النتائج الكاملة في `../../../plan/plans/phase0-results.md` (قسم Spike-1)، والقرارات: D4 مغلق + D25 (علامة `SIM.ClipMarker`).

## البناء والتشغيل

1. `cargo build --release`
2. المراقب: `run-watch.cmd` — يكتب كل حدث حافظة في `spike1-report.txt` (polling كل 50ms على `GetClipboardSequenceNumber`؛ الإنتاج سيستخدم WM_CLIPBOARDUPDATE لأن الـ polling يدمج الأحداث المتقاربة).
3. أوامر نسخ للاختبار: `spike1-clip.exe copy text-en | text-ar | text-emoji | marked`
   - `marked` = كتابة خام (GlobalAlloc + SetClipboardData) مع علامة `SIMSpikeMarker` — نموذج كشف النسخ الذاتي.

## أدلة الجلسة (محفوظة)

- `spike1-report.txt` — جلسة نسخ فعلية: Steam، Chrome (نص + صورة)، Notepad، Snipping Tool، Explorer (ملفات)، ونسخة كلمة مرور بعلم استبعاد.

> مجلد تجربة مؤقت — لا يدخل الإصدار (يُؤرشف/يُحذف عند إغلاق المرحلة 0).
