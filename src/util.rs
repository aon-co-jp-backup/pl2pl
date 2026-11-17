//! 共通部品: ファイル収集・書き出し・レポート。
use std::fs;
use std::path::{Path, PathBuf};

pub fn files(input: &Path, exts: &[&str]) -> Vec<(PathBuf, PathBuf)> {
    // (実ファイル, 出力側の相対パス)
    let mut v = vec![];
    if input.is_file() {
        v.push((input.to_path_buf(), PathBuf::from(input.file_name().unwrap())));
        return v;
    }
    let w = walkdir::WalkDir::new(input).into_iter().filter_entry(|e| {
        let n = e.file_name().to_string_lossy();
        !(e.file_type().is_dir() && matches!(n.as_ref(), "target" | ".git" | "node_modules" | "build" | "__pycache__"))
    });
    for e in w.filter_map(|e| e.ok()).filter(|e| e.file_type().is_file()) {
        let ext = e.path().extension().map(|x| x.to_string_lossy().to_lowercase()).unwrap_or_default();
        if exts.iter().any(|x| *x == ext) {
            let rel = e.path().strip_prefix(input).unwrap().to_path_buf();
            v.push((e.path().to_path_buf(), rel));
        }
    }
    v
}

pub fn read(p: &Path) -> String {
    let b = fs::read(p).unwrap_or_default();
    String::from_utf8(b.clone()).unwrap_or_else(|_| b.iter().map(|c| if c.is_ascii() { *c as char } else { '?' }).collect())
}

pub fn write(out: &Path, rel: &Path, content: &str) -> std::io::Result<()> {
    let p = out.join(rel);
    if let Some(d) = p.parent() {
        fs::create_dir_all(d)?;
    }
    fs::write(p, content.replace("\r\n", "\n"))
}

pub fn line_of(src: &str, byte: usize) -> usize {
    src[..byte.min(src.len())].matches('\n').count() + 1
}

pub fn write_report(out: &Path, cmd: &str, rep: &[String]) -> std::io::Result<()> {
    let mut s = format!("# pl2pl レポート / report ({cmd})\n\n項目数 / items: {}\n\n", rep.len());
    for r in rep {
        s.push_str(&format!("- {r}\n"));
    }
    write(out, Path::new("PL2PL-REPORT.md"), &s)
}
