use rusqlite::{params, Connection};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

const N_IMAGES: usize = 1000;
const MIN_SIZE: usize = 50 * 1024;
const MAX_SIZE: usize = 5 * 1024 * 1024;
const THUMB_SIZE: usize = 12 * 1024;
const N_TEXTS: usize = 10_000;
const BATCH: usize = 100;

struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed | 1)
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
    fn fill(&mut self, buf: &mut [u8]) {
        let mut i = 0;
        while i < buf.len() {
            let v = self.next_u64().to_le_bytes();
            let n = (buf.len() - i).min(8);
            buf[i..i + n].copy_from_slice(&v[..n]);
            i += n;
        }
    }
}

struct Log {
    f: File,
}
impl Log {
    fn new(path: &str) -> Self {
        Log {
            f: File::create(path).expect("log file"),
        }
    }
    fn line(&mut self, s: &str) {
        println!("{s}");
        let _ = writeln!(self.f, "{s}");
        let _ = self.f.flush();
    }
}

fn ms(d: std::time::Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

fn image_size(rng: &mut Rng) -> usize {
    (MIN_SIZE as u64 + rng.below((MAX_SIZE - MIN_SIZE + 1) as u64)) as usize
}

fn blob_hash(i: usize) -> String {
    format!("{:016x}", (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15))
}

fn open_db(p: &Path) -> Connection {
    let c = Connection::open(p).expect("open db");
    c.execute_batch(
        "PRAGMA page_size=8192;
         PRAGMA auto_vacuum=INCREMENTAL;
         PRAGMA journal_mode=WAL;
         PRAGMA synchronous=NORMAL;
         PRAGMA cache_size=-16000;
         PRAGMA mmap_size=268435456;
         PRAGMA temp_store=MEMORY;
         PRAGMA busy_timeout=5000;",
    )
    .expect("pragmas");
    c
}

fn ddl(mode_blob: bool, c: &Connection) {
    let items = if mode_blob {
        "CREATE TABLE items(
            id INTEGER PRIMARY KEY,
            hash TEXT NOT NULL,
            size INTEGER NOT NULL,
            created INTEGER NOT NULL,
            data BLOB NOT NULL,
            thumb BLOB NOT NULL
        );"
    } else {
        "CREATE TABLE items(
            id INTEGER PRIMARY KEY,
            hash TEXT NOT NULL,
            size INTEGER NOT NULL,
            created INTEGER NOT NULL,
            path TEXT NOT NULL,
            thumb BLOB NOT NULL
        );"
    };
    c.execute_batch(&format!(
        "{items}
         CREATE INDEX idx_items_created ON items(created DESC);
         CREATE TABLE texts(
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            created INTEGER NOT NULL
         );
         CREATE INDEX idx_texts_created ON texts(created DESC);"
    ))
    .expect("ddl");
}

fn make_text(i: usize, rng: &mut Rng) -> String {
    let words_ar = ["مشروع", "خطأ", "شبكة", "ملف", "صورة", "نسخ", "لصق", "حافظة"];
    let words_en = ["error", "timeout", "connection", "clipboard", "image", "file", "paste", "copy"];
    let mut s = String::with_capacity(300);
    s.push_str(&format!("Record {i}: "));
    let n = 8 + rng.below(12) as usize;
    for k in 0..n {
        if k % 2 == 0 {
            s.push_str(words_en[rng.below(words_en.len() as u64) as usize]);
        } else {
            s.push_str(words_ar[rng.below(words_ar.len() as u64) as usize]);
        }
        s.push(' ');
    }
    s.push_str(&format!("— item {i} — token{i:05}"));
    s
}

fn insert_texts(c: &mut Connection, log: &mut Log, label: &str) -> f64 {
    let mut rng = Rng::new(77);
    let t = Instant::now();
    let tx = c.transaction().expect("tx");
    {
        let mut st = tx
            .prepare("INSERT INTO texts(content, created) VALUES(?1, ?2)")
            .expect("prep");
        for i in 0..N_TEXTS {
            let content = make_text(i, &mut rng);
            st.execute(params![content, 1_700_000_000i64 + i as i64])
                .expect("ins text");
        }
    }
    tx.commit().expect("commit");
    let d = ms(t.elapsed());
    log.line(&format!("  [{label}] texts insert x{N_TEXTS}: {d:.1}ms"));
    d
}

