//! `pl2pl llm setup`: この PC のメモリに合う aruaru-llm のモデルを選び、aruaru-llm の Connect API で導入・選択する。
//! 変換器そのものは LLM に依存しない(任意の補助)。ダウンロードはこのコマンドを実行したときだけ行う。
use std::io::Read;

/// aruaru-llm の QWEN_CATALOG(approx_size_mb)に合わせた候補。必要なメモリは重みの約 2.5 倍と見積もる(余裕込み・未実測)。
pub const CANDIDATES: [(&str, u64); 3] = [("qwen2.5-3b-instruct", 6200), ("qwen2.5-1.5b-instruct", 2900), ("qwen2.5-0.5b-instruct", 942)];

pub fn pick(available_mb: u64) -> (&'static str, u64) {
    for (id, size) in CANDIDATES {
        if available_mb >= size * 5 / 2 {
            return (id, size);
        }
    }
    CANDIDATES[CANDIDATES.len() - 1] // 足りなくても最小を返す(呼び出し側が警告する)
}

fn post(url: &str, path: &str, body: &str) -> Result<String, String> {
    let r = ureq::post(&format!("{}/aruaru_llm.v1.Llm/{path}", url.trim_end_matches('/')))
        .set("content-type", "application/json")
        .timeout(std::time::Duration::from_secs(3600))
        .send_string(body);
    match r {
        Ok(resp) => {
            let mut s = String::new();
            resp.into_reader().take(1_000_000).read_to_string(&mut s).map_err(|e| e.to_string())?;
            Ok(s)
        }
        Err(ureq::Error::Status(code, resp)) => Err(format!("HTTP {code}: {}", resp.into_string().unwrap_or_default().chars().take(200).collect::<String>())),
        Err(e) => Err(e.to_string()),
    }
}

