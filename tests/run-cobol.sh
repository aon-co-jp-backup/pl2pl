#!/bin/bash
# COBOL+JCL → GnuCOBOL の通し試験(cobc が必要)
set -e
B=/tmp/pl_t/release/pl2pl
rm -rf /tmp/o3; $B cobol "$(dirname "$0")/samples/cobol" --out /tmp/o3 >/dev/null
cd /tmp/o3 && bash build.sh && mkdir -p data && printf 'A\nB\nC\n' > data/PROD.PAY.DATA && bash PAY.sh
