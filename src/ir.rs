//! 共通の中間表現(IR)の試作(サーバー側・ルートだけ)。
//! 入力側(フロントエンド)は言語ごとにルートを `Endpoint` に直し、出力側(バックエンド)は `Endpoint` から書き出す。
//! こうすると、入力 N 種 × 出力 M 種の組み合わせを N+M 個の部品で作れる。いまの入力は PHP、出力は JSON と OpenAPI。
use crate::php::Route;

#[derive(Debug, Clone, PartialEq)]
pub struct Endpoint {
    pub verb: String,
    pub path: String,
    pub name: String,
    pub middleware: Vec<String>,
    pub source: String,
    pub line: usize,
    /// 型つきの引数("id: int" の形)。空なら パスから推測する
    pub params: Vec<String>,
    /// FastAPI の `dependencies=[Depends(..)]` に入れる名前
    pub deps: Vec<String>,
    /// 関数の本体
    pub body: Body,
}

/// 関数の本体。変換できたら Python の行、できなければ理由つきの TODO(黙って落とさない)。
#[derive(Debug, Clone, PartialEq)]
pub enum Body {
    Todo(String),
    Python(Vec<String>),
}

/// 入力側: PHP(Laravel / Symfony)
pub fn from_php(routes: &[Route]) -> Vec<Endpoint> {
    routes.iter().map(|r| Endpoint { verb: r.verb.to_lowercase(), path: r.path.clone(), name: r.handler.clone(), middleware: r.mw.clone(), source: r.src.clone(), line: r.line, params: vec![], deps: vec![], body: Body::Todo(format!("{}:{}", r.src, r.line)) }).collect()
}

fn ep(verb: &str, path: &str, name: &str, src: &str, line: usize) -> Endpoint {
    Endpoint { verb: verb.to_lowercase(), path: path.to_string(), name: name.to_string(), middleware: vec![], source: src.to_string(), line, params: vec![], deps: vec![], body: Body::Todo(format!("{src}:{line}")) }
}

fn line_of(text: &str, pos: usize) -> usize {
    text[..pos].matches('\n').count() + 1
}

