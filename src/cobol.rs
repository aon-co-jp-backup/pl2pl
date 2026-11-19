//! COBOL: メインフレーム(IBM)→ GnuCOBOL(Linux 等)。ソースは書き換えず、非互換の検出・DD名の対応・JCL→bash を出す。
use regex::Regex;
use std::path::Path;

/// 対応する COBOL の置き場所(OS)。ソースは変えず、その OS で GnuCOBOL(cobc)を使うビルド手順を出す。
pub const OSES: &[&str] = &["linux", "macos", "unix", "windows"];
/// 方言: cobc の -std に対応(ibm=メインフレーム寄り、mf=Micro Focus 寄り、default=GnuCOBOL 既定)
pub const DIALECTS: &[&str] = &["ibm", "mf", "default"];

/// 戻り値: (ファイル名, 中身)。windows は build.bat、そのほかは build.sh。
pub fn build_script(os: &str, dialect: &str) -> (String, String) {
    let std = match dialect {
        "mf" => " -std=mf",
        "default" => "",
        _ => " -std=ibm",
    };
    if os == "windows" {
        let b = format!(
            "@echo off\r\nrem pl2pl cobol: for Windows. Put GnuCOBOL (cobc) on PATH first.\r\nif not exist bin mkdir bin\r\nfor %%f in (*.cbl *.cob) do cobc -x{std} -I . -o bin/%%~nf.exe %%f\r\n"
        );
        return ("build.bat".into(), b);
    }
    let hint = match os {
        "macos" => "# macOS: brew install gnu-cobol\n",
        "unix" => "# UNIX (AIX / Solaris / BSD ...): install cobc from a package or GnuCOBOL source\n",
        _ => "# Linux: apt install gnucobol (or dnf install gnucobol)\n",
    };
    let body = "set -e\nmkdir -p bin\nfor f in *.cbl *.cob; do\n  [ -e \"$f\" ] || continue\n  cobc -x__STD__ -I . -o \"bin/${f%.*}\" \"$f\"\ndone\n".replace("__STD__", std);
    ("build.sh".into(), format!("#!/bin/bash\n# pl2pl cobol: for {os}, dialect {dialect}\n{hint}{body}"))
}

pub fn convert(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let lower = name.to_lowercase();
    if lower.ends_with(".jcl") {
        let base = name.rsplit('/').next().unwrap();
        let stem = base.rsplit_once('.').map(|x| x.0).unwrap_or(base);
        return vec![(format!("{stem}.sh"), jcl_to_bash(src, name, rep))];
    }
    scan_cobol(src, name, rep);
    vec![(String::new(), src.to_string())]
}

fn scan_cobol(src: &str, name: &str, rep: &mut Vec<String>) {
    let checks: [(&str, &str); 9] = [
        (r"(?i)EXEC\s+SQL", "EXEC SQL(DB2): GnuCOBOL には前処理系(ocesql 等)が別に要る。接続先DBと方言の移行が必要"),
        (r"(?i)EXEC\s+CICS", "EXEC CICS: Linux には CICS が無い。画面・トランザクションは作り直し(Web等)"),
        (r"(?i)EXEC\s+DLI|CBLTDLI", "IMS(DL/I): 代替DBへの移行が必要"),
        (r"(?i)CALL\s+'CEE\w*'|CALL\s+'IGZ\w*'", "Language Environment の呼び出し: 代替の実装が必要"),
        (r"(?i)\bCOPY\s+[A-Z0-9-]+", "COPY(コピーブック): `.cpy` を同じ場所に置いて `-I .` で解決する"),
        (r"(?i)ORGANIZATION\s+IS\s+INDEXED|VSAM", "VSAM/索引ファイル: GnuCOBOL の索引ファイル(BDB/VBISAM等)へ。データの移行が別途必要"),
        (r"(?i)COMP-1|COMP-2|COMP-3|COMPUTATIONAL-[123]|PACKED-DECIMAL", "COMP-1/2/3: GnuCOBOL は対応するが、メインフレームのデータファイルはバイト並びが違う場合がある"),
        (r"(?i)\bSORT\b|\bMERGE\b", "SORT/MERGE: GnuCOBOL は対応。外部の SORT ユーティリティ(JCL側)は別途 sort 等へ"),
        (r"(?i)DISPLAY\s+UPON\s+(SYSOUT|CONSOLE)|ACCEPT\s+.*FROM\s+(SYSIN|CONSOLE)", "SYSIN/SYSOUT/CONSOLE: 環境変数 DD_xxx か標準入出力へ対応させる"),
    ];
    for (pat, msg) in checks {
        if let Some(m) = Regex::new(pat).unwrap().find(src) {
            rep.push(format!("{name}:{} {msg}", crate::util::line_of(src, m.start())));
        }
    }
    if let Some(c) = Regex::new(r"(?i)PROGRAM-ID\.\s*([A-Z0-9-]+)").unwrap().captures(src) {
        rep.push(format!("{name}: PROGRAM-ID {}(JCL の PGM= と同名で bin/ に出力される想定)", &c[1]));
    }
    for c in Regex::new(r"(?i)SELECT\s+(?:OPTIONAL\s+)?([A-Z0-9-]+)\s+ASSIGN\s+TO\s+(?:[A-Z]+-)?([A-Z0-9-]+)").unwrap().captures_iter(src) {
        rep.push(format!("{name}: ファイル {} は DD名 {} → 環境変数 `DD_{}`(または `{}`)で実ファイルへ対応させる", &c[1], &c[2], &c[2], &c[2]));
    }
    if Regex::new(r"[\x00-\x08\x0e-\x1f]").unwrap().is_match(src) {
        rep.push(format!("{name}: 制御文字を含む。EBCDIC のままの可能性。`iconv -f IBM037 -t UTF-8` で変換してから処理する"));
    }
}

