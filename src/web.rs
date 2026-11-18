//! `pl2pl web`: ディレクトリを調べ、既存の変換器(java_to / cobol_to / 2fastapi / 2poem)へ振り分けて実行する薄い入口。
use std::path::Path;
use std::process::Command;

#[derive(Debug, PartialEq)]
pub struct Job {
    pub label: String,
    pub program: String,
    pub args: Vec<String>,
    pub note: String,
}

pub fn detect(dir: &Path) -> (usize, usize, Vec<&'static str>) {
    // (Java 件数, COBOL 件数, Python のフレームワーク)
    let (mut j, mut c) = (0, 0);
    let mut py: Vec<&'static str> = vec![];
    for (p, _) in crate::util::files(dir, &["java", "cbl", "cob", "cobol", "py"]) {
        let ext = p.extension().map(|x| x.to_string_lossy().to_lowercase()).unwrap_or_default();
        match ext.as_str() {
            "java" => j += 1,
            "cbl" | "cob" | "cobol" => c += 1,
            _ => {
                let t = crate::util::read(&p);
                for (key, name) in [("flask", "flask"), ("django", "django"), ("fastapi", "fastapi")] {
                    let hit = t.lines().any(|l| {
                        let l = l.trim_start();
                        (l.starts_with("import ") || l.starts_with("from ")) && l.to_lowercase().contains(key)
                    });
                    if hit && !py.contains(&name) {
                        py.push(name);
                    }
                }
            }
        }
    }
    (j, c, py)
}

pub fn plan(dir: &Path, out: &Path, target: &str) -> Vec<Job> {
    let (j, c, py) = detect(dir);
    let d = dir.to_string_lossy().to_string();
    let o = |sub: &str| out.join(sub).to_string_lossy().to_string();
    let mut v = vec![];
    if j > 0 {
        v.push(Job { label: format!("Java {j} ファイル"), program: "java_to".into(), args: vec!["convert".into(), d.clone(), "--target".into(), target.into(), "--out".into(), o("java")], note: "https://github.com/aon-co-jp/Java_to".into() });
    }
    if c > 0 {
        v.push(Job { label: format!("COBOL {c} ファイル"), program: "cobol_to".into(), args: vec!["--src".into(), d.clone(), "--target".into(), target.into(), "--out".into(), o("cobol")], note: "https://github.com/aon-co-jp/COBOL_to".into() });
    }
    let poem = target == "rust-poem" || target == "rpoem";
    for fw in py {
        if poem {
            v.push(Job { label: format!("Python/{fw}"), program: "2poem".into(), args: vec!["convert".into(), d.clone(), "-o".into(), o("python"), "--target".into(), if target == "rpoem" { "rpoem".into() } else { "poem".into() }], note: "https://github.com/aon-co-jp/2Poem".into() });
            break; // 2poem はフレームワークを自動判定するので1回でよい
        }
        if fw == "fastapi" {
            continue; // すでに FastAPI
        }
        v.push(Job { label: format!("Python/{fw}"), program: "2fastapi".into(), args: vec!["convert".into(), "--kind".into(), fw.into(), "--path".into(), d.clone(), "-o".into(), o(&format!("python-{fw}")), "--target".into(), target.into()], note: "https://github.com/aon-co-jp/2FastAPI".into() });
    }
    v
}

/// 同梱の変換器を探す。pl2pl の実行ファイルと同じ場所(`<名前>`)か `components/bin/` にあればそれを使い、
/// 無ければ PATH 上の名前をそのまま返す(従来どおり)。
fn resolve_in(dirs: &[std::path::PathBuf], name: &str) -> std::path::PathBuf {
    let file = if cfg!(windows) { format!("{name}.exe") } else { name.to_string() };
    for d in dirs {
        for c in [d.join(&file), d.join("components").join("bin").join(&file)] {
            if c.is_file() {
                return c;
            }
        }
    }
    std::path::PathBuf::from(name)
}

fn resolve_program(name: &str) -> std::path::PathBuf {
    let dirs: Vec<std::path::PathBuf> = std::env::current_exe().ok().and_then(|e| e.parent().map(|p| vec![p.to_path_buf()])).unwrap_or_default();
    resolve_in(&dirs, name)
}

pub fn run(dir: &Path, out: &Path, target: &str, dry: bool) -> i32 {
    let jobs = plan(dir, out, target);
    if jobs.is_empty() {
        println!("pl2pl web: 変換できるもの(Java / COBOL / Flask・Django)が見つかりませんでした: {}", dir.display());
        return 1;
    }
    let mut fail = 0;
    for j in &jobs {
        println!("[{}] {} {}", j.label, j.program, j.args.join(" "));
        if dry {
            continue;
        }
        match Command::new(resolve_program(&j.program)).args(&j.args).status() {
            Ok(s) if s.success() => {}
            Ok(s) => {
                eprintln!("  失敗(終了コード {:?})", s.code());
                fail += 1;
            }
            Err(_) => {
                eprintln!("  `{}` が見つかりません。インストールしてください: {}", j.program, j.note);
                fail += 1;
            }
        }
    }
    if fail > 0 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bundled_converter_is_preferred() {
        let d = std::env::temp_dir().join(format!("pl2pl_res_{}", std::process::id()));
        std::fs::create_dir_all(d.join("components").join("bin")).unwrap();
        let file = if cfg!(windows) { "java_to.exe" } else { "java_to" };
        assert_eq!(resolve_in(&[d.clone()], "java_to"), std::path::PathBuf::from("java_to")); // 無ければ PATH の名前
        std::fs::write(d.join("components").join("bin").join(file), "x").unwrap();
        assert_eq!(resolve_in(&[d.clone()], "java_to"), d.join("components").join("bin").join(file));
        std::fs::write(d.join(file), "x").unwrap();
        assert_eq!(resolve_in(&[d.clone()], "java_to"), d.join(file)); // 実行ファイルの隣が先
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn plans_by_content() {
        let d = std::env::temp_dir().join(format!("pl2pl_web_{}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        std::fs::write(d.join("A.java"), "class A {}").unwrap();
        std::fs::write(d.join("p.cbl"), "PROGRAM-ID. P.").unwrap();
        std::fs::write(d.join("app.py"), "from flask import Flask\n").unwrap();
        let jobs = plan(&d, Path::new("out"), "fastapi");
        let progs: Vec<&str> = jobs.iter().map(|j| j.program.as_str()).collect();
        assert_eq!(progs, vec!["java_to", "cobol_to", "2fastapi"]);
        let poem = plan(&d, Path::new("out"), "rpoem");
        assert!(poem.iter().any(|j| j.program == "2poem" && j.args.contains(&"rpoem".to_string())));
        std::fs::remove_dir_all(&d).ok();
    }
}
