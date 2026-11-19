#!/bin/bash
# PHP → FastAPI の通し試験(fastapi/httpx が入った venv が必要)
set -e
if [ -n "$PL_VENV" ]; then . "$PL_VENV/bin/activate"; elif [ -f "$HOME/2fa-venv/bin/activate" ]; then . "$HOME/2fa-venv/bin/activate"; fi
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
ra = c.get("/after")
assert ra.json() == "x" and ra.headers.get("X-Local") == "1", ra.headers
assert c.get("/guestonly").json() == "g"
assert c.get("/guestonly", headers=ok, follow_redirects=False).status_code == 302
uid = c.post("/users", json={"name": "rel", "age": 40}).json()["id"]
p1 = c.post(f"/users/{uid}/posts", json={"title": "t1", "published": True})
assert p1.status_code == 201 and p1.json()["user_id"] == uid, p1.text
c.post(f"/users/{uid}/posts", json={"title": "t2", "published": False})
assert [p["title"] for p in c.get(f"/users/{uid}/posts").json()] == ["t1", "t2"]
assert c.get(f"/posts/{p1.json()['id']}/author").json()["name"] == "rel"
wp = [x for x in c.get("/with-posts").json() if x["id"] == uid][0]
assert len(wp["posts"]) == 2, wp
assert uid in [x["id"] for x in c.get("/published-authors").json()]
role = c.post("/roles", json={"name": "admin"}).json()
rr = c.post(f"/users/{uid}/roles", json={"role_id": role["id"]}).json()
assert [x["name"] for x in rr] == ["admin"], rr
rr = c.post(f"/users/{uid}/roles", json={"role_id": role["id"]}).json()
assert len(rr) == 1, rr
lb = c.post("/labels", json={"name": "hot"}).json()
pid = p1.json()["id"]
pl = c.post(f"/posts/{pid}/labels", json={"label_id": lb["id"]}).json()
assert [x["name"] for x in pl] == ["hot"], pl
assert len(c.post(f"/posts/{pid}/labels", json={"label_id": lb["id"]}).json()) == 1
assert c.post(f"/users/{uid}/labels", json={"label_id": lb["id"]}).json()[0]["name"] == "hot"
assert len(c.post(f"/users/{uid}/labels", json={"label_id": lb["id"]}).json()) == 1
lb2 = c.post("/labels", json={"name": "cold"}).json()
assert len(c.post(f"/posts/{pid}/labels", json={"label_id": lb2["id"]}).json()) == 2
assert [x["name"] for x in c.post(f"/users/{uid}/labels", json={"label_id": lb["id"]}).json()] == ["hot"]  # 型で分かれている(post の cold は user に出ない)
assert uid not in [x["id"] for x in c.get("/unpublished").json()]
lone = c.post("/users", json={"name": "lone", "age": 1}).json()["id"]
assert lone in [x["id"] for x in c.get("/unpublished").json()]
assert lone in [x["id"] for x in c.get("/no-posts").json()] and uid not in [x["id"] for x in c.get("/no-posts").json()]
cm = c.post(f"/posts/{p1.json()['id']}/comments", json={"body": "hi"})
assert cm.status_code == 200 and cm.json()["post_id"] == p1.json()["id"], cm.text
dp = [x for x in c.get("/deep").json() if x["id"] == uid][0]
assert [cc["body"] for cc in [q for q in dp["posts"] if q["id"] == p1.json()["id"]][0]["comments"]] == ["hi"], dp
assert [q for q in dp["posts"] if q["title"] == "t2"][0]["comments"] == [], dp
cu1 = c.get("/cursored").json()
assert len(cu1["data"]) == 2 and cu1["prev_cursor"] is None and cu1["next_cursor"], cu1
cu2 = c.get(cu1["next_page_url"]).json()
assert cu2["data"][0]["id"] > cu1["data"][-1]["id"] and cu2["prev_cursor"], cu2
seen, cur = [], c.get("/cursored").json()
while True:
    seen += [x["id"] for x in cur["data"]]
    if not cur["next_page_url"]:
        break
    cur = c.get(cur["next_page_url"]).json()
