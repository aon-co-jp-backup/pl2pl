//! Java の宣言レベルの構文解析(型・フィールド・メソッド・注釈)。メソッド本体はトークンのまま保持する。

use crate::lexer::{lex, Tok};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Ann {
    pub name: String,                  // 最後の要素(`org.x.Foo` → `Foo`)
    pub args: HashMap<String, String>, // 無名の第1引数は `value`
}
impl Ann {
    pub fn get(&self, k: &str) -> Option<&String> {
        self.args.get(k)
    }
    pub fn value(&self) -> Option<String> {
        self.args.get("value").or_else(|| self.args.get("path")).cloned()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Param {
    pub anns: Vec<Ann>,
    pub ty: String,
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct Field {
    pub anns: Vec<Ann>,
    pub ty: String,
    pub name: String,
    pub is_static: bool,
    pub init: Vec<Tok>,
}

#[derive(Clone, Debug, Default)]
pub struct Method {
    pub anns: Vec<Ann>,
    pub name: String,
    pub ret: String,
    pub params: Vec<Param>,
    pub body: Option<Vec<Tok>>, // 波括弧の内側
    pub is_static: bool,
    pub is_ctor: bool,
}
impl Method {
    pub fn ann(&self, n: &str) -> Option<&Ann> {
        self.anns.iter().find(|a| a.name == n)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Class {
    pub kind: String, // class / interface / enum / record / annotation
    pub name: String,
    pub anns: Vec<Ann>,
    pub extends: Vec<String>,
    pub implements: Vec<String>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub enum_consts: Vec<String>,
}
impl Class {
    pub fn ann(&self, n: &str) -> Option<&Ann> {
        self.anns.iter().find(|a| a.name == n)
    }
    pub fn has_ann(&self, ns: &[&str]) -> bool {
        self.anns.iter().any(|a| ns.contains(&a.name.as_str()))
    }
}

#[derive(Clone, Debug, Default)]
pub struct Unit {
    pub file: String,
    pub package: String,
    pub imports: Vec<String>,
    pub classes: Vec<Class>,
    pub tokens: Vec<Tok>,
}

const MODS: [&str; 13] = [
    "public", "private", "protected", "static", "final", "abstract", "default", "synchronized", "native", "transient", "volatile", "strictfp", "sealed",
];

struct P<'a> {
    t: &'a [Tok],
    i: usize,
}

impl<'a> P<'a> {
    fn peek(&self) -> Option<&Tok> {
        self.t.get(self.i)
    }
    fn at(&self, p: &str) -> bool {
        self.peek().is_some_and(|t| t.is(p))
    }
    fn at_id(&self, p: &str) -> bool {
        self.peek().is_some_and(|t| t.id(p))
    }
    fn eat(&mut self, p: &str) -> bool {
        if self.at(p) {
            self.i += 1;
            true
        } else {
            false
        }
    }
    fn ident(&mut self) -> Option<String> {
        if let Some(Tok::Id(s)) = self.peek() {
            let s = s.clone();
            self.i += 1;
            Some(s)
        } else {
            None
        }
    }
    /// `open` から対応する `close` まで(両端を含まない)を取り出す。
    fn balanced(&mut self, open: &str, close: &str) -> Vec<Tok> {
        let mut depth = 0;
        let mut out = vec![];
        while let Some(t) = self.peek().cloned() {
            self.i += 1;
            if t.is(open) {
                depth += 1;
                if depth == 1 {
                    continue;
                }
            } else if t.is(close) {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            out.push(t);
        }
        out
    }
    fn qualified(&mut self) -> String {
        let mut s = self.ident().unwrap_or_default();
        while self.at(".") && matches!(self.t.get(self.i + 1), Some(Tok::Id(_))) {
            self.i += 1;
            s.push('.');
            s.push_str(&self.ident().unwrap());
        }
        s
    }
    fn annotation(&mut self) -> Option<Ann> {
        if !self.at("@") || self.t.get(self.i + 1).is_some_and(|t| t.id("interface")) {
            return None;
        }
        self.i += 1;
        let q = self.qualified();
        let name = q.rsplit('.').next().unwrap_or("").to_string();
        let mut a = Ann { name, args: HashMap::new() };
        if self.at("(") {
            let toks = self.balanced("(", ")");
            for (n, part) in split_top(&toks, ",").into_iter().enumerate() {
                if part.is_empty() {
                    continue;
                }
                if part.len() >= 2 && matches!(part[0], Tok::Id(_)) && part[1].is("=") {
                    a.args.insert(part[0].text(), ann_value(&part[2..]));
                } else {
                    a.args.insert(if n == 0 { "value".into() } else { format!("_{n}") }, ann_value(&part));
                }
            }
        }
        Some(a)
    }
    fn annotations(&mut self) -> Vec<Ann> {
        let mut v = vec![];
        while let Some(a) = self.annotation() {
            v.push(a);
        }
        v
    }
    fn mods(&mut self) -> (Vec<Ann>, bool) {
        let mut anns = vec![];
        let mut st = false;
        loop {
            if let Some(a) = self.annotation() {
                anns.push(a);
            } else if let Some(Tok::Id(s)) = self.peek() {
                let next_colon = self.t.get(self.i + 1).is_some_and(|t| t.is(":") || t.is("->"));
                if MODS.contains(&s.as_str()) && !(s == "default" && next_colon) {
                    if s == "static" {
                        st = true;
                    }
                    self.i += 1;
                } else if s == "non" && self.t.get(self.i + 1).is_some_and(|t| t.is("-")) && self.t.get(self.i + 2).is_some_and(|t| t.id("sealed")) {
                    self.i += 3;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        (anns, st)
    }
    fn generics(&mut self) -> String {
        let mut depth = 0i32;
        let mut s = String::new();
        while let Some(t) = self.peek().cloned() {
            self.i += 1;
            match &t {
                Tok::P(p) if p == "<" => depth += 1,
                Tok::P(p) if p == ">" => depth -= 1,
                Tok::P(p) if p == ">>" => depth -= 2,
                Tok::P(p) if p == ">>>" => depth -= 3,
                _ => {}
            }
            s.push_str(&t.text());
            if matches!(t, Tok::P(ref p) if p == ",") {
                s.push(' ');
            }
            if depth <= 0 {
                break;
            }
        }
        s
    }
    fn jtype(&mut self) -> String {
        let _ = self.annotations();
        let mut s = self.qualified();
        if s.is_empty() {
            return s;
        }
        if self.at("<") {
            s.push_str(&self.generics());
        }
        while self.at("[") && self.t.get(self.i + 1).is_some_and(|t| t.is("]")) {
            self.i += 2;
            s.push_str("[]");
        }
        if self.eat("...") {
            s.push_str("[]");
        }
        s
    }
    fn params(&mut self) -> Vec<Param> {
        let toks = self.balanced("(", ")");
        let mut out = vec![];
        for part in split_top(&toks, ",") {
            if part.is_empty() {
                continue;
            }
            let mut q = P { t: &part, i: 0 };
            let (anns, _) = q.mods();
            let ty = q.jtype();
            let name = q.ident().unwrap_or_else(|| "arg".into());
            out.push(Param { anns, ty, name });
        }
        out
    }
    fn skip_to_semicolon(&mut self) -> Vec<Tok> {
        let mut depth = 0i32;
        let mut out = vec![];
        while let Some(t) = self.peek().cloned() {
            self.i += 1;
            if t.is("(") || t.is("{") || t.is("[") {
                depth += 1;
            } else if t.is(")") || t.is("}") || t.is("]") {
                depth -= 1;
            } else if t.is(";") && depth <= 0 {
                break;
            }
            out.push(t);
        }
        out
    }
    fn skip_throws(&mut self) {
        if self.at_id("throws") {
            self.i += 1;
            loop {
                self.jtype();
                if !self.eat(",") {
                    break;
                }
            }
        }
    }

    fn type_decl(&mut self, anns: Vec<Ann>, out: &mut Vec<Class>) -> bool {
        let kind = match self.peek() {
            Some(Tok::Id(s)) if matches!(s.as_str(), "class" | "interface" | "enum" | "record") => s.clone(),
            Some(Tok::P(p)) if p == "@" && self.t.get(self.i + 1).is_some_and(|t| t.id("interface")) => {
                self.i += 1;
                "annotation".to_string()
            }
            _ => return false,
        };
        // `record` は型名が続くときだけ宣言(変数名 record と区別する)
        if kind == "record" && !matches!(self.t.get(self.i + 1), Some(Tok::Id(_))) {
            return false;
        }
        self.i += 1;
        let name = self.ident().unwrap_or_default();
        let mut c = Class { kind: kind.clone(), name, anns, ..Default::default() };
        if self.at("<") {
            let _ = self.generics();
        }
        let mut rec_params = vec![];
        if kind == "record" && self.at("(") {
            rec_params = self.params();
        }
        loop {
            if self.at_id("extends") {
                self.i += 1;
                loop {
                    c.extends.push(self.jtype());
                    if !self.eat(",") {
                        break;
                    }
                }
            } else if self.at_id("implements") || self.at_id("permits") {
                let imp = self.at_id("implements");
                self.i += 1;
                loop {
                    let t = self.jtype();
                    if imp {
                        c.implements.push(t);
                    }
                    if !self.eat(",") {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        for p in rec_params {
            c.fields.push(Field { anns: p.anns, ty: p.ty, name: p.name, ..Default::default() });
        }
        if !self.at("{") {
            self.skip_to_semicolon();
            out.push(c);
            return true;
        }
        let body = self.balanced("{", "}");
        let mut q = P { t: &body, i: 0 };
        if kind == "enum" {
            loop {
                let _ = q.annotations();
                match q.ident() {
                    Some(n) => {
                        c.enum_consts.push(n);
                        if q.at("(") {
                            let _ = q.balanced("(", ")");
                        }
                        if q.at("{") {
                            let _ = q.balanced("{", "}");
                        }
                    }
                    None => break,
                }
                if !q.eat(",") {
                    break;
                }
            }
            q.eat(";");
        }
        q.members(&mut c, out);
        out.push(c);
        true
    }

    fn members(&mut self, c: &mut Class, nested: &mut Vec<Class>) {
        while self.peek().is_some() {
            let start = self.i;
            let (anns, is_static) = self.mods();
            if self.at("{") {
                let _ = self.balanced("{", "}");
                continue;
            }
            if self.eat(";") {
                continue;
            }
            if self.type_decl(anns.clone(), nested) {
                continue;
            }
            if self.at("<") {
                let _ = self.generics();
            }
            // コンストラクター(レコードのコンパクト形 `Name {` も含む)
            if self.peek().is_some_and(|t| t.id(&c.name)) && self.t.get(self.i + 1).is_some_and(|t| t.is("(") || t.is("{")) {
                self.i += 1;
                let params = if self.at("(") { self.params() } else { c.fields.iter().map(|f| Param { anns: vec![], ty: f.ty.clone(), name: f.name.clone() }).collect() };
                self.skip_throws();
                let body = if self.at("{") {
                    Some(self.balanced("{", "}"))
                } else {
                    self.skip_to_semicolon();
                    None
                };
                c.methods.push(Method { anns, name: c.name.clone(), ret: String::new(), params, body, is_static, is_ctor: true });
                continue;
            }
            let ty = self.jtype();
            let Some(name) = self.ident() else {
                if self.i == start {
                    self.i += 1;
                }
                continue;
            };
            if self.at("(") {
                let params = self.params();
                while self.at("[") {
                    self.i += 2;
                }
                self.skip_throws();
                let body = if self.at("{") {
                    Some(self.balanced("{", "}"))
                } else {
                    if self.at_id("default") {
                        self.i += 1;
                    }
                    self.skip_to_semicolon();
                    None
                };
                c.methods.push(Method { anns, name, ret: ty, params, body, is_static, is_ctor: false });
            } else {
                let mut init = vec![];
                if self.eat("=") {
                    init = self.skip_to_semicolon();
                } else {
                    self.skip_to_semicolon();
                }
                c.fields.push(Field { anns, ty, name, is_static, init });
            }
        }
    }
}

fn ann_value(t: &[Tok]) -> String {
    // `{"a","b"}` は最初の要素、`"x" + "y"` は連結、`Foo.BAR` はそのまま
    let mut s = String::new();
    for x in t.iter().filter(|x| !x.is("{") && !x.is("}")) {
        match x {
            Tok::Str(v) => s.push_str(v),
            Tok::P(p) if p == "+" => {}
            Tok::P(p) if p == "," => break,
            o => s.push_str(&o.text()),
        }
    }
    s
}

/// 括弧・波括弧・ジェネリクスの外側にある `sep` で分割する。
pub fn split_top(t: &[Tok], sep: &str) -> Vec<Vec<Tok>> {
    let mut out = vec![vec![]];
    let mut d = 0i32;
    let mut gen = 0i32;
    for (k, x) in t.iter().enumerate() {
        if x.is("(") || x.is("{") || x.is("[") {
            d += 1;
        } else if x.is(")") || x.is("}") || x.is("]") {
            d -= 1;
        } else if x.is("<") && sep == "," && d == 0 && k > 0 && matches!(&t[k - 1], Tok::Id(s) if s.chars().next().is_some_and(|c| c.is_uppercase())) {
            gen += 1;
        } else if x.is(">") && gen > 0 {
            gen -= 1;
        } else if x.is(">>") && gen > 0 {
            gen = (gen - 2).max(0);
        }
        if d == 0 && gen == 0 && x.is(sep) {
            out.push(vec![]);
        } else {
            out.last_mut().unwrap().push(x.clone());
        }
    }
    out
}

pub fn parse_unit(file: &str, src: &str) -> Unit {
    let tokens = lex(src);
    let mut u = Unit { file: file.to_string(), tokens: tokens.clone(), ..Default::default() };
    let mut p = P { t: &tokens, i: 0 };
    let _ = p.annotations();
    if p.at_id("package") {
        p.i += 1;
        u.package = p.qualified();
        p.eat(";");
    }
    while p.at_id("import") {
        p.i += 1;
        if p.at_id("static") {
            p.i += 1;
        }
        let mut q = p.qualified();
        if p.at(".") && p.t.get(p.i + 1).is_some_and(|t| t.is("*")) {
            p.i += 2;
            q.push_str(".*");
        }
        p.eat(";");
        u.imports.push(q);
    }
    let mut classes = vec![];
    while p.peek().is_some() {
        let start = p.i;
        let (anns, _) = p.mods();
        if !p.type_decl(anns, &mut classes) {
            p.eat(";");
            if p.i == start {
                p.i += 1;
            }
        }
    }
    u.classes = classes;
    u
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_spring_controller() {
        let u = parse_unit(
            "A.java",
            r#"package a.b; import org.springframework.web.bind.annotation.*;
            @RestController @RequestMapping("/api")
            public class C { private final S s;
              @GetMapping("/u/{id}") public User get(@PathVariable("id") Long id, @RequestParam(defaultValue="x") String q) { return s.find(id); }
              @PostMapping(value="/u", produces={"application/json"}) public User add(@RequestBody User u) { return u; } }
            record User(Long id, String name) {}"#,
        );
        assert_eq!(u.package, "a.b");
        let c = u.classes.iter().find(|c| c.name == "C").unwrap();
        assert_eq!(c.ann("RequestMapping").unwrap().value().unwrap(), "/api");
        assert_eq!(c.methods.len(), 2);
        assert_eq!(c.methods[0].params[0].anns[0].name, "PathVariable");
        assert_eq!(c.methods[0].params[1].anns[0].get("defaultValue").unwrap(), "x");
        let r = u.classes.iter().find(|c| c.name == "User").unwrap();
        assert_eq!(r.fields.len(), 2);
    }
    #[test]
    fn generics_and_arrays() {
        let u = parse_unit("A.java", "class A { Map<String, List<Integer>> m = new HashMap<>(); int[] xs; public <T> T f(List<T> a, int... b) { return null; } }");
        let c = &u.classes[0];
        assert_eq!(c.fields[0].ty, "Map<String, List<Integer>>");
        assert_eq!(c.methods[0].params[1].ty, "int[]");
    }
}
