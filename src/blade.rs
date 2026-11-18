//! Blade(Laravel のテンプレート)→ Jinja2。FastAPI の `Jinja2Templates` で表示できる形にする。
//! 分からない式・ディレクティブは黙って落とさず `{# TODO(pl2pl): ... #}` として残し、レポートに出す。
use regex::Regex;

/// `users.index` → `users/index.html`
pub fn template_name(view: &str) -> String {
    format!("{}.html", view.trim().replace('.', "/"))
}

/// `resources/views/users/index.blade.php` → `users/index.html`
pub fn out_path(rel: &str) -> String {
    let r = rel.replace('\\', "/");
    let r = match r.find("resources/views/") {
        Some(i) => r[i + "resources/views/".len()..].to_string(),
        None => r,
    };
    format!("{}.html", r.trim_end_matches(".blade.php"))
}

fn split_quoted(s: &str) -> Vec<(bool, String)> {
    // (引用符の中か, 文字列)
    let mut v: Vec<(bool, String)> = vec![];
    let mut cur = String::new();
    let mut q: Option<char> = None;
    for ch in s.chars() {
        match q {
            Some(qc) => {
                cur.push(ch);
                if ch == qc {
                    v.push((true, std::mem::take(&mut cur)));
                    q = None;
                }
            }
            None if ch == '\'' || ch == '"' => {
                if !cur.is_empty() {
                    v.push((false, std::mem::take(&mut cur)));
                }
                cur.push(ch);
                q = Some(ch);
            }
            None => cur.push(ch),
        }
    }
    if !cur.is_empty() {
        v.push((q.is_some(), cur));
    }
    v
}