assert seen == sorted(set(seen)) and len(seen) >= 3, seen
assert c.get("/cursored?cursor=garbage").json()["data"][0]["id"] == cu1["data"][0]["id"]
ex = c.get("/extras")
assert ex.status_code == 200 and "<b>two</b>" in ex.text and "<b>one</b>" not in ex.text, ex.text
assert "<i>out</i>" in ex.text and "<i>in</i>" not in ex.text and "checked" in ex.text, ex.text
assert '<script src="/x.js"></script>' in ex.text and ex.text.index("/x.js") > ex.text.index("n=2"), ex.text
assert "var d = {&#34;a&#34;: 1}" in ex.text or 'var d = {"a": 1}' in ex.text or "var d = {" in ex.text, ex.text
assert "<i>in</i>" in c.get("/extras", headers={"Authorization": "Bearer z"}).text
bd = c.get("/badges").text
assert 'class="badge big"' in bd and 'data-id="7"' in bd and "ok:hi" in bd and "info:plain" in bd and 'kind=' not in bd, bd
assert '<span class="badge"' in bd and "{{" not in bd, bd
ch = c.get("/chips").text
assert 'class="chip"' in ch and "calm/3:a" in ch and "hot/9:b" in ch and 'id="c1"' in ch and 'class="chip z"' in ch and 'tone=' not in ch and 'size=' not in ch, ch
ps = c.get("/phpset")
assert ps.status_code == 200 and "<p>n=5 t=ab</p>" in ps.text, ps.text
assert "<u>cannot</u>" in ex.text and "<u>can</u>" not in ex.text, ex.text
cp = c.get("/comp")
assert cp.status_code == 200 and 'class="alert error"' in cp.text and 'class="alert info"' in cp.text, cp.text
assert "<h4>T</h4>" in cp.text and "body <b>H&lt;1&gt;</b>" in cp.text and "plain" in cp.text, cp.text
pid = p1.json()["id"]
nt = c.post(f"/posts/{pid}/notes", json={"text": "n1"})
assert nt.status_code == 200 and nt.json()["noteable_id"] == pid and nt.json()["noteable_type"] == "Post", nt.text
assert [n["text"] for n in c.get(f"/posts/{pid}/notes").json()] == ["n1"]
assert c.get(f"/notes/{nt.json()['id']}/owner").json()["title"] == "t1"
nw = [x for x in c.get("/notes-with").json() if x["id"] == pid][0]
assert [n["text"] for n in nw["tags"]] == ["n1"], nw
pg = c.get("/paged?page=1").json()
assert pg["current_page"] == 1 and pg["per_page"] == 2 and len(pg["data"]) == 2, pg
assert pg["total"] >= 3 and pg["last_page"] == -(-pg["total"] // 2), pg
assert pg["next_page_url"] == "/paged?page=2" and pg["prev_page_url"] is None, pg
last = c.get(f"/paged?page={pg['last_page']}").json()
assert last["next_page_url"] is None and last["prev_page_url"] is not None and last["to"] == pg["total"], last
assert c.get("/paged?page=abc").json()["current_page"] == 1
pv = c.get("/paged-view?page=2&x=1")
assert pv.status_code == 200 and f"Total {pg['total']}" in pv.text and 'class="u"' in pv.text, pv.text
assert 'rel="prev"' in pv.text and "x=1" in pv.text and "<nav" in pv.text, pv.text
ids = [u["id"] for u in c.get("/skipped").json()]
assert len(ids) == 2 and ids == sorted(ids), ids
assert c.get("/search?q=hi").json() == {"q": "hi"} and c.get("/search").json() == {"q": "none"}
paths = {r.path for r in app.routes}
assert "/users/{id}" in paths and "/api/items/{item}" in paths, paths
print("php e2e OK", len(paths))
PYEOF
