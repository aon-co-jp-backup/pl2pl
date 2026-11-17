//! Python の数値関数へ Numba の `@njit` を付ける(保守的)。`--mpi` で mpi4py の雛形を別ファイルで出す。
use regex::Regex;

pub const MPI_SCAFFOLD: &str = "\"\"\"mpi4py の雛形(pl2pl pyhpc --mpi が生成)。領域分割と集約の置き場所を示すだけで、元のコードは自動では並列化しない。
実行: mpiexec -n 4 python mpi_scaffold.py
\"\"\"
from mpi4py import MPI

comm = MPI.COMM_WORLD
rank = comm.Get_rank()
size = comm.Get_size()


def split_range(n: int, rank: int, size: int) -> range:
    \"\"\"0..n を size 個にほぼ均等に分け、rank 番目の範囲を返す。\"\"\"
    base, extra = divmod(n, size)
    start = rank * base + min(rank, extra)
    return range(start, start + base + (1 if rank < extra else 0))


if __name__ == \"__main__\":
    n = 1_000_000
    local = 0.0
    for i in split_range(n, rank, size):
        local += i * 0.5  # TODO(pl2pl): ここに独立な反復の本体を置く
    total = comm.allreduce(local, op=MPI.SUM)
    if rank == 0:
        print(total)
";

const REJECT: [&str; 20] = ["print(", "open(", "import ", "lambda", "yield", "try:", "with ", ".append(", "self", "{", "class ", "input(", "os.", "sys.", "random.", "dict(", "list(", "str(", "f\"", "f'"];

fn strip_docstrings(s: &str) -> String {
    Regex::new(r#"(?s)("""|''').*?("""|''')"#).unwrap().replace_all(s, "").to_string()
}

pub fn convert(src: &str, name: &str, rep: &mut Vec<String>, _mpi: bool) -> Vec<(String, String)> {
    let lines: Vec<&str> = src.lines().collect();
    let def = Regex::new(r"^def\s+(\w+)\s*\(").unwrap();
    let mut out: Vec<String> = vec![];
    let mut applied = 0;
    let mut i = 0;
    while i < lines.len() {
        let l = lines[i];
        if let Some(c) = def.captures(l) {
            // 本体 = 次の「字下げ 0 の空でない行」まで
            let mut j = i + 1;
            while j < lines.len() && (lines[j].trim().is_empty() || lines[j].starts_with(' ') || lines[j].starts_with('\t')) {
                j += 1;
            }
            let block = lines[i..j].join("\n");
            let has_deco = out.last().map(|p| p.trim_start().starts_with('@')).unwrap_or(false);
            let code = strip_docstrings(&block);
            let fname = &c[1];
            let verdict: Result<(), String> = if has_deco {
                Err("既にデコレーターがある".into())
            } else if !(code.contains("for ") && code.contains("range(")) {
                Err("`for ... in range(...)` の数値ループが無い(Numba の効果が薄い)".into())
            } else if let Some(r) = REJECT.iter().find(|r| code.contains(**r)) {
                Err(format!("Numba が苦手/非対応の要素 `{}` を含む", r.trim()))
            } else if code.contains('"') || code.contains('\'') {
                Err("文字列を使っている".into())
            } else {
                Ok(())
            };
            match verdict {
                Ok(()) => {
                    out.push("@njit(cache=True)".into());
                    applied += 1;
                    rep.push(format!("{name}:{} 関数 {fname} に @njit を付与", i + 1));
                }
                Err(why) => rep.push(format!("{name}:{} 関数 {fname} は見送り: {why}", i + 1)),
            }
        }
        out.push(l.to_string());
        i += 1;
    }
    let mut text = out.join("\n");
    if src.ends_with('\n') {
        text.push('\n');
    }
    if applied > 0 && !text.contains("from numba import") {
        // 最後のトップレベル import の後ろへ
        let ls: Vec<&str> = text.lines().collect();
        let pos = ls.iter().rposition(|l| l.starts_with("import ") || l.starts_with("from ")).map(|p| p + 1).unwrap_or(0);
        let mut v: Vec<String> = ls.iter().map(|s| s.to_string()).collect();
        v.insert(pos, "from numba import njit".into());
        text = v.join("\n") + "\n";
        rep.push(format!("{name}: numba が必要(`pip install numba`)。初回呼び出しはコンパイルで遅い。型が揺れる関数は失敗することがある。結果は元のコードと突き合わせる"));
    }
    vec![(String::new(), text)]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decorates_numeric_only() {
        let src = "import math\n\ndef dot(a, b, n):\n    s = 0.0\n    for i in range(n):\n        s += a[i] * b[i]\n    return s\n\ndef show(x):\n    print(x)\n    for i in range(3):\n        pass\n";
        let mut r = vec![];
        let o = convert(src, "m.py", &mut r, false).remove(0).1;
        assert!(o.contains("@njit(cache=True)\ndef dot"));
        assert!(!o.contains("@njit(cache=True)\ndef show"));
        assert!(o.contains("from numba import njit"));
        assert!(r.iter().any(|x| x.contains("show は見送り")));
    }
}
