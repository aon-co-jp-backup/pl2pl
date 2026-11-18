#!/usr/bin/env bash
# 取り込んだ変換器(converters/cobol_to・converters/java_to)をビルドし、components/bin/ へ置く。
# `pl2pl web` は、実行ファイルの隣か components/bin/ にあるものを先に使う(無ければ PATH)。
# 使い方: bash installer/build-converters.sh   (配布物を作る前の開発者の作業。bundle-components.sh と同じ置き場所)
set -e
cd "$(dirname "$0")/.."
mkdir -p components/bin
for c in cobol_to java_to; do
  cargo build --release --manifest-path "converters/$c/Cargo.toml" --target-dir "converters/$c/target"
  for f in "converters/$c/target/release/$c" "converters/$c/target/release/$c.exe"; do
    [ -f "$f" ] && cp "$f" components/bin/
  done
done
ls -l components/bin
