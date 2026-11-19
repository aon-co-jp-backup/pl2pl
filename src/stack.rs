//! COBOL(など)の変換結果に、組み合わせを選んだ「アプリの骨組み」を足す。
//! 選べるもの: backend(rust-axum / rust-poem / fastapi / mfastapi / rpoem / rporea) x frontend(bootstrap-ts / rust-wasm / none) x db(none / mysql / mariadb / postgres / sqlite)。
//! 変換器(cobol_to)の出力は、Rust なら `src/lib.rs` の `PROGRAMS`、Python なら `app.py` の `run_program` を持つ。ここはそれを包むだけで、COBOL の中身は変えない。
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Stack {
    pub backend: String,
    pub frontend: String,
    pub db: String,
}

pub const BACKENDS: &[&str] = &["rust-axum", "rust-poem", "fastapi", "mfastapi", "rpoem", "rporea"];
pub const FRONTENDS: &[&str] = &["bootstrap-ts", "rust-wasm", "none"];
pub const DBS: &[&str] = &["none", "mysql", "mariadb", "postgres", "sqlite"];

impl Stack {
    pub fn new(backend: &str, frontend: Option<&str>, db: Option<&str>) -> Result<Stack, String> {
        if !BACKENDS.contains(&backend) {
            return Err(format!("backend は {} から選んでください", BACKENDS.join(" | ")));
        }
        let wasm_only = matches!(backend, "rpoem" | "rporea");
        let python = backend.ends_with("fastapi");
        let frontend = frontend.unwrap_or(if wasm_only { "rust-wasm" } else { "bootstrap-ts" });
        let db = db.unwrap_or("none");
        if !FRONTENDS.contains(&frontend) {
            return Err(format!("frontend は {} から選んでください", FRONTENDS.join(" | ")));
        }
        if !DBS.contains(&db) {
            return Err(format!("db は {} から選んでください", DBS.join(" | ")));
        }
        if wasm_only && frontend == "bootstrap-ts" {
            return Err("rpoem / rporea は Bootstrap・TypeScript を使いません(rust-wasm か none を選んでください)".into());
        }
        if python && frontend == "rust-wasm" {
            return Err("Python の backend で rust-wasm は選べません(bootstrap-ts か none)".into());
        }
        Ok(Stack { backend: backend.into(), frontend: frontend.into(), db: db.into() })
    }
    /// cobol_to / java_to に渡す `--target`
    pub fn converter_target(&self) -> &'static str {
        match self.backend.as_str() {
            "fastapi" => "fastapi",
            "mfastapi" => "mfastapi",
            "rpoem" | "rporea" => "rpoem",
            _ => "rust-poem", // axum も poem も同じ Rust の登録表(PROGRAMS)から作る
        }
    }
    fn is_python(&self) -> bool {
        self.backend.ends_with("fastapi")
    }
}

fn w(out: &Path, rel: &str, s: &str) {
    let p = out.join(rel);
    if let Some(d) = p.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    let _ = std::fs::write(p, s);
}

fn sqlx_feature(db: &str) -> &'static str {
    match db {
        "mysql" | "mariadb" => "mysql",
        "postgres" => "postgres",
        _ => "sqlite",
    }
}

fn db_url(db: &str) -> &'static str {
    match db {
        "mysql" | "mariadb" => "mysql://user:password@127.0.0.1:3306/app", // MariaDB は MySQL 互換の接続
        "postgres" => "postgres://user:password@127.0.0.1:5432/app",
        _ => "sqlite://app.db",
    }
}

const SCHEMA: &str = "-- COBOL のプログラムの実行記録(例)。必要に応じて直してください。\nCREATE TABLE IF NOT EXISTS program_runs (\n  id BIGINT PRIMARY KEY AUTO_INCREMENT,\n  program VARCHAR(64) NOT NULL,\n  output TEXT,\n  ran_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP\n);\n";
const SCHEMA_SQLITE: &str = "CREATE TABLE IF NOT EXISTS program_runs (\n  id INTEGER PRIMARY KEY AUTOINCREMENT,\n  program TEXT NOT NULL,\n  output TEXT,\n  ran_at TEXT DEFAULT CURRENT_TIMESTAMP\n);\n";
const SCHEMA_PG: &str = "CREATE TABLE IF NOT EXISTS program_runs (\n  id BIGSERIAL PRIMARY KEY,\n  program VARCHAR(64) NOT NULL,\n  output TEXT,\n  ran_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP\n);\n";

const CSS: &str = "body { font-family: system-ui, sans-serif; }\n#out { white-space: pre-wrap; background: #f6f8fa; padding: 1rem; border-radius: .5rem; min-height: 6rem; }\n";

