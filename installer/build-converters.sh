#!/usr/bin/env bash
# 取り込んだ変換器(converters/cobol_to・converters/java_to)をビルドし、components/bin/ へ置く。
# `pl2pl web` は、実行ファイルの隣・components/bin/・../lib/pl2pl のものを先に使う(無ければ PATH)。
# 使い方: bash installer/build-converters.sh [ターゲット三つ組]   (配布物を作る前に実行。bundle-components.sh と同じ置き場所)
set -e
cd "$(dirname "$0")/.."
tgt="${1:-}"
mkdir -p components/bin
for c in cobol_to java_to; do
  if [ -n "$tgt" ]; then
    cargo build --release --manifest-path "converters/$c/Cargo.toml" --target-dir "converters/$c/target" --target "$tgt"
    rel="converters/$c/target/$tgt/release"
  else
    cargo build --release --manifest-path "converters/$c/Cargo.toml" --target-dir "converters/$c/target"
    rel="converters/$c/target/release"
  fi
  for f in "$rel/$c" "$rel/$c.exe"; do
    [ -f "$f" ] && cp "$f" components/bin/
  done
done
ls -l components/bin
