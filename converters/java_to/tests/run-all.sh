#!/bin/bash
# unit tests + all four targets (WSL/Linux). e2e needs a python venv with fastapi/httpx/pytest.
export PATH=$HOME/.cargo/bin:$PATH
cd "$(dirname "$0")/.."
CARGO_TARGET_DIR=/tmp/jt_target cargo test -q 2>&1 | tail -4
bash tests/run-rust.sh 2>&1 | grep -E "^error|== " 
bash tests/run-rpoem.sh 2>&1 | tail -15
