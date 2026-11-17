#!/bin/bash
# 全部の通し試験(WSL/Linux): 単体 + OpenMP + COBOL + PHP
export PATH=$HOME/.cargo/bin:$PATH
cd "$(dirname "$0")/.."
CARGO_TARGET_DIR=/tmp/pl_t cargo test -q 2>&1 | grep "test result"
CARGO_TARGET_DIR=/tmp/pl_t cargo build --release -q 2>&1 | grep -E "^error" -A5
B=/tmp/pl_t/release/pl2pl
echo "== omp"; rm -rf /tmp/o1; $B omp tests/samples/hpc --out /tmp/o1 >/dev/null
gcc -O2 tests/samples/hpc/vec.c -o /tmp/vec_s && gcc -O2 -fopenmp /tmp/o1/vec.c -o /tmp/vec_p
[ "$(/tmp/vec_s)" = "$(OMP_NUM_THREADS=4 /tmp/vec_p)" ] && echo "omp: same result OK"
echo "== cobol"; bash tests/run-cobol.sh
echo "== php"; bash tests/run-php.sh
echo "== cport"; rm -rf /tmp/o5; $B cport tests/samples/cport --out /tmp/o5 >/dev/null
gcc -Wall -o /tmp/legacy_o /tmp/o5/legacy.c -I/tmp/o5 && /tmp/legacy_o
echo "== pyhpc"; rm -rf /tmp/o6; $B pyhpc tests/samples/py --out /tmp/o6 --mpi >/dev/null
[ -f "$HOME/2fa-venv/bin/activate" ] && . "$HOME/2fa-venv/bin/activate"
python -c "import numba" 2>/dev/null || python -m pip install -q numba 2>&1 | tail -1
[ "$(python tests/samples/py/num.py)" = "$(python /tmp/o6/num.py)" ] && echo "numba: same result OK ($(python /tmp/o6/num.py))"
echo "== kokkos (output only; Kokkos not installed)"; rm -rf /tmp/o7; $B kokkos tests/samples/hpc --out /tmp/o7 >/dev/null; grep -c "Kokkos::parallel" /tmp/o7/vec.cpp
echo "== gpu"; rm -rf /tmp/o8; $B gpu tests/samples/hpc --out /tmp/o8 --target open-directx >/dev/null; ls /tmp/o8 | grep -c hlsl
echo "== doctor"; $B doctor | head -3
