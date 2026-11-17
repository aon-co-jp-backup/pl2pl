#!/bin/bash
# Numba で実際に同じ結果になるか確認(uv で専用 venv を作る)
set -e
export PATH=$HOME/.local/bin:$PATH
[ -d /tmp/nbvenv ] || (uv venv -q /tmp/nbvenv && uv pip install -q --python /tmp/nbvenv/bin/python numba numpy)
cd "$(dirname "$0")/.."
export PATH=$HOME/.cargo/bin:$PATH; CARGO_TARGET_DIR=/tmp/pl_t cargo build --release -q; B=/tmp/pl_t/release/pl2pl
rm -rf /tmp/o6; $B pyhpc tests/samples/py --out /tmp/o6 --mpi >/dev/null
a=$(/tmp/nbvenv/bin/python tests/samples/py/num.py); b=$(/tmp/nbvenv/bin/python /tmp/o6/num.py 2>/dev/null)
echo "orig=$a numba=$b"; [ "$a" = "$b" ] && echo "numba: same result OK"
