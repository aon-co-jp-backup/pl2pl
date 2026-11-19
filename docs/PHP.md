# PHP 変換の対応範囲(pl2pl php)

- **ルート**: Laravel の `Route::get/post/put/patch/delete/any/resource/apiResource/view`、`prefix` グループ、Symfony の `#[Route]`。
- **本体**: クロージャとコントローラの単純な `return`、複数文(取得 → 更新・保存・削除 → 返却)、`response()->json($x, 201)`、`noContent()`。
- **Eloquent → SQLAlchemy**: `db.py`・`models.py` を生成。列はマイグレーションの `Schema::create` か `$fillable` から取り、無い項目は `extra`(JSON)へ入れる。接続先は環境変数 `DATABASE_URL`(既定は SQLite)。
- **クエリビルダ**: `where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip` と、`paginate / simplePaginate / cursorPaginate`(cursor は件数位置ベースの近似、`?page=N`、Laravel と同じ JSON の形、Blade の `$users->links()`・`->total()`)、`$request->query('k', 既定値)` と、`get / first / firstOrFail / count / exists / delete`。`orWhere` は左から順に評価(Laravel の括弧グループとは異なる)。
- **Blade → Jinja2**(`templates/`): 出力・`@if`・`@foreach`・`@forelse`・`@for`・`@extends/@section/@yield/@include`・`@push/@stack`・`@auth/@guest`(Authorization ヘッダーの有無による簡易判定)・`@switch/@case`・`@checked/@selected/@disabled`・`@json`・`@props` と無名コンポーネント `<x-name attr="v">本文</x-name>`(名前付きスロット対応、`components/` のテンプレートを include)など。同じスタックへの `@push` は1回だけ。`view()`・`compact`・`->with`・`Route::view` は `TemplateResponse` へ。
- **ミドルウェア → `Depends`**(`middleware.py`): `auth`(Bearer と環境変数 `API_TOKENS` の簡易判定。**本物の認証へ差し替えること**)、`guest`、`throttle:N,M`(メモリ上の簡易版)、独自ミドルウェアの「条件で拒否 → `return $next($request)`」の形と、`$r = $next($request); $r->header('K','V'); return $r;` の形(ヘッダー追加のみ。`Response` を直接返すルートには付かない)。
- **リレーション**: `hasMany` / `hasOne` / `belongsTo` / `belongsToMany`(中間表を自動生成)/ `morphMany` / `morphOne` / `morphTo`(型の列には持ち主のクラス名を保存。Laravel の完全修飾名とは異なる)。`$u->posts`、`$p->user`、`$u->posts()->create([...])`、`attach / detach / sync`、`with('posts')`(`with('posts.comments')` のネストも可)、`whereHas('posts', fn($q) => $q->where(...))`、`has('posts')`・`whereDoesntHave`・`doesntHave` に対応。外部キー列が無ければ追加する。
- **認可**: `@can` / `@cannot` / `@canany` は、`request.state.abilities`(権限名の一覧)を見る簡易版(無ければ全て拒否)。Gate/Policy へ差し替えること。
- **未対応(TODO として報告)**: クラスベースの Blade コンポーネント(`@component`)・`$attributes`。

