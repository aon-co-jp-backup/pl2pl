//! 試作: 独立と判定できたループから GPU カーネルを書き出す。
//!   open-cuda    → CUDA 形式の `.cu`(`__global__`)
//!   open-directx → HLSL の `.hlsl`(コンピュートシェーダー。open-directx の directx-shader-translate への入力想定)
//! 縮約(和など)は未対応。要素型は float と仮定する。open-cuda / open-directx 上での実行は未検証。
use crate::loops::{analyze, find_loops, Loop};
use regex::Regex;

const MATH: [&str; 24] = ["sin", "cos", "tan", "sqrt", "exp", "log", "log10", "pow", "fabs", "abs", "fmax", "fmin", "floor", "ceil", "fmod", "atan", "atan2", "sinh", "cosh", "tanh", "sizeof", "static_cast", "if", "else"];
const TYPES: [&str; 11] = ["int", "long", "double", "float", "size_t", "unsigned", "char", "bool", "auto", "short", "const"];

fn parts(l: &Loop) -> (Vec<String>, Vec<String>, Vec<String>) {
    // (配列(読み専用), 配列(書き込み), スカラー引数)
    let arr = Regex::new(r"\b([A-Za-z_]\w*)\s*\[").unwrap();
    let write = Regex::new(r"\b([A-Za-z_]\w*)\s*\[[^\]]*\]\s*(?:\+=|-=|\*=|/=|=)[^=]").unwrap();
    let mut written: Vec<String> = write.captures_iter(&l.body).map(|c| c[1].to_string()).collect();
    written.sort();
    written.dedup();
    let mut all: Vec<String> = arr.captures_iter(&l.body).map(|c| c[1].to_string()).collect();
    all.sort();
    all.dedup();
    let ro: Vec<String> = all.iter().filter(|a| !written.contains(a)).cloned().collect();
    let decl = Regex::new(r"\b(?:int|long|double|float|size_t|unsigned|char|bool|auto|short)\s+([A-Za-z_]\w*)").unwrap();
    let locals: Vec<String> = decl.captures_iter(&l.body).map(|c| c[1].to_string()).collect();
    let id = Regex::new(r"\b([A-Za-z_]\w*)\b").unwrap();
    let nolit = Regex::new(r"\b\d[\w.]*\b").unwrap().replace_all(&l.body, " ").to_string();
    let mut sc: Vec<String> = vec![];
    for c in id.captures_iter(&nolit) {
        let n = c[1].to_string();
        if n != l.var && !all.contains(&n) && !locals.contains(&n) && !MATH.contains(&n.as_str()) && !TYPES.contains(&n.as_str()) && !sc.contains(&n) {
            sc.push(n);
        }
    }
    (ro, written, sc)
}

fn cuda(l: &Loop, name: &str) -> String {
    let (ro, wr, sc) = parts(l);
    let mut params: Vec<String> = ro.iter().map(|a| format!("const float* __restrict__ {a}")).collect();
    params.extend(wr.iter().map(|a| format!("float* __restrict__ {a}")));
    params.extend(sc.iter().map(|s| format!("float {s}")));
    params.push("int pl2pl_n".into());
    params.push("int pl2pl_lo".into());
    format!(
        "// pl2pl gpu(試作): {name} の {} 行目のループ。要素型は float と仮定(要確認)。open-cuda での実行は未検証\n__global__ void pl2pl_kernel_{}({}) {{\n    int {v} = pl2pl_lo + blockIdx.x * blockDim.x + threadIdx.x;\n    if ({v} >= pl2pl_lo + pl2pl_n) return;\n{}\n}}\n// 起動: pl2pl_kernel_{}<<<(n + 255) / 256, 256>>>(..., n, lo);  n = ({}) - ({}){}\n",
        l.line,
        l.line,
        params.join(", "),
        l.body.trim_matches('\n'),
        l.line,
        l.bound,
        l.init,
        if l.cmp == "<=" { " + 1" } else { "" },
        v = l.var,
    )
}

fn hlsl(l: &Loop, name: &str) -> String {
    let (ro, wr, sc) = parts(l);
    let mut s = format!("// pl2pl gpu(試作): {name} の {} 行目のループ。要素型は float と仮定(要確認)。open-directx での実行は未検証\n", l.line);
    let mut reg = 0;
    for a in &ro {
        s.push_str(&format!("StructuredBuffer<float> {a} : register(t{reg});\n"));
        reg += 1;
    }
    let mut ureg = 0;
    for a in &wr {
        s.push_str(&format!("RWStructuredBuffer<float> {a} : register(u{ureg});\n"));
        ureg += 1;
    }
    s.push_str("cbuffer Params : register(b0) {\n");
    for v in &sc {
        s.push_str(&format!("    float {v};\n"));
    }
    s.push_str("    int pl2pl_n;\n    int pl2pl_lo;\n};\n");
    let body = l.body.trim_matches('\n').replace("fabs(", "abs(").replace("fmax(", "max(").replace("fmin(", "min(").replace("fmod(", "fmod(");
    s.push_str(&format!("\n[numthreads(256, 1, 1)]\nvoid main(uint3 pl2pl_id : SV_DispatchThreadID) {{\n    int {v} = pl2pl_lo + (int)pl2pl_id.x;\n    if ({v} >= pl2pl_lo + pl2pl_n) return;\n{body}\n}}\n", v = l.var));
    s
}

pub fn convert(src: &str, name: &str, rep: &mut Vec<String>, target: &str) -> Vec<(String, String)> {
    let stem = name.rsplit('/').next().unwrap().rsplit_once('.').map(|x| x.0).unwrap_or(name).to_string();
    let mut out: Vec<(String, String)> = vec![];
    let mut covered = 0;
    for l in find_loops(src) {
        if l.start < covered {
            continue;
        }
        match analyze(&l) {
            Ok(p) if p.reductions.is_empty() => {
                covered = l.end;
                let (ext, text) = if target == "open-directx" { ("hlsl", hlsl(&l, name)) } else { ("cu", cuda(&l, name)) };
                out.push((format!("{stem}_loop{}.{ext}", l.line), text));
                rep.push(format!("{name}:{} {target} 用カーネルを出力(試作・未検証)", l.line));
            }
            Ok(_) => rep.push(format!("{name}:{} 見送り: 縮約(和など)は GPU カーネル出力が未対応", l.line)),
            Err(why) => rep.push(format!("{name}:{} 見送り: {why}", l.line)),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    const SRC: &str = "void f(){\n    for (int i = 0; i < n; i++) {\n        c[i] = a[i] * s + b[i];\n    }\n}\n";
    #[test]
    fn cuda_kernel() {
        let mut r = vec![];
        let o = convert(SRC, "v.c", &mut r, "open-cuda");
        assert_eq!(o[0].0, "v_loop2.cu");
        assert!(o[0].1.contains("const float* __restrict__ a, const float* __restrict__ b, float* __restrict__ c, float s"));
        assert!(o[0].1.contains("__global__"));
    }
    #[test]
    fn hlsl_shader() {
        let mut r = vec![];
        let o = convert(SRC, "v.c", &mut r, "open-directx");
        assert!(o[0].1.contains("StructuredBuffer<float> a : register(t0);"));
        assert!(o[0].1.contains("RWStructuredBuffer<float> c : register(u0);"));
        assert!(o[0].1.contains("float s;"));
        assert!(o[0].1.contains("[numthreads(256, 1, 1)]"));
    }
}
