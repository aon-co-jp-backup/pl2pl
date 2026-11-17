#!/usr/bin/env bash
# 配布物を作る前に、aruaru-llm の公開リリースを components/bin/ へ取り込む(開発者の作業。利用者の PC では実行しない)。
# 使い方: bash installer/bundle-components.sh <linux|windows> [版]
# モデルの重みは含めない(利用者が `pl2pl llm setup` で PC に合うものを自動導入)。
set -e
os=${1:?linux か windows}; ver=${2:-$(gh release view --repo aon-co-jp/aruaru-llm --json tagName -q .tagName)}
mkdir -p components/bin; tmp=$(mktemp -d)
case "$os" in
  linux)   gh release download "$ver" --repo aon-co-jp/aruaru-llm --pattern 'aruaru-llm-linux-x86_64.tar.gz' --dir "$tmp"; tar xzf "$tmp"/aruaru-llm-linux-x86_64.tar.gz -C components/bin ;;
  windows) gh release download "$ver" --repo aon-co-jp/aruaru-llm --pattern 'aruaru-llm-windows-x86_64.zip' --dir "$tmp"; unzip -oq "$tmp"/aruaru-llm-windows-x86_64.zip -d components/bin ;;
  *) echo "不明な OS: $os"; exit 2 ;;
esac
( cd components/bin && sha256sum * > SHA256SUMS.txt )
echo "取り込み済み(aruaru-llm $ver): components/bin"
