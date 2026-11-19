mod cobol;
mod cport;
mod gpu;
mod ir;
mod ir_emit;
mod kokkos;
mod pyhpc;
mod llm;
mod loops;
mod omp;
mod blade;
mod middleware;
mod php;
mod watch;
mod web;
mod util;

use std::path::{Path, PathBuf};

const USAGE: &str = "pl2pl — プラットフォーム/HPC 移植コンバーター / platform and HPC porting converters

  pl2pl cobol  <dir|file> --out <dir>        COBOL(IBM)+JCL → GnuCOBOL(Linux)+bash
  pl2pl cport  <dir|file> --out <dir>        UNIX の C → Windows/Linux で通る C(互換ヘッダー+報告)
  pl2pl omp    <dir|file> --out <dir>        C/C++ の独立ループへ OpenMP 指示行を付ける
  pl2pl kokkos <dir|file> --out <dir>        C++ の独立ループを Kokkos へ
  pl2pl pyhpc  <dir|file> --out <dir> [--mpi]  Python の数値関数へ Numba、--mpi で mpi4py の雛形
  pl2pl web    <dir> --target <fastapi|mfastapi|rust-poem|rpoem> --out <dir> [--dry-run]
                                              Java/COBOL/Flask/Django を判定し java_to・cobol_to・2fastapi・2poem へ振り分け
  pl2pl php    <dir> --out <dir> [--target fastapi|mfastapi]   PHP(Laravel/Symfony)のルートを FastAPI の骨組みへ
  pl2pl gpu    <dir|file> --out <dir> --target <open-cuda|open-directx>   (試作)独立ループから GPU カーネル(.cu / .hlsl)
  pl2pl doctor                                   open-cpu(同梱)で検出した CPU 情報と、同梱コンポーネントの一覧
  pl2pl llm setup [--url http://127.0.0.1:4600] [--model <id>] [--dry-run]
                                              この PC のメモリに合う aruaru-llm のモデルを自動選択して導入(実行したときだけダウンロード)
  pl2pl llm start [--url ...]                  同梱の aruaru-llm を 127.0.0.1 で起動(動いていれば何もしない)
  pl2pl config [llm <auto|off|モデルID>]        LLM の自動導入を外す(off)/ 既定(auto)/ モデル固定。PL2PL_NO_LLM=1 でも外せる
  pl2pl watch [--registry F] [--repo DIR] [--vc version-control] [--dry-run]
                                              言語・FWの新版を検知→通し試験→自動追従→version-controlで修正→メール/LINE通知(毎晩・毎朝に実行)
  pl2pl ir <dir> --out <dir> [--format json|openapi|fastapi]   (試作)共通の中間表現: PHP・Python・Java のルートを JSON / OpenAPI へ
  pl2pl --version
";

fn arg(a: &[String], k: &str) -> Option<String> {
    a.iter().position(|x| x == k).and_then(|i| a.get(i + 1).cloned())
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let cmd = a.get(1).map(String::as_str).unwrap_or("");
    if cmd == "--version" || cmd == "-V" {
        println!("pl2pl {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if cmd == "doctor" {
        println!("{}", open_cpu::runtime_report());
        println!("同梱コンポーネント / bundled components: open-cpu(ライブラリとして内蔵)。open-cuda・open-directx(試作の出力先)・aruaru-llm(任意のAI補助、別途導入)は components/README.md を参照");
        return;
    }
    if cmd == "llm" && a.get(2).map(String::as_str) == Some("setup") {
        let url = arg(&a, "--url").unwrap_or_else(|| "http://127.0.0.1:4600".into());
        std::process::exit(llm::setup(&url, a.iter().any(|x| x == "--dry-run"), arg(&a, "--model").as_deref()));
    }
    if cmd == "config" {
        match (a.get(2).map(String::as_str), a.get(3)) {
            (Some("llm"), Some(v)) => match llm::set_choice(v) {
                Ok(()) => println!("llm = {v}(保存先 {})", llm::config_path().display()),
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(2);
                }
            },
            _ => println!("llm = {}(設定ファイル {})
変更: pl2pl config llm <auto|off|モデルID>。環境変数 PL2PL_NO_LLM=1 でも外せます", llm::llm_choice(), llm::config_path().display()),
        }
        return;
    }
    if cmd == "llm" && a.get(2).map(String::as_str) == Some("start") {
        let url = arg(&a, "--url").unwrap_or_else(|| "http://127.0.0.1:4600".into());
        match llm::ensure_running(&url) {
            Ok(()) => println!("aruaru-llm は {url} で動いています"),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        return;
    }
    if cmd == "watch" {
        // 毎晩・毎朝のスケジューラから呼ぶ: 新版の検知 → 確認 → 自動追従 → version-control で修正 → 通知
        let o = watch::Opts {
            registry: arg(&a, "--registry").map(Into::into).unwrap_or_else(|| "registry/ecosystem.toml".into()),
            repo: arg(&a, "--repo").map(Into::into).unwrap_or_else(|| ".".into()),
            dry: a.iter().any(|x| x == "--dry-run"),
            vc: arg(&a, "--vc").map(Into::into),
        };
        std::process::exit(watch::run(&o));
    }
    let (Some(input), Some(out)) = (a.get(2).filter(|x| !x.starts_with("--")), arg(&a, "--out")) else {
        println!("{USAGE}");
        std::process::exit(if cmd.is_empty() { 0 } else { 2 });
    };
    let (input, out) = (PathBuf::from(input), PathBuf::from(out));
    if !input.exists() {
        eprintln!("pl2pl: 入力がありません: {}", input.display());
        std::process::exit(2);
    }
    if cmd == "doctor" {
        println!("{}", open_cpu::runtime_report());
        println!("同梱コンポーネント / bundled components: open-cpu(ライブラリとして内蔵)。open-cuda・open-directx(試作の出力先)・aruaru-llm(任意のAI補助、別途導入)は components/README.md を参照");
        return;
    }
    if cmd == "web" {
        let t = arg(&a, "--target").unwrap_or_else(|| "fastapi".into());
        std::process::exit(web::run(&input, &out, &t, a.iter().any(|x| x == "--dry-run")));
    }
    if cmd == "ir" {
        // 共通の中間表現(試作): 入力のルートを IR にして、json か openapi で書き出す
        let fmt = arg(&a, "--format").unwrap_or_else(|| "json".into());
        let files: Vec<(String, String)> = util::files(&input, &["php", "py", "java"]).into_iter().map(|(p, rel)| (rel.to_string_lossy().replace('\\', "/"), util::read(&p))).collect();
        let mut w: Vec<String> = vec![];
        let php_files: Vec<(String, String)> = files.iter().filter(|(n, _)| n.ends_with(".php")).cloned().collect();
        // 入力側は拡張子で自動判定(PHP / Python / Java)。出力側は --format で選ぶ
        let mut eps = ir::from_php(&php::extract(&php_files, &mut w));
        eps.extend(ir::from_python(&files));
        eps.extend(ir::from_java(&files));
        eps.extend(ir::from_java_other(&files));
        // ルートが1つも見つからなかったファイルは黙って捨てず、報告する(正規表現による検出なので取りこぼしがありうる)
        let silent: Vec<&str> = files.iter().filter(|(n, _)| !eps.iter().any(|e| &e.source == n)).map(|(n, _)| n.as_str()).collect();
        if !silent.is_empty() {
            println!("pl2pl ir: ルートを検出しなかったファイル {} 件(ルートが無いか、未対応の書き方): {}", silent.len(), silent.iter().take(5).cloned().collect::<Vec<_>>().join(", "));
        }
        let (name, text) = match fmt.as_str() { "openapi" => ("openapi.json", ir::to_openapi(&eps)), "fastapi" => ("main.py", ir::to_fastapi(&eps)), _ => ("ir.json", ir::to_json(&eps)) };
        let _ = util::write(&out, Path::new(name), &text);
        println!("pl2pl ir: {} 件のルート → {}/{}", eps.len(), out.display(), name);
        return;
    }
    let mut rep: Vec<String> = vec![];
    if cmd == "php" {
        let t = arg(&a, "--target").unwrap_or_else(|| "fastapi".into());
        let files: Vec<(String, String)> = util::files(&input, &["php"]).into_iter().map(|(p, rel)| (rel.to_string_lossy().replace('\\', "/"), util::read(&p))).collect();
        for (n, c) in php::generate(&files, &t, &mut rep) {
            let _ = util::write(&out, Path::new(&n), &c);
        }
        let _ = util::write_report(&out, "php", &rep);
        println!("pl2pl php: {} ファイル処理、レポート {} 項目 → {}", files.len(), rep.len(), out.display());
        return;
    }
    let n = match cmd {
        "cobol" => run(&input, &out, &["cbl", "cob", "cobol", "cpy", "jcl"], &mut rep, cobol::convert),
        "cport" => run(&input, &out, &["c", "h"], &mut rep, cport::convert),
        "kokkos" => run(&input, &out, &["cc", "cpp", "cxx", "hpp"], &mut rep, kokkos::convert),
        "pyhpc" => {
            let mpi = a.iter().any(|x| x == "--mpi");
            run(&input, &out, &["py"], &mut rep, |s, n, r| pyhpc::convert(s, n, r, mpi))
        }
        "gpu" => {
            let t = arg(&a, "--target").unwrap_or_else(|| "open-cuda".into());
            run(&input, &out, &["c", "cc", "cpp", "cxx"], &mut rep, |s, n, r| gpu::convert(s, n, r, &t))
        }
        "omp" => run(&input, &out, &["c", "cc", "cpp", "cxx", "h", "hpp"], &mut rep, omp::convert),
        _ => {
            println!("{USAGE}");
            std::process::exit(2);
        }
    };
    if cmd == "cport" {
        let _ = util::write(&out, Path::new("pl2pl_compat.h"), cport::COMPAT_H);
    }
    if cmd == "pyhpc" && a.iter().any(|x| x == "--mpi") {
        let _ = util::write(&out, Path::new("mpi_scaffold.py"), pyhpc::MPI_SCAFFOLD);
    }
    if cmd == "cobol" {
        let _ = util::write(&out, Path::new("build.sh"), &cobol::build_script(&out));
    }
    let _ = util::write_report(&out, cmd, &rep);
    println!("pl2pl {cmd}: {n} ファイル処理、レポート {} 項目 → {}", rep.len(), out.display());
}

fn run<F: FnMut(&str, &str, &mut Vec<String>) -> Vec<(String, String)>>(input: &Path, out: &Path, exts: &[&str], rep: &mut Vec<String>, mut f: F) -> usize {
    let fs = util::files(input, exts);
    for (p, rel) in &fs {
        let src = util::read(p);
        let name = rel.to_string_lossy().replace('\\', "/");
        for (rn, content) in f(&src, &name, rep) {
            // rn が空なら元の相対パス、そうでなければ同じディレクトリ内の別名
            let target = if rn.is_empty() { rel.clone() } else { rel.parent().map(|d| d.join(&rn)).unwrap_or_else(|| PathBuf::from(&rn)) };
            if let Err(e) = util::write(out, &target, &content) {
                eprintln!("pl2pl: 書き出せません {}: {e}", target.display());
            }
        }
    }
    fs.len()
}
