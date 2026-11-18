#!/bin/bash
# Kokkos 変換結果の実ビルドと同一結果の確認(Kokkos は /tmp に取得してビルド、Serial バックエンド)
set -e
export PATH=$HOME/.cargo/bin:$PATH
cd "$(dirname "$0")/.."
ROOT=$(pwd)
if [ ! -d /tmp/kokkos-inst ]; then
  [ -d /tmp/kokkos-src ] || git clone -q --depth 1 --branch 4.5.01 https://github.com/kokkos/kokkos.git /tmp/kokkos-src
  cmake -S /tmp/kokkos-src -B /tmp/kokkos-bld -DCMAKE_BUILD_TYPE=Release -DKokkos_ENABLE_SERIAL=ON -DCMAKE_CXX_STANDARD=17 -DCMAKE_INSTALL_PREFIX=/tmp/kokkos-inst >/dev/null
  cmake --build /tmp/kokkos-bld -j"$(nproc)" >/dev/null && cmake --install /tmp/kokkos-bld >/dev/null
fi
CARGO_TARGET_DIR=/tmp/pl_t cargo build --release -q; B=/tmp/pl_t/release/pl2pl
rm -rf /tmp/o7; $B kokkos tests/samples/hpc/vec.cpp --out /tmp/o7 >/dev/null
g++ -O2 -std=c++17 tests/samples/hpc/vec.cpp -o /tmp/vec_orig
OUT=$(ls /tmp/o7/*.cpp | head -1)
sed -i "s/int main(void) {/int main(void) { Kokkos::ScopeGuard guard;/" "$OUT"  # 変換器は挿入しない(レポートで指摘)
g++ -O2 -std=c++17 -I/tmp/kokkos-inst/include "$OUT" -L/tmp/kokkos-inst/lib -L/tmp/kokkos-inst/lib64 -lkokkoscore -lkokkossimd -ldl -o /tmp/vec_kk 2>&1 | tail -20
a=$(/tmp/vec_orig); b=$(/tmp/vec_kk)
echo "orig=$a kokkos=$b"; [ "$a" = "$b" ] && echo "kokkos: same result OK"
