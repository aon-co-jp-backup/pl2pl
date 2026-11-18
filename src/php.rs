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
    if let Some(c) = Regex::new(r#"^\$request->(?:input|get|post)\(\s*['"](\w+)['"]\s*\)$"#).unwrap().captures(a) {
        return Some(format!("(payload or {{}}).get({:?})", &c[1]));
    }
    if let Some(c) = Regex::new(r"^\$(\w+)->(\w+)$").unwrap().captures(a) {
        return Some(format!("{}[{:?}]", &c[1], &c[2]));
    }
    php_value(a)
}

/// 複数文の本体(Eloquent の取得→更新→保存→返却)を Python の行へ。1文でも分からなければ None(全体を TODO にする)。
fn translate_body(body: &str, models: &mut Vec<String>, need_response: &mut bool) -> Option<Vec<String>> {
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
            if Regex::new(r"^response\(\)->noContent\(\)$").unwrap().is_match(r) {
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
"##;

fn models_py(models: &[String], schema: &Schema) -> String {
    let mut o = String::from("\"\"\"pl2pl が生成したモデル(SQLAlchemy)。\"\"\"\nfrom fastapi import HTTPException\nfrom sqlalchemy import JSON, Boolean, Column, Float, Integer, String, Text\n\nfrom db import Base, Session, engine\n");
    for m in models {
        let (t, cols) = schema.get(m).cloned().unwrap_or_else(|| (guess_table(m), vec![]));
        let cs: String = cols.iter().map(|(n, ty)| format!("    {n} = Column({ty})\n")).collect();
        let names: String = cols.iter().map(|(n, _)| format!("{n:?}, ")).collect();
        o.push_str(&MODEL_PY.replace("__M__", m).replace("__T__", &t).replace("__COLS__", &cs).replace("__NAMES__", &names));
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
    let bodies = collect_bodies(files);
    let schema = collect_schema(files);
    rep.extend(warn.iter().cloned());
    let mut used: Vec<String> = vec![];
    let mut models: Vec<String> = vec![];
    let mut need_response = false;
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
        let raw_body = bodies.get(&(r.class.clone(), r.method.clone()));
        let mut ctl_body = if r.body.is_none() { raw_body.and_then(|b| simple_return(b)) } else { None };
        if r.body.is_none() && ctl_body.is_none() {
            if let Some((model, expr)) = raw_body.and_then(|b| eloquent_return(b)) {
                if !models.contains(&model) {
                    models.push(model);
                }
                ctl_body = Some(expr);
            }
        }
        let multi = if r.body.is_none() && ctl_body.is_none() { raw_body.and_then(|b| translate_body(b, &mut models, &mut need_response)) } else { None };
        match r.body.as_ref().or(ctl_body.as_ref()) {
            None if multi.is_some() => {
                for l in multi.unwrap() {
                    m.push_str(&format!("    {l}\n"));
                }
            }
            Some(b) => m.push_str(&format!("    return {b}\n")),
            None => {
                m.push_str(&format!("    # PHP: {} ({} {})\n    raise NotImplementedError(\"TODO(pl2pl): {} を移植する\")\n", r.handler, r.verb.to_uppercase(), r.path, r.handler));
                rep.push(format!("{}:{} {} {} の本体は未変換(TODO)", "routes", r.line, r.verb.to_uppercase(), r.path));
            }
        }
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
        outf.push(("models.py".to_string(), models_py(&models, &schema)));
        outf[1].1 = "fastapi\nuvicorn\nsqlalchemy\n".to_string();
    }
    if target == "mfastapi" {
        outf.push(("run_mfastapi.py".to_string(), "import mfastapi\nfrom main import app\n\nmfastapi.accelerate(app)\n\nif __name__ == \"__main__\":\n    import uvicorn\n    uvicorn.run(app, host=\"127.0.0.1\", port=8000)\n".to_string()));
        outf[1].1 = format!("{}mfastapi\n", outf[1].1);
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

    #[test]
    fn multi_statement_body() {
        let mut m = vec![];
        let mut r = false;
        let l = translate_body("$u = User::findOrFail($id); $u->update($request->all()); return response()->json($u);", &mut m, &mut r).unwrap();
        assert_eq!(l, vec!["u = User.find_or_fail(id)", "u = User.update(u[\"id\"], (payload or {}))", "return u"]);
        assert_eq!(m, vec!["User"]);
        assert!(translate_body("$x = foo(); return $x;", &mut m, &mut r).is_none());
    }
}
