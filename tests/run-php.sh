#!/bin/bash
# PHP → FastAPI の通し試験(fastapi/httpx が入った venv が必要)
set -e
[ -f "$HOME/2fa-venv/bin/activate" ] && . "$HOME/2fa-venv/bin/activate"
B=${B:-/tmp/pl_t/release/pl2pl}
rm -rf /tmp/o4; "$B" php "$(dirname "$0")/samples/php" --out /tmp/o4
cd /tmp/o4 && rm -f app.db && python - <<'PYEOF'
from fastapi.testclient import TestClient
from main import app
c = TestClient(app)
assert c.get("/").json() == "home"
assert c.get("/api/ping").json() == {"ok": True, "v": 1}
assert c.get("/health").json() == {"status": "ok"}
assert c.get("/echo/7").json() == {"id": 7}
r = c.post("/users", json={"name": "a", "age": 3, "note": "x"}).json()
assert r["id"] == 1 and r["name"] == "a", r
u = c.get("/users/1").json()
assert u["name"] == "a" and u["age"] == 3 and u["note"] == "x", u
assert c.delete("/users/1").json() == 1 and c.get("/users/1").json() is None
assert c.get("/users/99").json() is None
m = c.post("/make", json={"name": "m", "age": 1})
assert m.status_code == 201 and m.json()["name"] == "m", m.text
mid = m.json()["id"]
assert c.put(f"/users/{mid}", json={"age": 9, "tag": "t"}).json()["age"] == 9
assert c.post(f"/users/{mid}/rename").json()["name"] == "renamed"
assert c.get(f"/users/{mid}").json()["tag"] == "t"
assert c.delete(f"/users/{mid}/remove").status_code == 204
assert c.get(f"/users/{mid}").json() is None
assert c.put("/users/9999", json={}).status_code == 404
paths = {r.path for r in app.routes}
assert "/users/{id}" in paths and "/api/items/{item}" in paths, paths
print("php e2e OK", len(paths))
PYEOF
