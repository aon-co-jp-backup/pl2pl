#!/usr/bin/env bash
# 通し試験: サンプルの Java を各ターゲットへ変換し、Python はテストクライアントで実行、Rust は cargo check で確認する(WSL)。
# 使い方(PowerShell): wsl -d Ubuntu -- bash /mnt/f/Java_to/tests/run-e2e.sh
set -e
export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"
HERE="$(cd "$(dirname "$0")" && pwd)"; ROOT="$HERE/.."
OUT="${E2E_OUT:-/tmp/java_to_e2e}"; rm -rf "$OUT"; mkdir -p "$OUT"
( cd "$ROOT" && cargo build --release --target-dir ${CARGO_T:-/tmp/java_to_target} 2>&1 | tail -1 )
BIN=${CARGO_T:-/tmp/java_to_target}/release/java_to
for s in spring jaxrs servlet spark; do "$BIN" convert "$HERE/samples/$s" --target fastapi --out "$OUT/$s" > /dev/null; done
[ -f "$HOME/2fa-venv/bin/activate" ] && . "$HOME/2fa-venv/bin/activate"
E2E_DIR="$OUT" python -m pytest -q "$HERE/e2e/test_converted.py"