fn insert_images_blob(c: &mut Connection, log: &mut Log) -> f64 {
    let mut rng = Rng::new(1000);
    let t = Instant::now();
    for chunk in 0..(N_IMAGES / BATCH) {
        let tx = c.transaction().expect("tx");
        {
            let mut st = tx
                .prepare("INSERT INTO items(hash, size, created, data, thumb) VALUES(?1, ?2, ?3, ?4, ?5)")
                .expect("prep");
            for k in 0..BATCH {
                let i = chunk * BATCH + k;
                let size = image_size(&mut rng);
                let mut data = vec![0u8; size];
                rng.fill(&mut data);
                let mut thumb = vec![0u8; THUMB_SIZE];
                rng.fill(&mut thumb);
                st.execute(params![
                    blob_hash(i),
                    size as i64,
                    1_700_000_000i64 + i as i64,
                    data,
                    thumb
                ])
                .expect("ins img");
            }
        }
        tx.commit().expect("commit");
    }
    let d = ms(t.elapsed());
    log.line(&format!("  [A] images insert x{N_IMAGES} (blob in db): {d:.1}ms"));
    d
}

fn insert_images_files(c: &mut Connection, blobs: &Path, log: &mut Log) -> f64 {
    let mut rng = Rng::new(1000);
    fs::create_dir_all(blobs).expect("blobs dir");
    let t = Instant::now();
    for chunk in 0..(N_IMAGES / BATCH) {
        let tx = c.transaction().expect("tx");
        {
            let mut st = tx
                .prepare("INSERT INTO items(hash, size, created, path, thumb) VALUES(?1, ?2, ?3, ?4, ?5)")
                .expect("prep");
            for k in 0..BATCH {
                let i = chunk * BATCH + k;
                let size = image_size(&mut rng);
                let mut data = vec![0u8; size];
                rng.fill(&mut data);
                let hash = blob_hash(i);
                let dir = blobs.join(&hash[0..2]);
                fs::create_dir_all(&dir).expect("subdir");
                let path = dir.join(format!("{hash}.bin"));
                fs::write(&path, &data).expect("write blob");
                let mut thumb = vec![0u8; THUMB_SIZE];
                rng.fill(&mut thumb);
                st.execute(params![
                    hash,
                    size as i64,
                    1_700_000_000i64 + i as i64,
                    path.to_string_lossy(),
                    thumb
                ])
                .expect("ins img");
            }
        }
        tx.commit().expect("commit");
    }
    let d = ms(t.elapsed());
    log.line(&format!("  [B] images insert x{N_IMAGES} (files on disk): {d:.1}ms"));
    d
}

fn cold_reads_blob(db: &Path, log: &mut Log) {
    let c = Connection::open(db).expect("reopen");
    for round in ["cold", "warm"] {
        for id in [7i64, 493, 986] {
            let t = Instant::now();
            let mut st = c
                .prepare("SELECT size, data FROM items WHERE id=?1")
                .expect("prep");
            let mut rows = st.query(params![id]).expect("q");
            let row = rows.next().expect("r").expect("some");
            let size: i64 = row.get(0).expect("sz");
            let data: Vec<u8> = row.get(1).expect("data");
            log.line(&format!(
                "  blob read [{round}] id={id} sqlite_size={size} actual={} time={:.2}ms",
                data.len(),
                ms(t.elapsed())
            ));
        }
    }
}

fn cold_reads_files(db: &Path, log: &mut Log) {
    let c = Connection::open(db).expect("reopen");
    for round in ["cold", "warm"] {
        for id in [7i64, 493, 986] {
            let t = Instant::now();
            let path: String = c
                .query_row("SELECT path FROM items WHERE id=?1", params![id], |r| r.get(0))
                .expect("path");
            let data = fs::read(&path).expect("read file");
            log.line(&format!(
                "  file read [{round}] id={id} actual={} time={:.2}ms",
                data.len(),
                ms(t.elapsed())
            ));
        }
    }
}

