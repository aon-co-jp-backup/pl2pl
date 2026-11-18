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
for n, a, t in [("bob", 30, None), ("amy", 20, "vip"), ("kid", 5, None), ("zed", 2, "vip")]:
    c.post("/users", json={"name": n, "age": a, **({"tag": t} if t else {})})
assert [u["name"] for u in c.get("/adults").json()] == ["amy", "bob"], c.get("/adults").text
assert c.get("/named").json()["age"] == 30
assert [u["name"] for u in c.get("/many").json()] == ["zed", "kid"], c.get("/many").text
assert c.get("/total").json()["n"] >= 4
assert sorted(u["name"] for u in c.get("/tagged").json()) == ["amy", "zed"]
h = c.get("/people").text
assert "<title>People</title>" in h and "<h1>All</h1>" in h and "amy (20)" in h and "vip" in h and "<small>many</small>" in h, h
assert "<h1>Compact</h1>" in c.get("/people-compact").text and "nobody" in c.get("/people-compact").text
assert "<h1>With</h1>" in c.get("/people-with").text and "<small>few</small>" in c.get("/people-with").text
ab = c.get("/about").text
assert "About us &lt;b&gt;" in ab and "user@example.com" in ab, ab
assert "About us" in c.get("/welcome").text
assert c.get("/secure").status_code == 401 and c.get("/secure").json() == {"error": "no token"}
assert c.get("/secure", headers={"X-Token": "bad"}).status_code == 403
assert c.get("/secure", headers={"X-Token": "secret"}).json() == {"secure": True}
import os
os.environ["API_TOKENS"] = "t1,t2"
assert c.get("/private/me").status_code == 401
ok = {"Authorization": "Bearer t1"}
assert c.get("/private/me", headers={"Authorization": "Bearer nope"}).status_code == 401
assert c.get("/private/me", headers=ok).json() == {"me": 1}
assert c.get("/private/me", headers=ok).json() == {"me": 1}
r3 = c.get("/private/me", headers=ok)
assert r3.status_code == 429 and "Retry-After" in r3.headers, r3.text
assert c.get("/after").json() == "x"
assert c.get("/guestonly").json() == "g"
assert c.get("/guestonly", headers=ok, follow_redirects=False).status_code == 302
paths = {r.path for r in app.routes}
assert "/users/{id}" in paths and "/api/items/{item}" in paths, paths
print("php e2e OK", len(paths))
PYEOF
