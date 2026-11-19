//! `pl2pl c2rust`: とても単純な C(整数・小数の変数、四則、if / while / for、関数、printf)を Rust へ書き換える最初の一歩。
//! 行ごとの変換で、わからない行は黙って捨てず `// TODO(c2rust): 元の行` として残し、レポートにも載せる。
//! ポインタ・構造体・配列・マクロ・ファイル I/O は未対応(TODO のまま)。
use regex::Regex;

fn rty(t: &str) -> Option<&'static str> {
    match t.trim() {
        "int" | "short" | "unsigned" | "unsigned int" => Some("i32"),
        "long" | "long long" => Some("i64"),
        "double" => Some("f64"),
        "float" => Some("f32"),
        _ => None,
    }
}

/// printf の書式を Rust の `{}` へ。戻り値: (書式, 引数)
fn fmt(args: &str) -> Option<String> {
    let parts = split_args(args);
    let f = parts.first()?.trim();
    if !(f.starts_with('"') && f.ends_with('"') && f.len() >= 2) {
        return None;
    }
    let spec = Regex::new(r"%(\.\d+)?(l|ll)?[dfsc]|%%|%(l|ll)?u").unwrap();
    let mut n = 0;
    let body = &f[1..f.len() - 1];
    let out = spec.replace_all(body, |c: &regex::Captures| {
        let m = &c[0];
        if m == "%%" {
            return "%".to_string();
        }
        n += 1;
        format!("\u{1}{}\u{3}", c.get(1).map(|p| p.as_str()).unwrap_or(""))
    });
    // Rust の書式で意味を持つ { } をそのまま出すために倍にする(上で作った {} は壊さない)
    let esc = out.replace('{', "{{").replace('}', "}}");
    let tmp = Regex::new("\u{1}([^\u{3}]*)\u{3}").unwrap().replace_all(&esc, |c: &regex::Captures| if c[1].is_empty() { "{}".to_string() } else { format!("{{:{}}}", &c[1]) }).to_string();
    if n != parts.len() - 1 {
        return None;
    }
    let rest: Vec<String> = parts[1..].iter().map(|x| expr(x.trim())).collect();
    Some(if rest.is_empty() { format!("\"{tmp}\"") } else { format!("\"{tmp}\", {}", rest.join(", ")) })
}

fn split_args(s: &str) -> Vec<String> {
    let (mut v, mut d, mut q, mut cur) = (vec![], 0, false, String::new());
    let mut prev = ' ';
    for c in s.chars() {
        if c == '"' && prev != '\\' {
            q = !q;
        }
        if !q {
            if c == '(' {
                d += 1;
            }
            if c == ')' {
                d -= 1;
            }
            if c == ',' && d == 0 {
                v.push(std::mem::take(&mut cur));
                prev = c;
                continue;
            }
        }
        cur.push(c);
        prev = c;
    }
    if !cur.trim().is_empty() {
        v.push(cur);
    }
    v
}

thread_local! { static FLOATS: std::cell::RefCell<std::collections::HashSet<String>> = Default::default(); }

/// 小数の変数を含む式の中の整数リテラルを小数にする(Rust は f64 * 2 を許さないため)
fn float_lits(e: &str) -> String {
    let has = FLOATS.with(|f| Regex::new(r"[A-Za-z_]\w*").unwrap().find_iter(e).any(|m| f.borrow().contains(m.as_str())));
    if !has {
        return e.to_string();
    }
    Regex::new(r"\b(\d+)\b(\.\d+)?").unwrap().replace_all(e, |c: &regex::Captures| if c.get(2).is_some() { c[0].to_string() } else { format!("{}.0", &c[1]) }).to_string()
}

fn expr(e: &str) -> String {
    let e = &float_lits(e);
    // (double)x → x as f64
    let cast = Regex::new(r"\(\s*(int|long|double|float)\s*\)\s*(\w+)").unwrap();
    cast.replace_all(e, |c: &regex::Captures| format!("({} as {})", &c[2], rty(&c[1]).unwrap_or("i32"))).to_string()
}

