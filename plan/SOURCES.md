# المصادر (Sources)

> كل مصدر استُخدم لتبرير قرار أو فهم مشكلة. يُضاف مصدر جديد عند اعتماد أي فكرة.
> الملاحظات: ما استُخلص من المصدر مكتوب بجانبه — المصدر بلا خلاصة لا يُضاف.

## 1. أبحاث السوق — لماذا مدير الحافظة أولًا؟

| المصدر | ما يثبته |
|---|---|
| The Register — [10 Windows features that would actually help productivity](https://www.theregister.com/software/2025/08/16/10-windows-features-that-would-actually-help-productivity/264083) | "حافظات متعددة" (Multiple Clipboards) مطلب موثق — يؤكد ميزة اللوحات؛ كما يذكر صراحة أنه لا يعرف مدير حافظة يدعم عدة حافظات منفصلة → تفردنا |
| MakeUseOf — [الوظائف التي نساها مايكروسوفت](https://www.makeuseof.com/free-apps-for-windows-features-microsoft-forgot-to-build/) | ويندوز بلا مدير حافظة حقيقي؛ كيف يبدو الحل الجيد |
| xda-developers — [أدوات مفقودة من ويندوز](https://www.xda-developers.com/microsoft-should-upgrade-windows-utilities/) | ضعف أدوات النظام (Storage/Compare/Clipboard) — طلبات مجتمعية متكررة |
| ريديت r/windowsapps — [أداة ويندوز تمنيت وجودها](https://www.reddit.com/r/windowsapps/comments/1wmdljk/what_tiny_windows_tool_do_you_wish_existed/) | طلب أدوات صغيرة native غير ثقيلة — يبرر فلسفة "خفيف" |
| ريديت r/Windows10 — [برامج تتمنى وجودها](https://www.reddit.com/r/Windows10/comments/1jhou6b/what_windows_software_doyou_wish_existed/) | طلبات نظامية (تحكم كامل، خفة، بلا Bloat) — يبرر الحزمة ككل |
| ريديت r/AppIdeas — [تحليل 9,300 طلب تطبيق](https://www.reddit.com/r/AppIdeas/comments/1q5mecg/i_analyzed_9300_i_wish_there_was_an_app_for_this/) | 7% يطلبون أدوات محلية/خصوصية (Anti-Cloud) — يبرر فلسفة Offline-first |
| Markethunt — [تحليل نفس البيانات](https://markethunt.io/insights/reddit-market-validation-analysis) | "الإنتاجية" أعلى طلب و"الاشتراكات" أكبر إزعاج — يبرر تطبيق مجاني محلي + رؤية الحزمة |
| PCWorld — [5 ميزات غائبة عن ويندوز 11](https://www.pcworld.com/article/2817639/these-5-missing-windows-11-features-are-the-most-wanted-among-users.html) | التحكم الكامل ناقص — يبرر "مركز التحكم" |
| Windows Central — [ما ينقص ويندوز 11 حسب المجتمع](https://www.windowscentral.com/microsoft/windows-11/windows-11-should-have-these-features-already-according-to-the-community) | النظام ثقيل ومغلق — يبرر حزمة تحكم خفيفة من طرف ثالث |

## 2. دراسة المنافسين (لبناء خط الأساس — راجع GOAL)

| المصدر | الخلاصات المعتمدة |
|---|---|
| MakeUseOf — [جربت 6 مديري حافظة وكتبت المقارنة](https://www.makeuseof.com/ditto-open-source-clipboard-manager-windows/) | سلوك مستخدم حقيقي يريد: سجل دائم بعد الريستارت، 500 عنصر، بحث فوري، F3 معاينة كاملة، تعديل قبل اللصق، مجموعات Ctrl+G، اختصارات مخصصة — قائمة ميزات خط الأساس |
| edgedrop — [Ditto vs CopyQ vs Win+V](https://www.edgedrop.app/blog/ditto-vs-copyq-vs-win-plus-v-which-should-you-use) | جدول مقارنة دقيق (السعة، بعد الريستارت، الاستبعادات، المزامنة): Win+V = 25 عنصر و4MB حد لكل عنصر؛ Ditto = SQLite محلي + شبكة؛ CopyQ = تبويبات + سكربتات. **نقطة مهمة: CopyQ يحترم تنسيق "Clipboard Viewer Ignore"** → أضفناه لأعلام الاستبعاد |
| Slant — [Ditto vs CopyQ](https://www.slant.co/versus/18557/20261/~ditto_vs_copyq) | عيوب موثقة عند المنافسين: Ditto لا يدعم HDPI جيدًا وتدهور صيانته؛ CopyQ واجهته مربكة والبحث غير مباشر → مساحة التفوق لنا |
| allthings.how — [أفضل 8 مديري حافظة](https://allthings.how/8-best-clipboard-managers-for-windows/) | ميزات Ditto الإضافية: Special Paste (حالات نص)، ترتيب حر، إحصائيات → Backlog |
| xda-developers — [مديرو حافظة يغيرون طريقة عملك](https://www.xda-developers.com/clipboard-managers-windows-that-change-how-you-work/) | تعليقات مستخدمين عن استهلاك الذاكرة (ClipClip 120-200MB مقابل Ditto 5-10MB) → يعزز ميزانية الذاكرة لدينا |
| GitHub — [sabrogden/Ditto](https://github.com/sabrogden/Ditto) | 7.3k نجمة / 787 مشكلة مفتوحة — شعبية عالية وصيانة متباطئة = فرصة سوقية |
| GitHub — [hluk/CopyQ](https://www.github.com/hluk/CopyQ) | 11.9k نجمة — منافس جدي؛ أفضل ما فيه (التبويبات، الاستبعادات) نستوعبه بمفهوم اللوحات |
| منتدى appinn — [نقاش مديري الحافظة](https://meta.appinn.net/t/topic/67777) | طلبات مستخدمين متقدمين: استبعاد حسب المصدر، معالجة محتوى (Upper/QR)، تصنيف تلقائي، خفة — تؤكد خطتنا في v1.1 |

## 3. مصادر ويندوز التقنية (MS Learn)

| المصدر | الاستخدام في تصميمنا |
|---|---|
| [Clipboard](https://learn.microsoft.com/en-us/windows/win32/dataxchg/clipboard) | قائمة دوال الحافظة الكاملة — مصدر خارطة APIs في ARCHITECTURE |
| [Standard Clipboard Formats](https://learn.microsoft.com/en-us/windows/win32/dataxchg/standard-clipboard-formats) | الجدول الرسمي (CF_UNICODETEXT=13، CF_DIB=8، CF_DIBV5=17، CF_HDROP=15) — أساس أولوية القراءة |
| [AddClipboardFormatListener](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-addclipboardformatlistener) | آلية الالتقاط الأساسية (بدون Hook للكيبورد) |
| [WM_CLIPBOARDUPDATE](https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-clipboardupdate) | الرسالة التي يتلقاها خيط المستمع |
| [GetClipboardOwner](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclipboardowner) | منع حلقة نسخنا + تحديد عملية المالك للاستبعاد |
| [SendInput](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput) | اللصق التلقائي |
| [User Interface Privilege Isolation](https://learn.microsoft.com/en-us/windows/win32/secauthn/user-interface-privilege-isolation) | سبب فشل الإرسال للنوافذ المرفوعة — أساس سياسة التدهور الرشيق |
| [Run and RunOnce Registry Keys](https://learn.microsoft.com/en-us/windows/win32/setupapi/run-and-runonce-registry-keys) | مسار autostart الاحتياطي |

## 4. مصادر Tauri v2 (معتمدة في Plan A)

| المصدر | ما استُخلص |
|---|---|
| [System Tray (دليل رسمي)](https://v2.tauri.app/learn/system-tray/) | `TrayIconBuilder`، `show_menu_on_left_click(false)`، أحداث Click/Enter/Move/Leave — أساس تصميم أيقونتنا |
| [Positioner Plugin (دليل رسمي)](https://v2.tauri.app/plugin/positioner/) | `Position::TrayCenter`، شرط `on_tray_event` الإلزامي، ميزة `tray-icon` — يحسم نصف مشكلة U04 |
| [Global Shortcut Plugin (دليل رسمي)](https://v2.tauri.app/plugin/global-shortcut/) | `with_shortcuts` + `ShortcutState` + `isRegistered` (كشف التعارض — D11) + صلاحيات capabilities |
| [plugins-workspace/positioner (GitHub)](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/positioner) | تأكيد دعم Windows رسميًا + أمثلة الاستخدام |
| Stuffbucket — [Tauri v2 Menu-Bar App Pattern](https://stuffbucket.github.io/skills/catalog/tauri-tray-menu-bar-app/) | الوصفة الكاملة: نافذة بلا إطار/شفافة/alwaysOnTop/skipTaskbar، TrayCenter، إخفاء عند Blur، تحذير انكماش أيقونة الـ Tray تحت سهم "الزائد" على ويندوز (→ بند U16) |
| Codegiz — [Menu Bar Timer in Tauri 2](https://www.codegiz.com/blog/tauri-patterns-episode-10-build-a-menu-bar-timer-in-tauri-2/) | تأكيد سلوك `MouseButtonState::Up` (تجنب الوميض) وإعدادات النافذة الخمسة |
| Ramon Claudio — [Building Gitbar](https://ramonclaudio.com/posts/building-gitbar/) | مشكلة `fit_to_monitor` عند تعدد الشاشات و DPI (تقسيم على scale factor) — أساس بند U12 |
| dev.to — [Menubar App with Tauri v2](https://dev.to/hiyoyok/building-a-menubar-app-with-tauri-v2-what-nobody-tells-you-2nae) | صيغة الحساب اليدوي للتموضع تحت الأيقونة (Fallback إن فشل positioner) |

## 5. مصادر مكتبات الحافظة (المقارنة الفنية — D4)

| المصدر | ما استُخلص |
|---|---|
| docs.rs — [clipboard-win: get_owner](https://docs.rs/clipboard-win/latest/clipboard_win/raw/fn.get_owner.html) | `get_owner() -> Option<NonNull<c_void>>` موثقة — أساس منع الحلقات |
| docs.rs — [clipboard-win: CF_HDROP](https://docs.rs/clipboard-win/latest/clipboard_win/formats/constant.CF_HDROP.html) | دعم قوائم الملفات معلن |
| docs.rs — [clipboard-win: المصدر الكامل (raw)](https://docs.rs/clipboard-win/latest/src/clipboard_win/raw.rs.html) | `get_file_list` (DragQueryFileW) و`get_bitmap` (BITMAPINFO) و`get_string` (UTF-16→UTF-8) — تحقق قراءة الكود لا الوعد فقط |
| docs.rs — [arboard: Clipboard](https://docs.rs/arboard/latest/arboard/struct.Clipboard.html) | تحذيرات مهمة: الحافظة على ويندوز كائن عام وفتح متوازٍ يفشل بـ ClipboardOccupied → التصميم بأمر واحد (Actor) صحيح |
| docs.rs — [arboard: مصدر ويندوز الكامل](https://docs.rs/crate/arboard/latest/source/src/platform/windows.rs) | كتابة CF_HDROP + أعلام `exclude_from_monitoring/cloud/history` — دليل قدرة الكتابة بأعلام الاستبعاد |
| SQLite (منافس ضمني) — Win+V | حد 25 عنصر و4MB للعنصر الواحد بعد الريستارت يمسح — فجوة نملؤها |

## 6. مصادر التخزين SQLite (المبنى عليها D5)

| المصدر | ما استُخلص |
|---|---|
| SQLite — [Internal Versus External BLOBs](https://sqlite.org/intern-v-extern-blob.html) | **الرقم الحاكم:** أقل من 100KB داخل القاعدة أسرع؛ أكبر من 100KB في ملفات أسرع؛ صفحة 8192/16384 الأفضل للـ BLOB |
| SQLite — [35% Faster Than The Filesystem](https://sqlite.org/fasterthanfs.html) | BLOBs الصغيرة (10KB) أسرع 35% من ملفات؛ وعلى ويندوز مع AV: القراءة من القاعدة أسرع 5x |
| SQLite Forum — [تخزين صور 2MB](https://sqlite.org/forum/forumpost/e2b8d0cb727b10b6?t=c) | اعتراف المطورين أنه فوق ~2MB الملفات غالبًا أفضل → عتبتنا 96KB متحفظة وبآمان |
| SQLite Forum — [تجربة تخزين ملفات وصور](https://sqlite.org/forum/forumpost/7702be1a335f95f3?raw=) | النسخ الاحتياطي/النقل أسهل مع ملفات منفصلة — لكن عندنا تصدير ZIP يذلل هذا |
| ريديت r/sqlite — [آراء تخزين ملفات في SQLite](https://www.reddit.com/r/sqlite/comments/1coipqw/opinions_on_storing_files_in_sqlite/) | قاعدة التقريب ~50KB + الخلاصة: قِس على جهازك (→ Spike-3) |

## 7. مصادر إضافية مرجعية

- [Standard Clipboard Formats — جدول مختصر بالمعنى](https://learn.microsoft.com/en-us/windows/win32/dataxchg/clipboard-formats) — شامل أعلام الاستبعاد (ExcludeClipboardContentFromMonitorProcessing / CanIncludeInClipboardHistory).
- Tauri Capabilities Overview — لتوثيق صلاحيات الإضافات (نحتاجها في المرحلة 1 و 6).

> ملاحظة إدارة: عند إضافة مصدر جديد — يُرفق بخلاصة من فائدة واحدة على الأقل وإلا لا يُضاف.

## 8. مصادر الميزة الثانية (السطوع + تتبع الاستخدام)

| المصدر | ما استُخلص |
|---|---|
| GitHub — [xanderfrangos/twinkle-tray](https://github.com/xanderfrangos/twinkle-tray) | تأكيد الفجوة: ويندوز لا يدعم سطوع الشاشات الخارجية أصلًا؛ يستخدم DDC/CI + WMI؛ أخطاء معروفة نوثقها في واجهتنا: AMD Radeon "Custom Colors" يعطب، VGA/DVI قد لا يعمل، دوكات USB/Thunderbolt/سيرفس غالبًا تفصل DDC، NVIDIA Surround/AMD Eyefinity غير متوافقتين، أغلب التلفزيونات لا تدعم |
| Twinkle Tray Wiki — [Display Detection & Support Issues](https://github.com/xanderfrangos/twinkle-tray/wiki/Display-Detection-&-Support-Issues) | 8 أسباب موثقة لعدم الدعم + طرق الاكتشاف المتعددة (WMIC / WMI-Bridge / DisplayConfig) — أساس صفحات "لماذا شاشتي غير مدعومة؟" لدينا |
| DeepWiki — [Twinkle Tray: Monitor Management](https://deepwiki.com/xanderfrangos/twinkle-tray/3-monitor-management) | المعمارية المرجعية: خيط منفصل للاتصال بالشاشات، تطبيع 0..100، سلسلة بدائل كود السطوع (0x10→0x13→0x6B→0x12)، Skip-list لشاشات مشاكسة، حفظ آخر القيم لإعادة التوصيل، استجابة لأحداث الفصل/السبات — خط أساس تصميمي نعتمده |
| Twinkle Tray — [إصدار v1.16.0](https://github.com/xanderfrangos/twinkle-tray/releases/tag/v1.16.0) | قراءة أكواد VCP المدعومة من الـ capabilities؛ تحذير متكرر بالتحقق الفيزيائي من كل شاشة (نتخذه قاعدة إلزامية في F2-1) |
| MS Learn — [Monitor Configuration](https://learn.microsoft.com/en-us/windows/win32/api/_monitor/) + [Using the Low-Level Functions](https://learn.microsoft.com/en-us/windows/win32/monitor/using-the-low-level-monitor-configuration-functions) | خارطة النفاذ: EnumDisplayMonitors → GetPhysicalMonitorsFromHMONITOR → Capabilities → Get/SetVCPFeature — الثلاث خطوات المرجعية لمسار التنفيذ |
| MS Learn — [GetVCPFeatureAndVCPFeatureReply](https://learn.microsoft.com/en-us/windows/win32/api/lowlevelmonitorconfigurationapi/nf-lowlevelmonitorconfigurationapi-getvcpfeatureandvcpfeaturereply) | **الرقم الحاكم:** كل نداء ≈ 40ms + تحذير "لا تستخدمها على شاشات عشوائية بلا تحقق" ⇒ خيط منفصل + Debounce + إظهار المخاطر بصدق |
| MS Learn — [SetWinEventHook](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwineventhook) + [Event Constants](https://learn.microsoft.com/en-us/windows/win32/winauto/event-constants) | `EVENT_SYSTEM_FOREGROUND` (0x0003) يُرسل دائمًا عند تغير نافذة المقدمة؛ `WINEVENT_OUTOFCONTEXT` غير متزامن وآمن بلا DLL ⇒ أساس خيط التتبع |
| The Old New Thing — [Logging the foreground process](https://devblogs.microsoft.com/oldnewthing/20131202-00/?p=2503) | الوصفة الرسمية: GetWindowThreadProcessId → OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION) → QueryFullProcessImageName — نعتمدها حرفيًا |
| StackOverflow — [SetWinEventHook مقابل Polling](https://stackoverflow.com/questions/41059514/performance-cost-of-setwineventhook-vs-polling-in-c) | الخطاف أدق من Polling للنوافذ؛ الخمول عبر GetLastInputInfo + مؤقت أرخص بكثير من خطافات الإدخال |
| crates.io / docs.rs — [ddc-hi](https://crates.io/crates/ddc-hi) | v0.4.1 (2021) MIT؛ `Display::enumerate` + `get_vcp_feature`/`set_vcp_feature`؛ backends: ddc-winapi + nvapi — مرشحنا الأول مع مسار سقوط موثق |
| GitHub — [HarmanPreet-Singh-XYT/Scolect-ScreenTimeApp](https://github.com/HarmanPreet-Singh-XYT/Scolect-ScreenTimeApp) | تتبع المقدمة عبر API أصلية (C++ ثم Rust في v2.1.2)؛ تخزين محلي (Hive)؛ ميزات بنينا منها نطاقنا: Today، تصنيفات، إخفاء، إيقاف — وحدود/Alerts أبقيناها Backlog |
| MakeUseOf — [تطبيقات ويندوز مفتوحة مهملة (Scolect)](https://www.makeuseof.com/underrated-open-source-windows-apps/) | "ويندوز لا يقدم قياس إنتاجية/وقت استخدام" — تأكيد الفجوة السوقية لقسم Activity |
| docs.rs — [Tauri SubmenuBuilder](https://docs.rs/tauri/latest/tauri/menu/struct.SubmenuBuilder.html) | القوائم الفرعية مدعومة رسميًا في قوائم Tauri (بما فيها Tray) — أساس D15/D18 (يُتحقق عمليًا في F2-2/U26) |

## 9. مصادر جولة الفحص الشامل (مشاكل موثقة وحلولها — 2026-10-18)

| المصدر | ما استُخلص |
|---|---|
| MS Learn — [Using the Clipboard](https://learn.microsoft.com/en-us/windows/win32/dataxchg/using-the-clipboard) | الطرق الثلاث لمراقبة الحافظة؛ الرسمي: "استخدم Clipboard Format Listener أو رقم التسلسل — **ليس polling**" — تأكيد أن مستمعنا هو المسار الرسمي الصحيح |
| Raymond Chen (2025) — [Clipboard History تفوت التغييرات السريعة](https://devblogs.microsoft.com/oldnewthing/20250508-00/?p=111162) | خدمة Win+V غير متزامنة وتفوت النسخ السريع المتتالي؛ مستمعنا + رقم التسلسل يلتقط أكثر منها — ميزة صغيرة تفاضلية وسبب لعدم الاعتماد عليها |
| MS Learn — [OpenClipboard](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openclipboard) + Raymond Chen — [locked out of the clipboard](https://devblogs.microsoft.com/oldnewthing/20240410-00/?p=109632) | "OpenClipboard يفشل إن كانت نافذة أخرى تمسك الحافظة"؛ `GetOpenClipboardWindow` يكشف المتسبب (تشخيص CLIPBRD_E_CANT_OPEN) — وفرقها عن GetClipboardOwner |
| Delphi-Praxis — [رسائل WM_CLIPBOARDUPDATE الزائفة](https://en.delphipraxis.net/topic/11631-delphi-clipboard-wm_clipboardupdate-sent-but-no-changes/) | أدوات نظام (رصد فعلي: MMC) تستدعي `SetClipboardViewer(0)` فتطلق تحديثات بلا تغيّر حقيقي؛ الحل الموثق: مقارنة `GetClipboardSequenceNumber` لكل رسالة (D21) |
| dotnet/winforms PR [#14271](https://github.com/dotnet/winforms/pull/14271) + StackOverflow — [blocked clipboard](https://stackoverflow.com/questions/930219/how-to-handle-a-blocked-clipboard-and-other-oddities) | إعادة المحاولة القياسية 10×100ms (نمط .NET) لفك CLIPBRD_E_CANT_OPEN — سقف توسيع مقترح لـ Spike-6؛ وGetOpenClipboardWindow للتشخيص |
| MS Learn — [SetForegroundWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow) + SO — [ForegroundLockTimeout على Win10+](https://stackoverflow.com/questions/73735129/setforegroundwindow-relationship-between-the-foregroundlocktimeout-registry-val) | القيود الرسمية؛ على Win10+ القيمة الافتراضية تعادل **تعطيل الرفع المبرمج** إلا بشروط؛ الحل الشائع الموثق: خدعة ضغطة ALT قبل إعادة المحاولة (D20) |
| Raymond Chen — [خطر AttachThreadInput](https://devblogs.microsoft.com/oldnewthing/20080801-00/?p=21393) | تجميد موثق عند ربط قوائم الإدخال — نمنعه صراحة في سلسلة اللصق (D20) |
| MS Learn — [Using low-level monitor functions](https://learn.microsoft.com/en-us/windows/win32/monitor/using-the-low-level-monitor-configuration-functions) + [GetPhysicalMonitorsFromHMONITOR](https://learn.microsoft.com/en-us/windows/win32/api/physicalmonitorenumerationapi/nf-physicalmonitorenumerationapi-getphysicalmonitorsfromhmonitor) + [Q&A: مقبض الشاشة النائمة](https://learn.microsoft.com/en-us/answers/questions/207421/how-to-get-the-handle-and-information-of-sleep-mon) | التسلسل الرسمي + `DestroyPhysicalMonitors` إلزامي + **HMONITOR يُبطل بمجرد خروج الشاشة من سطح المكتب (WM_DISPLAYCHANGE)** ⇒ إعادة تعداد دائمًا وبلا كاش مقابض دائم |
| StackOverflow — [ERROR_GRAPHICS_I2C_ERROR_TRANSMITTING_DATA](https://stackoverflow.com/questions/62598348/unable-to-get-the-physical-monitor-handle-on-windows-7-for-color-calibration) | رمز الفشل الشائع لـ DDC/CI (ناقل I2C) — متوقع عمليًا ويُصنّف "غير مدعومة/متعذرة" مع سبب مقروء |
| microsoft/PowerToys — [MonitorDiscoveryHelper.cs](https://github.com/microsoft/PowerToys/blob/92014c81/src/modules/powerdisplay/PowerDisplay.Lib/Drivers/DDC/MonitorDiscoveryHelper.cs) + وحدة [PowerDisplay](https://github.com/microsoft/PowerToys) | تنفيذ مرجعي MIT حيّ: **فلترة مقابض NULL** من GetPhysicalMonitorsFromHMONITOR + تسجيل حالة العدد صفر — درس مباشر لوحدة display لدينا؛ وجود PowerDisplay الرسمي يرفع سقف الجودة المطلوب |
| Twinkle Tray Issues — [#1238](https://github.com/xanderfrangos/twinkle-tray/issues/1238) (دوك USB يعطّل الاكتشاف كله) + [#1258](https://github.com/xanderfrangos/twinkle-tray/issues/1258) (كارت استخراج DDC) + [#1233](https://github.com/xanderfrangos/twinkle-tray/issues/1233) (DP غير مستقر) + [#1180](https://github.com/xanderfrangos/twinkle-tray/issues/1180) (بيئات تحتاج مسؤول) + [#1160](https://github.com/xanderfrangos/twinkle-tray/issues/1160) (GSP من NVIDIA) | **الدرس المركزي: عزل الفشل لكل شاشة** — شاشة واحدة فاشلة لا تُسقط التعداد كله (U29)؛ نستبعد صلاحيات المسؤول (D7) ونوثق الحد؛ مشاكل التعريفات حقيقية وتُدرج في صفحة "لماذا غير مدعومة؟" |
| Tauri Issues — [#8869](https://github.com/tauri-apps/tauri/issues/8869) (نقرة Tray × إخفاء-on-Blur = وميض) + [#12936](https://github.com/tauri-apps/tauri/issues/12936) (single-instance: التركيز يفشل على نافذة مخفية — أظهر أولًا) + [wry fix لتنقل TAB](https://github.com/tauri-apps/wry/commit/0185644040184c43084814cb8692acb4e1004d86) | حواجز إلزامية: حارس زمني/حالة بين الإخفاء وToggle (المرحلة 4)، show قبل set_focus (المرحلة 1)، وإبقاء Tauri/wry محدّثين (إصلاحات تركيز فعلية) |
| fluux-messenger — [حل استعادة تركيز WebView2](https://github.com/processone/fluux-messenger/blob/58345900/apps/fluux/src/hooks/useTauriFocusRestore.ts) | على ويندوز: Alt-Tab يعيد تركيز الإطار لكن WebView2 لا يأخذ تركيز لوحة المفاتيح؛ الحل الموثق: `webview.setFocus()` عند Focused(true) — ضروري لواجهة كيبورد-first (المرحلة 4) |