fn shq(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn dsn_path(dsn: &str) -> String {
    let d = dsn.trim();
    if let Some(t) = d.strip_prefix("&&") {
        return format!("$DATA_DIR/_tmp.{t}");
    }
    let safe: String = d.chars().map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' { c } else { '_' }).collect();
    format!("$DATA_DIR/{safe}")
}

type Step = (String, String, Vec<String>); // (ステップ名, プログラム, 実行前の設定)

fn finish(cur: &mut Option<Step>, o: &mut String) {
    if let Some((s, pgm, ex)) = cur.take() {
        o.push_str(&format!("\n# --- STEP {s} PGM={pgm}\n(\n"));
        for e in &ex {
            o.push_str(&format!("  {e}\n"));
        }
        match pgm.as_str() {
            "IEFBR14" => o.push_str("  :  # IEFBR14: 何もしない\n"),
            "SORT" | "ICEMAN" => o.push_str("  # TODO(pl2pl): SORT ユーティリティ。SYSIN の制御文を `sort` コマンドへ手で置き換える\n  false\n"),
            "IEBGENER" | "IDCAMS" | "IEBCOPY" => o.push_str(&format!("  # TODO(pl2pl): ユーティリティ {pgm}。cp 等へ手で置き換える\n  false\n")),
            _ => o.push_str(&format!("  ./bin/{pgm}\n")),
        }
        o.push_str(&format!(")\nstep_rc=$?\necho \"STEP {s} RC=$step_rc\"\n[ $step_rc -gt $rc ] && rc=$step_rc\n"));
    }
}

pub fn jcl_to_bash(src: &str, name: &str, rep: &mut Vec<String>) -> String {
    // 継続行を結合(行末が , で、次行が `// ` で始まる)
    let mut lines: Vec<String> = vec![];
    for l in src.lines() {
        let l = if l.len() > 72 { &l[..72] } else { l };
        if l.starts_with("//") && !l.starts_with("//*") && l.len() > 2 && l[2..].starts_with(' ') && lines.last().map(|p| p.trim_end().ends_with(',')).unwrap_or(false) {
            let p = lines.pop().unwrap();
            lines.push(format!("{}{}", p.trim_end(), l[2..].trim()));
        } else {
            lines.push(l.to_string());
        }
    }
    let mut o = String::from("#!/bin/bash\n# pl2pl cobol が JCL から生成(骨組み)。DD名は GnuCOBOL の環境変数 DD_<名前> へ対応\nDATA_DIR=\"${DATA_DIR:-./data}\"\nmkdir -p \"$DATA_DIR\" \"$DATA_DIR/_inline\"\nrc=0\n");
    let step = Regex::new(r"^//(\S*)\s+EXEC\s+(.*)$").unwrap();
    let dd = Regex::new(r"^//(\S*)\s+DD\s*(.*)$").unwrap();
    let job = Regex::new(r"^//(\S+)\s+JOB\b").unwrap();
    let mut cur: Option<Step> = None;
    let mut i = 0;
    while i < lines.len() {
        let l = lines[i].clone();
        i += 1;
        if let Some(c) = l.strip_prefix("//*") {
            o.push_str(&format!("# {}\n", c.trim()));
        } else if let Some(c) = job.captures(&l) {
            o.push_str(&format!("# JOB {}\n", &c[1]));
        } else if let Some(c) = step.captures(&l) {
            finish(&mut cur, &mut o);
            let params = c[2].to_string();
            let pgm = Regex::new(r"PGM=([A-Z0-9#@$]+)").unwrap().captures(&params).map(|x| x[1].to_string());
            match pgm {
                Some(p) => {
                    let stepname = if c[1].is_empty() { "STEP".to_string() } else { c[1].to_string() };
                    let mut ex = vec![];
                    if let Some(pm) = Regex::new(r"PARM='([^']*)'|PARM=([^,\s]+)").unwrap().captures(&params) {
                        let v = pm.get(1).or(pm.get(2)).unwrap().as_str();
                        ex.push(format!("export PL2PL_PARM={}", shq(v)));
                    }
                    if params.contains("COND=") {
                        rep.push(format!("{name}: STEP {stepname} の COND= は未変換(前ステップの RC による実行制御を手で書く)"));
                    }
                    cur = Some((stepname, p, ex));
                }
                None => {
                    rep.push(format!("{name}: `{}` はカタログ手続き(PROC)の呼び出し。展開は未対応(手作業)", l.trim()));
                    o.push_str(&format!("# TODO(pl2pl): {}\n", l.trim()));
                }
            }
        } else if let Some(c) = dd.captures(&l) {
            let (ddn, params) = (c[1].to_string(), c[2].trim().to_string());
            let Some((s, _, ex)) = cur.as_mut() else { continue };
            if ddn.is_empty() {
                rep.push(format!("{name}: STEP {s} の連結DD(名前なし)は未変換"));
                continue;
            }
            if params.starts_with('*') || params.starts_with("DATA") {
                let mut data = vec![];
                while i < lines.len() && !lines[i].starts_with("/*") && !lines[i].starts_with("//") {
                    data.push(lines[i].clone());
                    i += 1;
                }
                if i < lines.len() && lines[i].starts_with("/*") {
                    i += 1;
                }
                let path = format!("$DATA_DIR/_inline/{s}.{ddn}");
                ex.push(format!("cat > \"{path}\" <<'PL2PL_EOF'\n{}\nPL2PL_EOF", data.join("\n")));
                ex.push(format!("export DD_{ddn}=\"{path}\""));
            } else if params.starts_with("DUMMY") {
                ex.push(format!("export DD_{ddn}=/dev/null"));
            } else if params.contains("SYSOUT=") {
                ex.push(format!("export DD_{ddn}=/dev/stdout"));
            } else if let Some(d) = Regex::new(r"DSN=([^,\s]+)").unwrap().captures(&params) {
                ex.push(format!("export DD_{ddn}=\"{}\"", dsn_path(&d[1])));
                if params.contains("DISP=(NEW") || params.contains("DISP=NEW") {
                    ex.push(format!("# DD {ddn}: 新規作成(DISP=NEW)。属性(RECFM/LRECL)は GnuCOBOL 側のファイル定義に合わせる"));
                }
            } else {
                rep.push(format!("{name}: STEP {s} の DD {ddn}({params})は未変換"));
            }
        } else if l.starts_with("/*") || l.trim() == "//" {
        } else if l.trim().len() > 2 {
            rep.push(format!("{name}: 解釈できない行: `{}`", l.trim().chars().take(50).collect::<String>()));
        }
    }
    finish(&mut cur, &mut o);
    o.push_str("\nexit $rc\n");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn jcl_basic() {
        let jcl = "//PAY JOB (1),'X'\n//S1 EXEC PGM=PAYROLL,PARM='2026'\n//INFILE DD DSN=PROD.IN.DATA,DISP=SHR\n//OUT DD SYSOUT=*\n//SYSIN DD *\nABC\n/*\n//S2 EXEC PGM=IEFBR14\n";
        let mut r = vec![];
        let b = jcl_to_bash(jcl, "p.jcl", &mut r);
        assert!(b.contains("export DD_INFILE=\"$DATA_DIR/PROD.IN.DATA\""));
        assert!(b.contains("export DD_OUT=/dev/stdout"));
        assert!(b.contains("./bin/PAYROLL"));
        assert!(b.contains("PL2PL_PARM='2026'"));
        assert!(b.contains("IEFBR14"));
        assert!(b.contains("ABC"));
    }
    #[test]
    fn cobol_scan() {
        let src = "       PROGRAM-ID. PAYROLL.\n       SELECT IN-F ASSIGN TO INFILE.\n       EXEC SQL SELECT 1 END-EXEC.\n";
        let mut r = vec![];
        scan_cobol(src, "a.cbl", &mut r);
        assert!(r.iter().any(|x| x.contains("DD_INFILE")));
        assert!(r.iter().any(|x| x.contains("EXEC SQL")));
    }
}
