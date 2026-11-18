# pl2pl 引き継ぎ / Hand-off
## 到達点 (2026-11-18) / Status
**日本語**: 雛形(README・設計メモ・`docs/LANGUAGES.md`)のみ。コードなし。所有者が想定した変換元(HTML5〜aruaru-db の31項目)と、網羅した候補一覧を記録済み。最初に変換する言語は選定中。
**English**: Skeleton only (README, design notes, `docs/LANGUAGES.md`); no code. The owner's proposed sources (31 items) and the exhaustive candidate list are recorded. The first language is still being chosen.
## 未了 / Open
1. 最初の言語の決定(案: PHP)。/ Choose the first language (suggestion: PHP).
2. `docs/DESIGN.md` の各節(目的・IR・入出力・取り込み順)を所有者が書く。/ The owner fills in DESIGN.md.
3. 共通IRの試作(COBOL_to・Java_to のコードから共通部分を抽出)。/ Prototype a shared IR from COBOL_to and Java_to.
4. 一覧の最新性の検証(検索)。/ Verify the list is current.
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
