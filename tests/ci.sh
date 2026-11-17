#!/bin/bash
# CI 用の通し試験(Linux): cargo test + COBOL/JCL 実行 + OpenMP 同一結果 + cport + PHP→FastAPI
set -e
cd "$(dirname "$0")/.."
cargo test --locked -q 2>&1 | grep "test result"
cargo build --release --locked -q
B=$PWD/target/release/pl2pl
rm -rf /tmp/o1 /tmp/o3 /tmp/o4 /tmp/o5
$B omp tests/samples/hpc --out /tmp/o1 >/dev/null
gcc -O2 tests/samples/hpc/vec.c -o /tmp/vec_s; gcc -O2 -fopenmp /tmp/o1/vec.c -o /tmp/vec_p
[ "$(/tmp/vec_s)" = "$(OMP_NUM_THREADS=4 /tmp/vec_p)" ] && echo "omp: same result OK"
$B cobol tests/samples/cobol --out /tmp/o3 >/dev/null
( cd /tmp/o3 && bash build.sh && mkdir -p data && printf 'A\nB\nC\n' > data/PROD.PAY.DATA && bash PAY.sh | grep -q "RECORDS=0003" ) && echo "cobol+jcl: OK"
$B cport tests/samples/cport --out /tmp/o5 >/dev/null
gcc -Wall -o /tmp/legacy_o /tmp/o5/legacy.c -I/tmp/o5 && /tmp/legacy_o
B=$B bash tests/run-php.sh
