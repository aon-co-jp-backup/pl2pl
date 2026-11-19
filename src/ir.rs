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
}

/// 入力側: PHP(Laravel / Symfony)
pub fn from_php(routes: &[Route]) -> Vec<Endpoint> {
    routes.iter().map(|r| Endpoint { verb: r.verb.to_lowercase(), path: r.path.clone(), name: r.handler.clone(), middleware: r.mw.clone(), source: r.src.clone(), line: r.line }).collect()
}

fn ep(verb: &str, path: &str, name: &str, src: &str, line: usize) -> Endpoint {
    Endpoint { verb: verb.to_lowercase(), path: path.to_string(), name: name.to_string(), middleware: vec![], source: src.to_string(), line }
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
                .map(|e| format!("      {}: {{\"summary\": {}, \"description\": {}, \"responses\": {{\"200\": {{\"description\": \"OK\"}}}}}}", q(&e.verb), q(&e.name), q(&format!("{}:{}", e.source, e.line))))
                .collect();
            format!("    {}: {{\n{}\n    }}", q(p), ops.join(",\n"))
        })
        .collect();
    format!("{{\n  \"openapi\": \"3.0.3\",\n  \"info\": {{\"title\": \"pl2pl ir\", \"version\": \"0.1.0\"}},\n  \"paths\": {{\n{}\n  }}\n}}\n", body.join(",\n"))
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