fn thumbs_list(c: &Connection, log: &mut Log, label: &str) -> f64 {
    let t = Instant::now();
    let mut st = c
        .prepare("SELECT id, size, thumb FROM items ORDER BY created DESC LIMIT 50")
        .expect("prep");
    let mut rows = st.query([]).expect("q");
    let mut count = 0usize;
    let mut bytes = 0usize;
    while let Some(row) = rows.next().expect("row") {
        let _id: i64 = row.get(0).expect("id");
        let _size: i64 = row.get(1).expect("sz");
        let thumb: Vec<u8> = row.get(2).expect("thumb");
        bytes += thumb.len();
        count += 1;
    }
    let d = ms(t.elapsed());
    log.line(&format!(
        "  [{label}] thumbs list x{count} (thumb bytes={bytes}): {d:.2}ms"
    ));
    d
}

fn delete_images(c: &mut Connection, log: &mut Log, mode_files: bool) -> (f64, usize) {
    let ids: Vec<i64> = (0..N_IMAGES as i64).filter(|i| i % 10 == 0).collect();
    let mut paths: Vec<String> = Vec::new();
    if mode_files {
        let mut st = c.prepare("SELECT path FROM items WHERE id=?1").expect("prep");
        for id in &ids {
            if let Ok(p) = st.query_row(params![id], |r| r.get::<_, String>(0)) {
                paths.push(p);
            }
        }
    }
    let t = Instant::now();
    let tx = c.transaction().expect("tx");
    {
        let mut st = tx.prepare("DELETE FROM items WHERE id=?1").expect("prep");
        for id in &ids {
            st.execute(params![id]).expect("del");
        }
    }
    tx.commit().expect("commit");
    let mut deleted_files = 0usize;
    for p in &paths {
        if fs::remove_file(p).is_ok() {
            deleted_files += 1;
        }
    }
    let d = ms(t.elapsed());
    log.line(&format!(
        "  delete x{} rows (+{} files): {d:.1}ms",
        ids.len(),
        deleted_files
    ));
    (d, deleted_files)
}

fn file_size(p: &Path) -> u64 {
    fs::metadata(p).map(|m| m.len()).unwrap_or(0)
}

fn dir_size(p: &Path) -> u64 {
    let mut total = 0u64;
    if let Ok(rd) = fs::read_dir(p) {
        for e in rd.flatten() {
            let path = e.path();
            if path.is_dir() {
                total += dir_size(&path);
            } else {
                total += file_size(&path);
            }
        }
    }
    total
}

fn mb(v: f64) -> f64 {
    v / (1024.0 * 1024.0)
}

fn size_report(c: &Connection, db: &Path, log: &mut Log) {
    let _ = c.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);");
    let db_s = file_size(db) as f64;
    let wal_s = file_size(&PathBuf::from(format!("{}-wal", db.display()))) as f64;
    let page_count: i64 = c
        .query_row("PRAGMA page_count;", [], |r| r.get(0))
        .unwrap_or(0);
    let freelist: i64 = c
        .query_row("PRAGMA freelist_count;", [], |r| r.get(0))
        .unwrap_or(0);
    log.line(&format!(
        "  size: db={:.1}MB wal={:.2}MB pages={page_count} freelist={freelist}",
        mb(db_s),
        mb(wal_s)
    ));
}

fn text_bench(c: &Connection, log: &mut Log, label: &str) {
    let t = Instant::now();
    let mut st = c
        .prepare("SELECT id, content FROM texts ORDER BY created DESC LIMIT 50")
        .expect("prep");
    let mut rows = st.query([]).expect("q");
    let mut count = 0usize;
    let mut bytes = 0usize;
    while let Some(row) = rows.next().expect("row") {
        let _id: i64 = row.get(0).expect("id");
        let content: String = row.get(1).expect("c");
        bytes += content.len();
        count += 1;
    }
    log.line(&format!(
        "  [{label}] texts list x{count} (content bytes={bytes}): {:.2}ms",
        ms(t.elapsed())
    ));

    for (pat_label, pat) in [("like-arabic", "%مشروع%"), ("like-english", "%timeout%")] {
        let t = Instant::now();
        let n: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM texts WHERE content LIKE ?1",
                params![pat],
                |r| r.get(0),
            )
            .expect("like");
        log.line(&format!(
            "  [{label}] {pat_label} count={n}: {:.1}ms",
            ms(t.elapsed())
        ));
    }
}

