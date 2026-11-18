#!/usr/bin/env bash
# macOS .pkg を作る(macOS上で実行)。使い方: build-pkg.sh <cobol_to バイナリ> <版> <arm64|x64> <出力ディレクトリ>
# 署名・公証は Apple のシークレットが必要なため、ここでは未署名。署名する場合は pkgbuild に --sign を付ける。
set -euo pipefail
bin="$1"; ver="$2"; arch="$3"; out="$4"
root="$(cd "$(dirname "$0")/.." && pwd)"
stage="$(mktemp -d)"
mkdir -p "$stage/usr/local/bin" "$stage/usr/local/share/doc/cobol_to" "$out"
install -m 0755 "$bin" "$stage/usr/local/bin/cobol_to"
install -m 0644 "$root/README.md" "$root/NOTIFY.md" "$root/LICENSE" "$stage/usr/local/share/doc/cobol_to/"
pkgbuild --root "$stage" --identifier jp.co.aon.cobol-to --version "$ver" --install-location / "$out/COBOL_to-${ver}-macos-${arch}.pkg"
echo "$out/COBOL_to-${ver}-macos-${arch}.pkg"
