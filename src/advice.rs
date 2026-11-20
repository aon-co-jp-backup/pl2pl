//! 組み合わせ選択の案内と注意(日本語 / English)。
//! (1) RPoem / RPoRea を選んだときの説明: Tauri 互換・WunderGraph Cosmo 互換・tokio + WASM、HTML5/CSS3 は最小限、JavaScript/TypeScript/Node.js/REST API は使わない設計。
//! (2) `--also` で足そうとした物が、選んだ物に既に含まれていれば「既に〇〇に含んでいますので、ご選択できません」と断る。

/// 名前の別名 → 正規名
fn canon(s: &str) -> Option<&'static str> {
    Some(match s.trim().to_lowercase().replace(['_', ' '], "-").as_str() {
        "tauri" => "tauri",
        "wundergraph-cosmo" | "wundergraph" | "cosmo" => "wundergraph-cosmo",
        "node" | "nodejs" | "node.js" => "nodejs",
        "javascript" | "js" => "javascript",
        "typescript" | "ts" => "typescript",
        "rest" | "rest-api" | "restapi" => "rest",
        "tokio" => "tokio",
        "wasm" | "webassembly" => "wasm",
        "html5" | "html" => "html5",
        "css3" | "css" => "css3",
        "bootstrap" => "bootstrap",
        "poem" => "poem",
        "rpoem" => "rpoem",
        "reaxt" => "reaxt",
        "fastapi" => "fastapi",
        "python" => "python",
        "mojo" => "mojo",
        "open-cosmo" => "open-cosmo",
        "rust" => "rust",
        _ => return None,
    })
}

fn label(n: &str) -> &'static str {
    match n {
        "tauri" => "Tauri",
        "wundergraph-cosmo" => "WunderGraph Cosmo",
        "nodejs" => "Node.js",
        "javascript" => "JavaScript",
        "typescript" => "TypeScript",
        "rest" => "REST API",
        "tokio" => "tokio",
        "wasm" => "WASM",
        "html5" => "HTML5",
        "css3" => "CSS3",
        "bootstrap" => "Bootstrap",
        "poem" => "Poem",
        "rpoem" => "RPoem",
        "reaxt" => "Reaxt",
        "fastapi" => "FastAPI",
        "python" => "Python",
        "mojo" => "Mojo",
        "open-cosmo" => "open-cosmo",
        _ => "Rust",
    }
}

/// 選んだ物(backend / frontend)が、すでに中に含んでいるもの
fn includes(choice: &str) -> &'static [&'static str] {
    match choice {
        "rust-axum" => &["rust", "tokio"],
        "rust-poem" => &["rust", "poem", "tokio"],
        // RPoem = Poem の Rust 版 + Tauri 互換 + WunderGraph Cosmo 互換(open-cosmo)+ tokio + WASM
        "rpoem" => &["rust", "poem", "rpoem", "tokio", "wasm", "tauri", "wundergraph-cosmo", "open-cosmo", "html5", "css3"],
        "rporea" => &["rust", "poem", "rpoem", "reaxt", "tokio", "wasm", "tauri", "wundergraph-cosmo", "open-cosmo", "html5", "css3"],
        "fastapi" => &["python", "fastapi"],
        "mfastapi" => &["python", "fastapi", "mojo"],
        "bootstrap-ts" => &["html5", "css3", "javascript", "typescript", "bootstrap"],
        "rust-wasm" => &["rust", "wasm", "html5", "css3"],
        _ => &[],
    }
}

/// RPoem / RPoRea では設計上使わないもの
fn not_used(backend: &str) -> &'static [&'static str] {
    if matches!(backend, "rpoem" | "rporea") {
        &["nodejs", "javascript", "typescript", "rest"]
    } else {
        &[]
    }
}

/// RPoem / RPoRea を選んだときの案内(日本語と English)
pub fn backend_notice(backend: &str) -> Option<String> {
    if !matches!(backend, "rpoem" | "rporea") {
        return None;
    }
    let name = if backend == "rporea" { "RPoRea(RPoem + Reaxt)" } else { "RPoem" };
    Some(format!(
        "【{name} について / About {name}】\n\
         日本語: RPoem は Poem の Rust 版で、Tauri 互換・WunderGraph Cosmo 互換の機能と、tokio、WASM を含んでいます。そのため Tauri と WunderGraph Cosmo は、別に選ばないでください(選んでも、既に含んでいるためお断りします)。画面は HTML5 と CSS3 を使いますが、使う量はなるべく少なくします。JavaScript・TypeScript・Node.js・REST API は、設計上、使いません。\n\
         English: RPoem is the Rust edition of Poem. It already includes Tauri-compatible and WunderGraph Cosmo-compatible features, plus tokio and WASM, so please do not choose Tauri or WunderGraph Cosmo separately (if you do, the request is declined because they are already included). The UI uses HTML5 and CSS3, kept as small as possible. By design, JavaScript, TypeScript, Node.js and REST APIs are not used.\n"
    ))
}

/// `--also tauri,nodejs` のような追加の選択を調べる。戻り値: 断る理由(日本語と English)の一覧。空なら全部受け付け。
pub fn check_also(backend: &str, frontend: &str, also: &str) -> Vec<String> {
    let mut out = vec![];
    for raw in also.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let Some(c) = canon(raw) else {
            out.push(format!("「{raw}」は選択肢にありません。/ \"{raw}\" is not a known option."));
            continue;
        };
        let l = label(c);
        let holder = [backend, frontend].into_iter().find(|ch| includes(ch).contains(&c));
        if let Some(h) = holder {
            let hl = match h {
                "rpoem" => "RPoem",
                "rporea" => "RPoRea",
                "rust-axum" => "Axum",
                "rust-poem" => "Poem",
                "fastapi" => "FastAPI",
                "mfastapi" => "MFastAPI",
                "bootstrap-ts" => "Bootstrap + TypeScript",
                _ => "Rust + WASM",
            };
            out.push(format!("既に {hl} に {l} を含んでいますので、ご選択できません。/ {l} is already included in {hl}, so it cannot be selected separately."));
        } else if not_used(backend).contains(&c) {
            out.push(format!("{l} は、設計上、使用しない仕様ですので、ご選択できません。/ {l} is not used by design, so it cannot be selected."));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rpoem_notice_mentions_everything() {
        let n = backend_notice("rpoem").unwrap();
        for k in ["Tauri", "WunderGraph Cosmo", "tokio", "WASM", "HTML5", "CSS3", "Node.js", "REST"] {
            assert!(n.contains(k), "{k}");
        }
        assert!(backend_notice("fastapi").is_none());
    }
    #[test]
    fn included_is_declined() {
        let r = check_also("rporea", "rust-wasm", "tauri,cosmo,wasm");
        assert_eq!(r.len(), 3);
        assert!(r[0].contains("既に RPoRea に Tauri を含んでいますので、ご選択できません"));
        assert!(r[1].contains("WunderGraph Cosmo is already included in RPoRea"));
        assert!(r[2].contains("WASM"));
    }
    #[test]
    fn not_used_by_design() {
        let r = check_also("rpoem", "rust-wasm", "nodejs,rest,typescript,js");
        assert_eq!(r.len(), 4);
        assert!(r.iter().all(|x| x.contains("設計上") && x.contains("not used by design")));
    }
    #[test]
    fn mfastapi_includes_fastapi_and_unknown() {
        let r = check_also("mfastapi", "bootstrap-ts", "fastapi,mojo,bootstrap,foo");
        assert_eq!(r.len(), 4);
        assert!(r[3].contains("foo"));
        assert!(check_also("rust-axum", "none", "tauri").is_empty());
    }
}
