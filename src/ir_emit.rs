//! IR の出力側(FastAPI + SQLAlchemy): モデル(db.py・models.py)とミドルウェア(middleware.py)を書き出す。
use crate::ir::{Middleware, Model, Relation, Template};

pub const DB_PY: &str = r##""""pl2pl が生成した DB 層(SQLAlchemy)。接続先は環境変数 DATABASE_URL(既定は SQLite のファイル)。"""
import os

from sqlalchemy import create_engine
from sqlalchemy.orm import declarative_base, sessionmaker

engine = create_engine(os.environ.get("DATABASE_URL", "sqlite:///./app.db"))
Session = sessionmaker(engine, expire_on_commit=False)
Base = declarative_base()
"##;

const MODEL_PY: &str = r##"

class __M__(Base):
    """Eloquent モデル __M__ の SQLAlchemy 版(テーブル __T__)。マイグレーションに無い項目は extra(JSON)へ入る。"""
    __tablename__ = "__T__"
    id = Column(Integer, primary_key=True, autoincrement=True)
__COLS__    extra = Column(JSON, default=dict)
    COLUMNS = (__NAMES__)

    def to_dict(self):
        d = {"id": self.id}
        for c in self.COLUMNS:
            d[c] = getattr(self, c)
        d.update(self.extra or {})
        return d

    @classmethod
    def query(cls):
        return Query(cls)

    @classmethod
    def all(cls):
        with Session() as s:
            return [r.to_dict() for r in s.query(cls).order_by(cls.id)]

    @classmethod
    def find(cls, id):
        with Session() as s:
            r = s.get(cls, int(id))
            return r.to_dict() if r else None

    @classmethod
    def find_or_fail(cls, id):
        r = cls.find(id)
        if r is None:
            raise HTTPException(status_code=404, detail="__M__ not found")
        return r

    @classmethod
    def update(cls, id, data):
        with Session() as s:
            r = s.get(cls, int(id))
            if r is None:
                return None
            data = dict(data or {})
            data.pop("id", None)
            for k in list(data):
                if k in cls.COLUMNS:
                    setattr(r, k, data.pop(k))
            r.extra = {**(r.extra or {}), **data}
            s.commit()
            return r.to_dict()

    @classmethod
    def destroy(cls, id):
        with Session() as s:
            r = s.get(cls, int(id))
            if r is None:
                return 0
            s.delete(r)
            s.commit()
            return 1

    @classmethod
    def create(cls, data):
        data = dict(data or {})
        row = cls(**{k: data.pop(k) for k in list(data) if k in cls.COLUMNS}, extra=data)
        with Session() as s:
            s.add(row)
            s.commit()
            return row.to_dict()
__RELS__"##;

const QUERY_PY: &str = r##"

import operator as _op
import re as _re
from urllib.parse import urlencode as _urlencode

_OPS = {"=": _op.eq, "==": _op.eq, "!=": _op.ne, "<>": _op.ne, "<": _op.lt, "<=": _op.le, ">": _op.gt, ">=": _op.ge}


def _py_match(row, col, op, val):
    v = row.get(col)
    if op == "in":
        return v in val
    if op == "like":
        pat = "^" + _re.escape(val).replace("%", ".*").replace("_", ".") + "$"
        return v is not None and _re.match(pat, str(v), _re.I) is not None
    try:
        return _OPS[op](v, val)
    except TypeError:
        return False


def _load(cls, rows, parts):
    """with('a.b') の読み込み。a を各行へ付け、その子に対して b を再帰的に付ける。"""
    rel, rest = parts[0], parts[1:]
    for r in rows:
        if rel not in r:
            r[rel] = getattr(cls, rel)(r)
    if rest:
        sub = globals()[getattr(cls, "REL_CLASS", {})[rel]]
        kids = [k for r in rows for k in ([r[rel]] if isinstance(r[rel], dict) else (r[rel] or []))]
        _load(sub, kids, rest)