const HTML: &str = r#"<!doctype html>
<html lang="ja"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>COBOL → Web</title>
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5/dist/css/bootstrap.min.css">
<link rel="stylesheet" href="style.css"></head>
<body class="container py-4">
<h1 class="h3">COBOL プログラムを実行 / Run a COBOL program</h1>
<div class="input-group my-3"><input id="name" class="form-control" placeholder="program (例: hello)"><button id="go" class="btn btn-primary">実行 / Run</button></div>
<pre id="out"></pre>
<script src="app.js"></script></body></html>
"#;

const TS: &str = "// Connect 形式: POST /cobol_to.Program/Run  {\"program\": \"名前\"}\nconst go = document.getElementById('go') as HTMLButtonElement;\nconst name = document.getElementById('name') as HTMLInputElement;\nconst out = document.getElementById('out') as HTMLElement;\ngo.onclick = async () => {\n  const r = await fetch('/cobol_to.Program/Run', { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ program: name.value }) });\n  const j = await r.json();\n  out.textContent = j.output ?? JSON.stringify(j);\n};\n";

const JS: &str = "const go = document.getElementById('go');\nconst name = document.getElementById('name');\nconst out = document.getElementById('out');\ngo.onclick = async () => {\n  const r = await fetch('/cobol_to.Program/Run', { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ program: name.value }) });\n  const j = await r.json();\n  out.textContent = j.output ?? JSON.stringify(j);\n};\n";

const TSCONFIG: &str = "{ \"compilerOptions\": { \"target\": \"es2020\", \"lib\": [\"es2020\", \"dom\"], \"strict\": true, \"outDir\": \".\" }, \"files\": [\"app.ts\"] }\n";

const AXUM_MAIN: &str = r#"use axum::{routing::post, Json, Router};
use serde_json::{json, Value};
use tower_http::services::ServeDir;

