#!/bin/bash
# rpoem target: cargo check against a local RPoem checkout (default /mnt/f/RPoem).
export PATH=$HOME/.cargo/bin:$PATH
RP=${RPOEM:-/mnt/f/RPoem}
B=/tmp/jt_target/release/java_to
rm -rf /tmp/jt_rp; mkdir -p /tmp/jt_rp
for s in spring jaxrs; do
  $B convert $(dirname "$0")/samples/$s --target rpoem --rpoem-path $RP --out /tmp/jt_rp/$s --name jt-$s >/dev/null
  (cd /tmp/jt_rp/$s && CARGO_TARGET_DIR=/tmp/jt_rp_target cargo check -q 2>&1 | grep -E "^error" -A6 | head -20)
  echo "== $s done"
done