pub fn convert(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let func = Regex::new(r"^(int|long|double|float|void)\s+(\w+)\s*\(([^)]*)\)\s*\{?\s*$").unwrap();
    let decl = Regex::new(r"^(int|long long|long|double|float|short|unsigned int|unsigned)\s+(\w+)\s*(=\s*(.+))?;$").unwrap();
    let forl = Regex::new(r"^for\s*\(\s*(?:int|long)?\s*(\w+)\s*=\s*([^;]+);\s*(\w+)\s*(<=|<)\s*([^;]+);\s*(\w+)(\+\+|\s*\+=\s*1)\s*\)\s*\{$").unwrap();
    let ifl = Regex::new(r"^(\}\s*else\s+)?if\s*\((.*)\)\s*\{$").unwrap();
    let whl = Regex::new(r"^while\s*\((.*)\)\s*\{$").unwrap();
    let call = Regex::new(r"^(printf|puts)\s*\((.*)\)\s*;$").unwrap();
    let inc = Regex::new(r"^(\w+)(\+\+|--)\s*;$").unwrap();
    FLOATS.with(|f| f.borrow_mut().clear());
    let mut out = String::from("// pl2pl c2rust で変換(最初の一歩)。TODO(c2rust) の行は手で直してください。\n#![allow(unused_mut, unused_variables, unused_parens)]\n\n");
    let mut in_main = false;
    let mut stack: Vec<bool> = vec![]; // 各 { が main の本体か
    let mut todo = 0;
    for (i, raw) in src.lines().enumerate() {
        let l = raw.split("//").next().unwrap_or("").trim();
        if l.is_empty() || l.starts_with("#include") {
            continue;
        }
        let ind = "    ".repeat(stack.len());
        if let Some(c) = func.captures(l) {
            let (ret, nm, params) = (&c[1], &c[2], c[3].trim());
            let mut ps = vec![];
            let mut ok = true;
            if params != "void" && !params.is_empty() {
                for p in params.split(',') {
                    let mut it = p.trim().rsplitn(2, char::is_whitespace);
                    let (v, t) = (it.next().unwrap_or(""), it.next().unwrap_or(""));
                    match rty(t) {
                        Some(rt) if !v.is_empty() => ps.push(format!("{v}: {rt}")),
                        _ => ok = false,
                    }
                }
            }
            if ok {
                let is_main = nm == "main";
                let r = if is_main || ret == "void" { String::new() } else { format!(" -> {}", rty(ret).unwrap_or("i32")) };
                out.push_str(&format!("{ind}fn {nm}({}){r} {{\n", ps.join(", ")));
                stack.push(is_main);
                in_main = is_main || in_main;
                continue;
            }
        }
        if l == "}" {
            if let Some(m) = stack.pop() {
                if m {
                    in_main = false;
                }
            }
            out.push_str(&format!("{}}}\n", "    ".repeat(stack.len())));
            continue;
        }
        if l.starts_with("} else {") || l == "else {" {
            out.push_str(&format!("{}}} else {{\n", "    ".repeat(stack.len().saturating_sub(1))));
            continue;
        }
        if let Some(c) = forl.captures(l) {
            if c[1] == c[3] && c[1] == c[6] {
                out.push_str(&format!("{ind}for {} in {}..{}{} {{\n", &c[1], expr(c[2].trim()), if &c[4] == "<=" { "=" } else { "" }, expr(c[5].trim())));
                stack.push(false);
                continue;
            }
        }
        if let Some(c) = ifl.captures(l) {
            let pre = if c.get(1).is_some() { format!("{}}} else ", "    ".repeat(stack.len().saturating_sub(1))) } else { ind.clone() };
            out.push_str(&format!("{pre}if {} {{\n", expr(c[2].trim())));
            if c.get(1).is_none() {
                stack.push(false);
            }
            continue;
        }
        if let Some(c) = whl.captures(l) {
            out.push_str(&format!("{ind}while {} {{\n", expr(c[1].trim())));
            stack.push(false);
            continue;
        }
        if let Some(c) = call.captures(l) {
            let done = if &c[1] == "puts" {
                let a = c[2].trim();
                if a.starts_with('"') && a.ends_with('"') { Some(format!("println!({a});")) } else { None }
            } else {
                fmt(&c[2]).map(|f| format!("print!({f});"))
            };
            if let Some(d) = done {
                out.push_str(&format!("{ind}{d}\n"));
                continue;
            }
        }
        if let Some(c) = decl.captures(l) {
            let t = rty(&c[1]).unwrap_or("i32");
            if t.starts_with('f') {
                FLOATS.with(|f| f.borrow_mut().insert(c[2].to_string()));
            }
            let init = c.get(4).map(|m| expr(m.as_str())).unwrap_or_else(|| if t.starts_with('f') { "0.0".into() } else { "0".into() });
            out.push_str(&format!("{ind}let mut {}: {t} = {init};\n", &c[2]));
            continue;
        }
        if let Some(c) = inc.captures(l) {
            out.push_str(&format!("{ind}{} {}= 1;\n", &c[1], if &c[2] == "++" { "+" } else { "-" }));
            continue;
        }
        if let Some(r) = l.strip_prefix("return") {
            let r = r.trim().trim_end_matches(';').trim();
            out.push_str(&if in_main && stack.iter().all(|m| *m || true) && stack.first() == Some(&true) { format!("{ind}return;\n") } else if r.is_empty() { format!("{ind}return;\n") } else { format!("{ind}return {};\n", expr(r)) });
            continue;
        }
        // 代入・複合代入(単純なもの)
        if Regex::new(r"^\w+\s*(=|\+=|-=|\*=|/=|%=)\s*[^=].*;$").unwrap().is_match(l) {
            out.push_str(&format!("{ind}{}\n", expr(l)));
            continue;
        }
        todo += 1;
        out.push_str(&format!("{ind}// TODO(c2rust): {l}\n"));
        rep.push(format!("{name}:{}: 未対応 `{l}`", i + 1));
    }
    if todo > 0 {
        rep.push(format!("{name}: 未対応の行 {todo} 個(TODO(c2rust) として残しました)"));
    }
    let base = name.rsplit('/').next().unwrap_or(name).trim_end_matches(".c");
    vec![(format!("{base}.rs"), out)]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut r = vec![];
        let o = convert("#include <stdio.h>\nint main(void) {\n  int s = 0;\n  for (int i = 1; i <= 3; i++) {\n    s += i;\n  }\n  printf(\"sum=%d %.1f\\n\", s, 2.5);\n  return 0;\n}\n", "a.c", &mut r);
        let t = &o[0].1;
        assert!(t.contains("fn main() {"), "{t}");
        assert!(t.contains("for i in 1..=3 {"), "{t}");
        assert!(t.contains("print!(\"sum={} {:.1}\\n\", s, 2.5);"), "{t}");
        assert!(r.is_empty(), "{r:?}");
    }
    #[test]
    fn unsupported_kept() {
        let mut r = vec![];
        let o = convert("int main() {\n  int *p = 0;\n}\n", "b.c", &mut r);
        assert!(o[0].1.contains("TODO(c2rust)") && !r.is_empty());
    }
}
