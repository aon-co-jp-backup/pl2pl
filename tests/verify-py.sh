#!/bin/bash
# 新版の確認: 指定パッケージ(最新版)を新しい venv に入れて通し試験を走らせる。使い方: verify-py.sh <pypi名>
set -e
export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"
PKG=${1:?package}
V=$(mktemp -d)/venv
if command -v uv >/dev/null 2>&1; then uv venv -q "$V" && uv pip install -q --python "$V/bin/python" fastapi httpx sqlalchemy jinja2 python-multipart && uv pip install -q --python "$V/bin/python" -U "$PKG"
else python3 -m venv "$V" && "$V/bin/pip" -q install fastapi httpx sqlalchemy jinja2 python-multipart && "$V/bin/pip" -q install -U "$PKG"; fi
echo "verify: $PKG $("$V/bin/python" -c "import importlib.metadata as m;print(m.version('$PKG'))")"
PL_VENV="$V" bash "$(dirname "$0")/run-php.sh"
