//! C++ の独立ループを Kokkos の parallel_for / parallel_reduce(和のみ)へ。OpenMP と同じ独立性の判定を使う。
use crate::loops::{analyze, find_loops};
use regex::Regex;

pub fn convert(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let mut out = String::new();
    let mut pos = 0;
    let mut covered = 0;
    let mut n = 0;
    for l in find_loops(src) {
        if l.start < covered {
            continue;
        }
        let plan = match analyze(&l) {
            Ok(p) => p,
            Err(why) => {
                rep.push(format!("{name}:{} 見送り: {why}", l.line));
                continue;
            }
        };
        let count = if l.cmp == "<" { format!("({}) - ({})", l.bound, l.init) } else { format!("({}) - ({}) + 1", l.bound, l.init) };
        let ind = &l.indent;
        let label = format!("pl2pl_loop_{}", l.line);
        let var = &l.var;
        let init = &l.init;
        let text = match plan.reductions.as_slice() {
            [] => format!("Kokkos::parallel_for(\"{label}\", {count}, KOKKOS_LAMBDA(const int k_) {{\n{ind}    const int {var} = ({init}) + k_;\n{}\n{ind}}});", l.body.trim_matches('\n')),
            [(op, v)] if op == "+" || op == "-" => {
                let body = Regex::new(&format!(r"\b{}\b", regex::escape(v))).unwrap().replace_all(&l.body, format!("{v}_lsum_").as_str()).to_string();
                format!(
                    "{{\n{ind}    using pl2pl_T_ = std::remove_cv_t<std::remove_reference_t<decltype({v})>>;\n{ind}    pl2pl_T_ {v}_part_ = 0;\n{ind}    Kokkos::parallel_reduce(\"{label}\", {count}, KOKKOS_LAMBDA(const int k_, pl2pl_T_& {v}_lsum_) {{\n{ind}        const int {var} = ({init}) + k_;\n{}\n{ind}    }}, {v}_part_);\n{ind}    {v} += {v}_part_;\n{ind}}}",
                    body.trim_matches('\n')
                )
            }
            other => {
                rep.push(format!("{name}:{} 見送り: Kokkos への縮約は単一の和(+=, -=)のみ対応({} 個)", l.line, other.len()));
                continue;
            }
        };
        out.push_str(&src[pos..l.start]);
        out.push_str(&text);
        pos = l.end;
        covered = l.end;
        n += 1;
        rep.push(format!("{name}:{} Kokkos へ変換(ループ変数 {var})", l.line));
    }
    out.push_str(&src[pos..]);
    if n > 0 {
        if let Some(m) = Regex::new(r"(?m)^#include[^\n]*\n").unwrap().find_iter(&out).last() {
            let at = m.end();
            out.insert_str(at, "#include <Kokkos_Core.hpp>\n#include <type_traits>\n");
        } else {
            out.insert_str(0, "#include <Kokkos_Core.hpp>\n#include <type_traits>\n");
        }
        if out.contains("int main") && !out.contains("Kokkos::initialize") && !out.contains("ScopeGuard") {
            rep.push(format!("{name}: main の先頭に `Kokkos::ScopeGuard guard(argc, argv);` を足す必要がある(未挿入)"));
        }
        rep.push(format!("{name}: ラムダは値捕獲。素の配列・ポインタはホスト実行(Serial/OpenMP)でのみ動く。GPU 用は Kokkos::View へ置き換える(未変換)"));
        rep.push(format!("{name}: ビルドは CMake の find_package(Kokkos) と target_link_libraries(... Kokkos::kokkos)。性能は保証しない"));
    }
    vec![(String::new(), out)]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn for_and_reduce() {
        let src = "#include <vector>\nvoid f(double* a, double* b, double* c, int n) {\n    double sum = 0;\n    for (int i = 0; i < n; i++) {\n        c[i] = a[i] + b[i];\n    }\n    for (int i = 0; i < n; i++) {\n        sum += c[i];\n    }\n}\n";
        let mut r = vec![];
        let o = convert(src, "x.cpp", &mut r).remove(0).1;
        assert!(o.contains("Kokkos::parallel_for(\"pl2pl_loop_4\""));
        assert!(o.contains("Kokkos::parallel_reduce(\"pl2pl_loop_7\""));
        assert!(o.contains("sum_lsum_ += c[i]"));
        assert!(o.contains("#include <Kokkos_Core.hpp>"));
    }
}
