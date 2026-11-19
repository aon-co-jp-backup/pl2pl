//! `pl2pl watch`: 言語・フレームワークの新版を毎晩/毎朝チェックし、通し試験が通れば自動で追従、
//! 通らなければ version-control(aruaru-llm 等のエージェント)で直し、それでも駄目なら登録済みのメール/LINEへ通知する。
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Eco {
    pub name: String,
    pub source: String,
    pub package: String,
    pub tested: String,
    pub verify: String,
}

pub fn parse_registry(t: &str) -> Vec<Eco> {
    let mut v: Vec<Eco> = vec![];
    for l in t.lines() {
        let l = l.trim();
        if l.starts_with('#') || l.is_empty() {
            continue;
        }
        if l == "[[eco]]" {
            v.push(Eco::default());
            continue;
        }
        if let (Some(e), Some((k, val))) = (v.last_mut(), l.split_once('=')) {
            let val = val.trim().to_string();
            match k.trim() {
                "name" => e.name = val,
                "source" => e.source = val,
                "package" => e.package = val,
                "tested" => e.tested = val,
                "verify" => e.verify = val,
                _ => {}
            }
        }
    }
    v
}

fn json_field(body: &str, key: &str) -> Option<String> {
    let re = regex::Regex::new(&format!(r#""{key}"\s*:\s*"([^"]+)""#)).ok()?;
    re.captures(body).map(|c| c[1].to_string())
}

fn get(url: &str) -> Result<String, String> {
    ureq::get(url).set("User-Agent", "pl2pl-watch").timeout(std::time::Duration::from_secs(30)).call().map_err(|e| e.to_string())?.into_string().map_err(|e| e.to_string())
}

pub fn latest(e: &Eco) -> Result<String, String> {
    let (url, key) = match e.source.as_str() {
        "pypi" => (format!("https://pypi.org/pypi/{}/json", e.package), "version"),
        "crates" => (format!("https://crates.io/api/v1/crates/{}", e.package), "max_stable_version"),
        "npm" => (format!("https://registry.npmjs.org/{}/latest", e.package), "version"),
        "github" => (format!("https://api.github.com/repos/{}/releases/latest", e.package), "tag_name"),
        s => return Err(format!("未対応の source: {s}")),
    };
    let v = json_field(&get(&url)?, key).ok_or_else(|| "版の項目が見つかりません".to_string())?;
    let l = v.to_lowercase();
    if ["alpha", "beta", "rc", "dev", "preview"].iter().any(|w| l.contains(w)) {
        return Err(format!("prerelease: {v}"));
    }
    Ok(v)
}

/// "v12.3.1" / "0.115.6" → "12.3" / "0.115"
pub fn major_minor(v: &str) -> String {
    let v = v.trim_start_matches(|c: char| !c.is_ascii_digit());
    let mut it = v.split(|c: char| !c.is_ascii_digit());
    let a = it.next().unwrap_or("0");
    let b = it.next().filter(|s| !s.is_empty()).unwrap_or("0");
    format!("{a}.{b}")
}

pub fn drifted(tested: &str, latest: &str) -> bool {
    major_minor(tested) != major_minor(latest)
}

fn run_sh(cmd: &str, dir: &Path) -> bool {
    if cmd.trim().is_empty() {
        return true;
    }
    let mut it = cmd.split_whitespace();
    let prog = it.next().unwrap();
    Command::new(prog).args(it).current_dir(dir).status().map(|s| s.success()).unwrap_or(false)
}

fn rewrite_tested(t: &str, name: &str, new: &str) -> String {
    let mut out = vec![];
    let mut cur = false;
    for l in t.lines() {
        let tr = l.trim();
        if let Some(n) = tr.strip_prefix("name =") {
            cur = n.trim() == name;
        }
        if cur && tr.starts_with("tested =") {
            out.push(format!("tested = {new}"));
        } else {
            out.push(l.to_string());
        }
    }
    out.join("\n") + "\n"
}

pub struct Opts {
    pub registry: PathBuf,
    pub repo: PathBuf,
    pub dry: bool,
    pub vc: Option<PathBuf>, // version-control の実行ファイル
}

/// 戻り値: 0=異常なし(または自動追従)、1=人の確認が必要(通知した)
pub fn run(o: &Opts) -> i32 {
    let text = std::fs::read_to_string(&o.registry).unwrap_or_else(|_| include_str!("../registry/ecosystem.toml").to_string());
    let mut text_new = text.clone();
    let mut unresolved: Vec<String> = vec![];
    for e in parse_registry(&text) {
        let lat = match latest(&e) {
            Ok(l) => l,
            Err(err) if err.starts_with("prerelease") => {
                println!("[{}] 安定版ではないので見送り ({err})", e.name);
                continue;
            }
            Err(err) => {
                println!("[{}] 最新版を取得できません: {err}", e.name);
                unresolved.push(format!("{}: 最新版を取得できません ({err})", e.name));
                continue;
            }
        };
        if !drifted(&e.tested, &lat) {
            println!("[{}] 最新 {lat} = 試験済み {} (変更なし)", e.name, e.tested);
            continue;
        }
        println!("[{}] 新版 {lat}(試験済み {})", e.name, e.tested);
        if o.dry {
            continue;
        }
        if e.verify.trim().is_empty() {
            println!("  確認コマンドが未登録のため未検証(試験済み版は更新しません。registry の verify を登録してください)");
            continue;
        }
        let mut ok = run_sh(&e.verify, &o.repo);
        if !ok {
            if let Some(vc) = &o.vc {
                println!("  通し試験が失敗 → version-control で修正を試みます");
                let goal = format!("{} {lat} に pl2pl の変換結果が対応できるよう直す。通し試験 `{}` を通す。未対応は TODO で報告する。", e.name, e.verify);
                let _ = Command::new(vc).args(["--loop", "--only", "pl2pl", "--goal", &goal]).current_dir(vc_home(vc)).status();
                ok = run_sh(&e.verify, &o.repo);
            }
        }
        if ok {
            println!("  確認済み → 試験済み版を {} に更新", major_minor(&lat));
            text_new = rewrite_tested(&text_new, &e.name, &major_minor(&lat));
        } else {
            unresolved.push(format!("{}: 新版 {lat} で通し試験が通りません(自動修正でも解決せず)", e.name));
        }
    }
    if !o.dry && text_new != text {
        let _ = std::fs::write(&o.registry, text_new);
    }
    if unresolved.is_empty() {
        return 0;
    }
    println!("人の確認が必要:\n  {}", unresolved.join("\n  "));
    if !o.dry {
        notify(o, &unresolved);
    }
    1
}

/// version-control の config.toml は実行ファイルの 2 つ上(target/release/ の親)にある前提。無ければ今のフォルダ。
fn vc_home(vc: &Path) -> PathBuf {
    vc.canonicalize().ok().and_then(|p| p.ancestors().nth(3).map(Path::to_path_buf)).filter(|d| d.join("config.toml").is_file()).unwrap_or_else(|| PathBuf::from("."))
}

fn notify(o: &Opts, items: &[String]) {
    let f = std::env::temp_dir().join(format!("pl2pl-watch-{}.txt", std::process::id()));
    let body = format!("[pl2pl] 言語・フレームワークの新版に自動対応できませんでした\n{}\n\n対応: pl2pl の registry/ecosystem.toml と変換器を確認してください。", items.join("\n"));
    let _ = std::fs::write(&f, body);
    match &o.vc {
        Some(vc) => match Command::new(vc).arg("--notify-file").arg(&f).current_dir(vc_home(vc)).status() {
            Ok(s) if s.success() => println!("登録済みのメール/LINEへ通知しました"),
            _ => eprintln!("通知に失敗しました(version-control の [notify] 設定を確認)"),
        },
        None => eprintln!("version-control が見つからないため通知できません(--vc で指定)"),
    }
    let _ = std::fs::remove_file(&f);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn versions_and_registry() {
        assert_eq!(major_minor("v12.3.1"), "12.3");
        assert_eq!(major_minor("0.115.6"), "0.115");
        assert!(!drifted("3.1", "3.1.9"));
        assert!(drifted("3.1", "3.2.0"));
        let r = parse_registry(include_str!("../registry/ecosystem.toml"));
        assert!(r.len() >= 8 && r[0].name == "fastapi" && r[0].source == "pypi");
        let t = rewrite_tested("name = a\ntested = 1.0\nname = b\ntested = 2.0\n", "b", "2.1");
        assert!(t.contains("tested = 1.0") && t.contains("tested = 2.1"));
    }
}
