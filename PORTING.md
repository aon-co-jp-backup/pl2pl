# pl2pl 引き継ぎ
## 到達点(2026-11-19)
v0.1.0 公開済み(Windows/Linux/macOS のインストーラーと CI の通し試験)。実装済みのコマンドは README の表のとおりです。
PHP 変換は、ルート・複数文の本体・Eloquent(SQLAlchemy)・クエリビルダ・Blade(Jinja2)・ミドルウェア(`Depends`)まで実FastAPIで通し試験済みです。リレーション(`hasMany`・`hasOne`・`belongsTo`・`belongsToMany`・`with`・`whereHas`)も実SQLiteで確認済みです。Numba・Kokkos(Serial)・`pl2pl web` も実機で確認済みです。
## 未了(優先順)
1. PHP: `@can` 系の認可・クラスベースの Blade コンポーネント(`@component`)・`$attributes`・`@php`・`morphToMany`。
2. COBOL_to・Java_to は `converters/` へ取り込み済み。残り: 元リポジトリのその後の変更の取り込み・元リポジトリの削除(所有者の指示。**削除は元に戻せないので最終確認を取る**。別セッションが元リポジトリで開発を続けている可能性がある)。
3. 共通 IR の試作(COBOL_to・Java_to のコードから共通部分を抽出)。
4. aruaru-llm との連携(翻訳・説明の補助は変換器と別コンポーネント)。`llm setup` の実導入は未確認。
5. Kokkos の GPU 実行・`Kokkos::View` への置き換え、open-cuda / open-directx 上のカーネル実行。
6. `docs/DESIGN.md` の各節を所有者が書く。一覧(`docs/LANGUAGES.md`)の最新性の検証。
## 再開用メッセージ
「F:\pl2pl を再開。PORTING.md の未了の1から進める(PHP の残りの項目)。変換できない箇所は黙って落とさず TODO で残す。実際に動かして確認してから完了と報告する。返答は日本語のみ。push は区切りのよいところで確認なしに進めてよい(所有者の指示)。」
---
## 決定済み(訂正済み: WEB版は説明とダウンロードのみ、変換はPCアプリ。主要変換元=OSACCESS・ACCESS VBA・VB・VB.NET・VBA・C・C++・C#.NET・COBOL) / Decided (corrected: web = downloads and manuals only; conversion in the PC app)
以下は訂正前の記述 / Text below predates the correction:
日本語: 全部は網羅せず、選んだものだけ実装。WEB版はHDD上の変換器を要求ごとに呼び出す(常駐させない)。`docs/DESIGN.md` 参照。 / English: Not exhaustive; only chosen entries. The web version calls on-disk converters per request (not resident). See `docs/DESIGN.md`.
## 再開用メッセージ / Resume message
「F:\pl2pl を再開。docs/LANGUAGES.md の一覧を見て、最初に変換する言語を決める。決まったら DESIGN.md に所有者の指示どおり追記。」

**最新の想定 / Latest assumption**: 主要OS=メインフレーム・UNIX・Linux・Windows・Mac、主要言語21項目の相互変換(`docs/DESIGN.md` 末尾)。直前の主要変換元の一覧はこれで置き換え。 / Main OSes and 21 languages, both-way (end of `docs/DESIGN.md`); replaces the previous priority list.

**2026-11-18**: `docs/COMBINATIONS.md` を追加(OS×言語・モデル表、既存ツール、優先フレームワーク、最初の変換案)。次: 最初の変換の組み合わせを選ぶ。 / Added `docs/COMBINATIONS.md`; next: choose the first conversion pair.

## 実装の到達点(2026-11-18) / Implementation status
**日本語**: 4案(COBOL→Linux、OpenMP、PHP→FastAPI、既存変換器への入口 `web`)と、cport・pyhpc・kokkos・gpu(試作)・doctor を実装。実機確認済み=COBOL(JCL経由)、OpenMP(同一結果)、PHP(TestClient)、cport(gcc)。**未確認**: Numba の実行(WSL の venv 作成が長引いた)、Kokkos のビルド、open-cuda/open-directx 上でのカーネル実行、`web` の実行(振り分け計画の単体試験のみ)。**未実装**: aruaru-llm との連携、リリース用インストーラー/CI、Java・COBOL→FastAPI/Poem の本体強化、PHP の本体変換。
**English**: Implemented the four chosen pairs plus cport, pyhpc, kokkos, gpu (prototype) and doctor. Verified for real: COBOL via JCL, OpenMP (same result), PHP (TestClient), cport (gcc). **Unverified**: Numba execution, Kokkos build, kernels on open-cuda/open-directx, `web` execution (plan unit-tested only). **Not implemented**: aruaru-llm wiring, release installers/CI, deeper Java/COBOL conversion, PHP bodies.

