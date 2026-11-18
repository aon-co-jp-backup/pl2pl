"""変換した Python アプリを実際に動かして、元の Java の挙動どおりか確かめる(FastAPI の TestClient)。
環境変数 E2E_DIR に変換出力のルート(spring/ jaxrs/ servlet/ spark/ を含む)を渡す。"""
import importlib
import os
import sys

import pytest
from fastapi.testclient import TestClient

ROOT = os.environ["E2E_DIR"]
MODS = ["main", "models", "services", "controllers", "java_rt"]


def load(sample):
    for m in MODS:
        sys.modules.pop(m, None)
    sys.path.insert(0, os.path.join(ROOT, sample))
    try:
        main = importlib.import_module("main")
    finally:
        sys.path.pop(0)
    return TestClient(main.app)


@pytest.fixture()
def spring():
    return load("spring")


def test_spring_create_get_list_delete(spring):
    c = spring
    r = c.post("/api/users", json={"name": "Ann Lee", "email": "a@x.jp", "age": 30})
    assert r.status_code == 201, r.text
    u = r.json()
    assert u["id"] == 1 and u["name"] == "Ann Lee" and u["active"] is True
    assert c.get("/api/users/1").json()["email"] == "a@x.jp"
    assert c.get("/api/users/99").status_code == 404
    assert len(c.get("/api/users").json()) == 1
    assert len(c.get("/api/users", params={"name": "Ann Lee"}).json()) == 1
    assert c.get("/api/users", params={"name": "nobody"}).json() == []
    assert c.delete("/api/users/1").status_code == 204
    assert c.delete("/api/users/1").status_code == 404


def test_spring_validation_errors(spring):
    c = spring
    r = c.post("/api/users", json={"name": " ", "email": "b@x.jp", "age": 5})
    assert r.status_code == 400 and "name is required" in r.json()["detail"]
    r = c.post("/api/users", json={"name": "Bob", "email": "b@x.jp", "age": 200})
    assert r.status_code == 400 and "bad age: 200" in r.json()["detail"]
    assert c.post("/api/users", json={"name": "Bob", "email": "b@x.jp", "age": 20}).status_code == 201
    # IllegalStateException は Java では 500 になる(コントローラーが捕まえない)。変換後も例外が伝わる(409 に写像)
    r = c.post("/api/users", json={"name": "Bob2", "email": "b@x.jp", "age": 20})
    assert r.status_code == 409


def test_spring_logic(spring):
    c = spring
    assert c.get("/api/users/grade", params={"score": 95}).json() == "A"
    assert c.get("/api/users/grade", params={"score": 85}).json() == "B"
    assert c.get("/api/users/grade", params={"score": 10}).json() == "C!!"
    assert c.get("/api/users/grade").json() == "C!!"  # defaultValue = 0
    assert c.post("/api/users/sum", json=[1, 2, 3, 4]).json() == 10
    assert c.get("/api/users/initials", params={"name": "ann marie lee"}).json() == "AML"
    assert c.get("/api/users/initials").status_code == 422  # 必須パラメータなし


def test_spring_openapi_has_routes(spring):
    paths = spring.get("/openapi.json").json()["paths"]
    assert "/api/users/{id}" in paths and "/api/users/sum" in paths


def test_jaxrs():
    c = load("jaxrs")
    assert c.get("/items/7").json() == "item 7 q=none"
    assert c.get("/items/7", params={"q": "z"}).json() == "item 7 q=z"
    assert c.post("/items", content="hello", headers={"content-type": "text/plain"}).status_code in (200, 422)


def test_servlet():
    c = load("servlet")
    assert c.get("/hello").text == "Hello, world!\n"
    assert c.get("/hello", params={"name": "Tom"}).text == "Hello, Tom!\n"


def test_spark():
    c = load("spark")
    assert c.get("/hi/Bob").json() == "Hi Bob" or c.get("/hi/Bob").text == "Hi Bob"
    assert c.get("/add", params={"a": "2", "b": "5"}).json() in ("7", 7)
