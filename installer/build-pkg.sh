#!/usr/bin/env bash
# macOS .pkg を作る(macOS上で実行)。使い方: build-pkg.sh <pl2pl バイナリ> <版> <arm64|x64> <出力ディレクトリ>
# 署名・公証は Apple のシークレットが必要なため、ここでは未署名。署名する場合は pkgbuild に --sign を付ける。
set -euo pipefail
bin="$1"; ver="$2"; arch="$3"; out="$4"
root="$(cd "$(dirname "$0")/.." && pwd)"
stage="$(mktemp -d)"
mkdir -p "$stage/usr/local/bin" "$stage/usr/local/share/doc/pl2pl" "$out"
install -m 0755 "$bin" "$stage/usr/local/bin/pl2pl"
# 取り込んだ変換器(あれば)。pl2pl web は ../lib/pl2pl を探す。環境変数 CONVERTERS_DIR=components/bin
if [ -n "${CONVERTERS_DIR:-}" ] && ls "$CONVERTERS_DIR"/* >/dev/null 2>&1; then
  mkdir -p "$stage/usr/local/lib/pl2pl"; install -m 0755 "$CONVERTERS_DIR"/* "$stage/usr/local/lib/pl2pl/"
fi
install -m 0644 "$root/README.md" "$root/LICENSE" "$stage/usr/local/share/doc/pl2pl/"
pkgbuild --root "$stage" --identifier jp.co.aon.pl2pl --version "$ver" --install-location / "$out/pl2pl-${ver}-macos-${arch}.pkg"
echo "$out/pl2pl-${ver}-macos-${arch}.pkg"