**2026-11-18 追記**: `pl2pl llm setup/start` と `installer/bundle-components.sh` を追加(aruaru-llm を同梱し、モデルは空きメモリに合わせて自動導入。重みは同梱しない)。実導入は未確認。 / Added `llm setup/start` and `bundle-components.sh`; weights are not bundled; a real install is unverified.

**2026-11-18 追記**: `v0.1.0` を公開。Windows x64/ARM64 exe・Linux amd64/arm64 deb+tar.gz・macOS arm64/x64 pkg(未署名)を CI が作り、インストール→実行→削除の試験が全て成功(Linux 側は COBOL+JCL 実行・OpenMP 同一結果・cport・PHP→FastAPI の通し試験も成功)。 / `v0.1.0` released; all OS installers built and smoke-tested in CI.

**2026-11-18 追記**: PHP→FastAPI で、コントローラの単純な `return <値>` と `return $引数` を本体として変換(従来はTODO)。`tests/run-php.sh` に `/health`・`/echo/{id}` を追加し WSL で成功。複雑な本体(Eloquent等)は従来どおりTODO。 / PHP→FastAPI now translates simple controller `return` bodies (incl. returning path params); e2e extended and passing; complex bodies stay TODO.

**2026-11-18 追記2**: Numba の実行を WSL で確認(元=9999900000.0、変換後=9999900000.0、同一結果。`tests/numba-check.sh`)。Kokkos は未確認のまま(ソースの取得が要るため許可待ち)。 / Numba verified in WSL (same result); Kokkos still unverified (needs a source download, awaiting permission).

**2026-11-18 追記3**: Kokkos 4.5.01(Serial)を WSL でビルドし、変換結果が元と同一出力(sum=1249998750.000 a_last=999999.0)になることを確認(`tests/kokkos-check.sh`)。`Kokkos::ScopeGuard` は変換器が挿入しないため、試験側で足している(従来どおりレポートで指摘)。GPU/View は未確認。 / Kokkos (Serial) verified in WSL, same output; ScopeGuard is added by the test, not the converter; GPU/View unverified.

**2026-11-18 追記4**: (1) `pl2pl web` を実機で確認(Java 7ファイル・COBOL 1ファイルを実際に java_to/cobol_to へ振り分けて出力、終了コード0)。(2) PHP: Eloquent の `Model::all/find/findOrFail/create/destroy` を SQLAlchemy(`db.py`・`models.py`、列はマイグレーション/`$fillable`から、無い項目は extra JSON、接続は `DATABASE_URL`)へ生成し、実SQLiteで作成・取得・削除の通し試験が成功。**未対応**: 複数文の本体(`$u=find; $u->update(...)`)、`where()->get()` 等のクエリビルダ、ミドルウェア・Blade、`apiResource` のコントローラ未同梱時。 / `web` verified for real; Eloquent static calls now generate SQLAlchemy with a real-SQLite e2e; multi-statement bodies, query builder, middleware, Blade remain TODO.

**2026-11-18 追記5**: PHP の複数文の本体(`find→update/save/delete→return`、`response()->json($x, 201)`・`noContent()`)を変換。分からない文が1つでもあれば全体をTODOに(黙って落とさない)。実SQLiteの通し試験(PUT/rename/DELETE 204/404)成功。未対応は where()->get() 等のクエリビルダ・ミドルウェア・Blade。 / Multi-statement controller bodies translated; any unknown statement keeps the whole body as TODO; real-SQLite e2e passes.

**2026-11-18 追記6**: PHP のクエリビルダ(`where/orWhere/whereIn/orderBy/latest/oldest/limit/take` → `get/first/firstOrFail/count/exists/delete`)を生成した `Query` クラス(SQLAlchemy)へ変換。列は SQL、`extra` の項目や `orWhere` は Python 側で絞る(`orWhere` は左から順の評価で、Laravel の括弧グループとは異なる)。実SQLiteの通し試験成功。未対応: `whereHas`・リレーション・`paginate`・`$request->query()`・ミドルウェア・Blade。 / Query builder translated to a generated SQLAlchemy `Query`; real-SQLite e2e passes; relations, paginate, `$request->query()`, middleware, Blade remain TODO.

