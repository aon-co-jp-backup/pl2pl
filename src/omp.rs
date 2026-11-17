//! 独立と判定できた最外側のループへ `#pragma omp parallel for` を付ける。
use crate::loops::{analyze, find_loops};

pub fn convert(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let mut out = String::new();
    let mut pos = 0;
    let mut covered = 0;
    let mut n = 0;
    for l in find_loops(src) {
        if l.start < covered {
            continue;
        }
        match analyze(&l) {
            Ok(p) => {
                out.push_str(&src[pos..l.start]);
                let red: String = p.reductions.iter().map(|(o, v)| format!(" reduction({o}:{v})")).collect();
                out.push_str(&format!("#pragma omp parallel for{red}\n{}", l.indent));
                pos = l.start;
                covered = l.end;
                n += 1;
                rep.push(format!("{name}:{} OpenMP を付与(ループ変数 {}{})", l.line, l.var, if red.is_empty() { String::new() } else { format!("、縮約{red}") }));
            }
            Err(why) => rep.push(format!("{name}:{} 見送り: {why}", l.line)),
        }
    }
    out.push_str(&src[pos..]);
    if n > 0 {
        rep.push(format!("{name}: ビルドに OpenMP が必要(gcc/clang: `-fopenmp`、MSVC: `/openmp`)。性能の向上は保証しない"));
    }
    vec![(String::new(), out)]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adds_pragma_and_skips_dependent() {
        let src = "void f(){\n    for (int i = 0; i < n; i++) {\n        c[i] = a[i] * 2;\n    }\n    for (int i = 1; i < n; i++) { a[i] = a[i-1]; }\n}\n";
        let mut r = vec![];
        let o = convert(src, "x.c", &mut r).remove(0).1;
        assert_eq!(o.matches("#pragma omp parallel for").count(), 1);
        assert!(o.contains("#pragma omp parallel for\n    for (int i = 0"));
        assert!(r.iter().any(|x| x.contains("見送り")));
    }
}
