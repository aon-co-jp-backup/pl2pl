#!/bin/bash
# Convert the samples to rust-poem and cargo check them (needs network for crates).
set -e
export PATH=$HOME/.cargo/bin:$PATH
cd "$(dirname "$0")/.."
CARGO_TARGET_DIR=/tmp/jt_target cargo build --release -q
B=/tmp/jt_target/release/java_to
rm -rf /tmp/jt_rs; mkdir -p /tmp/jt_rs
for s in spring jaxrs servlet spark; do
  $B convert tests/samples/$s --target rust-poem --out /tmp/jt_rs/$s --name jt-$s >/dev/null
  (cd /tmp/jt_rs/$s && CARGO_TARGET_DIR=/tmp/jt_rs_target cargo check -q 2>&1 | tail -15) && echo "== $s checked"
done