class Page(dict):
    """paginate() の結果。JSON にすると Laravel と同じ形、テンプレートでは繰り返せて links() も使える。"""

    def __init__(self, url, **kw):
        super().__init__(**kw)
        self._url = url

    def __iter__(self):
        return iter(self["data"])

    def currentPage(self):
        return self["current_page"]

    def lastPage(self):
        return self["last_page"]

    def perPage(self):
        return self["per_page"]

    def hasPages(self):
        return self["last_page"] > 1

    def hasMorePages(self):
        return self["current_page"] < self["last_page"]

    def onFirstPage(self):
        return self["current_page"] <= 1

    def links(self):
        from markupsafe import Markup

        if not self.hasPages():
            return Markup("")
        cur, last = self["current_page"], self["last_page"]
        parts = []
        if cur > 1:
            parts.append(Markup('<a href="{}" rel="prev">&laquo; Previous</a>').format(self._url(cur - 1)))
        for p in range(1, last + 1):
            if p == cur:
                parts.append(Markup("<strong>{}</strong>").format(p))
            else:
                parts.append(Markup('<a href="{}">{}</a>').format(self._url(p), p))
        if cur < last:
            parts.append(Markup('<a href="{}" rel="next">Next &raquo;</a>').format(self._url(cur + 1)))
        return Markup('<nav class="pagination">') + Markup(" ").join(parts) + Markup("</nav>")