async fn run(Json(b): Json<Value>) -> Json<Value> {
    let name = b["program"].as_str().unwrap_or("");
    match cobol_to_out::PROGRAMS.iter().find(|(n, _)| *n == name) {
        Some((_, f)) => Json(json!({ "output": f() })),
        None => Json(json!({ "code": "not_found" })),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/cobol_to.Program/Run", post(run)).fallback_service(ServeDir::new("web"));
    let l = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(l, app).await.unwrap();
}
"#;

const POEM_MAIN: &str = r#"use poem::{endpoint::StaticFilesEndpoint, handler, listener::TcpListener, post, web::Json, Route, Server};
use serde_json::{json, Value};

#[handler]
fn run(Json(b): Json<Value>) -> Json<Value> {
    let name = b["program"].as_str().unwrap_or("");
    match cobol_to_out::PROGRAMS.iter().find(|(n, _)| *n == name) {
        Some((_, f)) => Json(json!({ "output": f() })),
        None => Json(json!({ "code": "not_found" })),
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/cobol_to.Program/Run", post(run)).nest("/", StaticFilesEndpoint::new("web").index_file("index.html"));
    Server::new(TcpListener::bind("127.0.0.1:8080")).run(app).await
}
"#;

const RPOEM_MAIN: &str = r#"// RPoem / RPoRea の入口。サーバーへの登録は各リポジトリの手順に従い、登録表 cobol_to_out::PROGRAMS を呼ぶ。
// 外部の CALL が未登録のプログラムなどで落ちても、ほかのプログラムは続けて実行する。
fn main() {
    for (n, f) in cobol_to_out::PROGRAMS {
        match std::panic::catch_unwind(f) {
            Ok(out) => println!("== {n} ==\n{out}"),
            Err(_) => println!("== {n} == (実行できませんでした。上のメッセージを確認してください)"),
        }
    }
}
"#;

const PY_MAIN: &str = "from fastapi import FastAPI\nfrom fastapi.staticfiles import StaticFiles\nfrom app import run_program\n\nweb = FastAPI()\n\n\n@web.post(\"/cobol_to.Program/Run\")\ndef run(body: dict):\n    return run_program(body)\n\n\nweb.mount(\"/\", StaticFiles(directory=\"web\", html=True), name=\"web\")\n# 起動: uvicorn main:web\n";

/// 骨組みを `out` に書く。書いたファイル数を返す。
pub fn scaffold(out: &Path, s: &Stack) -> usize {
    let mut n = 0;
    let mut put = |rel: &str, c: &str| {
        w(out, rel, c);
        n += 1;
    };
    if s.db != "none" {
        put("db/schema.sql", match sqlx_feature(&s.db) {
            "postgres" => SCHEMA_PG,
            "sqlite" => SCHEMA_SQLITE,
            _ => SCHEMA,
        });
        put(".env.example", &format!("DATABASE_URL={}\n", db_url(&s.db)));
    }
    match s.frontend.as_str() {
        "bootstrap-ts" => {
            put("web/index.html", HTML);
            put("web/style.css", CSS);
            put("web/app.ts", TS);
            put("web/app.js", JS); // tsc が無くても動くよう、同じ中身の素の JS も置く
            put("web/tsconfig.json", TSCONFIG);
        }
        "rust-wasm" => {
            let html = HTML.replace("<script src=\"app.js\"></script>", "<script type=\"module\" src=\"app.js\"></script>").replace("  <link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/bootstrap@5/dist/css/bootstrap.min.css\">\n", "").replace("<link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/bootstrap@5/dist/css/bootstrap.min.css\">\n", "");
            put("web/index.html", &html);
            put("web/style.css", CSS);
            put("web/app.js", JS);
            put("web/README.md", "RPoem 方針: 画面は Rust→WASM で作る(Bootstrap・TypeScript は使いません)。ここは最小の HTML+CSS+JS で、WASM 化は RPoem 側の手順に従います。\n");
        }
        _ => {}
    }
    if s.is_python() {
        let extra = match s.db.as_str() {
            "none" => "",
            "mysql" | "mariadb" => "sqlalchemy\npymysql\n",
            "postgres" => "sqlalchemy\npsycopg[binary]\n",
            _ => "sqlalchemy\n",
        };
        let cur = std::fs::read_to_string(out.join("requirements.txt")).unwrap_or_default();
        let sep = if cur.is_empty() || cur.ends_with('\n') { "" } else { "\n" };
        put("requirements.txt", &format!("{cur}{sep}{extra}"));
        put("main.py", PY_MAIN);
        return n;
    }
    // Rust 系
    let feat = sqlx_feature(&s.db);
    let (web_dep, main_rs) = match s.backend.as_str() {
        "rust-axum" => ("axum = \"*\"\ntower-http = { version = \"*\", features = [\"fs\"] }\nserde_json = \"*\"\n", AXUM_MAIN),
        "rust-poem" => ("poem = { version = \"*\", features = [\"static-files\"] }\nserde_json = \"*\"\n", POEM_MAIN),
        "rporea" => ("# RPoRea: path = \"../RPoRea\" (リポジトリ aon-co-jp/RPoRea)\n", RPOEM_MAIN),
        _ => ("# RPoem: path = \"../RPoem\" (リポジトリ aon-co-jp/RPoem)\n", RPOEM_MAIN),
    };
    let mut deps = String::new();
    if matches!(s.backend.as_str(), "rust-axum" | "rust-poem") {
        deps.push_str("tokio = { version = \"*\", features = [\"full\"] }\n");
    }
    deps.push_str(web_dep);
    if s.db != "none" {
        deps.push_str(&format!("sqlx = {{ version = \"*\", features = [\"runtime-tokio\", \"{feat}\"] }}\n"));
    }
    let cargo = std::fs::read_to_string(out.join("Cargo.toml")).unwrap_or_default();
    put("Cargo.toml", &cargo_with(&cargo, &deps));
    put("src/main.rs", main_rs);
    n
}

/// 変換器が作った Cargo.toml に、[lib]・[[bin]] と依存を足す(`[dependencies]` の中身は保つ)。
fn cargo_with(cargo: &str, deps: &str) -> String {
    let mut pre = vec![];
    let mut old_deps = vec![];
    let mut in_deps = false;
    for l in cargo.lines() {
        let t = l.trim();
        if t.starts_with('[') {
            in_deps = t == "[dependencies]";
            if in_deps {
                continue;
            }
        }
        if in_deps {
            old_deps.push(l.to_string());
        } else {
            pre.push(l.to_string());
        }
    }
    let mut s = pre.join("\n").trim_end().to_string();
    s.push_str("\n\n[lib]\nname = \"cobol_to_out\"\npath = \"src/lib.rs\"\n\n[[bin]]\nname = \"app\"\npath = \"src/main.rs\"\n\n[dependencies]\n");
    s.push_str(&old_deps.join("\n"));
    s.push('\n');
    s.push_str(deps);
    s
}

/// C / C++ / Fortran の「実行できるプログラム」(main か PROGRAM を持つファイル)を集める。(名前, 言語, 相対パス)
pub fn legacy_programs(dir: &Path) -> Vec<(String, &'static str, String)> {
    let mut v = vec![];
    for (p, rel) in crate::util::files(dir, &["c", "cc", "cpp", "cxx", "f", "for", "f77", "f90", "f95", "f03", "f08"]) {
        let ext = p.extension().map(|x| x.to_string_lossy().to_lowercase()).unwrap_or_default();
        let t = crate::util::read(&p);
        let (lang, is_main) = match ext.as_str() {
            "c" => ("c", t.contains("main(") || t.contains("main (")),
            "cc" | "cpp" | "cxx" => ("cpp", t.contains("main(") || t.contains("main (")),
            _ => ("fortran", t.lines().any(|l| l.trim_start().to_lowercase().starts_with("program "))),
        };
        if !is_main {
            continue;
        }
        let rels = rel.to_string_lossy().replace(char::from(92), "/");
        let name: String = rels.rsplit('.').nth(1).unwrap_or("prog").replace(['/', '.', '-', ' '], "_");
        v.push((name, lang, rels));
    }
    v
}

/// C/C++/Fortran をそのまま(書き換えずに)ビルドして、選んだ backend から呼べる形にする橋渡し。標準出力を返す。
pub fn scaffold_legacy(out: &Path, src: &Path, progs: &[(String, &'static str, String)], s: &Stack) -> usize {
    let mut build = String::from("#!/bin/sh\n# 元のソースをそのままビルドする(gcc / g++ / gfortran が必要)\nset -e\ncd \"$(dirname \"$0\")\"\nmkdir -p bin\n");
    for (name, lang, rel) in progs {
        let from = src.join(rel);
        let to = out.join("legacy").join(rel);
        if let Some(d) = to.parent() {
            let _ = std::fs::create_dir_all(d);
        }
        let _ = std::fs::copy(&from, &to);
        let cc = match *lang { "c" => "gcc", "cpp" => "g++", _ => "gfortran" };
        build.push_str(&format!("{cc} -O2 legacy/{rel} -o bin/{name}\n"));
    }
    w(out, "build.sh", &build);
    let mut n = 1 + progs.len();
    if s.is_python() {
        let mut app = String::from("import subprocess, os\n\nBASE = os.path.dirname(os.path.abspath(__file__))\nPROGRAMS = {\n");
        for (name, _, _) in progs {
            app.push_str(&format!("    \"{name}\": \"bin/{name}\",\n"));
        }
        app.push_str("}\n\n\ndef run_program(body: dict) -> dict:\n    p = PROGRAMS.get(body.get(\"program\"))\n    if p is None:\n        return {\"code\": \"not_found\"}\n    r = subprocess.run([os.path.join(BASE, p)], capture_output=True, text=True, timeout=30)\n    return {\"output\": r.stdout, \"exit\": r.returncode}\n");
        w(out, "app.py", &app);
        w(out, "requirements.txt", "fastapi\nuvicorn\n");
    } else {
        let mut lib = String::from("use std::process::Command;\n\nfn run(bin: &str) -> String {\n    match Command::new(bin).output() {\n        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),\n        Err(e) => format!(\"error: {e} (build.sh を先に実行してください)\"),\n    }\n}\n\npub const PROGRAMS: &[(&str, fn() -> String)] = &[\n");
        for (name, _, _) in progs {
            lib.push_str(&format!("    (\"{name}\", || run(\"bin/{name}\")),\n"));
        }
        lib.push_str("];\n");
        w(out, "src/lib.rs", &lib);
        w(out, "Cargo.toml", "[package]\nname = \"legacy-web\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n");
    }
    n += scaffold(out, s);
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn combos() {
        assert!(Stack::new("rust-axum", None, Some("mariadb")).is_ok());
        assert!(Stack::new("fastapi", Some("bootstrap-ts"), Some("mysql")).is_ok());
        assert!(Stack::new("rpoem", Some("bootstrap-ts"), None).is_err());
        assert!(Stack::new("fastapi", Some("rust-wasm"), None).is_err());
        assert!(Stack::new("cobol", None, None).is_err());
        assert_eq!(Stack::new("rporea", None, None).unwrap().frontend, "rust-wasm");
        assert_eq!(Stack::new("rust-axum", None, None).unwrap().converter_target(), "rust-poem");
    }
    #[test]
    fn legacy_detect() {
        let d = std::env::temp_dir().join("pl2pl_legacy_t");
        let _ = std::fs::create_dir_all(&d);
        std::fs::write(d.join("a.c"), "int main(void){return 0;}").unwrap();
        std::fs::write(d.join("lib.c"), "int f(void){return 1;}").unwrap();
        std::fs::write(d.join("h.f90"), "program h
print *, 1
end program h
").unwrap();
        let v = legacy_programs(&d);
        assert_eq!(v.len(), 2);
    }
    #[test]
    fn cargo_deps_gathered() {
        let c = cargo_with("[package]\nname = \"x\"\n\n[dependencies]\n# c\n", "axum = \"*\"\n");
        assert!(c.contains("[lib]") && c.trim_end().ends_with("axum = \"*\""));
        assert_eq!(c.matches("[dependencies]").count(), 1);
    }
}
