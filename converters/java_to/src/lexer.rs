//! Java の字句解析(コメントと空白を捨てる。テキストブロック・文字列・文字・数値・記号)。

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Id(String),
    Num(String),
    Str(String),
    Chr(String),
    P(String),
}

impl Tok {
    pub fn text(&self) -> String {
        match self {
            Tok::Id(s) | Tok::Num(s) | Tok::P(s) => s.clone(),
            Tok::Str(s) => format!("{:?}", s),
            Tok::Chr(s) => format!("'{}'", s),
        }
    }
    pub fn is(&self, p: &str) -> bool {
        matches!(self, Tok::P(s) if s == p)
    }
    pub fn id(&self, p: &str) -> bool {
        matches!(self, Tok::Id(s) if s == p)
    }
}

const PUNCT3: [&str; 4] = [">>>", "<<=", ">>=", "..."];
const PUNCT2: [&str; 22] = [
    "->", "::", "==", "!=", "<=", ">=", "&&", "||", "++", "--", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<", ">>", "?.", "@@",
];

pub fn lex(src: &str) -> Vec<Tok> {
    let c: Vec<char> = src.chars().collect();
    let mut i = 0;
    let mut out = vec![];
    while i < c.len() {
        let ch = c[i];
        if ch.is_whitespace() || ch == '\u{feff}' {
            i += 1;
        } else if ch == '/' && c.get(i + 1) == Some(&'/') {
            while i < c.len() && c[i] != '\n' {
                i += 1;
            }
        } else if ch == '/' && c.get(i + 1) == Some(&'*') {
            i += 2;
            while i + 1 < c.len() && !(c[i] == '*' && c[i + 1] == '/') {
                i += 1;
            }
            i += 2;
        } else if ch == '"' && c.get(i + 1) == Some(&'"') && c.get(i + 2) == Some(&'"') {
            // テキストブロック
            i += 3;
            let mut s = String::new();
            while i + 2 < c.len() && !(c[i] == '"' && c[i + 1] == '"' && c[i + 2] == '"') {
                s.push(c[i]);
                i += 1;
            }
            i += 3;
            let s = s.trim_start_matches([' ', '\t']).trim_start_matches('\n').to_string();
            out.push(Tok::Str(s.lines().map(|l| l.trim_start()).collect::<Vec<_>>().join("\n")));
        } else if ch == '"' {
            i += 1;
            let mut s = String::new();
            while i < c.len() && c[i] != '"' {
                if c[i] == '\\' && i + 1 < c.len() {
                    i += 1;
                    s.push(match c[i] {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '0' => '\0',
                        o => o,
                    });
                } else {
                    s.push(c[i]);
                }
                i += 1;
            }
            i += 1;
            out.push(Tok::Str(s));
        } else if ch == '\'' {
            i += 1;
            let mut s = String::new();
            while i < c.len() && c[i] != '\'' {
                if c[i] == '\\' && i + 1 < c.len() {
                    s.push(c[i]);
                    i += 1;
                }
                s.push(c[i]);
                i += 1;
            }
            i += 1;
            out.push(Tok::Chr(s));
        } else if ch.is_ascii_digit() || (ch == '.' && c.get(i + 1).is_some_and(|d| d.is_ascii_digit())) {
            let st = i;
            while i < c.len() && (c[i].is_alphanumeric() || c[i] == '.' || c[i] == '_') {
                i += 1;
            }
            out.push(Tok::Num(c[st..i].iter().collect::<String>().replace('_', "")));
        } else if ch.is_alphabetic() || ch == '_' || ch == '$' {
            let st = i;
            while i < c.len() && (c[i].is_alphanumeric() || c[i] == '_' || c[i] == '$') {
                i += 1;
            }
            out.push(Tok::Id(c[st..i].iter().collect()));
        } else {
            let rest: String = c[i..(i + 3).min(c.len())].iter().collect();
            if let Some(p) = PUNCT3.iter().find(|p| rest.starts_with(**p)) {
                out.push(Tok::P(p.to_string()));
                i += 3;
            } else if let Some(p) = PUNCT2.iter().find(|p| rest.starts_with(**p)) {
                out.push(Tok::P(p.to_string()));
                i += 2;
            } else {
                out.push(Tok::P(ch.to_string()));
                i += 1;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn skips_comments_and_reads_strings() {
        let t = lex("// x\n/* y */ int a = 1_000; String s = \"a\\n\"; @Get(\"/x\")");
        assert!(t.contains(&Tok::Num("1000".into())));
        assert!(t.contains(&Tok::Str("a\n".into())));
        assert!(t.iter().any(|x| x.is("@")));
    }
}