class Query:
    """Eloquent のクエリビルダの代わり。列(マイグレーション/$fillable)は SQL、extra の項目や or_where は Python 側で絞る。"""

    def __init__(self, cls):
        self.cls = cls
        self.conds = []
        self.orders = []
        self._limit = None
        self._offset = 0
        self._with = []
        self.has = []

    def with_(self, rels):
        self._with += [rels] if isinstance(rels, str) else list(rels)
        return self

    def where_has(self, rel, conds=None, negate=False):
        self.has.append((rel, conds or [], negate))
        return self

    def _decorate(self, rows):
        for path in self._with:
            _load(self.cls, rows, path.split("."))
        return rows

    def where(self, col, op, val=None):
        self.conds.append(("and", col, op, val))
        return self

    def or_where(self, col, op, val=None):
        self.conds.append(("or", col, op, val))
        return self

    def order_by(self, col, direction="asc"):
        self.orders.append((col, str(direction).lower() == "desc"))
        return self

    def limit(self, n):
        self._limit = int(n)
        return self

    def _real(self, col):
        return col == "id" or col in self.cls.COLUMNS

    def get(self):
        c = self.cls
        use_sql = all(k == "and" and self._real(col) for k, col, _, _ in self.conds) and all(self._real(col) for col, _ in self.orders) and not self.has
        with Session() as s:
            q = s.query(c)
            if use_sql:
                for _, col, op, val in self.conds:
                    a = getattr(c, col)
                    if op == "in":
                        q = q.filter(a.in_(val))
                    elif op == "like":
                        q = q.filter(a.like(val))
                    else:
                        q = q.filter(_OPS[op](a, val))
                for col, desc in self.orders:
                    q = q.order_by(getattr(c, col).desc() if desc else getattr(c, col))
                if not self.orders:
                    q = q.order_by(c.id)
                if self._offset:
                    q = q.offset(self._offset)
                if self._limit is not None:
                    q = q.limit(self._limit)
                return self._decorate([r.to_dict() for r in q])
            rows = [r.to_dict() for r in q.order_by(c.id)]
        if self.conds:
            def keep(row):
                res = None
                for k, col, op, val in self.conds:
                    m = _py_match(row, col, op, val)
                    res = m if res is None else ((res or m) if k == "or" else (res and m))
                return res
            rows = [r for r in rows if keep(r)]
        for rel, hc, neg in self.has:
            rows = [r for r in rows if any(all(_py_match(x, c, o, v) for c, o, v in hc) for x in getattr(self.cls, rel)(r)) != neg]
        for col, desc in reversed(self.orders):
            rows.sort(key=lambda r: (r.get(col) is None, r.get(col)), reverse=desc)
        rows = rows[self._offset:]
        return self._decorate(rows[: self._limit] if self._limit is not None else rows)

    def offset(self, n):
        self._offset = max(int(n), 0)
        return self

    def paginate(self, per_page=15, request=None, page_name="page"):
        """Laravel の paginate()。?page=N を読み、Laravel と同じキーの Page(辞書)を返す。"""
        per_page = max(int(per_page), 1)
        qp = dict(request.query_params) if request is not None else {}
        try:
            page = max(int(qp.get(page_name, 1)), 1)
        except ValueError:
            page = 1
        total = self.count()
        saved = (self._limit, self._offset)
        self._limit, self._offset = per_page, (page - 1) * per_page
        rows = self.get()
        self._limit, self._offset = saved
        last = max(-(-total // per_page), 1)
        qp.pop(page_name, None)
        path = request.url.path if request is not None else ""

        def url(p):
            return path + "?" + _urlencode({**qp, page_name: p})

        return Page(
            url,
            data=rows, current_page=page, per_page=per_page, total=total, last_page=last,
            **{"from": (page - 1) * per_page + 1 if rows else None, "to": (page - 1) * per_page + len(rows) if rows else None},
            next_page_url=url(page + 1) if page < last else None,
            prev_page_url=url(page - 1) if page > 1 else None,
        )

    def cursor_paginate(self, per_page=15, request=None):
        """Laravel の cursorPaginate()。cursor は「何件目から」を base64 にしたもの(Laravel の列値ベースとは異なる近似)。"""
        import base64
        import json

        per_page = max(int(per_page), 1)
        qp = dict(request.query_params) if request is not None else {}
        try:
            start = max(int(json.loads(base64.urlsafe_b64decode(qp.get("cursor", "") + "==")).get("offset", 0)), 0)
        except Exception:
            start = 0
        qp.pop("cursor", None)
        path = request.url.path if request is not None else ""
        saved = (self._limit, self._offset)
        self._limit, self._offset = per_page + 1, start
        rows = self.get()
        self._limit, self._offset = saved

        def cur(n):
            return base64.urlsafe_b64encode(json.dumps({"offset": n}).encode()).decode().rstrip("=")

        more = len(rows) > per_page
        rows = rows[:per_page]
        nxt = cur(start + per_page) if more else None
        prv = cur(max(start - per_page, 0)) if start > 0 else None
        return {
            "data": rows, "per_page": per_page, "path": path,
            "next_cursor": nxt, "prev_cursor": prv,
            "next_page_url": path + "?" + _urlencode({**qp, "cursor": nxt}) if nxt else None,
            "prev_page_url": path + "?" + _urlencode({**qp, "cursor": prv}) if prv else None,
        }

    def first(self):
        saved, self._limit = self._limit, 1
        rows = self.get()
        self._limit = saved
        return rows[0] if rows else None

    def first_or_fail(self):
        r = self.first()
        if r is None:
            raise HTTPException(status_code=404, detail="not found")
        return r

    def count(self):
        saved = (self._limit, self._offset)
        self._limit, self._offset = None, 0
        n = len(self.get())
        self._limit, self._offset = saved
        return n

    def exists(self):
        return self.first() is not None

    def delete(self):
        n = 0
        for r in self.get():
            n += self.cls.destroy(r["id"])
        return n
"##;


fn rel_methods(r: &Relation) -> String {
    let (n, rel, fk) = (&r.name, &r.related, &r.fk);
    match r.kind.as_str() {
        "hasMany" | "hasOne" => {
            let get = if r.kind == "hasMany" { "get" } else { "first" };
            format!("\n    @classmethod\n    def {n}(cls, row):\n        return {rel}.query().where(\"{fk}\", \"=\", row[\"id\"]).{get}()\n\n    @classmethod\n    def {n}_create(cls, row, data):\n        return {rel}.create({{**(data or {{}}), \"{fk}\": row[\"id\"]}})\n")
        }
        "morphMany" | "morphOne" => {
            let (get, t) = (if r.kind == "morphMany" { "get" } else { "first" }, &r.other_fk);
            format!("\n    @classmethod\n    def {n}(cls, row):\n        return {rel}.query().where(\"{fk}\", \"=\", row[\"id\"]).where(\"{t}\", \"=\", \"{}\").{get}()\n\n    @classmethod\n    def {n}_create(cls, row, data):\n        return {rel}.create({{**(data or {{}}), \"{fk}\": row[\"id\"], \"{t}\": \"{}\"}})\n", r.pivot, r.pivot)
        }
        "morphTo" => {
            let t = &r.other_fk;
            format!("\n    @classmethod\n    def {n}(cls, row):\n        t, v = row.get(\"{t}\"), row.get(\"{fk}\")\n        return None if t is None or v is None else globals()[t].find(v)\n")
        }
        "belongsTo" => format!("\n    @classmethod\n    def {n}(cls, row):\n        v = row.get(\"{fk}\")\n        return None if v is None else {rel}.find(v)\n"),
        _ => {
            let (p, o) = (&r.pivot, &r.other_fk);
            let (tcw, tci) = if r.type_col.is_empty() { (String::new(), String::new()) } else { (format!(".where({p}.c.{} == \"{}\")", r.type_col, r.type_val), format!(", \"{}\": \"{}\"", r.type_col, r.type_val)) };
            format!("\n    @classmethod\n    def {n}(cls, row):\n        with Session() as s:\n            ids = [x[0] for x in s.execute(select({p}.c.{o}).where({p}.c.{fk} == row[\"id\"]){tcw})]\n        return [x for x in ({rel}.find(i) for i in ids) if x]\n\n    @classmethod\n    def {n}_attach(cls, row, ids):\n        ids = ids if isinstance(ids, (list, tuple)) else [ids]\n        have = {{x[\"id\"] for x in cls.{n}(row)}}\n        with Session() as s:\n            for i in ids:\n                if i is not None and int(i) not in have:\n                    s.execute({p}.insert().values({{\"{fk}\": row[\"id\"], \"{o}\": int(i){tci}}}))\n            s.commit()\n\n    @classmethod\n    def {n}_detach(cls, row, ids=None):\n        with Session() as s:\n            q = {p}.delete().where({p}.c.{fk} == row[\"id\"]){tcw}\n            if ids is not None:\n                ids = ids if isinstance(ids, (list, tuple)) else [ids]\n                q = q.where({p}.c.{o}.in_([int(i) for i in ids]))\n            s.execute(q)\n            s.commit()\n\n    @classmethod\n    def {n}_sync(cls, row, ids):\n        cls.{n}_detach(row)\n        cls.{n}_attach(row, ids)\n")
        }
    }
}


pub fn models_py(models: &[Model]) -> String {
    let mut o = String::from("\"\"\"pl2pl が生成したモデル(SQLAlchemy)。\"\"\"\nfrom fastapi import HTTPException\nfrom sqlalchemy import JSON, Boolean, Column, Float, Integer, String, Table, Text, select\n\nfrom db import Base, Session, engine\n");
    o.push_str(QUERY_PY);
    let mut pivots: Vec<(String, String, String, String)> = vec![];
    for m in models {
        for r in m.rels.iter().filter(|r| matches!(r.kind.as_str(), "belongsToMany" | "morphToMany")) {
            if !pivots.iter().any(|p| p.0 == r.pivot) {
                pivots.push((r.pivot.clone(), r.fk.clone(), r.other_fk.clone(), r.type_col.clone()));
            }
        }
    }
    for (p, a, b, t) in &pivots {
        let tcol = if t.is_empty() { String::new() } else { format!(", Column(\"{t}\", String(255))") };
        o.push_str(&format!("

{p} = Table(\"{p}\", Base.metadata, Column(\"{a}\", Integer), Column(\"{b}\", Integer){tcol})
"));
    }
    for md in models {
        let (m, t, cols) = (&md.name, &md.table, &md.cols);
        let cs: String = cols.iter().map(|(n, ty)| format!("    {n} = Column({ty})
")).collect();
        let names: String = cols.iter().map(|(n, _)| format!("{n:?}, ")).collect();
        let mut relcode: String = md.rels.iter().map(rel_methods).collect();
        if !md.rels.is_empty() {
            let pairs: Vec<String> = md.rels.iter().map(|r| format!("{:?}: {:?}", r.name, r.related)).collect();
            relcode.push_str(&format!("
    REL_CLASS = {{{}}}
", pairs.join(", ")));
        }
        o.push_str(&MODEL_PY.replace("__RELS__", &relcode).replace("__M__", m).replace("__T__", t).replace("__COLS__", &cs).replace("__NAMES__", &names));
    }
    o.push_str("\n\nBase.metadata.create_all(engine)\n");
    o
}


/// 出力側: ミドルウェア(Depends)。前置きの共通コード + 各依存の定義。
pub fn middleware_py(defs: &[Middleware]) -> String {
    let mut o = String::from(PRELUDE);
    for d in defs {
        o.push('\n');
        o.push_str(&d.code);
    }
    o
}

const PRELUDE: &str = r##""""pl2pl が生成したミドルウェア(FastAPI の Depends)。
auth は `Authorization: Bearer <トークン>` が環境変数 API_TOKENS(カンマ区切り)にあるかで判定する。本物の認証(セッション・Sanctum 等)へ差し替えること。
throttle はメモリ上の簡易版(プロセスを再起動すると数え直し、複数プロセス間では共有されない)。"""
import fnmatch
import os
import time

from fastapi import Request, Response
from fastapi.responses import JSONResponse, RedirectResponse


class MiddlewareAbort(Exception):
    """ミドルウェアがリクエストを止めて、このレスポンスを返す。"""

    def __init__(self, response):
        self.response = response


def _bearer(request: Request):
    h = request.headers.get("authorization", "")
    return h[7:].strip() if h.lower().startswith("bearer ") else None


def _valid_token(token) -> bool:
    return bool(token) and token in [t.strip() for t in os.environ.get("API_TOKENS", "").split(",") if t.strip()]


def _is(request: Request, pattern: str) -> bool:
    return fnmatch.fnmatch(request.url.path.lstrip("/"), pattern)


def require_auth(request: Request):
    if not _valid_token(_bearer(request)):
        raise MiddlewareAbort(JSONResponse({"message": "Unauthenticated."}, status_code=401))


def require_guest(request: Request):
    if _valid_token(_bearer(request)):
        raise MiddlewareAbort(RedirectResponse("/", status_code=302))


_hits: dict = {}


def throttle(limit: int = 60, minutes: int = 1, name: str = "default"):
    def dep(request: Request):
        now = time.time()
        window = minutes * 60
        route = request.scope.get("route")
        key = (name, route.path if route else request.url.path, request.client.host if request.client else "?")
        recent = [t for t in _hits.get(key, []) if now - t < window]
        if len(recent) >= limit:
            retry = int(window - (now - recent[0])) + 1
            raise MiddlewareAbort(JSONResponse({"message": "Too Many Attempts."}, status_code=429, headers={"Retry-After": str(retry)}))
        recent.append(now)
        _hits[key] = recent

    return dep

"##;


/// 出力側: テンプレート(Jinja2)。`templates/` の下へ置く。
pub fn templates(ts: &[Template]) -> Vec<(String, String)> {
    ts.iter().map(|t| (format!("templates/{}", t.name), t.text.clone())).collect()
}
