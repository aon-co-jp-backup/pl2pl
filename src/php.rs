//! PHP(Laravel / Symfony)→ FastAPI の骨組み。ルートと引数の型を写し、本体は TODO(黙って落とさない)。
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    pub verb: String,
    pub path: String,
    pub handler: String, // 表示用 "UserController@index" / "closure"
    pub class: String,
    pub method: String,
    pub body: Option<String>, // 単純なクロージャの翻訳済み return 式
    pub mw: Vec<String>,      // ミドルウェア名(グループ・ルート指定・コントローラの __construct)
    pub raw: Option<String>,  // クロージャ本体の PHP 原文(複数文の変換用)
    pub src: String,
    pub line: usize,
}

pub(crate) fn matching(s: &str, open: usize, o: char, c: char) -> Option<usize> {
    let mut depth = 0;
    let mut quote: Option<char> = None;
    for (off, ch) in s[open..].char_indices() {
        if let Some(q) = quote {
            if ch == q {
                quote = None;
            }
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
        } else if ch == o {
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

pub(crate) fn split_args(s: &str) -> Vec<String> {
    // トップレベルのカンマで分ける
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
            ',' if depth == 0 => {
                v.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() {
        v.push(cur.trim().to_string());
    }
    v
}

pub(crate) fn unq(s: &str) -> String {
    s.trim().trim_matches(|c| c == '\'' || c == '"').to_string()
}

pub fn snake(s: &str) -> String {
    let mut o = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !o.ends_with('_') {
                o.push('_');
            }
            o.extend(c.to_lowercase());
        } else if c.is_alphanumeric() {
            o.push(c);
        } else if !o.ends_with('_') {
            o.push('_');
        }
    }
    o.trim_matches('_').to_string()
}

/// 単純な PHP の値を Python へ(文字列・数・真偽・平らな配列)。できなければ None。
pub(crate) fn php_value(s: &str) -> Option<String> {
    let s = s.trim().trim_end_matches(';').trim();
    if let Some(i) = s.strip_prefix("@py(").and_then(|x| x.strip_suffix(')')) {
        return Some(i.to_string()); // translate_body が差し込んだ Python 式
    }
    if (s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2) || (s.starts_with('"') && s.ends_with('"') && s.len() >= 2 && !s.contains('$')) {
        return Some(format!("{:?}", &s[1..s.len() - 1]));
    }
    if s.parse::<f64>().is_ok() {
        return Some(s.to_string());
    }
    if Regex::new(r"^\$\w+$").unwrap().is_match(s) {
        return Some(s[1..].to_string()); // 引数(パス変数)をそのまま返す
    }
    match s {
        "true" => return Some("True".into()),
        "false" => return Some("False".into()),
        "null" => return Some("None".into()),
        _ => {}
    }
    let inner = if s.starts_with('[') && s.ends_with(']') { &s[1..s.len() - 1] } else if s.to_lowercase().starts_with("array(") && s.ends_with(')') { &s[6..s.len() - 1] } else { return None };
    let items = split_args(inner);
    if items.iter().all(|i| i.contains("=>")) && !items.is_empty() {
        let mut parts = vec![];
        for it in items {
            let (k, v) = it.split_once("=>")?;
            parts.push(format!("{}: {}", php_value(k)?, php_value(v)?));
        }
        return Some(format!("{{{}}}", parts.join(", ")));
    }
    if items.iter().all(|i| !i.contains("=>")) {
        let vs: Option<Vec<String>> = items.iter().map(|i| php_value(i)).collect();
        return vs.map(|v| format!("[{}]", v.join(", ")));
    }
    None
}

/// クロージャ本体が `return <単純な値>;` だけならその値(Python)。`response()->json(...)` / `Response::json` も許す。
fn simple_return(body: &str) -> Option<String> {
    let b = body.trim();
    let b = b.strip_prefix("return")?.trim().trim_end_matches(';').trim();
    let b = Regex::new(r"^(?:response\(\)->json|Response::json|json_encode)\((.*)\)$").unwrap().captures(b).map(|c| c[1].to_string()).unwrap_or(b.to_string());
    let b = split_args(&b).into_iter().next().unwrap_or_default();
    php_value(&b)
}

/// `return Model::find($id);` など単純な Eloquent の静的呼び出し → Python(`Model.find(id)`)。使うモデル名も返す。
fn eloquent_return(body: &str) -> Option<(String, String)> {
    let b = body.trim().strip_prefix("return")?.trim().trim_end_matches(';').trim();
    let c = Regex::new(r"^(\w+)::(all|find|findOrFail|create|destroy)\((.*)\)$").unwrap().captures(b)?;
    let (model, op, arg) = (c[1].to_string(), c[2].to_string(), c[3].trim().to_string());
    let a = if Regex::new(r"^\$\w+$").unwrap().is_match(&arg) { arg[1..].to_string() } else if Regex::new(r"^\$\w+->all\(\)$").unwrap().is_match(&arg) { "payload or {}".to_string() } else if arg.is_empty() { String::new() } else { return None };
    let op = if op == "findOrFail" { "find_or_fail" } else { op.as_str() };
    Some((model.clone(), format!("{model}.{op}({a})")))
}

/// トップレベルの `;` で文に分ける(文字列・括弧の中は分けない)。
fn split_stmts(s: &str) -> Vec<String> {
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
            ';' if depth == 0 => {
                v.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() {
        v.push(cur.trim().to_string());
    }
    v.into_iter().filter(|x| !x.is_empty()).collect()
}

/// 式を Python へ(`$v`・`$v->f`・`$request->all()/input('k')`・単純な値・配列)。できなければ None。
fn arg_expr(a: &str) -> Option<String> {
    let a = a.trim();
    if a.is_empty() {
        return Some(String::new());
    }
    if a == "$request->all()" {
        return Some("(payload or {})".into());
    }
    if let Some(c) = Regex::new(r#"^\$request->query\(\s*['"](\w+)['"]\s*(?:,\s*([^,]+?))?\s*\)$"#).unwrap().captures(a) {
        return Some(match c.get(2).and_then(|d| php_value(d.as_str())) {
            Some(d) => format!("request.query_params.get({:?}, {d})", &c[1]),
            None => format!("request.query_params.get({:?})", &c[1]),
        });
    }
    if let Some(c) = Regex::new(r#"^\$request->(?:input|get|post)\(\s*['"](\w+)['"]\s*\)$"#).unwrap().captures(a) {
        return Some(format!("(payload or {{}}).get({:?})", &c[1]));
    }
    if let Some(c) = Regex::new(r"^\$(\w+)->(\w+)$").unwrap().captures(a) {
        return Some(format!("{}[{:?}]", &c[1], &c[2]));
    }
    php_value(a)
}

/// 複数文の本体(Eloquent の取得→更新→保存→返却)を Python の行へ。1文でも分からなければ None(全体を TODO にする)。
fn translate_body(body: &str, models: &mut Vec<String>, need_response: &mut bool, need_tpl: &mut bool, rels: &Rels) -> Option<Vec<String>> {
    let mut vars: HashMap<String, String> = HashMap::new(); // 変数 → モデル
    let mut lines: Vec<String> = vec![];
    let mut returned = false;
    let mut add_model = |m: &str, models: &mut Vec<String>| {
        if !models.iter().any(|x| x == m) {
            models.push(m.to_string());
        }
    };
    for st in split_stmts(body) {
        if returned {
            return None; // return の後に文がある
        }
        let mut st = st;
        for (v, model) in &vars {
            for r in rels.get(model).into_iter().flatten() {
                st = subst_rel(&st, v, model, &r.name);
            }
        }
        let st = st;
        let assign_rel = Regex::new(r"^\$(\w+)\s*=\s*(.+)$").unwrap().captures(&st).and_then(|c| rel_call(c[2].trim(), &vars, rels).map(|x| (c[1].to_string(), x)));
        if let Some((v, (py, result))) = assign_rel {
            if let Some(m) = result {
                add_model(&m, models);
                vars.insert(v.clone(), m);
            }
            lines.push(format!("{v} = {py}"));
            continue;
        }
        if let Some((py, _)) = rel_call(&st, &vars, rels) {
            lines.push(py);
            continue;
        }
        if let Some(r) = st.strip_prefix("return") {
            if let Some((py, _)) = rel_call(r.trim(), &vars, rels) {
                lines.push(format!("return {py}"));
                returned = true;
                continue;
            }
        }
        if let Some(c) = Regex::new(r"^\$(\w+)\s*=\s*(.+)$").unwrap().captures(&st) {
            if let Some((model, py, one)) = query_expr(&c[2]) {
                add_model(&model, models);
                if one {
                    vars.insert(c[1].to_string(), model);
                }
                lines.push(format!("{} = {}", &c[1], py));
                continue;
            }
        }
        if let Some(r) = st.strip_prefix("return") {
            if let Some((model, py, _)) = query_expr(r) {
                add_model(&model, models);
                lines.push(format!("return {py}"));
                returned = true;
                continue;
            }
        }
        if let Some(c) = Regex::new(r"^\$(\w+)\s*=\s*(\w+)::(find|findOrFail|create)\((.*)\)$").unwrap().captures(&st) {
            let op = if &c[3] == "findOrFail" { "find_or_fail" } else { &c[3] };
            let a = arg_expr(&c[4])?;
            add_model(&c[2], models);
            vars.insert(c[1].to_string(), c[2].to_string());
            lines.push(format!("{} = {}.{}({})", &c[1], &c[2], op, a));
        } else if let Some(c) = Regex::new(r"^\$(\w+)->update\((.*)\)$").unwrap().captures(&st) {
            let model = vars.get(&c[1])?.clone();
            let a = arg_expr(&c[2])?;
            lines.push(format!("{} = {}.update({}[\"id\"], {})", &c[1], model, &c[1], a));
        } else if let Some(c) = Regex::new(r"^\$(\w+)->delete\(\)$").unwrap().captures(&st) {
            let model = vars.get(&c[1])?.clone();
            lines.push(format!("{}.destroy({}[\"id\"])", model, &c[1]));
        } else if let Some(c) = Regex::new(r"^\$(\w+)->save\(\)$").unwrap().captures(&st) {
            let model = vars.get(&c[1])?.clone();
            lines.push(format!("{} = {}.update({}[\"id\"], {})", &c[1], model, &c[1], &c[1]));
        } else if let Some(c) = Regex::new(r"^\$(\w+)->(\w+)\s*=\s*(.+)$").unwrap().captures(&st) {
            vars.get(&c[1])?;
            lines.push(format!("{}[{:?}] = {}", &c[1], &c[2], arg_expr(&c[3])?));
        } else if let Some(c) = Regex::new(r"^\$(\w+)\s*=\s*(.+)$").unwrap().captures(&st) {
            lines.push(format!("{} = {}", &c[1], arg_expr(&c[2])?));
        } else if let Some(r) = st.strip_prefix("return") {
            let r = r.trim();
            let r = r.trim();
            if let Some(e) = view_expr(r) {
                *need_tpl = true;
                lines.push(format!("return {e}"));
            } else if Regex::new(r"^response\(\)->noContent\(\)$").unwrap().is_match(r) {
                *need_response = true;
                lines.push("return Response(status_code=204)".into());
            } else if let Some(c) = Regex::new(r"^(?:response\(\)->json|Response::json)\((.*)\)$").unwrap().captures(r) {
                let args = split_args(&c[1]);
                let v = arg_expr(args.first()?)?;
                if let Some(code) = args.get(1).and_then(|x| x.trim().parse::<u16>().ok()) {
                    *need_response = true;
                    lines.push(format!("return JSONResponse({v}, status_code={code})"));
                } else {
                    lines.push(format!("return {v}"));
                }
            } else {
                lines.push(format!("return {}", arg_expr(r)?));
            }
            returned = true;
        } else {
            return None;
        }
    }
    if returned { Some(lines) } else { None }
}

/// `a->b(x)->c(y)` をトップレベルの `->` で分ける(文字列・括弧の中は分けない)。
fn split_chain(s: &str) -> Vec<String> {
    let (mut v, mut cur, mut depth, mut q) = (vec![], String::new(), 0i32, None::<char>);
    let cs: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        let ch = cs[i];
        if let Some(qc) = q {
            cur.push(ch);
            if ch == qc {
                q = None;
            }
            i += 1;
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
            '-' if depth == 0 && cs.get(i + 1) == Some(&'>') => {
                v.push(cur.trim().to_string());
                cur.clear();
                i += 1;
            }
            _ => cur.push(ch),
        }
        i += 1;
    }
    v.push(cur.trim().to_string());
    v
}

/// Eloquent のクエリビルダ(`Model::where(..)->orderBy(..)->get()` など)→ `Model.query().where(..).get()`。
/// 戻り値は (モデル名, Python 式, 1行(辞書)を返すか)。
fn query_expr(expr: &str) -> Option<(String, String, bool)> {
    let e = expr.trim().trim_end_matches(';').trim();
    let head = Regex::new(r"^(\w+)::(.*)$").unwrap().captures(e)?;
    let model = head[1].to_string();
    let chain = split_chain(&head[2]);
    let call = Regex::new(r"^(\w+)\((.*)\)$").unwrap();
    let mut py = format!("{model}.query()");
    let mut terminal = false;
    let mut one = false;
    let mut first = true;
    let mut paged = false;
    for seg in chain {
        let c = call.captures(&seg)?;
        let (name, raw) = (c[1].to_string(), c[2].to_string());
        let args: Vec<String> = if raw.trim().is_empty() { vec![] } else { split_args(&raw) };
        if terminal && paged && (name == "withQueryString" || name == "appends") {
            continue; // links() が現在のクエリ文字列を常に引き継ぐので不要
        }
        if terminal {
            return None; // 終端(get/first/...)の後に続きがある
        }
        if (name == "paginate" || name == "simplePaginate") && args.len() <= 1 {
            let n = match args.first() {
                Some(x) => arg_expr(x)?,
                None => "15".to_string(),
            };
            terminal = true;
            paged = true;
            py.push_str(&format!(".paginate({n}, request)"));
            first = false;
            continue;
        }
        if matches!(name.as_str(), "whereHas" | "has" | "whereDoesntHave" | "doesntHave") {
            let neg = if name.contains("oesnt") { ", True" } else { "" };
            let rel = unq(args.first()?);
            let conds = match args.get(1) {
                None => "[]".to_string(),
                Some(c) => has_conds(c)?,
            };
            py.push_str(&format!(".where_has({rel:?}, {conds}{neg})"));
            first = false;
            continue;
        }
        let a: Option<Vec<String>> = args.iter().map(|x| arg_expr(x)).collect();
        let a = a?;
        let m = match (name.as_str(), a.len()) {
            ("query", 0) if first => continue,
            ("with", 1) => format!(".with_({})", a[0]),
            ("where", 2) => format!(".where({}, \"=\", {})", a[0], a[1]),
            ("where", 3) => format!(".where({}, {}, {})", a[0], a[1], a[2]),
            ("orWhere", 2) => format!(".or_where({}, \"=\", {})", a[0], a[1]),
            ("orWhere", 3) => format!(".or_where({}, {}, {})", a[0], a[1], a[2]),
            ("whereIn", 2) => format!(".where({}, \"in\", {})", a[0], a[1]),
            ("orderBy", 1) => format!(".order_by({}, \"asc\")", a[0]),
            ("orderBy", 2) => format!(".order_by({}, {})", a[0], a[1]),
            ("latest", 0) => ".order_by(\"id\", \"desc\")".to_string(),
            ("latest", 1) => format!(".order_by({}, \"desc\")", a[0]),
            ("oldest", 0) => ".order_by(\"id\", \"asc\")".to_string(),
            ("oldest", 1) => format!(".order_by({}, \"asc\")", a[0]),
            ("limit", 1) | ("take", 1) => format!(".limit({})", a[0]),
            ("offset", 1) | ("skip", 1) => format!(".offset({})", a[0]),
            ("get", 0) | ("all", 0) => {
                terminal = true;
                ".get()".to_string()
            }
            ("first", 0) => {
                terminal = true;
                one = true;
                ".first()".to_string()
            }
            ("firstOrFail", 0) => {
                terminal = true;
                one = true;
                ".first_or_fail()".to_string()
            }
            ("count", 0) => {
                terminal = true;
                ".count()".to_string()
            }
            ("exists", 0) => {
                terminal = true;
                ".exists()".to_string()
            }
            ("delete", 0) => {
                terminal = true;
                ".delete()".to_string()
            }
            _ => return None,
        };
        first = false;
        py.push_str(&m);
    }
    if !terminal {
        return None;
    }
    Some((model, py, one))
}

/// `view('a.b', [...])` / `view('a.b', compact('x'))` / `->with(...)` → `templates.TemplateResponse(request, "a/b.html", {...})`。
fn view_expr(r: &str) -> Option<String> {
    let segs = split_chain(r.trim().trim_end_matches(';'));
    let first = Regex::new(r"^view\((.*)\)$").unwrap().captures(&segs[0])?;
    let args = split_args(&first[1]);
    let name = crate::blade::template_name(&unq(args.first()?));
    let mut pairs: Vec<String> = vec![];
    let add_array = |arr: &str, pairs: &mut Vec<String>| -> Option<()> {
        let a = arr.trim();
        if let Some(c) = Regex::new(r"^compact\((.*)\)$").unwrap().captures(a) {
            for n in split_args(&c[1]) {
                let n = unq(&n);
                pairs.push(format!("{n:?}: {n}"));
            }
            return Some(());
        }
        let inner = a.strip_prefix('[').and_then(|x| x.strip_suffix(']'))?;
        for it in split_args(inner) {
            let (k, v) = it.split_once("=>")?;
            pairs.push(format!("{:?}: {}", unq(k), arg_expr(v)?));
        }
        Some(())
    };
    if let Some(a) = args.get(1) {
        add_array(a, &mut pairs)?;
    }
    for seg in &segs[1..] {
        let c = Regex::new(r"^with\((.*)\)$").unwrap().captures(seg)?;
        let wa = split_args(&c[1]);
        if wa.len() == 2 {
            pairs.push(format!("{:?}: {}", unq(&wa[0]), arg_expr(&wa[1])?));
        } else if wa.len() == 1 {
            add_array(&wa[0], &mut pairs)?;
        } else {
            return None;
        }
    }
    Some(format!("templates.TemplateResponse(request, {:?}, {{{}}})", name, pairs.join(", ")))
}

#[derive(Clone, Debug)]
pub struct Rel {
    name: String,
    kind: String, // hasMany / hasOne / belongsTo / belongsToMany
    related: String,
    fk: String,       // hasMany/hasOne: 相手側の列、belongsTo: 自分側の列、belongsToMany: 中間表の自分側の列
    other_fk: String, // belongsToMany: 中間表の相手側の列
    pivot: String,
}
type Rels = HashMap<String, Vec<Rel>>;

fn collect_relations(files: &[(String, String)]) -> Rels {
    let mut out = Rels::new();
    let cls = Regex::new(r"class\s+(\w+)\s+extends\s+(?:Model|Authenticatable|\w*Model)\b").unwrap();
    let f = Regex::new(r"function\s+(\w+)\s*\(\s*\)\s*(?::\s*[\w\\?|]+\s*)?\{\s*return\s+\$this->(hasMany|hasOne|belongsTo|belongsToMany)\(([^;]*)\)\s*;\s*\}").unwrap();
    let cr = Regex::new(r"([\w\\]+)::class").unwrap();
    for (_, src) in files {
        let Some(c) = cls.captures(src) else { continue };
        let owner = c[1].to_string();
        for m in f.captures_iter(src) {
            let args = split_args(&m[3]);
            let Some(related) = args.first().and_then(|a| cr.captures(a)).map(|x| x[1].rsplit('\\').next().unwrap_or("").to_string()) else { continue };
            let arg = |i: usize| args.get(i).map(|a| unq(a)).filter(|a| !a.is_empty());
            let kind = m[2].to_string();
            let name = m[1].to_string();
            let (fk, other_fk, pivot) = match kind.as_str() {
                "belongsTo" => (arg(1).unwrap_or_else(|| format!("{}_id", lower_snake(&name))), String::new(), String::new()),
                "belongsToMany" => {
                    let mut names = [lower_snake(&owner), lower_snake(&related)];
                    names.sort();
                    (arg(2).unwrap_or_else(|| format!("{}_id", lower_snake(&owner))), arg(3).unwrap_or_else(|| format!("{}_id", lower_snake(&related))), arg(1).unwrap_or_else(|| names.join("_")))
                }
                _ => (arg(1).unwrap_or_else(|| format!("{}_id", lower_snake(&owner))), String::new(), String::new()),
            };
            out.entry(owner.clone()).or_default().push(Rel { name, kind, related, fk, other_fk, pivot });
        }
    }
    out
}

/// リレーションが使う外部キー列がスキーマに無ければ足す(SQL で絞れるように実列にする)。
fn apply_relations(schema: &mut Schema, rels: &Rels) {
    for (owner, list) in rels {
        for r in list {
            let target = match r.kind.as_str() {
                "hasMany" | "hasOne" => &r.related,
                "belongsTo" => owner,
                _ => continue,
            };
            if let Some((_, cols)) = schema.get_mut(target) {
                if !cols.iter().any(|(n, _)| *n == r.fk) {
                    cols.push((r.fk.clone(), "Integer"));
                }
            }
        }
    }
}

fn rel_methods(r: &Rel) -> String {
    let (n, rel, fk) = (&r.name, &r.related, &r.fk);
    match r.kind.as_str() {
        "hasMany" | "hasOne" => {
            let get = if r.kind == "hasMany" { "get" } else { "first" };
            format!("\n    @classmethod\n    def {n}(cls, row):\n        return {rel}.query().where(\"{fk}\", \"=\", row[\"id\"]).{get}()\n\n    @classmethod\n    def {n}_create(cls, row, data):\n        return {rel}.create({{**(data or {{}}), \"{fk}\": row[\"id\"]}})\n")
        }
        "belongsTo" => format!("\n    @classmethod\n    def {n}(cls, row):\n        v = row.get(\"{fk}\")\n        return None if v is None else {rel}.find(v)\n"),
        _ => {
            let (p, o) = (&r.pivot, &r.other_fk);
            format!("\n    @classmethod\n    def {n}(cls, row):\n        with Session() as s:\n            ids = [x[0] for x in s.execute(select({p}.c.{o}).where({p}.c.{fk} == row[\"id\"]))]\n        return [x for x in ({rel}.find(i) for i in ids) if x]\n\n    @classmethod\n    def {n}_attach(cls, row, ids):\n        ids = ids if isinstance(ids, (list, tuple)) else [ids]\n        have = {{x[\"id\"] for x in cls.{n}(row)}}\n        with Session() as s:\n            for i in ids:\n                if i is not None and int(i) not in have:\n                    s.execute({p}.insert().values({{\"{fk}\": row[\"id\"], \"{o}\": int(i)}}))\n            s.commit()\n\n    @classmethod\n    def {n}_detach(cls, row, ids=None):\n        with Session() as s:\n            q = {p}.delete().where({p}.c.{fk} == row[\"id\"])\n            if ids is not None:\n                ids = ids if isinstance(ids, (list, tuple)) else [ids]\n                q = q.where({p}.c.{o}.in_([int(i) for i in ids]))\n            s.execute(q)\n            s.commit()\n\n    @classmethod\n    def {n}_sync(cls, row, ids):\n        cls.{n}_detach(row)\n        cls.{n}_attach(row, ids)\n")
        }
    }
}

/// `function ($q) { $q->where('a', 1); }` / `fn($q) => $q->where(..)` → `[("a", "=", 1)]`
fn has_conds(c: &str) -> Option<String> {
    let c = c.trim();
    let body = if c.starts_with("fn") {
        c[c.find("=>")? + 2..].to_string()
    } else if c.starts_with("function") {
        let o = c.find('{')?;
        let cl = matching(c, o, '{', '}')?;
        c[o + 1..cl].to_string()
    } else {
        return None;
    };
    let st = body.trim().trim_end_matches(';').trim();
    let st = st.strip_prefix("return").map(|x| x.trim()).unwrap_or(st);
    let segs = split_chain(st);
    if !Regex::new(r"^\$\w+$").unwrap().is_match(&segs[0]) {
        return None;
    }
    let mut out = vec![];
    for seg in &segs[1..] {
        let c = Regex::new(r"^where\((.*)\)$").unwrap().captures(seg)?;
        let a: Option<Vec<String>> = split_args(&c[1]).iter().map(|x| arg_expr(x)).collect();
        let a = a?;
        match a.len() {
            2 => out.push(format!("({}, \"=\", {})", a[0], a[1])),
            3 => out.push(format!("({}, {}, {})", a[0], a[1], a[2])),
            _ => return None,
        }
    }
    Some(format!("[{}]", out.join(", ")))
}

/// 変数のリレーション呼び出し(`$u->posts()->create(..)` / `->attach(..)` など)→ (Python の式, 結果のモデル)
fn rel_call(st: &str, vars: &HashMap<String, String>, rels: &Rels) -> Option<(String, Option<String>)> {
    let c = Regex::new(r"^\$(\w+)->(\w+)\(\)->(create|attach|detach|sync)\((.*)\)$").unwrap().captures(st)?;
    let model = vars.get(&c[1])?;
    let r = rels.get(model)?.iter().find(|r| r.name == c[2])?;
    let ok = match &c[3] {
        "create" => r.kind == "hasMany" || r.kind == "hasOne",
        _ => r.kind == "belongsToMany",
    };
    if !ok {
        return None;
    }
    let a = arg_expr(&c[4])?;
    let a = if a.is_empty() { "None".to_string() } else { a };
    let result = if &c[3] == "create" { Some(r.related.clone()) } else { None };
    Some((format!("{model}.{}_{}({}, {a})", r.name, &c[3], &c[1]), result))
}

/// `$var->rel`(括弧なし)→ `@py(Model.rel(var))`
fn subst_rel(st: &str, var: &str, model: &str, rel: &str) -> String {
    let re = Regex::new(&format!(r"\${}->{}\b", regex::escape(var), regex::escape(rel))).unwrap();
    let mut out = String::new();
    let mut last = 0;
    for m in re.find_iter(st) {
        if st[m.end()..].trim_start().starts_with('(') {
            continue;
        }
        out.push_str(&st[last..m.start()]);
        out.push_str(&format!("@py({model}.{rel}({var}))"));
        last = m.end();
    }
    out.push_str(&st[last..]);
    out
}

type Schema = HashMap<String, (String, Vec<(String, &'static str)>)>; // モデル → (テーブル, 列)

fn lower_snake(n: &str) -> String {
    let mut o = String::new();
    for (i, ch) in n.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            o.push('_');
        }
        o.extend(ch.to_lowercase());
    }
    o
}

fn guess_table(model: &str) -> String {
    let n = lower_snake(model);
    if let Some(stem) = n.strip_suffix('y') { format!("{stem}ies") } else if n.ends_with('s') { format!("{n}es") } else { format!("{n}s") }
}

fn col_type(laravel: &str) -> Option<&'static str> {
    Some(match laravel {
        "string" | "char" | "date" | "dateTime" | "timestamp" | "uuid" => "String(255)",
        "text" | "longText" | "mediumText" => "Text",
        "integer" | "bigInteger" | "unsignedBigInteger" | "unsignedInteger" | "smallInteger" | "tinyInteger" | "unsignedSmallInteger" | "unsignedTinyInteger" | "foreignId" => "Integer",
        "boolean" => "Boolean",
        "float" | "double" | "decimal" => "Float",
        "json" => "JSON",
        _ => return None,
    })
}

/// マイグレーション(`Schema::create`)と Eloquent モデル(`$fillable`・`$table`)から列を集める。
fn collect_schema(files: &[(String, String)]) -> Schema {
    let mut tables: HashMap<String, Vec<(String, &'static str)>> = HashMap::new();
    let create = Regex::new(r#"Schema::create\(\s*['"](\w+)['"]\s*,\s*function\s*\([^)]*\)\s*\{"#).unwrap();
    let colre = Regex::new(r#"\$table->(\w+)\(\s*['"](\w+)['"]"#).unwrap();
    for (_, src) in files {
        for c in create.captures_iter(src) {
            let open = c.get(0).unwrap().end() - 1;
            let Some(close) = matching(src, open, '{', '}') else { continue };
            let cols: Vec<(String, &'static str)> = colre.captures_iter(&src[open + 1..close]).filter_map(|m| col_type(&m[1]).map(|t| (m[2].to_string(), t))).collect();
            tables.insert(c[1].to_string(), cols);
        }
    }
    let mut out = Schema::new();
    let cls = Regex::new(r"class\s+(\w+)\s+extends\s+(?:Model|Authenticatable|\w*Model)\b").unwrap();
    let fill = Regex::new(r#"\$fillable\s*=\s*\[([^\]]*)\]"#).unwrap();
    let tbl = Regex::new(r#"\$table\s*=\s*['"](\w+)['"]"#).unwrap();
    for (_, src) in files {
        let Some(c) = cls.captures(src) else { continue };
        let model = c[1].to_string();
        let table = tbl.captures(src).map(|t| t[1].to_string()).unwrap_or_else(|| guess_table(&model));
        let cols = match tables.get(&table) {
            Some(c) if !c.is_empty() => c.clone(),
            _ => fill.captures(src).map(|f| Regex::new(r#"['"](\w+)['"]"#).unwrap().captures_iter(&f[1]).map(|m| (m[1].to_string(), "String(255)")).collect()).unwrap_or_default(),
        };
        out.insert(model, (table, cols));
    }
    out
}

const DB_PY: &str = r##""""pl2pl が生成した DB 層(SQLAlchemy)。接続先は環境変数 DATABASE_URL(既定は SQLite のファイル)。"""
import os

from sqlalchemy import create_engine
from sqlalchemy.orm import declarative_base, sessionmaker

engine = create_engine(os.environ.get("DATABASE_URL", "sqlite:///./app.db"))
Session = sessionmaker(engine, expire_on_commit=False)
Base = declarative_base()
"##;

const MODEL_PY: &str = r##"

class __M__(Base):
    """Eloquent モデル __M__ の SQLAlchemy 版(テーブル __T__)。マイグレーションに無い項目は extra(JSON)へ入る。"""
    __tablename__ = "__T__"
    id = Column(Integer, primary_key=True, autoincrement=True)
__COLS__    extra = Column(JSON, default=dict)
    COLUMNS = (__NAMES__)

    def to_dict(self):
        d = {"id": self.id}
        for c in self.COLUMNS:
            d[c] = getattr(self, c)
        d.update(self.extra or {})
        return d

    @classmethod
    def query(cls):
        return Query(cls)

    @classmethod
    def all(cls):
        with Session() as s:
            return [r.to_dict() for r in s.query(cls).order_by(cls.id)]

    @classmethod
    def find(cls, id):
        with Session() as s:
            r = s.get(cls, int(id))
            return r.to_dict() if r else None

    @classmethod
    def find_or_fail(cls, id):
        r = cls.find(id)
        if r is None:
            raise HTTPException(status_code=404, detail="__M__ not found")
        return r

    @classmethod
    def update(cls, id, data):
        with Session() as s:
            r = s.get(cls, int(id))
            if r is None:
                return None
            data = dict(data or {})
            data.pop("id", None)
            for k in list(data):
                if k in cls.COLUMNS:
                    setattr(r, k, data.pop(k))
            r.extra = {**(r.extra or {}), **data}
            s.commit()
            return r.to_dict()

    @classmethod
    def destroy(cls, id):
        with Session() as s:
            r = s.get(cls, int(id))
            if r is None:
                return 0
            s.delete(r)
            s.commit()
            return 1

    @classmethod
    def create(cls, data):
        data = dict(data or {})
        row = cls(**{k: data.pop(k) for k in list(data) if k in cls.COLUMNS}, extra=data)
        with Session() as s:
            s.add(row)
            s.commit()
            return row.to_dict()
__RELS__"##;

const QUERY_PY: &str = r##"

import operator as _op
import re as _re
from urllib.parse import urlencode as _urlencode

_OPS = {"=": _op.eq, "==": _op.eq, "!=": _op.ne, "<>": _op.ne, "<": _op.lt, "<=": _op.le, ">": _op.gt, ">=": _op.ge}


def _py_match(row, col, op, val):
    v = row.get(col)
    if op == "in":
        return v in val
    if op == "like":
        pat = "^" + _re.escape(val).replace("%", ".*").replace("_", ".") + "$"
        return v is not None and _re.match(pat, str(v), _re.I) is not None
    try:
        return _OPS[op](v, val)
    except TypeError:
        return False


def _load(cls, rows, parts):
    """with('a.b') の読み込み。a を各行へ付け、その子に対して b を再帰的に付ける。"""
    rel, rest = parts[0], parts[1:]
    for r in rows:
        if rel not in r:
            r[rel] = getattr(cls, rel)(r)
    if rest:
        sub = globals()[getattr(cls, "REL_CLASS", {})[rel]]
        kids = [k for r in rows for k in ([r[rel]] if isinstance(r[rel], dict) else (r[rel] or []))]
        _load(sub, kids, rest)


class Page(dict):
    """paginate() の結果。JSON にすると Laravel と同じ形、テンプレートでは繰り返せて links() も使える。"""

    def __init__(self, url, **kw):
        super().__init__(**kw)
        self._url = url

    def __iter__(self):
        return iter(self["data"])

    def currentPage(self):
        return self["current_page"]

    def lastPage(self):
        return self["last_page"]

    def perPage(self):
        return self["per_page"]

    def hasPages(self):
        return self["last_page"] > 1

    def hasMorePages(self):
        return self["current_page"] < self["last_page"]

    def onFirstPage(self):
        return self["current_page"] <= 1

    def links(self):
        from markupsafe import Markup

        if not self.hasPages():
            return Markup("")
        cur, last = self["current_page"], self["last_page"]
        parts = []
        if cur > 1:
            parts.append(Markup('<a href="{}" rel="prev">&laquo; Previous</a>').format(self._url(cur - 1)))
        for p in range(1, last + 1):
            if p == cur:
                parts.append(Markup("<strong>{}</strong>").format(p))
            else:
                parts.append(Markup('<a href="{}">{}</a>').format(self._url(p), p))
        if cur < last:
            parts.append(Markup('<a href="{}" rel="next">Next &raquo;</a>').format(self._url(cur + 1)))
        return Markup('<nav class="pagination">') + Markup(" ").join(parts) + Markup("</nav>")


class Query:
    """Eloquent のクエリビルダの代わり。列(マイグレーション/$fillable)は SQL、extra の項目や or_where は Python 側で絞る。"""

    def __init__(self, cls):
        self.cls = cls
        self.conds = []
        self.orders = []
        self._limit = None
        self._offset = 0
        self._with = []
        self.has = []

    def with_(self, rels):
        self._with += [rels] if isinstance(rels, str) else list(rels)
        return self

    def where_has(self, rel, conds=None, negate=False):
        self.has.append((rel, conds or [], negate))
        return self

    def _decorate(self, rows):
        for path in self._with:
            _load(self.cls, rows, path.split("."))
        return rows

    def where(self, col, op, val=None):
        self.conds.append(("and", col, op, val))
        return self

    def or_where(self, col, op, val=None):
        self.conds.append(("or", col, op, val))
        return self

    def order_by(self, col, direction="asc"):
        self.orders.append((col, str(direction).lower() == "desc"))
        return self

    def limit(self, n):
        self._limit = int(n)
        return self

    def _real(self, col):
        return col == "id" or col in self.cls.COLUMNS

    def get(self):
        c = self.cls
        use_sql = all(k == "and" and self._real(col) for k, col, _, _ in self.conds) and all(self._real(col) for col, _ in self.orders) and not self.has
        with Session() as s:
            q = s.query(c)
            if use_sql:
                for _, col, op, val in self.conds:
                    a = getattr(c, col)
                    if op == "in":
                        q = q.filter(a.in_(val))
                    elif op == "like":
                        q = q.filter(a.like(val))
                    else:
                        q = q.filter(_OPS[op](a, val))
                for col, desc in self.orders:
                    q = q.order_by(getattr(c, col).desc() if desc else getattr(c, col))
                if not self.orders:
                    q = q.order_by(c.id)
                if self._offset:
                    q = q.offset(self._offset)
                if self._limit is not None:
                    q = q.limit(self._limit)
                return self._decorate([r.to_dict() for r in q])
            rows = [r.to_dict() for r in q.order_by(c.id)]
        if self.conds:
            def keep(row):
                res = None
                for k, col, op, val in self.conds:
                    m = _py_match(row, col, op, val)
                    res = m if res is None else ((res or m) if k == "or" else (res and m))
                return res
            rows = [r for r in rows if keep(r)]
        for rel, hc, neg in self.has:
            rows = [r for r in rows if any(all(_py_match(x, c, o, v) for c, o, v in hc) for x in getattr(self.cls, rel)(r)) != neg]
        for col, desc in reversed(self.orders):
            rows.sort(key=lambda r: (r.get(col) is None, r.get(col)), reverse=desc)
        rows = rows[self._offset:]
        return self._decorate(rows[: self._limit] if self._limit is not None else rows)

    def offset(self, n):
        self._offset = max(int(n), 0)
        return self

    def paginate(self, per_page=15, request=None, page_name="page"):
        """Laravel の paginate()。?page=N を読み、Laravel と同じキーの Page(辞書)を返す。"""
        per_page = max(int(per_page), 1)
        qp = dict(request.query_params) if request is not None else {}
        try:
            page = max(int(qp.get(page_name, 1)), 1)
        except ValueError:
            page = 1
        total = self.count()
        saved = (self._limit, self._offset)
        self._limit, self._offset = per_page, (page - 1) * per_page
        rows = self.get()
        self._limit, self._offset = saved
        last = max(-(-total // per_page), 1)
        qp.pop(page_name, None)
        path = request.url.path if request is not None else ""

        def url(p):
            return path + "?" + _urlencode({**qp, page_name: p})

        return Page(
            url,
            data=rows, current_page=page, per_page=per_page, total=total, last_page=last,
            **{"from": (page - 1) * per_page + 1 if rows else None, "to": (page - 1) * per_page + len(rows) if rows else None},
            next_page_url=url(page + 1) if page < last else None,
            prev_page_url=url(page - 1) if page > 1 else None,
        )

    def first(self):
        saved, self._limit = self._limit, 1
        rows = self.get()
        self._limit = saved
        return rows[0] if rows else None

    def first_or_fail(self):
        r = self.first()
        if r is None:
            raise HTTPException(status_code=404, detail="not found")
        return r

    def count(self):
        saved = (self._limit, self._offset)
        self._limit, self._offset = None, 0
        n = len(self.get())
        self._limit, self._offset = saved
        return n

    def exists(self):
        return self.first() is not None

    def delete(self):
        n = 0
        for r in self.get():
            n += self.cls.destroy(r["id"])
        return n
"##;

fn models_py(models: &[String], schema: &Schema, rels: &Rels) -> String {
    let mut o = String::from("\"\"\"pl2pl が生成したモデル(SQLAlchemy)。\"\"\"\nfrom fastapi import HTTPException\nfrom sqlalchemy import JSON, Boolean, Column, Float, Integer, String, Table, Text, select\n\nfrom db import Base, Session, engine\n");
    o.push_str(QUERY_PY);
    let mut pivots: Vec<(String, String, String)> = vec![];
    for m in models {
        for r in rels.get(m).into_iter().flatten().filter(|r| r.kind == "belongsToMany") {
            if !pivots.iter().any(|p| p.0 == r.pivot) {
                pivots.push((r.pivot.clone(), r.fk.clone(), r.other_fk.clone()));
            }
        }
    }
    for (p, a, b) in &pivots {
        o.push_str(&format!("\n\n{p} = Table(\"{p}\", Base.metadata, Column(\"{a}\", Integer), Column(\"{b}\", Integer))\n"));
    }
    for m in models {
        let (t, cols) = schema.get(m).cloned().unwrap_or_else(|| (guess_table(m), vec![]));
        let cs: String = cols.iter().map(|(n, ty)| format!("    {n} = Column({ty})\n")).collect();
        let names: String = cols.iter().map(|(n, _)| format!("{n:?}, ")).collect();
        let mut relcode: String = rels.get(m).into_iter().flatten().map(rel_methods).collect();
        if let Some(rs) = rels.get(m).filter(|v| !v.is_empty()) {
            let pairs: Vec<String> = rs.iter().map(|r| format!("{:?}: {:?}", r.name, r.related)).collect();
            relcode.push_str(&format!("
    REL_CLASS = {{{}}}
", pairs.join(", ")));
        }
        o.push_str(&MODEL_PY.replace("__RELS__", &relcode).replace("__M__", m).replace("__T__", &t).replace("__COLS__", &cs).replace("__NAMES__", &names));
    }
    o.push_str("\n\nBase.metadata.create_all(engine)\n");
    o
}

type Sigs = HashMap<(String, String), Vec<(String, String)>>; // (class, method) → [(名前, 型)]

fn collect_controllers(files: &[(String, String)]) -> Sigs {
    let mut m = Sigs::new();
    let cls = Regex::new(r"class\s+(\w+)").unwrap();
    let func = Regex::new(r"public\s+function\s+(\w+)\s*\(([^)]*)\)").unwrap();
    for (_, src) in files {
        let Some(c) = cls.captures(src) else { continue };
        for f in func.captures_iter(src) {
            let params: Vec<(String, String)> = split_args(&f[2])
                .iter()
                .filter_map(|p| {
                    let p = p.split('=').next().unwrap().trim();
                    let mut it = p.rsplitn(2, '$');
                    let name = it.next()?.trim().to_string();
                    let ty = it.next().unwrap_or("").trim().trim_start_matches('?').to_string();
                    Some((name, ty))
                })
                .collect();
            m.insert((c[1].to_string(), f[1].to_string()), params);
        }
    }
    m
}

/// コントローラのメソッド本体(波括弧の中)を (class, method) で集める。
fn collect_bodies(files: &[(String, String)]) -> HashMap<(String, String), String> {
    let mut m = HashMap::new();
    let cls = Regex::new(r"class\s+(\w+)").unwrap();
    let func = Regex::new(r"public\s+function\s+(\w+)\s*\([^)]*\)\s*(?::\s*[\w\?|]+\s*)?\{").unwrap();
    for (_, src) in files {
        let Some(c) = cls.captures(src) else { continue };
        for f in func.captures_iter(src) {
            let open = f.get(0).unwrap().end() - 1;
            if let Some(close) = matching(src, open, '{', '}') {
                m.insert((c[1].to_string(), f[1].to_string()), src[open + 1..close].to_string());
            }
        }
    }
    m
}

fn join_path(prefix: &str, p: &str) -> String {
    let s = format!("/{}/{}", prefix.trim_matches('/'), p.trim_matches('/'));
    let s = Regex::new(r"/+").unwrap().replace_all(&s, "/").to_string();
    if s.len() > 1 { s.trim_end_matches('/').to_string() } else { s }
}

fn parse_laravel(src: &str, prefix: &str, line0: usize, out: &mut Vec<Route>, warn: &mut Vec<String>, name: &str) {
    // グループ(prefix)を先に処理し、その範囲を空白で潰してから残りを読む
    let grp = Regex::new(r#"Route::((?:\w+\([^)]*\)->)+)group\(\s*function\s*\([^)]*\)\s*\{"#).unwrap();
    let grp_prefix = Regex::new(r#"prefix\(\s*['"]([^'"]*)['"]\s*\)"#).unwrap();
    let grp_mw = Regex::new(r"middleware\(([^)]*)\)").unwrap();
    let mut masked = src.to_string();
    for m in grp.captures_iter(src) {
        let open = m.get(0).unwrap().end() - 1;
        if let Some(close) = matching(src, open, '{', '}') {
            let inner = &src[open + 1..close];
            let gp = grp_prefix.captures(&m[1]).map(|c| c[1].to_string()).unwrap_or_default();
            let gmw: Vec<String> = grp_mw.captures_iter(&m[1]).flat_map(|c| crate::middleware::parse_names(&c[1])).collect();
            let before = out.len();
            parse_laravel(inner, &join_path(prefix, &gp), line0 + crate::util::line_of(src, open) - 1, out, warn, name);
            for r in &mut out[before..] {
                r.mw.splice(0..0, gmw.iter().cloned());
            }
            let blank: String = src[m.get(0).unwrap().start()..close + 1].chars().map(|c| if c == '\n' { '\n' } else { ' ' }).collect();
            masked.replace_range(m.get(0).unwrap().start()..close + 1, &blank);
        }
    }
    let re = Regex::new(r"Route::(get|post|put|patch|delete|any|resource|apiResource|view)\(").unwrap();
    for m in re.captures_iter(&masked) {
        let open = m.get(0).unwrap().end() - 1;
        let Some(close) = matching(&masked, open, '(', ')') else { continue };
        let args = split_args(&masked[open + 1..close]);
        let line = line0 + crate::util::line_of(&masked, open) - 1;
        let verb = m[1].to_string();
        let mw = crate::middleware::route_mw(&masked[close + 1..]);
        if args.is_empty() {
            continue;
        }
        let path = join_path(prefix, &unq(&args[0]));
        let handler = args.get(1).cloned().unwrap_or_default();
        if verb == "view" {
            if args.len() >= 2 {
                out.push(Route { verb: "get".into(), path: path.clone(), handler: "view".into(), class: String::new(), method: "view".into(), body: None, raw: Some(format!("return view({});", args[1..].join(", "))), mw: mw.clone(), src: String::new(), line });
            }
            continue;
        }
        if verb == "resource" || verb == "apiResource" {
            let class = Regex::new(r"(\w+)::class").unwrap().captures(&handler).map(|c| c[1].to_string()).unwrap_or_else(|| unq(&handler).split('@').next().unwrap_or("").to_string());
            let single = path.rsplit('/').next().unwrap_or("item").trim_end_matches('s').to_string();
            let item = format!("{path}/{{{single}}}");
            let mut rs = vec![("get", path.clone(), "index"), ("post", path.clone(), "store"), ("get", item.clone(), "show"), ("put", item.clone(), "update"), ("delete", item, "destroy")];
            if verb == "resource" {
                warn.push(format!("{name}:{line} Route::resource の create/edit(HTML フォーム)は変換していません"));
            }
            for (v, p, meth) in rs.drain(..) {
                out.push(Route { verb: v.into(), path: p, handler: format!("{class}@{meth}"), class: class.clone(), method: meth.into(), body: None, raw: None, mw: mw.clone(), src: String::new(), line });
            }
            continue;
        }
        let verbs: Vec<&str> = if verb == "any" { vec!["get", "post", "put", "patch", "delete"] } else { vec![verb.as_str()] };
        let mut raw_closure: Option<String> = None;
        let (class, method, body, label) = if let Some(c) = Regex::new(r"^\[\s*([\w\\]+)::class\s*,\s*['\x22](\w+)['\x22]\s*\]$").unwrap().captures(&handler) {
            let cls = c[1].rsplit('\\').next().unwrap().to_string();
            (cls.clone(), c[2].to_string(), None, format!("{cls}@{}", &c[2]))
        } else if let Some((c, mth)) = unq(&handler).split_once('@').filter(|_| handler.starts_with('\'') || handler.starts_with('"')) {
            let cls = c.rsplit('\\').next().unwrap().to_string();
            (cls.clone(), mth.to_string(), None, format!("{cls}@{mth}"))
        } else if handler.starts_with("function") || handler.starts_with("fn") {
            let body = if let Some(i) = handler.find('{') { handler[i + 1..handler.rfind('}').unwrap_or(handler.len())].to_string() } else { handler.split_once("=>").map(|x| format!("return {};", x.1.trim())).unwrap_or_default() };
            let sv = simple_return(&body);
            raw_closure = Some(body.clone());
            (String::new(), "closure".into(), sv, "closure".to_string())
        } else {
            warn.push(format!("{name}:{line} ハンドラの形を解釈できません: {}", handler.chars().take(40).collect::<String>()));
            continue;
        };
        for v in verbs {
            out.push(Route { verb: v.into(), path: path.clone(), handler: label.clone(), class: class.clone(), method: method.clone(), body: body.clone(), raw: raw_closure.clone(), mw: mw.clone(), src: handler.chars().take(80).collect(), line });
        }
    }
}

fn parse_symfony(src: &str, name: &str, out: &mut Vec<Route>, warn: &mut Vec<String>) {
    let attr = Regex::new(r#"(?:#\[Route\(|@Route\()\s*['"]([^'"]*)['"]([^\)\]]*)[\)\]]"#).unwrap();
    let cls = Regex::new(r"class\s+(\w+)").unwrap();
    let func = Regex::new(r"function\s+(\w+)\s*\(").unwrap();
    let Some(cm) = cls.captures(src) else { return };
    let cpos = cm.get(0).unwrap().start();
    let prefix = attr.captures_iter(src).find(|a| a.get(0).unwrap().start() < cpos).map(|a| a[1].to_string()).unwrap_or_default();
    for a in attr.captures_iter(src).filter(|a| a.get(0).unwrap().start() > cpos) {
        let rest = &src[a.get(0).unwrap().end()..];
        let Some(f) = func.captures(rest) else { continue };
        let methods = Regex::new(r"methods\s*[:=]\s*[\[{]([^\]}]*)[\]}]").unwrap().captures(&a[2]).map(|m| m[1].to_string()).unwrap_or_default();
        let verbs: Vec<String> = if methods.trim().is_empty() { vec!["get".into()] } else { methods.split(',').map(|x| unq(x).to_lowercase()).filter(|x| !x.is_empty()).collect() };
        let line = crate::util::line_of(src, a.get(0).unwrap().start());
        for v in verbs {
            if !["get", "post", "put", "patch", "delete"].contains(&v.as_str()) {
                warn.push(format!("{name}:{line} 未対応の動詞 {v}"));
                continue;
            }
            out.push(Route { verb: v, path: join_path(&prefix, &a[1]), handler: format!("{}@{}", &cm[1], &f[1]), class: cm[1].to_string(), method: f[1].to_string(), body: None, raw: None, mw: vec![], src: String::new(), line });
        }
    }
}

pub fn extract(files: &[(String, String)], warn: &mut Vec<String>) -> Vec<Route> {
    let mut out = vec![];
    for (name, src) in files {
        if src.contains("Route::") {
            parse_laravel(src, "", 1, &mut out, warn, name);
        }
        if src.contains("#[Route(") || src.contains("@Route(") {
            parse_symfony(src, name, &mut out, warn);
        }
    }
    out
}

fn py_type(t: &str) -> &'static str {
    match t {
        "int" => "int",
        "float" => "float",
        "bool" => "bool",
        _ => "str",
    }
}

pub fn generate(all_files: &[(String, String)], target: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let (blade_files, files): (Vec<(String, String)>, Vec<(String, String)>) = all_files.iter().cloned().partition(|(n, _)| n.ends_with(".blade.php"));
    let files = &files[..];
    let mut warn = vec![];
    let routes = extract(files, &mut warn);
    let sigs = collect_controllers(files);
    let bodies = collect_bodies(files);
    let mut schema = collect_schema(files);
    let rels = collect_relations(files);
    apply_relations(&mut schema, &rels);
    let aliases = crate::middleware::collect_aliases(files);
    let custom_mw = crate::middleware::collect_custom(files);
    let ctor_mw = crate::middleware::collect_ctor(files);
    let mut mwreg = crate::middleware::Registry::default();
    rep.extend(warn.iter().cloned());
    let mut used: Vec<String> = vec![];
    let mut models: Vec<String> = vec![];
    let mut need_response = false;
    let mut need_tpl = false;
    let mut m = String::from("\"\"\"pl2pl php が生成した FastAPI の骨組み。ルートと型を写し、本体は TODO。\"\"\"\nfrom fastapi import FastAPI\n\napp = FastAPI()\n");
    let path_var = Regex::new(r"\{(\w+)\??\}").unwrap();
    for r in &routes {
        let mut fname = if r.class.is_empty() { format!("{}_{}", r.verb, snake(&r.path)) } else { format!("{}_{}", snake(&r.class), snake(&r.method)) };
        if fname.ends_with('_') || fname.is_empty() {
            fname.push_str("root");
        }
        let base = fname.clone();
        let mut i = 2;
        while used.contains(&fname) {
            fname = format!("{base}_{i}");
            i += 1;
        }
        used.push(fname.clone());
        let sig = sigs.get(&(r.class.clone(), r.method.clone()));
        let mut params: Vec<String> = vec![];
        let mut path = r.path.clone();
        for c in path_var.captures_iter(&r.path) {
            let n = c[1].to_string();
            let ty = sig.and_then(|s| s.iter().find(|(pn, _)| *pn == n)).map(|(_, t)| py_type(t)).unwrap_or("str");
            params.push(format!("{n}: {ty}"));
        }
        path = path_var.replace_all(&path, "{$1}").to_string();
        if matches!(r.verb.as_str(), "post" | "put" | "patch") {
            params.push("payload: dict | None = None".into());
        }
        let mut all_mw: Vec<String> = r.mw.clone();
        if let Some(list) = ctor_mw.get(&r.class) {
            for (n, only, except) in list {
                let ok_only = only.as_ref().map(|o| o.contains(&r.method)).unwrap_or(true);
                let ok_exc = except.as_ref().map(|e| !e.contains(&r.method)).unwrap_or(true);
                if ok_only && ok_exc {
                    all_mw.push(n.clone());
                }
            }
        }
        let mut deps: Vec<String> = vec![];
        for tok in &all_mw {
            if let Some(d) = mwreg.resolve(tok, &aliases, &custom_mw, &format!("{} {}", r.verb.to_uppercase(), r.path), rep) {
                if !deps.contains(&d) {
                    deps.push(d);
                }
            }
        }
        let dep_arg = if deps.is_empty() { String::new() } else { format!(", dependencies=[{}]", deps.iter().map(|d| format!("Depends({d})")).collect::<Vec<_>>().join(", ")) };
        let mut body_txt = String::new();
        let raw_body = bodies.get(&(r.class.clone(), r.method.clone())).or(r.raw.as_ref());
        let mut ctl_body = if r.body.is_none() { raw_body.and_then(|b| simple_return(b)) } else { None };
        if r.body.is_none() && ctl_body.is_none() {
            if let Some((model, expr)) = raw_body.and_then(|b| eloquent_return(b)) {
                if !models.contains(&model) {
                    models.push(model);
                }
                ctl_body = Some(expr);
            }
        }
        let multi = if r.body.is_none() && ctl_body.is_none() { raw_body.and_then(|b| translate_body(b, &mut models, &mut need_response, &mut need_tpl, &rels)) } else { None };
        match r.body.as_ref().or(ctl_body.as_ref()) {
            None if multi.is_some() => {
                for l in multi.unwrap() {
                    body_txt.push_str(&format!("    {l}\n"));
                }
            }
            Some(b) => body_txt.push_str(&format!("    return {b}\n")),
            None => {
                body_txt.push_str(&format!("    # PHP: {} ({} {})\n    raise NotImplementedError(\"TODO(pl2pl): {} を移植する\")\n", r.handler, r.verb.to_uppercase(), r.path, r.handler));
                rep.push(format!("{}:{} {} {} の本体は未変換(TODO)", "routes", r.line, r.verb.to_uppercase(), r.path));
            }
        }
        if (body_txt.contains("TemplateResponse(request,") || body_txt.contains(", request)") || body_txt.contains("request.query_params")) && !params.iter().any(|p| p.starts_with("request:")) {
            params.insert(0, "request: Request".into());
        }
        m.push_str(&format!("\n\n@app.{}({:?}{dep_arg})\ndef {fname}({}):\n", r.verb, path, params.join(", ")));
        m.push_str(&body_txt);
    }
    if !mwreg.is_empty() {
        m = m.replacen("from fastapi import FastAPI\n", &format!("from fastapi import FastAPI\nfrom fastapi import Depends\n\nfrom middleware import MiddlewareAbort, {}\n", mwreg.used.join(", ")), 1);
        m = m.replacen("app = FastAPI()\n", "app = FastAPI()\napp.add_exception_handler(MiddlewareAbort, lambda request, exc: exc.response)\n", 1);
    }
    // リレーション先のモデルも生成する(推移的に)
    let mut k = 0;
    while k < models.len() {
        for r in rels.get(&models[k].clone()).into_iter().flatten() {
            if !models.contains(&r.related) {
                models.push(r.related.clone());
            }
        }
        k += 1;
    }
    let has_tpl = need_tpl || !blade_files.is_empty();
    if has_tpl {
        m = m.replacen("from fastapi import FastAPI\n", "from fastapi import FastAPI\nfrom fastapi import Request\nfrom fastapi.templating import Jinja2Templates\n", 1);
        m = m.replacen("app = FastAPI()\n", "app = FastAPI()\ntemplates = Jinja2Templates(directory=\"templates\")\n", 1);
    }
    if !has_tpl && m.contains("request: Request") {
        m = m.replacen("from fastapi import FastAPI\n", "from fastapi import FastAPI\nfrom fastapi import Request\n", 1);
    }
    if !models.is_empty() {
        m = m.replacen("from fastapi import FastAPI\n", &format!("from fastapi import FastAPI\n\nfrom models import {}\n", models.join(", ")), 1);
    }
    if need_response {
        m = m.replacen("from fastapi import FastAPI\n", "from fastapi import FastAPI, Response\nfrom fastapi.responses import JSONResponse\n", 1);
    }
    for mo in &models {
        let n = schema.get(mo).map(|x| x.1.len()).unwrap_or(0);
        rep.push(format!("Eloquent モデル {mo} → SQLAlchemy(db.py・models.py、列 {n} 個。マイグレーション/$fillable に無い項目は extra JSON へ)"));
    }
    if routes.is_empty() {
        rep.push("ルートが見つかりませんでした(Laravel の `Route::`、Symfony の `#[Route]` が対象)".into());
    }
    let mut outf = vec![("main.py".to_string(), m), ("requirements.txt".to_string(), "fastapi\nuvicorn\n".to_string())];
    if !models.is_empty() {
        outf.push(("db.py".to_string(), DB_PY.to_string()));
        outf.push(("models.py".to_string(), models_py(&models, &schema, &rels)));
        outf[1].1 = "fastapi\nuvicorn\nsqlalchemy\n".to_string();
    }
    if !mwreg.is_empty() {
        outf.push(("middleware.py".to_string(), mwreg.render()));
        rep.push(format!("ミドルウェア {} 個を Depends として middleware.py へ生成(auth は Bearer と環境変数 API_TOKENS で判定する簡易版。本物の認証へ差し替えること)", mwreg.used.len()));
    }
    if has_tpl {
        for (n, src) in &blade_files {
            let t = crate::blade::convert(src, n, rep);
            outf.push((format!("templates/{}", crate::blade::out_path(n)), t));
        }
        rep.push(format!("Blade {} 枚を Jinja2(templates/)へ変換(@php・@auth・@can・@push 等は TODO として残し、上のレポートに列挙)", blade_files.len()));
        outf[1].1 = format!("{}jinja2\n", outf[1].1);
    }
    if target == "mfastapi" {
        outf.push(("run_mfastapi.py".to_string(), "import mfastapi\nfrom main import app\n\nmfastapi.accelerate(app)\n\nif __name__ == \"__main__\":\n    import uvicorn\n    uvicorn.run(app, host=\"127.0.0.1\", port=8000)\n".to_string()));
        outf[1].1 = format!("{}mfastapi\n", outf[1].1);
    }
    rep.push(format!("ルート {} 本を FastAPI へ写しました(ミドルウェアと、変換できない文を含む本体は TODO として報告)", routes.len()));
    outf
}

#[cfg(test)]
mod tests {
    use super::*;
    fn r(src: &str) -> Vec<Route> {
        let mut w = vec![];
        extract(&[("routes/web.php".into(), src.into())], &mut w)
    }
    #[test]
    fn laravel_basic() {
        let v = r("<?php\nRoute::get('/users/{id}', [UserController::class, 'show']);\nRoute::post('/users', 'UserController@store');\nRoute::get('/hi', function () { return 'hello'; });\n");
        assert_eq!(v.len(), 3);
        assert_eq!(v[0].path, "/users/{id}");
        assert_eq!(v[0].handler, "UserController@show");
        assert_eq!(v[2].body.as_deref(), Some("\"hello\""));
    }
    #[test]
    fn laravel_group_and_resource() {
        let v = r("Route::prefix('api')->group(function () {\n  Route::get('/ping', fn() => ['ok' => true]);\n  Route::apiResource('items', ItemController::class);\n});");
        assert!(v.iter().any(|x| x.path == "/api/ping" && x.body.as_deref() == Some("{\"ok\": True}")));
        assert!(v.iter().any(|x| x.path == "/api/items/{item}" && x.verb == "delete"));
        assert_eq!(v.len(), 6);
    }
    #[test]
    fn symfony_attr() {
        let v = r("<?php\n#[Route('/blog')]\nclass BlogController {\n  #[Route('/{slug}', name: 'show', methods: ['GET', 'HEAD'])]\n  public function show(string $slug) {}\n}\n");
        assert!(v.iter().any(|x| x.path == "/blog/{slug}" && x.verb == "get"));
    }
    #[test]
    fn generates_typed_params() {
        let files = vec![
            ("routes/web.php".to_string(), "Route::get('/users/{id}', [UserController::class, 'show']);".to_string()),
            ("app/UserController.php".to_string(), "class UserController { public function show(int $id) {} }".to_string()),
        ];
        let mut rep = vec![];
        let out = generate(&files, "fastapi", &mut rep);
        assert!(out[0].1.contains("def user_controller_show(id: int):"));
        assert!(out[0].1.contains("NotImplementedError"));
    }

    #[test]
    fn multi_statement_body() {
        let mut m = vec![];
        let mut r = false;
        let mut t = false;
        let l = translate_body("$u = User::findOrFail($id); $u->update($request->all()); return response()->json($u);", &mut m, &mut r, &mut t, &HashMap::new()).unwrap();
        assert_eq!(l, vec!["u = User.find_or_fail(id)", "u = User.update(u[\"id\"], (payload or {}))", "return u"]);
        assert_eq!(m, vec!["User"]);
        assert!(translate_body("$x = foo(); return $x;", &mut m, &mut r, &mut t, &HashMap::new()).is_none());
    }

    #[test]
    fn query_builder_chain() {
        let (m, py, one) = query_expr("User::where('age', '>=', 18)->orderBy('name', 'desc')->limit(5)->get()").unwrap();
        assert_eq!(m, "User");
        assert_eq!(py, "User.query().where(\"age\", \">=\", 18).order_by(\"name\", \"desc\").limit(5).get()");
        assert!(!one);
        assert!(query_expr("User::where('a', 1)->first()").unwrap().2);
        assert!(query_expr("User::where('a', 1)").is_none()); // 終端なし
        assert!(query_expr("User::where('a', 1)->whereHasMorph('x')->get()").is_none()); // 未対応は TODO のまま
        assert!(query_expr("User::doesntHave('posts')->get()").unwrap().1.contains(".where_has(\"posts\", [], True)"));
        let (_, py, _) = query_expr("User::with('posts')->whereHas('posts', function ($q) { $q->where('published', true); })->get()").unwrap();
        assert_eq!(py, "User.query().with_(\"posts\").where_has(\"posts\", [(\"published\", \"=\", True)]).get()");
    }
}