fn find_top(s: &str, pat: &str) -> Option<usize> {
    let (mut depth, mut q) = (0i32, None::<char>);
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        let ch = b[i] as char;
        if let Some(qc) = q {
            if ch == qc {
                q = None;
            }
        } else if ch == '\'' || ch == '"' {
            q = Some(ch);
        } else if ch == '(' || ch == '[' {
            depth += 1;
        } else if ch == ')' || ch == ']' {
            depth -= 1;
        } else if depth == 0 && s[i..].starts_with(pat) {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// PHP の式を Jinja2 の式へ。できなければ Err(理由)。
pub fn jexpr(e: &str) -> Result<String, String> {
    let e = e.trim();
    if let Some(i) = find_top(e, "??") {
        let (a, b) = (jexpr(&e[..i])?, jexpr(&e[i + 2..])?);
        return Ok(format!("({a} | default({b}))"));
    }
    if let Some(i) = find_top(e, "?") {
        // 三項演算子 a ? b : c(入れ子は非対応)
        if let Some(j) = find_top(&e[i + 1..], ":") {
            let (c, t, f) = (jexpr(&e[..i])?, jexpr(&e[i + 1..i + 1 + j])?, jexpr(&e[i + 2 + j..])?);
            return Ok(format!("({t} if {c} else {f})"));
        }
    }
    let mut s: String = split_quoted(e)
        .into_iter()
        .map(|(quoted, t)| {
            if quoted {
                return t;
            }
            let mut t = t;
            for (a, b) in [("===", "=="), ("!==", "!="), ("&&", " and "), ("||", " or "), ("$loop->iteration", "loop.index"), ("$loop->index", "loop.index0"), ("$loop->first", "loop.first"), ("$loop->last", "loop.last"), ("->", "."), (".total()", ".total"), (" . ", " ~ ")] {
                t = t.replace(a, b);
            }
            let t = Regex::new(r"!([^=])").unwrap().replace_all(&t, "not $1").to_string();
            let t = Regex::new(r"\$(\w+)").unwrap().replace_all(&t, "$1").to_string();
            Regex::new(r"\btrue\b").unwrap().replace_all(&Regex::new(r"\bfalse\b").unwrap().replace_all(&Regex::new(r"\bnull\b").unwrap().replace_all(&t, "none"), "False"), "True").to_string()
        })
        .collect();
    let fns: [(&str, &str); 9] = [
        (r"\bcount\(([^()]*)\)", "($1|length)"),
        (r"\bisset\(([^()]*)\)", "($1 is defined and $1 is not none)"),
        (r"\bempty\(([^()]*)\)", "(not $1)"),
        (r"\bstrtoupper\(([^()]*)\)", "($1|upper)"),
        (r"\bstrtolower\(([^()]*)\)", "($1|lower)"),
        (r"\bucfirst\(([^()]*)\)", "($1|capitalize)"),
        (r"\bstrlen\(([^()]*)\)", "($1|length)"),
        (r"\be\(([^()]*)\)", "($1)"),
        (r"\basset\(\s*'([^']*)'\s*\)", "'/static/$1'"),
    ];
    for _ in 0..4 {
        for (re, rep) in fns {
            s = Regex::new(re).unwrap().replace_all(&s, rep).to_string();
        }
    }
    for m in Regex::new(r"(?:^|[^.\w])(\w+)\(").unwrap().captures_iter(&s) {
        if !["default", "range", "super"].contains(&&m[1]) {
            return Err(format!("関数 `{}()` は未対応", &m[1]));
        }
    }
    if s.contains("::") || s.contains('$') {
        return Err("`::` や未対応の `$` を含む式は未対応".into());
    }
    Ok(Regex::new(r" {2,}").unwrap().replace_all(s.trim(), " ").to_string())
}

fn line_of(src: &str, at: usize) -> usize {
    src[..at.min(src.len())].matches('\n').count() + 1
}

fn matching_paren(s: &str, open: usize) -> Option<usize> {
    let (mut depth, mut q) = (0i32, None::<char>);
    for (off, ch) in s[open..].char_indices() {
        if let Some(qc) = q {
            if ch == qc {
                q = None;
            }
        } else if ch == '\'' || ch == '"' {
            q = Some(ch);
        } else if ch == '(' {
            depth += 1;
        } else if ch == ')' {
            depth -= 1;
            if depth == 0 {
                return Some(open + off);
            }
        }
    }
    None
}

fn split_top_commas(s: &str) -> Vec<String> {
    let (mut v, mut cur, mut depth, mut q) = (vec![], String::new(), 0i32, None::<char>);
    for ch in s.chars() {
        if let Some(qc) = q {
            cur.push(ch);
            if ch == qc {
                q = None;
            }
            continue;
        }
        match ch {
            '\'' | '"' => {
                q = Some(ch);
                cur.push(ch);
            }
            '(' | '[' | '{' => {
                depth += 1;
                cur.push(ch);
            }
            ')' | ']' | '}' => {
                depth -= 1;
                cur.push(ch);
            }
            ',' if depth == 0 => v.push(std::mem::take(&mut cur).trim().to_string()),
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() {
        v.push(cur.trim().to_string());
    }
    v
}

fn unquote(s: &str) -> String {
    s.trim().trim_matches(|c| c == '\'' || c == '"').to_string()
}

/// `['a' => $x, 'b' => 2]` → Jinja の `with a=x, b=2` 用の `a=x, b=2`
fn with_pairs(arr: &str) -> Result<String, String> {
    let a = arr.trim();
    let inner = a.strip_prefix('[').and_then(|x| x.strip_suffix(']')).ok_or("配列でない")?;
    let mut parts = vec![];
    for it in split_top_commas(inner) {
        let (k, v) = it.split_once("=>").ok_or("キー付きでない配列")?;
        parts.push(format!("{}={}", unquote(k), jexpr(v)?));
    }
    Ok(parts.join(", "))
}

/// 無名の Blade コンポーネント `<x-alert type="error">本文</x-alert>` → `{% with %}{% set slot %}..{% endset %}{% include %}{% endwith %}`
fn convert_components(src: &str, name: &str, warn: &mut Vec<String>) -> String {
    let attr = Regex::new(r#"(:?)([\w-]+)\s*=\s*"([^"]*)""#).unwrap();
    let tag = Regex::new(r"(?s)<x-([\w.-]+)((?:\s+[^<>]*?)?)\s*(/>|>)").unwrap();
    let slot = Regex::new(r#"(?s)<x-slot(?::(\w+)|\s+name="(\w+)")\s*>(.*?)</x-slot(?::\w+)?>"#).unwrap();
    let mut s = src.to_string();
    for _ in 0..50 {
        let Some(m) = tag.captures(&s) else { break };
        let whole = m.get(0).unwrap();
        let cname = m[1].to_string();
        if cname == "slot" {
            break;
        }
        let selfclose = &m[3] == "/>";
        let mut pairs: Vec<String> = vec![];
        let mut failed: Option<String> = None;
        for a in attr.captures_iter(&m[2]) {
            let key = a[2].replace('-', "_");
            let val = a[3].trim();
            let v = if &a[1] == ":" {
                match jexpr(val) {
                    Ok(j) => Some(j),
                    Err(e) => {
                        failed = Some(e);
                        None
                    }
                }
            } else if val.starts_with("{{") && val.ends_with("}}") && val.matches("{{").count() == 1 {
                Some(val[2..val.len() - 2].trim().to_string())
            } else {
                Some(format!("{:?}", val))
            };
            if let Some(v) = v {
                pairs.push(format!("{key}={v}"));
            }
        }
        let line = line_of(&s, whole.start());
        let (start, wend) = (whole.start(), whole.end());
        let (end, body) = if selfclose {
            (wend, String::new())
        } else {
            let close = format!("</x-{cname}>");
            match s[wend..].find(&close) {
                Some(c) => (wend + c + close.len(), s[wend..wend + c].to_string()),
                None => {
                    warn.push(format!("{name}:{line} <x-{cname}>: 閉じタグが無い"));
                    s.replace_range(start..wend, &format!("{{# TODO(pl2pl): <x-{cname}> #}}"));
                    continue;
                }
            }
        };
        if let Some(why) = failed {
            warn.push(format!("{name}:{line} <x-{cname}>: {why}"));
            s.replace_range(start..end, &format!("{{# TODO(pl2pl): <x-{cname}> #}}"));
            continue;
        }
        let mut sets = String::new();
        let mut rest = body.clone();
        for sl in slot.captures_iter(&body) {
            let n = sl.get(1).or(sl.get(2)).map(|x| x.as_str()).unwrap_or("");
            sets.push_str(&format!("{{% set {n} %}}{}{{% endset %}}", &sl[3]));
            rest = rest.replace(sl.get(0).unwrap().as_str(), "");
        }
        if !selfclose {
            sets.push_str(&format!("{{% set slot %}}{}{{% endset %}}", rest.trim()));
        }
        let t = format!("components/{}.html", cname.replace('.', "/"));
        let with = if pairs.is_empty() { String::new() } else { format!("{} ", pairs.join(", ")) };
        s.replace_range(start..end, &format!("{{% with {with}%}}{sets}{{% include \"{t}\" %}}{{% endwith %}}"));
    }
    s
}

pub fn convert(src: &str, name: &str, warn: &mut Vec<String>) -> String {
    // 1) コメント・出力
    let s = Regex::new(r"(?s)\{\{--(.*?)--\}\}").unwrap().replace_all(src, "{#$1#}").to_string();
    let echo_raw = Regex::new(r"(?s)\{!!(.*?)!!\}").unwrap();
    let mut errs: Vec<(usize, String)> = vec![];
    let s2 = echo_raw
        .replace_all(&s, |c: &regex::Captures| match jexpr(&c[1]) {
            Ok(j) => format!("{{{{ {j}|safe }}}}"),
            Err(why) => {
                errs.push((0, format!("{{!! {} !!}}: {why}", c[1].trim())));
                format!("{{# TODO(pl2pl): {{!! {} !!}} #}}", c[1].trim())
            }
        })
        .to_string();
    let echo = Regex::new(r"(?s)\{\{([^#].*?)\}\}").unwrap();
    let s3 = echo
        .replace_all(&s2, |c: &regex::Captures| {
            let inner = &c[1];
            if inner.trim_start().starts_with('-') {
                return c[0].to_string();
            }
            match jexpr(inner) {
                Ok(j) => format!("{{{{ {j} }}}}"),
                Err(why) => {
                    errs.push((0, format!("{{{{ {} }}}}: {why}", inner.trim())));
                    format!("{{# TODO(pl2pl): {{{{ {} }}}} #}}", inner.trim())
                }
            }
        })
        .to_string();
    for (_, e) in &errs {
        warn.push(format!("{name}: {e}"));
    }
    // 2) ディレクティブ
    let s3 = convert_components(&s3, name, warn);
    let known = Regex::new(r"@(\w+)").unwrap();
    let todo_names = ["php", "endphp", "can", "endcan", "cannot", "endcannot", "once", "endonce", "component", "endcomponent", "slot", "endslot", "livewire", "lang", "error", "enderror", "class", "style", "dd", "dump", "use", "inject", "continue", "while", "endwhile", "env", "endenv", "production", "endproduction", "hasSection", "sectionMissing", "csrf", "method", "vite"];
    let mut out = String::new();
    let mut i = 0;
    let mut stack: Vec<&str> = vec![]; // "forelse" / "other"
    let mut pushed: std::collections::HashSet<String> = std::collections::HashSet::new();
    while i < s3.len() {
        let Some(m) = known.find_at(&s3, i) else {
            out.push_str(&s3[i..]);
            break;
        };
        // メールアドレス(直前が英数字)や `@@` は対象外
        let prev = s3[..m.start()].chars().last();
        let dname = m.as_str()[1..].to_string();
        if prev.map(|c| c.is_alphanumeric() || c == '@' || c == '.').unwrap_or(false) {
            out.push_str(&s3[i..m.end()]);
            i = m.end();
            continue;
        }
        out.push_str(&s3[i..m.start()]);
        let after = m.end();
        let has_paren = s3[after..].starts_with('(');
        let (arg, end) = if has_paren {
            match matching_paren(&s3, after) {
                Some(c) => (s3[after + 1..c].to_string(), c + 1),
                None => (String::new(), after),
            }
        } else {
            (String::new(), after)
        };
        let line = line_of(&s3, m.start());
        macro_rules! fail {
            ($why:expr) => {{
                warn.push(format!("{name}:{line} @{dname}: {}", $why));
                out.push_str(&format!("{{# TODO(pl2pl): @{dname}({}) #}}", arg.trim()));
            }};
        }
        match dname.as_str() {
            "if" | "elseif" | "unless" => match jexpr(&arg) {
                Ok(j) => {
                    out.push_str(&match dname.as_str() {
                        "if" => format!("{{% if {j} %}}"),
                        "elseif" => format!("{{% elif {j} %}}"),
                        _ => format!("{{% if not ({j}) %}}"),
                    });
                    if dname != "elseif" {
                        stack.push("other");
                    }
                }
                Err(why) => fail!(&why),
            },
            "else" => out.push_str("{% else %}"),
            "endif" | "endunless" | "endisset" => {
                stack.pop();
                out.push_str("{% endif %}");
            }
            "isset" => match jexpr(&arg) {
                Ok(j) => {
                    out.push_str(&format!("{{% if {j} is defined and {j} is not none %}}"));
                    stack.push("other");
                }
                Err(why) => fail!(&why),
            },
            "foreach" | "forelse" => {
                let re = Regex::new(r"^\s*(.+?)\s+as\s+\$(\w+)(?:\s*=>\s*\$(\w+))?\s*$").unwrap();
                match re.captures(&arg).map(|c| (jexpr(&c[1]), c[2].to_string(), c.get(3).map(|x| x.as_str().to_string()))) {
                    Some((Ok(list), a, None)) => out.push_str(&format!("{{% for {a} in {list} %}}")),
                    Some((Ok(list), k, Some(v))) => out.push_str(&format!("{{% for {k}, {v} in {list}.items() %}}")),
                    Some((Err(why), _, _)) => {
                        fail!(&why);
                        continue_with(&mut stack, "other");
                        i = end;
                        continue;
                    }
                    None => {
                        fail!("`$配列 as $要素` の形でない");
                        continue_with(&mut stack, "other");
                        i = end;
                        continue;
                    }
                }
                stack.push(if dname == "forelse" { "forelse" } else { "other" });
            }
            "empty" if !has_paren && stack.last() == Some(&"forelse") => out.push_str("{% else %}"),
            "empty" => match jexpr(&arg) {
                Ok(j) => {
                    out.push_str(&format!("{{% if not {j} %}}"));
                    stack.push("other");
                }
                Err(why) => fail!(&why),
            },
            "endforeach" | "endforelse" | "endfor" => {
                stack.pop();
                out.push_str("{% endfor %}");
            }
            "for" => {
                let re = Regex::new(r"^\s*\$(\w+)\s*=\s*(\d+)\s*;\s*\$(\w+)\s*(<|<=)\s*([^;]+?)\s*;\s*\$(\w+)\+\+\s*$").unwrap();
                match re.captures(&arg) {
                    Some(c) if c[1] == c[3] && c[1] == c[6] => match jexpr(&c[5]) {
                        Ok(hi) => {
                            let hi = if &c[4] == "<=" { format!("({hi}) + 1") } else { hi };
                            out.push_str(&format!("{{% for {} in range({}, {}) %}}", &c[1], &c[2], hi));
                            stack.push("other");
                        }
                        Err(why) => fail!(&why),
                    },
                    _ => fail!("単純な `for ($i = 0; $i < n; $i++)` 以外は未対応"),
                }
            }
            "extends" => out.push_str(&format!("{{% extends \"{}\" %}}", template_name(&unquote(&arg)))),
            "include" | "includeIf" => {
                let parts = split_top_commas(&arg);
                let t = template_name(&unquote(parts.first().map(|x| x.as_str()).unwrap_or("")));
                match parts.get(1).map(|a| with_pairs(a)) {
                    None => out.push_str(&format!("{{% include \"{t}\" %}}")),
                    Some(Ok(p)) => out.push_str(&format!("{{% with {p} %}}{{% include \"{t}\" %}}{{% endwith %}}")),
                    Some(Err(why)) => fail!(&why),
                }
            }
            "section" => {
                let parts = split_top_commas(&arg);
                if parts.len() == 2 {
                    match jexpr(&parts[1]) {
                        Ok(v) => out.push_str(&format!("{{% block {} %}}{{{{ {v} }}}}{{% endblock %}}", unquote(&parts[0]))),
                        Err(why) => fail!(&why),
                    }
                } else {
                    out.push_str(&format!("{{% block {} %}}", unquote(&arg)));
                }
            }
            "endsection" | "show" | "stop" | "append" => out.push_str("{% endblock %}"),
            "yield" => {
                let parts = split_top_commas(&arg);
                let def = parts.get(1).map(|d| jexpr(d).map(|j| format!("{{{{ {j} }}}}")).unwrap_or_default()).unwrap_or_default();
                out.push_str(&format!("{{% block {} %}}{def}{{% endblock %}}", unquote(parts.first().map(|x| x.as_str()).unwrap_or(""))));
            }
            "parent" => out.push_str("{{ super() }}"),
            "props" => {
                let inner = arg.trim().strip_prefix('[').and_then(|x| x.strip_suffix(']')).unwrap_or("");
                for it in split_top_commas(inner) {
                    if let Some((k, v)) = it.split_once("=>") {
                        match jexpr(v) {
                            Ok(j) => out.push_str(&format!("{{% set {k} = {k}|default({j}) %}}", k = unquote(k))),
                            Err(why) => fail!(&why),
                        }
                    }
                }
            }
            "stack" => out.push_str(&format!("{{% block stack_{} %}}{{% endblock %}}", unquote(&arg))),
            "push" | "prepend" => {
                let n = unquote(&arg);
                if pushed.insert(n.clone()) {
                    out.push_str(&format!("{{% block stack_{n} %}}{{{{ super() }}}}"));
                } else {
                    fail!("同じスタックへの2回目の @push は未対応(Jinja2 のブロックは名前が1つだけ)");
                    out.push_str("{% block stack_dup_unsupported %}");
                }
            }
            "endpush" | "endprepend" => out.push_str("{% endblock %}"),
            "auth" | "guest" => {
                warn.push(format!("{name}:{line} @{dname}: 簡易判定(Authorization ヘッダーの有無)。本物の認証へ差し替えること"));
                let c = if dname == "auth" { "request.headers.get('authorization')" } else { "not request.headers.get('authorization')" };
                out.push_str(&format!("{{% if {c} %}}"));
                stack.push("other");
            }
            "endauth" | "endguest" | "endswitch" => {
                stack.pop();
                out.push_str("{% endif %}");
            }
            "switch" => match jexpr(&arg) {
                Ok(j) => {
                    out.push_str(&format!("{{% set _sw = {j} %}}{{% if false %}}"));
                    stack.push("other");
                }
                Err(why) => fail!(&why),
            },
            "case" => match jexpr(&arg) {
                Ok(j) => out.push_str(&format!("{{% elif _sw == {j} %}}")),
                Err(why) => fail!(&why),
            },
            "default" => out.push_str("{% else %}"),
            "break" => {}
            "checked" | "selected" | "disabled" => match jexpr(&arg) {
                Ok(j) => out.push_str(&format!("{{{{ '{dname}' if {j} else '' }}}}")),
                Err(why) => fail!(&why),
            },
            "json" => match jexpr(&arg) {
                Ok(j) => out.push_str(&format!("{{{{ {j}|tojson }}}}")),
                Err(why) => fail!(&why),
            },
            "csrf" | "method" => {
                warn.push(format!("{name}:{line} @{dname}: FastAPI に CSRF/メソッド偽装の仕組みは無いため出力しない(必要なら別途実装)"));
                out.push_str(&format!("{{# pl2pl: @{dname} は出力しない #}}"));
            }
            n if todo_names.contains(&n) => fail!("未対応のディレクティブ"),
            _ => {
                // @media など Blade でないもの
                out.push_str(&s3[m.start()..end.max(m.end())]);
            }
        }
        i = end.max(m.end());
    }
    out
}

fn continue_with(stack: &mut Vec<&'static str>, kind: &'static str) {
    // 変換に失敗した開始ディレクティブでも、対応する終了を数え違えないように積んでおく
    stack.push(kind);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stacks_auth_switch() {
        let mut w = vec![];
        let o = convert("@push('js')<a>@endpush @stack('js') @auth yes @endauth @switch($n) @case(1) one @break @default other @endswitch <i @checked($on)> @json($x)", "t", &mut w);
        assert!(o.contains("{% block stack_js %}{{ super() }}<a>{% endblock %}"), "{o}");
        assert!(o.contains("{% block stack_js %}{% endblock %}"), "{o}");
        assert!(o.contains("{% if request.headers.get('authorization') %} yes {% endif %}"), "{o}");
        assert!(o.contains("{% set _sw = n %}{% if false %}") && o.contains("{% elif _sw == 1 %}") && o.contains("{% else %}"), "{o}");
        assert!(o.contains("{{ 'checked' if on else '' }}") && o.contains("{{ x|tojson }}"), "{o}");
        let mut w2 = vec![];
        convert("@push('a')x@endpush @push('a')y@endpush", "t", &mut w2);
        assert!(w2.iter().any(|x| x.contains("2回目")), "{w2:?}");
    }

    #[test]
    fn components() {
        let mut w = vec![];
        let o = convert("<x-alert type=\"error\" :n=\"$count\" label=\"{{ $l }}\">Hi <b>x</b></x-alert><x-forms.input name=\"a\" />", "t", &mut w);
        assert!(o.contains("{% with type=\"error\", n=count, label=l %}{% set slot %}Hi <b>x</b>{% endset %}{% include \"components/alert.html\" %}{% endwith %}"), "{o}");
        assert!(o.contains("{% with name=\"a\" %}{% include \"components/forms/input.html\" %}{% endwith %}"), "{o}");
        let p = convert("@props(['type' => 'info'])<p class=\"{{ $type }}\">{{ $slot }}</p>", "c", &mut w);
        assert!(p.contains("{% set type = type|default('info') %}") && p.contains("{{ slot }}"), "{p}");
    }

    #[test]
    fn basic_blade() {
        let mut w = vec![];
        let src = "@extends('layouts.app')\n@section('content')\n<h1>{{ $title }}</h1>\n@foreach($users as $u)\n<p>{{ $u->name }} {!! $u->bio !!}</p>\n@endforeach\n@if(count($users) > 0 && $show)\nyes\n@else\nno\n@endif\n@endsection\n";
        let o = convert(src, "x.blade.php", &mut w);
        assert!(o.contains("{% extends \"layouts/app.html\" %}"), "{o}");
        assert!(o.contains("{% block content %}") && o.contains("{% endblock %}"));
        assert!(o.contains("{{ title }}") && o.contains("{{ u.name }}") && o.contains("{{ u.bio|safe }}"));
        assert!(o.contains("{% for u in users %}") && o.contains("{% endfor %}"));
        assert!(o.contains("{% if (users|length) > 0 and show %}"), "{o}");
        assert!(w.is_empty(), "{w:?}");
    }
    #[test]
    fn forelse_and_todo() {
        let mut w = vec![];
        let o = convert("@forelse($a as $k => $v){{ $k }}@empty none @endforelse @php echo 1; @endphp a@b.com", "y", &mut w);
        assert!(o.contains("{% for k, v in a.items() %}") && o.contains("{% else %}") && o.contains("{% endfor %}"), "{o}");
        assert!(o.contains("TODO(pl2pl): @php") && o.contains("a@b.com"));
        assert!(!w.is_empty());
        assert_eq!(out_path("resources/views/users/index.blade.php"), "users/index.html");
    }
}
