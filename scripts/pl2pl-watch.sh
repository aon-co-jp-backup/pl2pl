#!/bin/bash
# Linux/macOS: cron 例  0 3,7 * * *  /path/to/pl2pl/scripts/pl2pl-watch.sh
cd "$(dirname "$0")/.." || exit 1
VC=${VC:-$HOME/version-control/target/release/version-control}
[ -x "$VC" ] && A="--vc $VC" || A=""
echo "===== $(date)" >> watch.log
./target/release/pl2pl watch $A >> watch.log 2>&1
echo "exit=$?" >> watch.log