**2026-11-18 追記7**: Blade→Jinja2(`src/blade.rs`)を実装。`{{ }}`・`{!! !!}`(`|safe`)・`??`・`@if/@elseif/@else/@unless/@isset/@empty/@foreach/@forelse/@for`・`@extends/@section/@yield/@parent/@include`・コメントを変換し、`view()`(配列・`compact`・`->with`)と `Route::view` を `templates.TemplateResponse` へ。`@php/@auth/@can/@push/@component` 等や未対応の式は `{# TODO(pl2pl) #}` で残してレポートへ列挙。`@csrf`/`@method` は出力せず警告。実FastAPI+Jinja2で、レイアウト継承・forelse・エスケープ(Blade同様に既定でエスケープ)・`|safe`・メールアドレスの `@` を壊さないことを通し試験で確認。未対応: コンポーネント・スタック・認可ディレクティブ・`@while`・ミドルウェア・リレーション。 / Blade→Jinja2 implemented and e2e-verified; unsupported directives are left as TODO comments and listed in the report.

**2026-11-18 追記8**: Laravel ミドルウェアを FastAPI の `Depends`(生成 `middleware.py`)へ変換(`src/middleware.rs`)。ルートの `->middleware()`・`Route::middleware([...])->prefix()->group()`・コントローラ `__construct` の `$this->middleware()->only/except` を拾う。`auth`(Bearer と環境変数 API_TOKENS の簡易判定。**本物の認証へ差し替えること**)・`guest`(認証済みなら `/` へ302)・`throttle:N,M`(メモリ上の簡易版、ルート×IPで数える)と、独自ミドルウェアの「`if (条件) { abort(N) / response()->json(値,N) / redirect('/x') }` を並べて `return $next($request)`」を変換。`$next` の後でレスポンスを加工する形・else・`can`/`verified`/`signed` 等は依存を付けず、レポートに理由つきで列挙。実FastAPIで 401/403/200・throttle 429(Retry-After)・guest 302 を確認。 / Laravel middleware → FastAPI Depends; verified end-to-end; post-`$next` processing and unsupported names are reported, not silently dropped.

**2026-11-18 追記9**: PHP のリレーションを変換(`hasMany`・`hasOne`・`belongsTo`・`belongsToMany`)。モデルのメソッドから関係を読み、`models.py` に取得・`_create`・`_attach/_detach/_sync` を生成し、中間表は `Table` で自動作成、外部キー列が無ければ実列として追加。コントローラの `$u->posts`・`$p->user`・`$u->posts()->create()`・`->roles()->attach()`、クエリの `with('posts')`・`whereHas('posts', 条件)`・`has('posts')` を変換。実SQLiteの通し試験(作成・取得・eager load・whereHas・多対多の重複 attach 防止)成功、単体テスト30件成功。未対応: ネストした `with`・`morph` 系・`paginate`。

**2026-11-19 追記10**: PHP の `paginate()`/`simplePaginate()`・`offset/skip`・`withQueryString`・`$request->query('k', 既定)` を変換。生成する `Query.paginate(n, request)` が `?page=N` を読み、Laravel と同じキー(`data/current_page/per_page/total/last_page/from/to/next_page_url/prev_page_url`)の `Page` を返す(Blade では繰り返し・`links()`・`->total()` が使える。`links()` は簡易なナビで Laravel の見た目とは異なる)。実SQLiteの通し試験(ページ送り・範囲外・不正な page・`links()`・`offset`・`query`)成功、単体テスト30件成功。未対応: `cursorPaginate`・ネストした `with`・`morph` 系。

**2026-11-19 追記11**: PHP の `whereDoesntHave`/`doesntHave` を変換(`where_has` に否定フラグ)。実SQLiteの通し試験成功。未対応: ネストした `with`・`morph` 系・`cursorPaginate`。

**2026-11-19 追記12**: README・CLAUDE・PORTING を日本語+9言語(en・zh-CN・zh-TW・ko・de・fr・es・ru・pt、`docs/i18n/`)に展開。今できている部分と完成予定の機能を記載。AI翻訳でネイティブ検証前。原本(日本語)を更新したら各言語も追従させる。