pub fn setup(url: &str, dry: bool, force_id: Option<&str>) -> i32 {
    let choice = llm_choice();
    if choice == "off" && force_id.is_none() {
        println!("LLM の自動導入は「外す」設定です(pl2pl config llm auto で戻せます)。何もしません");
        return 0;
    }
    let force_id = force_id.or(if choice != "auto" && choice != "off" { Some(choice.as_str()) } else { None });
    let mut sys = sysinfo::System::new();
    sys.refresh_memory();
    let total = sys.total_memory() / 1024 / 1024;
    let avail = sys.available_memory() / 1024 / 1024;
    // 空きメモリで選ぶ(他のアプリが使っている分は除く)
    let (id, size) = match force_id {
        Some(i) => CANDIDATES.iter().find(|c| c.0 == i).copied().unwrap_or((CANDIDATES[2].0, CANDIDATES[2].1)),
        None => pick(avail),
    };
    println!("メモリ: 全体 {total} MB / 空き {avail} MB → 選んだモデル: {id}(約 {size} MB のダウンロード)");
    if avail < size * 5 / 2 {
        println!("注意: 空きメモリがこのモデルの目安({} MB)に足りません。動作が遅くなる/失敗する可能性があります", size * 5 / 2);
    }
    if dry {
        println!("--dry-run: 導入しません。実行すると POST {url}/aruaru_llm.v1.Llm/QwenInstall と QwenSelect を呼びます");
        return 0;
    }
    if let Err(e) = ensure_running(url) {
        eprintln!("{e}");
        return 1;
    }
    println!("導入中(数分かかることがあります)…");
    match post(url, "QwenInstall", &format!("{{\"id\":\"{id}\"}}")) {
        Ok(r) => println!("導入: {}", r.chars().take(200).collect::<String>()),
        Err(e) => {
            eprintln!("導入に失敗: {e}\n(aruaru-llm が {url} で動いているか確認してください。components/README.md)");
            return 1;
        }
    }
    match post(url, "QwenSelect", &format!("{{\"id\":\"{id}\"}}")) {
        Ok(r) => println!("選択: {}", r.chars().take(200).collect::<String>()),
        Err(e) => {
            eprintln!("選択に失敗: {e}");
            return 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_nothing_when_absent() {
        // テスト環境に aruaru-llm が無ければ None(あれば Some)。どちらでもパニックしない
        let _ = find_binary();
    }
    #[test]
    fn choice_validation() {
        assert!(set_choice("bogus").is_err());
    }
    #[test]
    fn env_turns_llm_off() {
        std::env::set_var("PL2PL_NO_LLM", "1");
        assert_eq!(llm_choice(), "off");
        std::env::remove_var("PL2PL_NO_LLM");
    }
    #[test]
    fn picks_by_memory() {
        assert_eq!(pick(1000).0, "qwen2.5-0.5b-instruct"); // 足りなくても最小
        assert_eq!(pick(3000).0, "qwen2.5-0.5b-instruct");
        assert_eq!(pick(8000).0, "qwen2.5-1.5b-instruct");
        assert_eq!(pick(32000).0, "qwen2.5-3b-instruct");
    }
}

// ---- 同梱された aruaru-llm の起動 ----

pub fn find_binary() -> Option<std::path::PathBuf> {
    let name = if cfg!(windows) { "aruaru-llm.exe" } else { "aruaru-llm" };
    let mut dirs: Vec<std::path::PathBuf> = vec![];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(d) = exe.parent() {
            dirs.push(d.to_path_buf());
            dirs.push(d.join("components"));
            dirs.push(d.join("components").join("bin"));
        }
    }
    if let Ok(p) = std::env::var("PATH") {
        dirs.extend(std::env::split_paths(&p));
    }
    dirs.into_iter().map(|d| d.join(name)).find(|p| p.is_file())
}

pub fn healthy(url: &str) -> bool {
    ureq::get(&format!("{}/healthz", url.trim_end_matches('/'))).timeout(std::time::Duration::from_secs(2)).call().is_ok()
}

/// 動いていなければ、同梱の aruaru-llm を 127.0.0.1 だけで待ち受ける形で起動する。
pub fn ensure_running(url: &str) -> Result<(), String> {
    if healthy(url) {
        return Ok(());
    }
    let Some(bin) = find_binary() else {
        return Err("aruaru-llm の実行ファイルが見つかりません(pl2pl と同じフォルダ、components/、または PATH)。同梱版の入れ方は components/README.md".into());
    };
    let bind = url.trim_start_matches("http://").trim_end_matches('/').to_string();
    println!("同梱の aruaru-llm を起動します: {}(待ち受け {bind}、この PC だけ)", bin.display());
    std::process::Command::new(&bin)
        .env("ARUARU_LLM_BIND", &bind)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("起動できません: {e}"))?;
    for _ in 0..60 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        if healthy(url) {
            return Ok(());
        }
    }
    Err("起動後 60 秒待っても応答しません".into())
}

// ---- 利用者の選択(自動導入を外す / モデルを固定する)----

pub fn config_path() -> std::path::PathBuf {
    let base = std::env::var_os("PL2PL_CONFIG_DIR")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("APPDATA").map(|a| std::path::PathBuf::from(a).join("pl2pl")))
        .or_else(|| std::env::var_os("XDG_CONFIG_HOME").map(|a| std::path::PathBuf::from(a).join("pl2pl")))
        .or_else(|| std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".config").join("pl2pl")))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    base.join("config.txt")
}

/// "auto"(既定)/ "off"(自動導入しない)/ モデル ID(固定)
pub fn llm_choice() -> String {
    if std::env::var("PL2PL_NO_LLM").map(|v| !v.is_empty() && v != "0").unwrap_or(false) {
        return "off".into();
    }
    std::fs::read_to_string(config_path())
        .ok()
        .and_then(|t| t.lines().find_map(|l| l.strip_prefix("llm=").map(|v| v.trim().to_string())))
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "auto".into())
}

pub fn set_choice(v: &str) -> Result<(), String> {
    let ok = v == "auto" || v == "off" || CANDIDATES.iter().any(|c| c.0 == v);
    if !ok {
        return Err(format!("不明な値: {v}(auto / off / {})", CANDIDATES.iter().map(|c| c.0).collect::<Vec<_>>().join(" / ")));
    }
    let p = config_path();
    if let Some(d) = p.parent() {
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    std::fs::write(&p, format!("llm={v}\n")).map_err(|e| e.to_string())
}
