export PATH=$HOME/.cargo/bin:$PATH
cd /mnt/f/pl2pl
CARGO_TARGET_DIR=/tmp/pl_t cargo build --release -q 2>&1 | tail -3
B=/tmp/pl_t/release/pl2pl
rm -rf /tmp/o1 /tmp/o2; 
$B omp tests/samples/hpc --out /tmp/o1; cat /tmp/o1/PL2PL-REPORT.md; grep -n pragma /tmp/o1/vec.c
gcc -O2 tests/samples/hpc/vec.c -o /tmp/vec_s && /tmp/vec_s
gcc -O2 -fopenmp /tmp/o1/vec.c -o /tmp/vec_p && OMP_NUM_THREADS=4 /tmp/vec_p
$B cobol tests/samples/cobol --out /tmp/o2; cat /tmp/o2/PL2PL-REPORT.md; cat /tmp/o2/PAY.sh
which cobc || echo "no cobc"