**2026-11-19 追記13**: PHP のネストした `with('posts.comments')` を変換(モデルに `REL_CLASS` を生成し、`_load` が再帰的に読み込む)。実SQLiteの通し試験成功。未対応: `morph` 系・`cursorPaginate`。

**2026-11-19 追記14**: `docs/i18n/` の9言語(en・zh-CN・zh-TW・ko・de・fr・es・ru・pt)を、直訳調をやめて各言語の自然な言い回しで書き直した(ネイティブの検証は未実施)。ネストした `with` の対応も反映。

**2026-11-19 追記15**: PHP の `cursorPaginate()` を変換(cursor は「何件目から」を base64 にした近似で、Laravel の列値ベースとは異なる)。実SQLiteの通し試験成功、i18n 9言語も追従。未対応: `morph` 系・Blade のコンポーネント/スタック/認可系・`$next` 後処理のミドルウェア。

**2026-11-19 追記16**: `$r = $next($request); $r->header('K','V'); return $r;` 形の独自ミドルウェアを変換(`Response` を依存で受けてヘッダーを付ける)。ヘッダー追加のみ対応で、ボディの加工は TODO。ルートが `Response` を直接返すときは付かない。実FastAPIの通し試験成功(単体30件)。未対応: `morph` 系・Blade のコンポーネント/スタック/認可系。

**2026-11-19 追記17**: Blade の `@push/@stack`(Jinja2 のブロック。同じスタックへの2回目の `@push` は TODO)・`@auth/@guest`(Authorization ヘッダーの簡易判定)・`@switch/@case/@default`・`@checked/@selected/@disabled`・`@json` を変換。実FastAPIの通し試験成功、単体31件成功。未対応: `morph` 系・Blade のコンポーネント(`@component`・`<x-...>`)・`@can` 系・`@php`。

**2026-11-19 追記18**: Blade の無名コンポーネント(`<x-alert type="error" :title="$h">本文</x-alert>`・自己閉じ・`<x-slot:名前>`/`<x-slot name="..">`・`@props` の既定値)を変換(`{% with %}`+`{% set slot %}`+`{% include "components/…" %}`)。実FastAPIの通し試験成功、単体32件成功。未対応: `morph` 系・`@component`/クラスベースのコンポーネント・`$attributes`・`@can` 系・`@php`。

**2026-11-19 追記19**: PHP の `morphMany`/`morphOne`/`morphTo` を変換(`<名前>_id`・`<名前>_type` 列を追加し、型の値には持ち主のクラス名を保存。`$post->tags`・`->tags()->create()`・`$note->noteable`・`with('tags')` に対応)。実SQLiteの通し試験成功、単体32件成功。未対応: `morphToMany`/`morphedByMany`・`@component`/クラスベースのコンポーネント・`$attributes`・`@can` 系・`@php`。

**2026-11-19 追記20**: `.github/workflows/test.yml` を追加し、push・PR ごとに単体テストと PHP 通し試験を Linux x64・Linux arm64(ubuntu-24.04-arm)・macOS で実行。3 OS とも成功(run 44416117420)。未了の7番(新サンプルの CI 実行)は解消。

**2026-11-19 追記21**: COBOL_to(`4bf7670`)と Java_to(`22a9193`)を `git subtree` で `converters/cobol_to`・`converters/java_to` へ履歴つきで取り込んだ。pl2pl 内で `cargo test` が通る(cobol_to 10件・java_to 15件)、pl2pl 本体のビルドも成功。**元リポジトリは削除していない**(所有者の最終確認待ち)。取り込み後に元で増えた変更は、`git subtree pull` で追従できる。`pl2pl web` はまだ外部の実行ファイル名を呼ぶ(同梱コピーへの切り替えは未了)。

**2026-11-19 追記22**: `pl2pl web` が、実行ファイルの隣か `components/bin/` にある変換器を先に使い、無ければ PATH の名前を呼ぶようにした(`resolve_in`、単体テストあり)。配布物を作る前に `bash installer/build-converters.sh` で `converters/` の2つをビルドして `components/bin/` へ置く(bundle-components.sh と同じ置き場所)。未了: リリースのワークフローがこのスクリプトを呼んで配布物へ入れること、元リポジトリの削除(最終確認待ち)。
