//! COBOL方言・版・書式の自動判定(根拠つき)。
#[derive(Debug, Clone, PartialEq)]
pub struct Detection {
    pub dialect: &'static str,
    pub standard: &'static str,
    pub free_format: bool,
    pub evidence: Vec<String>,
}

fn has(up: &str, k: &str) -> bool {
    up.contains(k)
}

pub fn detect(src: &str) -> Detection {
    let up = src.to_uppercase();
    let mut ev = Vec::new();
    let mut free = (has(&up, ">>SOURCE") && has(&up, "FREE")) || has(&up, "$SET SOURCEFORMAT\"FREE\"");
    if !free {
        let lines: Vec<&str> = src.lines().filter(|l| !l.trim().is_empty()).collect();
        let fixed_like = lines
            .iter()
            .filter(|l| {
                let b = l.as_bytes();
                b.len() > 7
                    && matches!(b[6], b' ' | b'*' | b'-' | b'/' | b'D')
                    && b[..6].iter().all(|c| c.is_ascii_digit() || *c == b' ')
            })
            .count();
        free = fixed_like * 2 < lines.len().max(1);
        ev.push(format!("書式: 固定形式らしい行 {fixed_like}/{}", lines.len()));
    }
    let mut dialect = "ISO/ANSI(汎用)";
    let mut std = "COBOL-85";
    if has(&up, "EXEC CICS")
        || (has(&up, "EXEC SQL") && has(&up, "DB2"))
        || has(&up, "JSON GENERATE")
        || has(&up, "XML PARSE")
    {
        dialect = "IBM Enterprise COBOL (メインフレーム)";
        ev.push("EXEC CICS/DB2/JSON GENERATE/XML PARSE".into());
    } else if has(&up, "$SET ") || has(&up, "ILUSING") {
        dialect = "Micro Focus";
        ev.push("$SET 指令".into());
    } else if has(&up, ">>SOURCE") || has(&up, "CBL_") || has(&up, "COB-") {
        dialect = "GnuCOBOL";
        ev.push("GnuCOBOL拡張(>>SOURCE/CBL_*)".into());
    } else if has(&up, "ACUCOBOL") {
        dialect = "ACUCOBOL-GT";
        ev.push("ACUCOBOL".into());
    } else if has(&up, "NETCOBOL") || has(&up, "FUJITSU") {
        dialect = "Fujitsu NetCOBOL";
        ev.push("NetCOBOL/FUJITSU".into());
    } else if has(&up, "HITAC") {
        dialect = "日立 COBOL";
        ev.push("HITAC".into());
    }
    if has(&up, "CLASS-ID") || has(&up, "INVOKE ") || has(&up, "METHOD-ID") {
        std = "COBOL 2002/2014 (オブジェクト指向)";
        ev.push("CLASS-ID/INVOKE".into());
    } else if has(&up, "FUNCTION ") && free {
        std = "COBOL 2002 以降";
        ev.push("組み込み関数+自由形式".into());
    } else if has(&up, "END-IF") || has(&up, "EVALUATE ") || has(&up, "END-PERFORM") {
        std = "COBOL-85";
        ev.push("END-IF/EVALUATE".into());
    } else if has(&up, "INSPECT ") || has(&up, "ALTER ") {
        std = "COBOL-74";
        ev.push("INSPECT/ALTER(85以前)".into());
    } else if has(&up, "EXAMINE ") || has(&up, "ENTER ") || has(&up, "NOTE ") {
        std = "COBOL-68 以前";
        ev.push("EXAMINE/ENTER/NOTE".into());
    }
    Detection { dialect, standard: std, free_format: free, evidence: ev }
}
