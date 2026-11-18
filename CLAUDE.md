# pl2pl 開発メモ / Dev notes
**日本語**
- 役割: COBOL_to・Java_to・2FastAPI・2MFastAPI・2Poem を共通の中間表現(IR)で統合・融合する構想リポジトリ。v0.1.0 公開済みで、コードがある(`src/`)。設計・仕様は所有者が少しずつ書く(`docs/DESIGN.md`)。勝手に仕様を決めて書き込まないこと。
- 変換元の候補一覧は `docs/LANGUAGES.md`(知識ベース、最新性は未検証)。最初の言語は選定中(案: PHP)。
- IR は3系統に分ける想定: ①サーバー側(ルート・型・本体)②DB/SQL ③画面側(出力は RPoem の Rust→WASM)。
- 既存リポジトリは当面そのまま残す。破壊的な統合はしない。
- 前身として pg2pg という名で作ったが、所有者の指示で削除し pl2pl に改めた(2026-11-18)。
**English**
- Role: planning repository to unify COBOL_to, Java_to, 2FastAPI, 2MFastAPI and 2Poem on a shared intermediate representation. No code yet. The owner writes design/spec gradually (`docs/DESIGN.md`); do not invent spec on their behalf.
- Source candidates: `docs/LANGUAGES.md` (knowledge-based, not freshly verified). First language undecided (suggestion: PHP).
- Three IR families expected: server side, DB/SQL, UI side (RPoem Rust→WASM).
- Existing repositories stay as they are; no destructive merge.
- Created first as `pg2pg`, deleted at the owner's request and renamed `pl2pl` (2026-11-18).

## 訂正(2026-11-18、所有者): WEB版は変換しない。ダウンロードと多言語の説明書のみ。変換はPCのインストール型アプリ(フォルダ選択を複数・自動判定・変換先指定)。主要変換元=OSACCESS・ACCESS VBA・VB・VB.NET・VBA・C・C++・C#.NET・COBOL。詳細 docs/DESIGN.md / Correction: the web version only offers downloads and manuals; conversion runs in the installed PC app. Details in docs/DESIGN.md.

## 以前の決定(上の訂正で置き換え) / Superseded decisions
- 日本語: 全組み合わせは作らない。一覧から選んだものだけ実装して完成とする。WEB版は変換器を常駐させず、ディスク上の実行ファイルを要求ごとに起動して結果を返す(メモリー節約)。詳細は `docs/DESIGN.md`。
- English: Do not build every combination; implement only the chosen ones. The web version never keeps converters resident; it starts the on-disk executable per request. Details in `docs/DESIGN.md`.

- 主要OS=メインフレーム・UNIX・Linux・Windows・Mac。主要言語21項目(ACCESS〜Rust)の相互変換を想定(全部は載せない)。詳細 docs/DESIGN.md / Main OSes and 21 main languages, both-way conversion assumed; details in docs/DESIGN.md.

- 組み合わせ(OS×言語・モデル)・既存ツールの現状・優先フレームワーク・最初の変換案は `docs/COMBINATIONS.md` / OS combinations, existing-tool status, priorities and first pairs are in `docs/COMBINATIONS.md`.

## 実装メモ(2026-11-18) / Implementation notes
- Rust CLI(`src/`): `loops.rs`(独立ループの保守的判定)を `omp`/`kokkos`/`gpu` が共有。`cobol.rs`(JCL→bash 含む)、`cport.rs`、`pyhpc.rs`、`php.rs`、`web.rs`(既存変換器への振り分け)。open-cpu は git 依存で内蔵。
- 判定は保守的: 依存・副作用の恐れがあれば**付けずに理由を報告**する。性能は保証しない。
- WSL で試験: `wsl -d Ubuntu -- bash /mnt/f/pl2pl/tests/run-all.sh`(`bash -c` に `$PATH` を書くとホスト側で展開される)。Windows の Git Bash では heredoc の `\` が消えることがあるので Rust ソースは Write ツールで書く。
- 同梱の実態は `components/README.md`(open-cpu のみバイナリ内。open-cuda/open-directx は試作の出力先、aruaru-llm は未連携)。

- 翻訳システム(AIによる自然言語の翻訳)は変換器と分ける。変換器は LLM に依存しない(`docs/DESIGN.md` 末尾) / The translation system is separate from the converters; converters do not depend on an LLM.

## PHP 変換の実装メモ(2026-11-18)
- `src/php.rs`(ルート・本体・Eloquent・クエリビルダ)、`src/blade.rs`(Blade → Jinja2)、`src/middleware.rs`(ミドルウェア → `Depends`)。変換できない文が1つでもあれば、その本体・テンプレートの該当箇所は TODO として残してレポートに出す。
- 通し試験は `tests/run-php.sh`(実 FastAPI・実 SQLite・実 Jinja2)。WSL から PowerShell 経由で `wsl -d Ubuntu -- bash <スクリプト>` で動かす(Git Bash はパスを書き換え、`$PATH` も展開される)。
- 日本語を含むファイルは Python に `-X utf8` を付けて書く(Windows の既定は cp932)。引用符が混ざる長い文字列はシェルの heredoc ではなく Write ツールでファイルにしてから使う。
- COBOL_to・Java_to の取り込み後の削除は、所有者の指示だが元に戻せない。取り込み内容を確認してから、最終確認を取って行う。
- paginate: `Query.paginate(n, request)` が `Page`(dict 継承、`__iter__` は `data`)を返す。`request` 引数は本文に `, request)` か `request.query_params` があれば自動で付く。
- 多言語ドキュメント: `docs/i18n/<言語>/`(en・zh-CN・zh-TW・ko・de・fr・es・ru・pt)に README・CLAUDE・PORTING をAI翻訳で置いた(2026-11-19、ネイティブ検証前)。日本語の原本を変えたら、各言語も同じ内容へ更新すること。
