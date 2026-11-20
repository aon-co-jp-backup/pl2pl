//! 互換の完成度の台帳(registry/maturity.toml)と、`pl2pl maturity`(完成かどうかの判定)。
//! 判定のしかた: ① probe(試験)を、open-cpu で調べた CPU 数の並列で実行 → 成功なら完成と判定。② aruaru-llm(open-cuda / open-cpu で動く)に参考の意見を聞く。AI だけでは変えない。
//! `--apply` で、利用者の設定フォルダーの maturity.toml(上書き用)へ書く。Claude(AI)が結果を見て適用してもよい(CLAUDE.md 参照)。
use std::path::{Path, PathBuf};

const DEFAULT: &str = include_str!("../registry/maturity.toml");

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub holder: String,
    pub item: String,
    pub status: String,
    pub dir: String,
    pub probe: String,
    pub note: String,
}

pub fn parse(t: &str) -> Vec<Entry> {
    let mut v: Vec<Entry> = vec![];
    for l in t.lines().map(str::trim) {
        if l == "[[compat]]" {
            v.push(Entry { holder: String::new(), item: String::new(), status: "partial".into(), dir: String::new(), probe: String::new(), note: String::new() });
        } else if let (Some(e), Some((k, val))) = (v.last_mut(), l.split_once('=')) {
            let val = val.trim().to_string();
            match k.trim() {
                "holder" => e.holder = val,
                "item" => e.item = val,
                "status" => e.status = val,
                "dir" => e.dir = val,
                "probe" => e.probe = val,
                "note" => e.note = val,
                _ => {}
            }
        }
    }
    v
}

fn ledger_path() -> PathBuf {
    std::env::var_os("PL2PL_MATURITY").map(PathBuf::from).unwrap_or_else(|| crate::llm::config_path().with_file_name("maturity.toml"))
}

fn ledger_text() -> String {
    std::fs::read_to_string(ledger_path()).unwrap_or_else(|_| DEFAULT.to_string())
}

/// holder が item を含むが、互換がまだ部分的(status が complete でない)か
pub fn is_partial(holder: &str, item: &str) -> bool {
    parse(&ledger_text()).iter().any(|e| e.holder == holder && e.item == item && e.status != "complete")
}

fn run(cmd: &str, dir: &Path, jobs: usize) -> bool {
    let mut it = cmd.split_whitespace();
    let Some(prog) = it.next() else { return false };
    std::process::Command::new(prog).args(it).current_dir(dir).env("CARGO_BUILD_JOBS", jobs.to_string()).status().map(|s| s.success()).unwrap_or(false)
}

pub fn set_complete(text: &str, holder: &str, item: &str) -> String {
    let (mut out, mut cur_h, mut cur_i) = (String::new(), String::new(), String::new());
    for l in text.lines() {
        if let Some((k, v)) = l.trim().split_once('=') {
            match k.trim() {
                "holder" => cur_h = v.trim().into(),
                "item" => cur_i = v.trim().into(),
                "status" if cur_h == holder && cur_i == item => {
                    out.push_str("status = complete\n");
                    continue;
                }
                _ => {}
            }
        }
        out.push_str(l);
        out.push('\n');
    }
    out
}

pub fn run_cmd(repo: &Path, llm_url: Option<&str>, apply: bool) -> i32 {
    // open-cpu: この PC の CPU を調べ、試験の並列数に使う
    let jobs = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(2);
    println!("{}\n試験の並列数 / jobs: {jobs}", open_cpu::runtime_summary());
    let mut text = ledger_text();
    let (mut changed, mut judged) = (false, 0);
    let mut done: Vec<(String, bool)> = vec![];
    for e in parse(&text.clone()).iter().filter(|e| e.status != "complete") {
        println!("\n[{} の {}] {}", e.holder, e.item, e.note);
        if e.probe.is_empty() {
            println!("  → 判定できません(試験がまだ無い)。partial のまま / cannot judge (no probe yet); stays partial");
            continue;
        }
        let dir = repo.join(&e.dir);
        if !dir.is_dir() {
            println!("  → {} が見つかりません。partial のまま / directory not found; stays partial", dir.display());
            continue;
        }
        let key = format!("{}|{}", e.dir, e.probe);
        let ok = match done.iter().find(|d| d.0 == key) {
            Some(d) => d.1,
            None => {
                let r = run(&e.probe, &dir, jobs);
                done.push((key, r));
                r
            }
        };
        judged += 1;
        if let Some(url) = llm_url {
            println!("  参考(AI、aruaru-llm): {}", crate::llm::opinion(url, &e.holder, &e.item, &e.note, ok));
        }
        if ok {
            println!("  → 試験成功。完成と判定 / probe passed: judged complete");
            if apply {
                text = set_complete(&text, &e.holder, &e.item);
                changed = true;
            }
        } else {
            println!("  → 試験失敗。partial のまま / probe failed; stays partial");
        }
    }
    if changed {
        let p = ledger_path();
        if let Some(d) = p.parent() {
            let _ = std::fs::create_dir_all(d);
        }
        match std::fs::write(&p, text) {
            Ok(()) => println!("\n台帳を更新しました / ledger updated: {}", p.display()),
            Err(e) => {
                eprintln!("書き込めません / cannot write: {e}");
                return 1;
            }
        }
    } else if judged > 0 && !apply {
        println!("\n(--apply を付けると、成功した物を台帳に反映します / add --apply to record passing items)");
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_and_flip() {
        let v = parse(DEFAULT);
        assert!(v.len() >= 6 && v.iter().any(|e| e.holder == "rpoem" && e.item == "tauri" && e.probe.is_empty()));
        let v2 = parse(&set_complete(DEFAULT, "rpoem", "open-cosmo"));
        assert_eq!(v2.iter().find(|e| e.holder == "rpoem" && e.item == "open-cosmo").unwrap().status, "complete");
        assert_eq!(v2.iter().find(|e| e.holder == "rporea" && e.item == "open-cosmo").unwrap().status, "partial");
    }
}
