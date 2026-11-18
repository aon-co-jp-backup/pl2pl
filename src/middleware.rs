//! Laravel のミドルウェア → FastAPI の `Depends`(生成する `middleware.py`)。
//! 対応: 組み込みの auth / guest / throttle、独自ミドルウェアの「条件で拒否してから `$next($request)`」の形。
//! `$next` の後に処理を足す形・未対応のミドルウェアは、依存を付けずにレポートへ出す(黙って落とさない)。
use crate::php::{matching, php_value, split_args, unq};
use regex::Regex;
use std::collections::HashMap;

pub fn snake_case(s: &str) -> String {
    crate::php::snake(s)
}

/// `middleware(...)` の引数 → 名前の並び(`'auth'`・`['auth','throttle:60,1']`・`Admin::class`)。
pub fn parse_names(raw: &str) -> Vec<String> {
    let re = Regex::new(r#"['"]([^'"]+)['"]|([\w\\]+)::class"#).unwrap();
    re.captures_iter(raw)
        .map(|c| match (c.get(1), c.get(2)) {
            (Some(a), _) => a.as_str().to_string(),
            (_, Some(b)) => b.as_str().rsplit('\\').next().unwrap_or("").to_string(),
            _ => String::new(),
        })
        .filter(|x| !x.is_empty())
        .collect()
}

/// ルート呼び出しの直後(`)` の後)に続く `->middleware(...)` を拾う。
pub fn route_mw(after: &str) -> Vec<String> {
    let chain = Regex::new(r"^(?:\s*->\s*\w+\((?:[^()]|\([^()]*\))*\))*").unwrap();
    let Some(m) = chain.find(after) else { return vec![] };
    let mw = Regex::new(r"middleware\(((?:[^()]|\([^()]*\))*)\)").unwrap();
    mw.captures_iter(m.as_str()).flat_map(|c| parse_names(&c[1])).collect()
}

/// `Kernel.php` / `bootstrap/app.php` の別名 → クラス名。
pub fn collect_aliases(files: &[(String, String)]) -> HashMap<String, String> {
    let mut m = HashMap::new();
    let pair = Regex::new(r#"['"]([\w.:\-]+)['"]\s*=>\s*([\w\\]+)::class"#).unwrap();
    for (_, src) in files {
        if src.contains("routeMiddleware") || src.contains("middlewareAliases") || src.contains("->alias(") {
            for c in pair.captures_iter(src) {
                m.insert(c[1].to_string(), c[2].rsplit('\\').next().unwrap_or("").to_string());
            }
        }
    }
    m
}

/// コントローラ `__construct` の `$this->middleware('x')->only([...])`。(クラス名 → [(名前, only, except)])
pub type CtorMw = HashMap<String, Vec<(String, Option<Vec<String>>, Option<Vec<String>>)>>;

pub fn collect_ctor(files: &[(String, String)]) -> CtorMw {
    let mut out: CtorMw = HashMap::new();
    let cls = Regex::new(r"class\s+(\w+)").unwrap();
    let re = Regex::new(r#"\$this->middleware\(\s*(['"][^'"]+['"]|\[[^\]]*\]|[\w\\]+::class)\s*\)((?:->(?:only|except)\([^)]*\))*)"#).unwrap();
    let only = Regex::new(r"->only\(([^)]*)\)").unwrap();
    let except = Regex::new(r"->except\(([^)]*)\)").unwrap();
    for (_, src) in files {
        let Some(c) = cls.captures(src) else { continue };
        for m in re.captures_iter(src) {
            let o = only.captures(&m[2]).map(|x| parse_names(&x[1]));
            let e = except.captures(&m[2]).map(|x| parse_names(&x[1]));
            for n in parse_names(&m[1]) {
                out.entry(c[1].to_string()).or_default().push((n, o.clone(), e.clone()));
            }
        }
    }
    out
}

fn req_expr(e: &str) -> Result<String, String> {
    let mut s = e.trim().to_string();
    for (re, rep) in [
        (r#"\$request->header\(\s*(['"][^'"]+['"])\s*\)"#, "request.headers.get($1)"),
        (r"\$request->bearerToken\(\)", "_bearer(request)"),
        (r#"\$request->(?:input|query|get)\(\s*(['"][^'"]+['"])\s*\)"#, "request.query_params.get($1)"),
        (r"\$request->ip\(\)", "(request.client.host if request.client else None)"),
        (r"\$request->method\(\)", "request.method"),
        (r"\$request->path\(\)", "request.url.path.lstrip(\"/\")"),
        (r#"\$request->is\(\s*(['"][^'"]+['"])\s*\)"#, "_is(request, $1)"),
        (r#"\benv\(\s*(['"][^'"]+['"])\s*,\s*(['"][^'"]*['"])\s*\)"#, "os.environ.get($1, $2)"),
        (r#"\benv\(\s*(['"][^'"]+['"])\s*\)"#, "os.environ.get($1)"),
        (r"\bempty\(([^()]*)\)", "(not $1)"),
        (r"\bisset\(([^()]*)\)", "($1 is not None)"),
        (r"\bstrlen\(([^()]*)\)", "len($1)"),
    ] {
        s = Regex::new(re).unwrap().replace_all(&s, rep).to_string();
    }
    let mut out = String::new();
    let mut q: Option<char> = None;
    let mut seg = String::new();
    let flush = |seg: &mut String, out: &mut String| {
        let mut t = std::mem::take(seg);
        for (a, b) in [("===", "=="), ("!==", "!="), ("&&", " and "), ("||", " or ")] {
            t = t.replace(a, b);
        }
        let t = Regex::new(r"!([^=])").unwrap().replace_all(&t, "not $1").to_string();
        let t = Regex::new(r"\bnull\b").unwrap().replace_all(&t, "None").to_string();
        let t = Regex::new(r"\btrue\b").unwrap().replace_all(&t, "True").to_string();
        out.push_str(&Regex::new(r"\bfalse\b").unwrap().replace_all(&t, "False"));
    };
    for ch in s.chars() {
        match q {
            Some(qc) => {
                out.push(ch);
                if ch == qc {
                    q = None;
                }
            }
            None if ch == '\'' || ch == '"' => {
                flush(&mut seg, &mut out);
                out.push(ch);
                q = Some(ch);
            }
            None => seg.push(ch),
        }
    }
    flush(&mut seg, &mut out);
    let out = Regex::new(r" {2,}").unwrap().replace_all(out.trim(), " ").to_string();
    // 引用符の外に `$` / `->` / 未対応の関数呼び出しが残れば拒否
    let unq_part: String = {
        let mut r = String::new();
        let mut q: Option<char> = None;
        for ch in out.chars() {
            match q {
                Some(qc) => {
                    if ch == qc {
                        q = None;
                    }
                }
                None if ch == '\'' || ch == '"' => q = Some(ch),
                None => r.push(ch),
            }
        }
        r
    };
    if unq_part.contains('$') || unq_part.contains("->") {
        return Err(format!("式 `{}` は未対応(`$request` の header/bearerToken/input/ip/method/path/is と env のみ)", e.trim()));
    }
    for m in Regex::new(r"(?:^|[^.\w])(\w+)\(").unwrap().captures_iter(&unq_part) {
        if !["_bearer", "_is", "len"].contains(&&m[1]) {
            return Err(format!("関数 `{}()` は未対応", &m[1]));
        }
    }
    Ok(out)
}

fn reject_stmt(stmt: &str) -> Result<String, String> {
    let st = stmt.trim().trim_end_matches(';').trim();
    let st = st.strip_prefix("return").unwrap_or(st).trim();
    if let Some(c) = Regex::new(r"^abort\(\s*(\d{3})\s*(?:,\s*(.+?))?\s*\)$").unwrap().captures(st) {
        let msg = c.get(2).map(|m| unq(m.as_str())).unwrap_or_else(|| match &c[1] {
            "401" => "Unauthenticated.".into(),
            "403" => "Forbidden".into(),
            "404" => "Not Found".into(),
            "429" => "Too Many Requests".into(),
            _ => "Error".into(),
        });
        return Ok(format!("raise MiddlewareAbort(JSONResponse({{\"message\": {msg:?}}}, status_code={}))", &c[1]));
    }
    if let Some(c) = Regex::new(r"^(?:response\(\)->json|Response::json)\((.*)\)$").unwrap().captures(st) {
        let a = split_args(&c[1]);
        let v = php_value(a.first().ok_or("json の引数が空")?).ok_or("json の中身が単純な値でない")?;
        let code = a.get(1).and_then(|x| x.trim().parse::<u16>().ok()).unwrap_or(200);
        return Ok(format!("raise MiddlewareAbort(JSONResponse({v}, status_code={code}))"));
    }
    if let Some(c) = Regex::new(r#"^redirect\(\s*(['"][^'"]+['"])\s*\)$"#).unwrap().captures(st) {
        return Ok(format!("raise MiddlewareAbort(RedirectResponse({}, status_code=302))", &c[1]));
    }
    Err(format!("拒否の返し方 `{}` は未対応(abort(N)・response()->json(値, N)・redirect('/x') のみ)", st.chars().take(40).collect::<String>()))
}

/// 独自ミドルウェアの `handle` の本体 → Python の行。
/// 形: `if (条件) { 拒否 }` を並べ、最後に `return $next($request);`。
pub fn compile_handle(body: &str) -> Result<Vec<String>, String> {
    let mut lines = vec![];
    let b = body.trim();
    let mut i = 0;
    let bytes = b.as_bytes();
    let mut done = false;
    while i < b.len() {
        while i < b.len() && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= b.len() {
            break;
        }
        if done {
            return Err("`return $next($request)` の後に文がある".into());
        }
        let rest = &b[i..];
        if rest.starts_with("if") && rest[2..].trim_start().starts_with('(') {
            let po = i + rest.find('(').unwrap();
            let pc = matching(b, po, '(', ')').ok_or("if の括弧が閉じていない")?;
            let cond = req_expr(&b[po + 1..pc])?;
            let after = b[pc + 1..].trim_start();
            let off = b.len() - after.len();
            if after.starts_with('{') {
                let bc = matching(b, off, '{', '}').ok_or("if の波括弧が閉じていない")?;
                let inner = b[off + 1..bc].trim();
                if inner.is_empty() || inner.matches(';').count() != 1 {
                    return Err("if の中が拒否1文だけでない".into());
                }
                lines.push(format!("    if {cond}:\n        {}", reject_stmt(inner)?));
                i = bc + 1;
            } else {
                let end = after.find(';').ok_or("if の文が閉じていない")?;
                lines.push(format!("    if {cond}:\n        {}", reject_stmt(&after[..end])?));
                i = off + end + 1;
            }
            if b[i..].trim_start().starts_with("else") {
                return Err("else は未対応".into());
            }
        } else if rest.starts_with("return") {
            let end = rest.find(';').ok_or("return が閉じていない")?;
            let r = rest[6..end].trim();
            if Regex::new(r"^\$next\(\s*\$request\s*\)$").unwrap().is_match(r) {
                done = true;
                i += end + 1;
            } else {
                return Err("`$next` の結果を使う処理(レスポンスの加工)は未対応".into());
            }
        } else {
            let end = rest.find(';').unwrap_or(rest.len());
            return Err(format!("文 `{}` は未対応(拒否の if と `return $next($request)` のみ)", rest[..end].trim().chars().take(40).collect::<String>()));
        }
    }
    if !done {
        return Err("`return $next($request)` が見つからない".into());
    }
    if lines.is_empty() {
        lines.push("    pass".into());
    }
    Ok(lines)
}

/// クラス名 → `handle` の本体
pub fn collect_custom(files: &[(String, String)]) -> HashMap<String, String> {
    let mut m = HashMap::new();
    let cls = Regex::new(r"class\s+(\w+)").unwrap();
    let h = Regex::new(r"function\s+handle\s*\([^)]*\)\s*(?::\s*[\w\\?|]+\s*)?\{").unwrap();
    for (_, src) in files {
        let (Some(c), Some(hm)) = (cls.captures(src), h.find(src)) else { continue };
        let open = hm.end() - 1;
        if let Some(close) = matching(src, open, '{', '}') {
            m.insert(c[1].to_string(), src[open + 1..close].to_string());
        }
    }
    m
}

#[derive(Default)]
pub struct Registry {
    defs: Vec<(String, String)>, // (依存の名前, Python のコード)
    pub used: Vec<String>,
}

impl Registry {
    fn mark(&mut self, name: &str) -> Option<String> {
        if !self.used.iter().any(|n| n == name) {
            self.used.push(name.to_string());
        }
        Some(name.to_string())
    }

    fn add(&mut self, name: &str, code: String) {
        if !self.defs.iter().any(|(n, _)| n == name) {
            self.defs.push((name.to_string(), code));
        }
        if !self.used.iter().any(|n| n == name) {
            self.used.push(name.to_string());
        }
    }

    /// ミドルウェアの名前 → 依存の名前。付けられないときは理由を `warn` へ。
    pub fn resolve(&mut self, token: &str, aliases: &HashMap<String, String>, custom: &HashMap<String, String>, ctx: &str, warn: &mut Vec<String>) -> Option<String> {
        let (base, params) = token.split_once(':').map(|(a, b)| (a, Some(b))).unwrap_or((token, None));
        match base {
            "auth" | "auth.basic" | "auth.session" => return self.mark("require_auth"),
            "guest" => return self.mark("require_guest"),
            "web" | "api" | "bindings" | "cors" | "csrf" | "substitute-bindings" => return None,
            "throttle" => {
                let (n, mnt) = match params {
                    Some(p) if p.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) => {
                        let v: Vec<&str> = p.split(',').collect();
                        (v[0].trim().parse::<u32>().unwrap_or(60), v.get(1).and_then(|x| x.trim().parse::<u32>().ok()).unwrap_or(1))
                    }
                    Some(p) => {
                        warn.push(format!("{ctx}: throttle:{p}(名前付きリミッター)は 60回/1分として扱った。RateLimiter::for の内容に合わせて直すこと"));
                        (60, 1)
                    }
                    None => (60, 1),
                };
                let name = format!("throttle_{n}_{mnt}");
                self.add(&name, format!("{name} = throttle({n}, {mnt}, \"{n}_{mnt}\")\n"));
                return Some(name);
            }
            _ => {}
        }
        let class = aliases.get(base).cloned().unwrap_or_else(|| base.to_string());
        match custom.get(&class) {
            Some(body) => match compile_handle(body) {
                Ok(lines) => {
                    let name = format!("mw_{}", snake_case(&class));
                    self.add(&name, format!("def {name}(request: Request):\n    \"\"\"Laravel ミドルウェア {class} の移植\"\"\"\n{}\n", lines.join("\n")));
                    Some(name)
                }
                Err(why) => {
                    warn.push(format!("{ctx}: ミドルウェア {class} は未変換(依存を付けていない): {why}"));
                    None
                }
            },
            None => {
                warn.push(format!("{ctx}: ミドルウェア `{token}` は未対応(依存を付けていない)。auth・guest・throttle・「拒否してから $next」の独自ミドルウェアのみ対応"));
                None
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.used.is_empty()
    }

    pub fn render(&self) -> String {
        let mut o = String::from(PRELUDE);
        for (_, code) in &self.defs {
            o.push('\n');
            o.push_str(code);
        }
        o
    }
}

const PRELUDE: &str = r##""""pl2pl が生成したミドルウェア(FastAPI の Depends)。
auth は `Authorization: Bearer <トークン>` が環境変数 API_TOKENS(カンマ区切り)にあるかで判定する。本物の認証(セッション・Sanctum 等)へ差し替えること。
throttle はメモリ上の簡易版(プロセスを再起動すると数え直し、複数プロセス間では共有されない)。"""
import fnmatch
import os
import time

from fastapi import Request
from fastapi.responses import JSONResponse, RedirectResponse


class MiddlewareAbort(Exception):
    """ミドルウェアがリクエストを止めて、このレスポンスを返す。"""

    def __init__(self, response):
        self.response = response


def _bearer(request: Request):
    h = request.headers.get("authorization", "")
    return h[7:].strip() if h.lower().startswith("bearer ") else None


def _valid_token(token) -> bool:
    return bool(token) and token in [t.strip() for t in os.environ.get("API_TOKENS", "").split(",") if t.strip()]


def _is(request: Request, pattern: str) -> bool:
    return fnmatch.fnmatch(request.url.path.lstrip("/"), pattern)


def require_auth(request: Request):
    if not _valid_token(_bearer(request)):
        raise MiddlewareAbort(JSONResponse({"message": "Unauthenticated."}, status_code=401))


def require_guest(request: Request):
    if _valid_token(_bearer(request)):
        raise MiddlewareAbort(RedirectResponse("/", status_code=302))


_hits: dict = {}


def throttle(limit: int = 60, minutes: int = 1, name: str = "default"):
    def dep(request: Request):
        now = time.time()
        window = minutes * 60
        route = request.scope.get("route")
        key = (name, route.path if route else request.url.path, request.client.host if request.client else "?")
        recent = [t for t in _hits.get(key, []) if now - t < window]
        if len(recent) >= limit:
            retry = int(window - (now - recent[0])) + 1
            raise MiddlewareAbort(JSONResponse({"message": "Too Many Attempts."}, status_code=429, headers={"Retry-After": str(retry)}))
        recent.append(now)
        _hits[key] = recent

    return dep

"##;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_and_chain() {
        assert_eq!(parse_names("['auth', 'throttle:60,1', \\App\\Http\\Middleware\\Admin::class]"), vec!["auth", "throttle:60,1", "Admin"]);
        assert_eq!(route_mw("->name('x')->middleware(['auth','admin'])->where('id','[0-9]+');\nRoute::get"), vec!["auth", "admin"]);
        assert!(route_mw(";\nRoute::get('/a')->middleware('x')").is_empty());
    }

    #[test]
    fn handle_guards() {
        let body = "if (!$request->header('X-Token')) { return response()->json(['error' => 'no token'], 401); }\n if ($request->ip() === '1.2.3.4') abort(403);\n return $next($request);";
        let l = compile_handle(body).unwrap();
        assert!(l[0].contains("if not request.headers.get('X-Token'):"), "{l:?}");
        assert!(l[0].contains("JSONResponse({\"error\": \"no token\"}, status_code=401)"), "{l:?}");
        assert!(l[1].contains("== '1.2.3.4'") && l[1].contains("status_code=403"), "{l:?}");
        assert!(compile_handle("$r = $next($request); $r->header('a','b'); return $r;").is_err());
        assert!(compile_handle("foo(); return $next($request);").is_err());
    }
}
