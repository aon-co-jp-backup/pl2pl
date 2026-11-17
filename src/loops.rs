//! C/C++ の `for` ループを見つけ、「繰り返しが互いに独立」と安全に言えるものだけを選ぶ(保守的)。
use regex::Regex;

pub struct Loop {
    pub start: usize,
    pub end: usize,
    pub var: String,
    pub init: String,
    pub cmp: String,
    pub bound: String,
    pub body: String,
    pub line: usize,
    pub indent: String,
}

pub struct Plan {
    pub reductions: Vec<(String, String)>, // (演算子, 変数)
}

fn matching(src: &str, open: usize, o: char, c: char) -> Option<usize> {
    let mut depth = 0;
    for (off, ch) in src[open..].char_indices() {
        if ch == o {
            depth += 1;
        } else if ch == c {
            depth -= 1;
            if depth == 0 {
                return Some(open + off);
            }
        }
    }
    None
}

pub fn find_loops(src: &str) -> Vec<Loop> {
    let re = Regex::new(r"for\s*\(\s*(?:(?:int|long|size_t|unsigned(?:\s+int)?|long\s+long)\s+)?([A-Za-z_]\w*)\s*=\s*([^;]+);\s*([A-Za-z_]\w*)\s*(<=|<)\s*([^;]+);\s*(?:([A-Za-z_]\w*)\+\+|\+\+([A-Za-z_]\w*)|([A-Za-z_]\w*)\s*\+=\s*1)\s*\)").unwrap();
    let mut v = vec![];
    for m in re.captures_iter(src) {
        let var = m[1].to_string();
        let inc = m.get(6).or(m.get(7)).or(m.get(8)).map(|x| x.as_str().to_string()).unwrap_or_default();
        if m[3] != var || inc != var {
            continue;
        }
        let whole = m.get(0).unwrap();
        let after = whole.end();
        let rest = &src[after..];
        let ws = rest.len() - rest.trim_start().len();
        let bstart = after + ws;
        let (body, end) = if src[bstart..].starts_with('{') {
            let Some(close) = matching(src, bstart, '{', '}') else { continue };
            (src[bstart + 1..close].to_string(), close + 1)
        } else {
            let Some(semi) = src[bstart..].find(';') else { continue };
            (src[bstart..bstart + semi + 1].to_string(), bstart + semi + 1)
        };
        let ls = src[..whole.start()].rfind('\n').map(|i| i + 1).unwrap_or(0);
        // 行頭以外(式の途中)にある for は対象にしない
        if !src[ls..whole.start()].trim().is_empty() {
            continue;
        }
        let indent: String = src[ls..whole.start()].to_string();
        v.push(Loop { start: whole.start(), end, var, init: m[2].trim().to_string(), cmp: m[4].to_string(), bound: m[5].trim().to_string(), body, line: crate::util::line_of(src, whole.start()), indent });
    }
    v
}

fn strip_balanced(s: &str, kw: &str) -> String {
    // `kw (...)` を丸ごと取り除く(入れ子の括弧に対応)
    let mut out = String::new();
    let re = Regex::new(&format!(r"\b{kw}\s*\(")).unwrap();
    let mut rest = s.to_string();
    while let Some(m) = re.find(&rest) {
        let open = m.end() - 1;
        let Some(close) = matching(&rest, open, '(', ')') else { break };
        out.push_str(&rest[..m.start()]);
        out.push(' ');
        rest = rest[close + 1..].to_string();
    }
    out.push_str(&rest);
    out
}

const SAFE_FUNCS: [&str; 22] = ["sin", "cos", "tan", "sqrt", "exp", "log", "log10", "pow", "fabs", "abs", "fmax", "fmin", "floor", "ceil", "fmod", "atan", "atan2", "sinh", "cosh", "tanh", "sizeof", "static_cast"];
const FORBID: [&str; 22] = ["break", "return", "goto", "printf", "puts", "scanf", "malloc", "free", "rand", "strtok", "exit", "realloc", "calloc", "fopen", "fwrite", "fread", "#pragma", "->", "cout", "cin", "while", "do "];