fn main() {
    let root = PathBuf::from("spike3-data");
    if root.exists() {
        fs::remove_dir_all(&root).expect("clean old");
    }
    fs::create_dir_all(&root).expect("root");

    let mut log = Log::new("spike3-log.txt");
    let started = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    log.line("=== Spike-3 SQLite bench (SIM phase 0) ===");
    log.line(&format!("started epoch_ms={started}"));
    log.line("AV: Windows Defender real-time real (verified externally = True)");
    log.line(&format!(
        "images: {N_IMAGES} sizes {MIN_SIZE}..{MAX_SIZE} bytes (uniform), thumb={THUMB_SIZE} bytes, texts={N_TEXTS}"
    ));
    log.line("");

    // ---------- MODE A: BLOB in DB ----------
    log.line("--- MODE A: image BLOB inside SQLite ---");
    let db_a = root.join("mode-a.db");
    let mut ca = open_db(&db_a);
    let t = Instant::now();
    ddl(true, &ca);
    log.line(&format!("  ddl: {:.2}ms", ms(t.elapsed())));

    let texts_ms_a = insert_texts(&mut ca, &mut log, "A");
    let ins_a = insert_images_blob(&mut ca, &mut log);
    size_report(&ca, &db_a, &mut log);

    log.line("  cold/warm reads (fresh connection):");
    cold_reads_blob(&db_a, &mut log);
    let thumbs_a = thumbs_list(&ca, &mut log, "A");
    let (del_a, _) = delete_images(&mut ca, &mut log, false);
    let t = Instant::now();
    let _ = ca.execute_batch("PRAGMA incremental_vacuum;");
    log.line(&format!("  incremental_vacuum: {:.1}ms", ms(t.elapsed())));
    size_report(&ca, &db_a, &mut log);
    text_bench(&ca, &mut log, "A");
    let _ = ca.execute_batch("PRAGMA optimize;");
    drop(ca);

    log.line("");

    // ---------- MODE B: files on disk ----------
    log.line("--- MODE B: image files on disk ---");
    let db_b = root.join("mode-b.db");
    let blobs = root.join("blobs");
    let mut cb = open_db(&db_b);
    let t = Instant::now();
    ddl(false, &cb);
    log.line(&format!("  ddl: {:.2}ms", ms(t.elapsed())));

    let texts_ms_b = insert_texts(&mut cb, &mut log, "B");
    let ins_b = insert_images_files(&mut cb, &blobs, &mut log);
    size_report(&cb, &db_b, &mut log);
    let blobs_size = dir_size(&blobs) as f64;
    log.line(&format!("  blob dir size: {:.1}MB", mb(blobs_size)));

    log.line("  cold/warm reads (fresh connection):");
    cold_reads_files(&db_b, &mut log);
    let thumbs_b = thumbs_list(&cb, &mut log, "B");
    let (del_b, _) = delete_images(&mut cb, &mut log, true);
    let t = Instant::now();
    let _ = cb.execute_batch("PRAGMA incremental_vacuum;");
    log.line(&format!("  incremental_vacuum: {:.1}ms", ms(t.elapsed())));
    size_report(&cb, &db_b, &mut log);
    let blobs_size_after = dir_size(&blobs) as f64;
    log.line(&format!("  blob dir size after delete: {:.1}MB", mb(blobs_size_after)));
    text_bench(&cb, &mut log, "B");
    let _ = cb.execute_batch("PRAGMA optimize;");
    drop(cb);

    log.line("");
    log.line("=== SUMMARY (ms) ===");
    log.line(&format!("texts insert 10k:      A={texts_ms_a:.1}   B={texts_ms_b:.1}"));
    log.line(&format!("images insert 1000:    A={ins_a:.1}   B={ins_b:.1}"));
    log.line(&format!("thumbs list 50:        A={thumbs_a:.2}   B={thumbs_b:.2}"));
    log.line(&format!("delete 100:            A={del_a:.1}   B={del_b:.1}"));
    log.line("done.");
}
