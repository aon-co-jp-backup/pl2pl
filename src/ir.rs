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