pub fn analyze(l: &Loop) -> Result<Plan, String> {
    let body = &l.body;
    for f in FORBID {
        if body.contains(f) {
            return Err(format!("本体に `{}` があるため(副作用・制御の飛び出しの恐れ)", f.trim()));
        }
    }
    let ty = r"(?:const\s+)?(?:int|long|double|float|size_t|unsigned|char|bool|auto|short)";
    let decl = Regex::new(&format!(r"\b{ty}\s+(?:\*\s*)?([A-Za-z_]\w*)")).unwrap();
    let locals: Vec<String> = decl.captures_iter(body).map(|c| c[1].to_string()).collect();
    // 入れ子の for の変数が外で宣言されていると競合する
    let inner = Regex::new(r"for\s*\(\s*((?:int|long|size_t|unsigned)\s+)?([A-Za-z_]\w*)\s*=").unwrap();
    for c in inner.captures_iter(body) {
        if c.get(1).is_none() && !locals.contains(&c[2].to_string()) {
            return Err(format!("入れ子のループ変数 `{}` が外で宣言されている(競合の恐れ)", &c[2]));
        }
    }
    let mut clean = strip_balanced(body, "for");
    clean = strip_balanced(&clean, "if");
    clean = Regex::new(r"\belse\b").unwrap().replace_all(&clean, " ").to_string();
    // 関数呼び出しは安全な数学関数だけ許す
    let call = Regex::new(r"\b([A-Za-z_]\w*)\s*\(").unwrap();
    for c in call.captures_iter(&clean) {
        if !SAFE_FUNCS.contains(&&c[1]) {
            return Err(format!("未知の関数 `{}` の呼び出し(副作用の恐れ)", &c[1]));
        }
    }
    let assign = Regex::new(&format!(r"^(?:{ty}\s+)?\*?\s*([A-Za-z_]\w*)((?:\s*\[[^\]]*\])*)\s*(\+=|-=|\*=|/=|=)\s*(.+)$")).unwrap();
    let incdec = Regex::new(r"^(?:\+\+|--)?\s*([A-Za-z_]\w*)\s*(?:\+\+|--)?$").unwrap();
    let decl_only = Regex::new(&format!(r"^{ty}\s+[A-Za-z_]\w*(?:\s*\[[^\]]*\])*$")).unwrap();
    let mut written: Vec<String> = vec![];
    let mut reds: Vec<(String, String)> = vec![];
    for stmt in clean.replace(['{', '}'], ";").split(';') {
        let st = stmt.trim();
        if st.is_empty() {
            continue;
        }
        if let Some(c) = assign.captures(st) {
            let (name, idx, op, rhs) = (c[1].to_string(), c[2].to_string(), c[3].to_string(), c[4].to_string());
            if rhs.starts_with('=') {
                return Err("比較式の文".into());
            }
            if st.starts_with('*') {
                return Err("ポインタ経由の書き込み".into());
            }
            if locals.contains(&name) {
                continue;
            }
            if !idx.is_empty() {
                let first = Regex::new(r"^\s*\[([^\]]*)\]").unwrap().captures(&idx).map(|x| x[1].trim().to_string()).unwrap_or_default();
                if first != l.var {
                    return Err(format!("配列 `{name}` の添字が `{}` ではない(要素間の依存の恐れ)", l.var));
                }
                written.push(name);
            } else if name == l.var {
                return Err("ループ変数への代入".into());
            } else if matches!(op.as_str(), "+=" | "-=" | "*=") {
                reds.push((op[..1].to_string(), name));
            } else {
                return Err(format!("外のスカラー `{name}` への代入(競合の恐れ)"));
            }
        } else if let Some(c) = incdec.captures(st) {
            if !locals.contains(&c[1].to_string()) && c[1] != l.var {
                return Err(format!("外の変数 `{}` の増減", &c[1]));
            }
        } else if decl_only.is_match(st) {
        } else {
            return Err(format!("解析できない文: `{}`", st.chars().take(40).collect::<String>()));
        }
    }
    let rd = Regex::new(r"\b([A-Za-z_]\w*)\s*\[([^\]]*)\]").unwrap();
    for c in rd.captures_iter(&clean) {
        if written.contains(&c[1].to_string()) && c[2].trim() != l.var {
            return Err(format!("書き込む配列 `{}` を別の添字 `{}` で読んでいる(依存の恐れ)", &c[1], c[2].trim()));
        }
    }
    reds.sort();
    reds.dedup();
    for (_, v) in &reds {
        let n = Regex::new(&format!(r"\b{}\b", regex::escape(v))).unwrap().find_iter(&clean).count();
        if n != 1 {
            return Err(format!("縮約変数 `{v}` が他の式でも使われている"));
        }
    }
    Ok(Plan { reductions: reds })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn first(src: &str) -> Result<Plan, String> {
        let l = find_loops(src);
        analyze(&l[0])
    }
    #[test]
    fn independent_ok() {
        assert!(first("for (int i = 0; i < n; i++) { c[i] = a[i] + b[i]; }").is_ok());
    }
    #[test]
    fn reduction() {
        let p = first("for (int i = 0; i < n; i++) { sum += a[i] * b[i]; }").unwrap();
        assert_eq!(p.reductions, vec![("+".to_string(), "sum".to_string())]);
    }
    #[test]
    fn dependence_rejected() {
        assert!(first("for (int i = 1; i < n; i++) { a[i] = a[i-1] + 1; }").is_err());
    }
    #[test]
    fn io_rejected() {
        assert!(first("for (int i = 0; i < n; i++) { printf(\"%d\", i); }").is_err());
    }
    #[test]
    fn outer_scalar_rejected() {
        assert!(first("for (int i = 0; i < n; i++) { last = a[i]; }").is_err());
    }
    #[test]
    fn nested_inner_outside_rejected() {
        assert!(first("for (int i = 0; i < n; i++) { for (j = 0; j < m; j++) { c[i] += 1.0; } }").is_err());
    }
}
