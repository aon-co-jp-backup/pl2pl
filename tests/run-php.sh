#!/bin/bash
# PHP → FastAPI の通し試験(fastapi/httpx が入った venv が必要)
set -e
[ -f "$HOME/2fa-venv/bin/activate" ] && . "$HOME/2fa-venv/bin/activate"
B=${B:-/tmp/pl_t/release/pl2pl}
rm -rf /tmp/o4; "$B" php "$(dirname "$0")/samples/php" --out /tmp/o4
cd /tmp/o4 && python - <<'PYEOF'
from fastapi.testclient import TestClient
from main import app
c = TestClient(app)
assert c.get("/").json() == "home"
assert c.get("/api/ping").json() == {"ok": True, "v": 1}
paths = {r.path for r in app.routes}
assert "/users/{id}" in paths and "/api/items/{item}" in paths, paths
try:
    c.get("/users/5")
except NotImplementedError as e:
    print("TODO as expected:", e)
print("php e2e OK", len(paths))
PYEOF
