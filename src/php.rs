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
    pub src: String,
    pub line: usize,
}

fn matching(s: &str, open: usize, o: char, c: char) -> Option<usize> {
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

fn split_args(s: &str) -> Vec<String> {
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

fn unq(s: &str) -> String {
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
fn php_value(s: &str) -> Option<String> {
    let s = s.trim().trim_end_matches(';').trim();
    if (s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2) || (s.starts_with('"') && s.ends_with('"') && s.len() >= 2 && !s.contains('$')) {
        return Some(format!("{:?}", &s[1..s.len() - 1]));
    }
    if s.parse::<f64>().is_ok() {
        return Some(s.to_string());
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

fn join_path(prefix: &str, p: &str) -> String {
    let s = format!("/{}/{}", prefix.trim_matches('/'), p.trim_matches('/'));
    let s = Regex::new(r"/+").unwrap().replace_all(&s, "/").to_string();
    if s.len() > 1 { s.trim_end_matches('/').to_string() } else { s }
}

fn parse_laravel(src: &str, prefix: &str, line0: usize, out: &mut Vec<Route>, warn: &mut Vec<String>, name: &str) {
    // グループ(prefix)を先に処理し、その範囲を空白で潰してから残りを読む
    let grp = Regex::new(r#"Route::(?:\w+\([^)]*\)->)*prefix\(\s*['"]([^'"]*)['"]\s*\)(?:->\w+\([^)]*\))*->group\(\s*function\s*\([^)]*\)\s*\{"#).unwrap();
    let mut masked = src.to_string();
    for m in grp.captures_iter(src) {
        let open = m.get(0).unwrap().end() - 1;
        if let Some(close) = matching(src, open, '{', '}') {
            let inner = &src[open + 1..close];
            parse_laravel(inner, &join_path(prefix, &m[1]), line0 + crate::util::line_of(src, open) - 1, out, warn, name);
            let blank: String = src[m.get(0).unwrap().start()..close + 1].chars().map(|c| if c == '\n' { '\n' } else { ' ' }).collect();
            masked.replace_range(m.get(0).unwrap().start()..close + 1, &blank);
        }
    }
    let re = Regex::new(r"Route::(get|post|put|patch|delete|any|resource|apiResource)\(").unwrap();
    for m in re.captures_iter(&masked) {
        let open = m.get(0).unwrap().end() - 1;
        let Some(close) = matching(&masked, open, '(', ')') else { continue };
        let args = split_args(&masked[open + 1..close]);
        let line = line0 + crate::util::line_of(&masked, open) - 1;
        let verb = m[1].to_string();
        if args.is_empty() {
            continue;
        }
        let path = join_path(prefix, &unq(&args[0]));
        let handler = args.get(1).cloned().unwrap_or_default();
        if verb == "resource" || verb == "apiResource" {
            let class = Regex::new(r"(\w+)::class").unwrap().captures(&handler).map(|c| c[1].to_string()).unwrap_or_else(|| unq(&handler).split('@').next().unwrap_or("").to_string());
            let single = path.rsplit('/').next().unwrap_or("item").trim_end_matches('s').to_string();
            let item = format!("{path}/{{{single}}}");
            let mut rs = vec![("get", path.clone(), "index"), ("post", path.clone(), "store"), ("get", item.clone(), "show"), ("put", item.clone(), "update"), ("delete", item, "destroy")];
            if verb == "resource" {
                warn.push(format!("{name}:{line} Route::resource の create/edit(HTML フォーム)は変換していません"));
            }
            for (v, p, meth) in rs.drain(..) {
                out.push(Route { verb: v.into(), path: p, handler: format!("{class}@{meth}"), class: class.clone(), method: meth.into(), body: None, src: String::new(), line });
            }
            continue;
        }
        let verbs: Vec<&str> = if verb == "any" { vec!["get", "post", "put", "patch", "delete"] } else { vec![verb.as_str()] };
        let (class, method, body, label) = if let Some(c) = Regex::new(r"^\[\s*([\w\\]+)::class\s*,\s*['\x22](\w+)['\x22]\s*\]$").unwrap().captures(&handler) {
            let cls = c[1].rsplit('\\').next().unwrap().to_string();
            (cls.clone(), c[2].to_string(), None, format!("{cls}@{}", &c[2]))
        } else if let Some((c, mth)) = unq(&handler).split_once('@').filter(|_| handler.starts_with('\'') || handler.starts_with('"')) {
            let cls = c.rsplit('\\').next().unwrap().to_string();
            (cls.clone(), mth.to_string(), None, format!("{cls}@{mth}"))
        } else if handler.starts_with("function") || handler.starts_with("fn") {
            let body = if let Some(i) = handler.find('{') { handler[i + 1..handler.rfind('}').unwrap_or(handler.len())].to_string() } else { handler.split_once("=>").map(|x| format!("return {};", x.1.trim())).unwrap_or_default() };
            let sv = simple_return(&body);
            (String::new(), "closure".into(), sv, "closure".to_string())
        } else {
            warn.push(format!("{name}:{line} ハンドラの形を解釈できません: {}", handler.chars().take(40).collect::<String>()));
            continue;
        };
        for v in verbs {
            out.push(Route { verb: v.into(), path: path.clone(), handler: label.clone(), class: class.clone(), method: method.clone(), body: body.clone(), src: handler.chars().take(80).collect(), line });
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
            out.push(Route { verb: v, path: join_path(&prefix, &a[1]), handler: format!("{}@{}", &cm[1], &f[1]), class: cm[1].to_string(), method: f[1].to_string(), body: None, src: String::new(), line });
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

pub fn generate(files: &[(String, String)], target: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let mut warn = vec![];
    let routes = extract(files, &mut warn);
    let sigs = collect_controllers(files);
    rep.extend(warn.iter().cloned());
    let mut used: Vec<String> = vec![];
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
        m.push_str(&format!("\n\n@app.{}({:?})\ndef {fname}({}):\n", r.verb, path, params.join(", ")));
        match &r.body {
            Some(b) => m.push_str(&format!("    return {b}\n")),
            None => {
                m.push_str(&format!("    # PHP: {} ({} {})\n    raise NotImplementedError(\"TODO(pl2pl): {} を移植する\")\n", r.handler, r.verb.to_uppercase(), r.path, r.handler));
                rep.push(format!("{}:{} {} {} の本体は未変換(TODO)", "routes", r.line, r.verb.to_uppercase(), r.path));
            }
        }
    }
    if routes.is_empty() {
        rep.push("ルートが見つかりませんでした(Laravel の `Route::`、Symfony の `#[Route]` が対象)".into());
    }
    let mut outf = vec![("main.py".to_string(), m), ("requirements.txt".to_string(), "fastapi\nuvicorn\n".to_string())];
    if target == "mfastapi" {
        outf.push(("run_mfastapi.py".to_string(), "import mfastapi\nfrom main import app\n\nmfastapi.accelerate(app)\n\nif __name__ == \"__main__\":\n    import uvicorn\n    uvicorn.run(app, host=\"127.0.0.1\", port=8000)\n".to_string()));
        outf[1].1 = "fastapi\nuvicorn\nmfastapi\n".to_string();
    }
    rep.push(format!("ルート {} 本を FastAPI へ写しました(PHP の本体・ミドルウェア・Eloquent・Blade は未変換)", routes.len()));
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
}