/// 入力側: Python(Flask の `@app.route` / `@bp.route`、FastAPI の `@app.get` など)
pub fn from_python(files: &[(String, String)]) -> Vec<Endpoint> {
    let re = regex::Regex::new(r#"@\w+\.(route|get|post|put|patch|delete)\(\s*['"]([^'"]+)['"]([^)]*)\)\s*
\s*(?:async\s+)?def\s+(\w+)"#).unwrap();
    let methods = regex::Regex::new(r#"methods\s*=\s*\[([^\]]*)\]"#).unwrap();
    let mut v = vec![];
    for (name, t) in files.iter().filter(|(n, _)| n.ends_with(".py")) {
        for c in re.captures_iter(t) {
            let line = line_of(t, c.get(0).unwrap().start());
            let path = c[2].replace('<', "{").replace('>', "}");
            if &c[1] == "route" {
                let ms: Vec<String> = methods.captures(&c[3]).map(|m| m[1].split(',').map(|x| x.trim().trim_matches(|ch| ch == '\'' || ch == '"').to_string()).filter(|x| !x.is_empty()).collect()).unwrap_or_else(|| vec!["GET".into()]);
                for m in ms {
                    v.push(ep(&m, &path, &c[4], name, line));
                }
            } else {
                v.push(ep(&c[1], &path, &c[4], name, line));
            }
        }
    }
    v
}

/// 入力側: Java(Spring の `@GetMapping("/x")` など、クラスの `@RequestMapping("/base")` を前に付ける)
pub fn from_java(files: &[(String, String)]) -> Vec<Endpoint> {
    let base_re = regex::Regex::new(r#"@RequestMapping\(\s*(?:value\s*=\s*)?"([^"]*)"[^)]*\)\s*(?:@\w+(?:\([^)]*\))?\s*)*(?:public\s+)?class"#).unwrap();
    let re = regex::Regex::new(r#"@(Get|Post|Put|Patch|Delete)Mapping(?:\(\s*(?:value\s*=\s*|path\s*=\s*)?"([^"]*)"[^)]*\))?\s*(?:@\w+(?:\([^)]*\))?\s*)*public\s+[\w<>\[\], ?]+\s+(\w+)\s*\("#).unwrap();
    let mut v = vec![];
    for (name, t) in files.iter().filter(|(n, _)| n.ends_with(".java")) {
        let base = base_re.captures(t).map(|c| c[1].trim_end_matches('/').to_string()).unwrap_or_default();
        for c in re.captures_iter(t) {
            let sub = c.get(2).map(|m| m.as_str()).unwrap_or("");
            let path = format!("{}{}{}", base, if sub.is_empty() || sub.starts_with('/') { "" } else { "/" }, sub);
            let path = if path.is_empty() { "/".to_string() } else { path };
            v.push(ep(&c[1], &path, &c[3], name, line_of(t, c.get(0).unwrap().start())));
        }
    }
    v
}

/// 入力側: Java の JAX-RS(`@Path` + `@GET` など)・Servlet(`@WebServlet` + `doGet` など)・Spark(`get("/x", ...)`)
pub fn from_java_other(files: &[(String, String)]) -> Vec<Endpoint> {
    let class_path = regex::Regex::new(r#"@Path\(\s*"([^"]*)"\s*\)\s*(?:@\w+(?:\([^)]*\))?\s*)*(?:public\s+)?(?:class|interface)"#).unwrap();
    let jaxrs = regex::Regex::new(r#"@(GET|POST|PUT|PATCH|DELETE)\s*(?:@Path\(\s*"([^"]*)"\s*\)\s*)?(?:@\w+(?:\([^)]*\))?\s*)*public\s+[\w<>\[\], ?.]+\s+(\w+)\s*\("#).unwrap();
    let jaxrs_rev = regex::Regex::new(r#"@Path\(\s*"([^"]*)"\s*\)\s*@(GET|POST|PUT|PATCH|DELETE)\s*(?:@\w+(?:\([^)]*\))?\s*)*public\s+[\w<>\[\], ?.]+\s+(\w+)\s*\("#).unwrap();
    let servlet = regex::Regex::new(r#"@WebServlet\(\s*(?:urlPatterns\s*=\s*|value\s*=\s*)?\{?\s*"([^"]*)""#).unwrap();
    let do_m = regex::Regex::new(r#"void\s+do(Get|Post|Put|Delete|Patch)\s*\("#).unwrap();
    let spark = regex::Regex::new(r#"(?m)^\s*(get|post|put|patch|delete)\(\s*"([^"]+)"\s*,"#).unwrap();
    let join = |base: &str, sub: &str| {
        let s = sub.trim_matches('/');
        let p = format!("{}{}{}", base.trim_end_matches('/'), if s.is_empty() { "" } else { "/" }, s);
        if p.is_empty() { "/".to_string() } else if p.starts_with('/') { p } else { format!("/{p}") }
    };
    let mut v = vec![];
    for (name, t) in files.iter().filter(|(n, _)| n.ends_with(".java")) {
        let base = class_path.captures(t).map(|c| c[1].to_string()).unwrap_or_default();
        let mut seen: Vec<usize> = vec![];
        for c in jaxrs_rev.captures_iter(t) {
            let m = c.get(0).unwrap();
            seen.push(m.start());
            v.push(ep(&c[2], &join(&base, &c[1]), &c[3], name, line_of(t, m.start())));
        }
        for c in jaxrs.captures_iter(t) {
            let m = c.get(0).unwrap();
            // `@Path` が先に付いている形は上で拾い済み(同じメソッドを二重に数えない)
            let before = &t[m.start().saturating_sub(80)..m.start()];
            if before.contains("@Path(") && before.trim_end().ends_with(')') && jaxrs_rev.is_match(&t[m.start().saturating_sub(80)..m.end()]) {
                continue;
            }
            let sub = c.get(2).map(|x| x.as_str()).unwrap_or("");
            v.push(ep(&c[1], &join(&base, sub), &c[3], name, line_of(t, m.start())));
        }
        if let Some(c) = servlet.captures(t) {
            for d in do_m.captures_iter(t) {
                v.push(ep(&d[1], &c[1], &format!("do{}", &d[1]), name, line_of(t, d.get(0).unwrap().start())));
            }
        }
        for c in spark.captures_iter(t) {
            let p = regex::Regex::new(r":(\w+)").unwrap().replace_all(&c[2], "{$1}").to_string();
            v.push(ep(&c[1], &p, "lambda", name, line_of(t, c.get(0).unwrap().start())));
        }
    }
    v
}

/// パスの `{id}` / `{int:id}` / `{id?}` を、きれいなパス(`{id}`)と引数の一覧(名前, 型)に分ける
pub fn split_params(path: &str) -> (String, Vec<(String, &'static str)>) {
    let re = regex::Regex::new(r"\{(?:(int|float|str|string|path|uuid):)?(\w+)\??(?::[^}]*)?\}").unwrap();
    let mut ps = vec![];
    let clean = re.replace_all(path, |c: &regex::Captures| {
        let ty = match c.get(1).map(|m| m.as_str()) {
            Some("int") => "integer",
            Some("float") => "number",
            _ => "string",
        };
        ps.push((c[2].to_string(), ty));
        format!("{{{}}}", &c[2])
    });
    (clean.to_string(), ps)
}

fn q(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

/// 出力側 1: そのままの JSON
pub fn to_json(eps: &[Endpoint]) -> String {
    let items: Vec<String> = eps
        .iter()
        .map(|e| format!("  {{\"verb\": {}, \"path\": {}, \"name\": {}, \"middleware\": [{}], \"source\": {}, \"line\": {}}}", q(&e.verb), q(&e.path), q(&e.name), e.middleware.iter().map(|m| q(m)).collect::<Vec<_>>().join(", "), q(&e.source), e.line))
        .collect();
    format!("[\n{}\n]\n", items.join(",\n"))
}

/// 出力側 2: OpenAPI 3 の骨組み(`{id}` 形式のパス。本体の型は分からないので説明にだけ元の場所を書く)
pub fn to_openapi(eps: &[Endpoint]) -> String {
    let mut paths: Vec<(String, Vec<&Endpoint>)> = vec![];
    for e in eps {
        match paths.iter_mut().find(|(p, _)| *p == e.path) {
            Some((_, v)) => v.push(e),
            None => paths.push((e.path.clone(), vec![e])),
        }
    }
    let body: Vec<String> = paths
        .iter()
        .map(|(p, v)| {
            let ops: Vec<String> = v
                .iter()
                .filter(|e| ["get", "post", "put", "patch", "delete", "head", "options"].contains(&e.verb.as_str()))
                .map(|e| {
                    let params: Vec<String> = split_params(&e.path).1.iter().map(|(n, t)| format!("{{\"name\": {}, \"in\": \"path\", \"required\": true, \"schema\": {{\"type\": \"{}\"}}}}", q(n), t)).collect();
                    format!("      {}: {{\"summary\": {}, \"description\": {}, \"parameters\": [{}], \"responses\": {{\"200\": {{\"description\": \"OK\"}}}}}}", q(&e.verb), q(&e.name), q(&format!("{}:{}", e.source, e.line)), params.join(", "))
                })
                .collect();
            format!("    {}: {{\n{}\n    }}", q(&split_params(p).0), ops.join(",\n"))
        })
        .collect();
    format!("{{\n  \"openapi\": \"3.0.3\",\n  \"info\": {{\"title\": \"pl2pl ir\", \"version\": \"0.1.0\"}},\n  \"paths\": {{\n{}\n  }}\n}}\n", body.join(",\n"))
}

/// 関数名にできる形へ直し、予約語・重複を避ける
pub fn unique_name(raw: &str, used: &mut Vec<String>) -> String {
    const KW: &[&str] = &["False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while", "with", "yield"];
    let mut base: String = raw.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    if base.is_empty() || base.starts_with(|c: char| c.is_ascii_digit()) {
        base = format!("route_{base}");
    }
    if KW.contains(&base.as_str()) {
        base.push('_');
    }
    let mut name = base.clone();
    let mut n = 2;
    while used.contains(&name) {
        name = format!("{base}_{n}");
        n += 1;
    }
    used.push(name.clone());
    name
}

/// 出力側 3(共通): 1 本のルートを FastAPI の関数にする。PHP 変換もこれを通る。
pub fn emit_fastapi_route(e: &Endpoint, fname: &str) -> String {
    let (path, ps) = split_params(&e.path);
    let mut args: Vec<String> = if e.params.is_empty() { ps.iter().map(|(p, t)| format!("{p}: {}", match *t { "integer" => "int", "number" => "float", _ => "str" })).collect() } else { e.params.clone() };
    if args.is_empty() && e.params.is_empty() {
        args = vec![];
    }
    let deps = if e.deps.is_empty() { String::new() } else { format!(", dependencies=[{}]", e.deps.iter().map(|d| format!("Depends({d})")).collect::<Vec<_>>().join(", ")) };
    let body = match &e.body {
        Body::Python(lines) => lines.iter().map(|l| format!("    {l}
")).collect::<String>(),
        Body::Todo(why) if why.contains(':') && e.params.is_empty() => format!("    # TODO(pl2pl): 本体は未変換です({why})
    raise NotImplementedError
"),
        Body::Todo(why) => format!("    # PHP: {why} ({} {})
    raise NotImplementedError(\"TODO(pl2pl): {why} を移植する\")
", e.verb.to_uppercase(), e.path),
    };
    format!("

@app.{}({}{deps})
def {}({}):
{}", e.verb, q(&path), fname, args.join(", "), body)
}

/// 出力側 3: FastAPI の骨組み。本体は IR に無ければ TODO を残す(黙って作り話をしない)。同じ関数名は連番にする。
pub fn to_fastapi(eps: &[Endpoint]) -> String {
    let mut out = String::from("from fastapi import FastAPI

app = FastAPI()
");
    let mut used: Vec<String> = vec![];
    for e in eps.iter().filter(|e| ["get", "post", "put", "patch", "delete"].contains(&e.verb.as_str())) {
        let name = unique_name(&e.name, &mut used);
        out.push_str(&emit_fastapi_route(e, &name));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn python_and_java_frontends() {
        let py = vec![("app.py".to_string(), "@app.route('/u/<int:id>', methods=['GET', 'POST'])
def u(id):
    pass
@app.post('/x')
async def x():
    pass
".to_string())];
        let e = from_python(&py);
        assert_eq!(e.len(), 3, "{e:?}");
        assert!(e.iter().any(|x| x.verb == "post" && x.path == "/u/{int:id}"), "{e:?}");
        let jv = vec![("C.java".to_string(), "@RestController
@RequestMapping(\"/api/users\")
public class C {
  @GetMapping(\"/{id}\")
  public User get(@PathVariable Long id) { return null; }
  @PostMapping
  public User add() { return null; }
}
".to_string())];
        let e = from_java(&jv);
        assert!(e.iter().any(|x| x.verb == "get" && x.path == "/api/users/{id}"), "{e:?}");
        assert!(e.iter().any(|x| x.verb == "post" && x.path == "/api/users"), "{e:?}");
    }

    #[test]
    fn java_jaxrs_servlet_spark() {
        let f = |n: &str, t: &str| vec![(n.to_string(), t.to_string())];
        let e = from_java_other(&f("R.java", "@Path(\"/items\")
public class R {
 @GET
 @Path(\"{id}\")
 public String one(int id) { return \"\"; }
 @POST
 public int add(String b) { return 1; }
}
"));
        assert!(e.iter().any(|x| x.verb == "get" && x.path == "/items/{id}"), "{e:?}");
        assert!(e.iter().any(|x| x.verb == "post" && x.path == "/items"), "{e:?}");
        assert_eq!(e.len(), 2, "{e:?}");
        let e = from_java_other(&f("S.java", "@WebServlet(\"/hello\")
public class S extends HttpServlet {
 protected void doGet(HttpServletRequest a, HttpServletResponse b) {}
 protected void doPost(HttpServletRequest a, HttpServletResponse b) {}
}
"));
        assert!(e.iter().any(|x| x.verb == "get" && x.path == "/hello") && e.iter().any(|x| x.verb == "post"), "{e:?}");
        let e = from_java_other(&f("A.java", "class A { void m() {
  get(\"/hi/:name\", (req, res) -> \"x\");
  post(\"/add\", (req, res) -> 1);
}}
"));
        assert!(e.iter().any(|x| x.path == "/hi/{name}") && e.iter().any(|x| x.verb == "post" && x.path == "/add"), "{e:?}");
    }

    #[test]
    fn fastapi_backend() {
        let eps = vec![ep("get", "/u/{int:id}", "show", "a.py", 3), ep("post", "/u", "show", "a.py", 9), ep("delete", "/x", "9bad-name", "a.py", 12), ep("get", "/l", "lambda", "a.py", 15)];
        let f = to_fastapi(&eps);
        assert!(f.contains("@app.get(\"/u/{id}\")") && f.contains("def show(id: int):") && f.contains("def show_2():") && f.contains("def route_9bad_name():") && f.contains("def lambda_():"), "{f}");
    }

    #[test]
    fn path_params() {
        let (p, ps) = split_params("/u/{int:id}/p/{slug}/{opt?}");
        assert_eq!(p, "/u/{id}/p/{slug}/{opt}");
        assert_eq!(ps, vec![("id".to_string(), "integer"), ("slug".to_string(), "string"), ("opt".to_string(), "string")]);
        let o = to_openapi(&[ep("get", "/u/{int:id}", "u", "a.py", 1)]);
        assert!(o.contains("\"/u/{id}\"") && o.contains("\"in\": \"path\"") && o.contains("\"integer\""), "{o}");
    }

    #[test]
    fn php_to_ir_to_openapi() {
        let files = vec![("routes/web.php".to_string(), "<?php\nRoute::get('/users/{id}', fn($id) => $id);\nRoute::post('/users', fn() => 1);\n".to_string())];
        let mut w = vec![];
        let eps = from_php(&crate::php::extract(&files, &mut w));
        assert!(eps.iter().any(|e| e.verb == "get" && e.path.contains("{id}")), "{eps:?}");
        let j = to_json(&eps);
        assert!(j.contains("\"verb\": \"post\""), "{j}");
        let o = to_openapi(&eps);
        assert!(o.contains("\"openapi\": \"3.0.3\"") && o.contains("\"post\""), "{o}");
    }
}
