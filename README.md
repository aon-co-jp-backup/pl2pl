# pl2pl

**Languages**: 日本語(このファイル) / [English](docs/i18n/en/README.md) / [简体中文](docs/i18n/zh-CN/README.md) / [繁體中文](docs/i18n/zh-TW/README.md) / [한국어](docs/i18n/ko/README.md) / [Deutsch](docs/i18n/de/README.md) / [Français](docs/i18n/fr/README.md) / [Español](docs/i18n/es/README.md) / [Русский](docs/i18n/ru/README.md) / [Português](docs/i18n/pt/README.md)
(AI翻訳。ネイティブ検証前。CLAUDE.md・PORTING.md も同じフォルダにあります)

プログラミング言語(programming language)から プログラミング言語へ、言語・フレームワーク・動作OSをまたいで変換する**統合ツール**です(v0.1.0 公開済み、開発中)。
Windows(x64/ARM64)・Linux(amd64/arm64)・macOS(arm64/x64)用のインストーラーがあります。変換はインストール型のPCアプリ(CLI)で行い、WEB版は説明書とダウンロードだけです。
設計・仕様は少しずつ書いています(`docs/DESIGN.md`)。変換は保守的に行い、**変換できない箇所は黙って落とさず `TODO` として残し、レポートに理由つきで出します**。

## 統合・融合の候補
| リポジトリ | 入力 → 出力 | 状態 |
|---|---|---|
| [COBOL_to](https://github.com/aon-co-jp/COBOL_to) | COBOL → FastAPI / MFastAPI / Poem / RPoem | 実装あり(別セッションで開発中) |
| [Java_to](https://github.com/aon-co-jp/Java_to) | Java(Spring・JAX-RS・Servlet・Spark など)→ FastAPI / MFastAPI / Poem / RPoem | 開発中 |
| [2FastAPI](https://github.com/aon-co-jp/2FastAPI) / 2FastAPI-VSC | Flask / Django → FastAPI | 実装あり |
| [2MFastAPI](https://github.com/aon-co-jp/2MFastAPI) | Flask / Django / FastAPI → MFastAPI | 実装あり |
| [2Poem](https://github.com/aon-co-jp/2Poem) | Flask / Django / FastAPI → Poem / RPoem | 実装あり |
| [MFastAPI](https://github.com/aon-co-jp/MFastAPI) / [RMojo](https://github.com/aon-co-jp/RMojo) / [RPoem](https://github.com/aon-co-jp/RPoem) | 変換先の実行基盤 | 実装あり |

共通の流れ: 入力の自動判定 → ルート・型の抽出 → 出力先(fastapi / mfastapi / rust-poem / rpoem)→ 変換レポート → 手作業が要る箇所は `TODO` で残す。
COBOL_to・Java_to は `converters/` に取り込み済み(2026-11-19、履歴つき)です。元のリポジトリは、所有者の指示で最終的に削除する予定ですが、削除は元に戻せないため事前に確認します(まだ削除していません)。

## 方針案(未確定)
- 共通の中間表現(IR: ルート・型・関数本体)を作り、各 `X_to` を「入力側フロントエンド」、出力を「バックエンド」に分ける(N×M を N+M に)。
- 既存リポジトリは当面そのまま残し、pl2pl が成熟してから取り込む(破壊的な統合はしない)。
- 翻訳システム(AIによる自然言語の翻訳)は変換器と分ける。変換器は翻訳にも LLM にも依存しない。

## 変換元の候補一覧
言語・フレームワーク・DB・画面側を網羅した一覧は[`docs/LANGUAGES.md`](docs/LANGUAGES.md)。組み合わせ表と優先フレームワークは[`docs/COMBINATIONS.md`](docs/COMBINATIONS.md)。

## 実装済みのコマンド(`src/`)
| コマンド | 内容 | 検証 |
|---|---|---|
| `pl2pl cobol` | COBOL(IBM)+JCL → GnuCOBOL(Linux)+bash。DD名→`DD_xxx`、非互換の検出 | GnuCOBOL で実際にビルド・実行(JCL 経由) |
| `pl2pl omp` | C/C++ の独立ループへ `#pragma omp parallel for`(和の縮約つき) | 元のコードと同じ結果 |
| `pl2pl php` | Laravel / Symfony → FastAPI。詳細は下の「PHP 変換の対応範囲」 | 実 FastAPI・実 SQLite・実 Jinja2 で通し試験 |
| `pl2pl web` | Java / COBOL / Flask・Django を判定し java_to・cobol_to・2fastapi・2poem へ振り分け | 実機(Java 7ファイル・COBOL 1ファイルを実際に変換、終了コード0) |
| `pl2pl cport` | UNIX の C → 互換ヘッダー+機械置換+移植の指針(fork・socket 等は報告) | gcc でビルド・実行 |
| `pl2pl pyhpc` | Python の数値関数へ Numba `@njit`、`--mpi` で mpi4py 雛形 | Numba 実行で元と同一結果(`tests/numba-check.sh`) |
| `pl2pl kokkos` | 独立ループ → Kokkos(`parallel_for` / 和の `parallel_reduce`) | Kokkos 4.5.01(Serial)で実ビルド、同一出力(`tests/kokkos-check.sh`) |
| `pl2pl gpu` | (試作)独立ループ → CUDA 形式 `.cu` / HLSL `.hlsl` | 出力のみ(open-cuda / open-directx 上の実行は未確認) |
| `pl2pl llm setup` / `llm start` | 同梱の aruaru-llm を起動し、空きメモリに合うモデル(Qwen2.5)を自動選択して導入 | 選択ロジックと --dry-run のみ(実導入は未確認) |
| `pl2pl doctor` | open-cpu(内蔵)の CPU 情報 | 実行確認 |

CI(`.github/workflows/test.yml`)が push ごとに単体テストと PHP の通し試験を Linux x64・Linux arm64・macOS で実行する。

同梱コンポーネントは [`components/README.md`](components/README.md)。試験は `bash tests/run-all.sh`(WSL)、PHP の通し試験は `bash tests/run-php.sh`。

## PHP 変換の対応範囲(`pl2pl php`)
- **ルート**: Laravel の `Route::get/post/put/patch/delete/any/resource/apiResource/view`、`prefix` グループ、Symfony の `#[Route]`。
- **本体**: クロージャとコントローラの単純な `return`、複数文(取得 → 更新・保存・削除 → 返却)、`response()->json($x, 201)`、`noContent()`。
- **Eloquent → SQLAlchemy**: `db.py`・`models.py` を生成。列はマイグレーションの `Schema::create` か `$fillable` から取り、無い項目は `extra`(JSON)へ入れる。接続先は環境変数 `DATABASE_URL`(既定は SQLite)。
- **クエリビルダ**: `where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip` と、`paginate / simplePaginate / cursorPaginate`(cursor は件数位置ベースの近似、`?page=N`、Laravel と同じ JSON の形、Blade の `$users->links()`・`->total()`)、`$request->query('k', 既定値)` と、`get / first / firstOrFail / count / exists / delete`。`orWhere` は左から順に評価(Laravel の括弧グループとは異なる)。
- **Blade → Jinja2**(`templates/`): 出力・`@if`・`@foreach`・`@forelse`・`@for`・`@extends/@section/@yield/@include`・`@push/@stack`・`@auth/@guest`(Authorization ヘッダーの有無による簡易判定)・`@switch/@case`・`@checked/@selected/@disabled`・`@json`・`@props` と無名コンポーネント `<x-name attr="v">本文</x-name>`(名前付きスロット対応、`components/` のテンプレートを include)など。同じスタックへの `@push` は1回だけ。`view()`・`compact`・`->with`・`Route::view` は `TemplateResponse` へ。
- **ミドルウェア → `Depends`**(`middleware.py`): `auth`(Bearer と環境変数 `API_TOKENS` の簡易判定。**本物の認証へ差し替えること**)、`guest`、`throttle:N,M`(メモリ上の簡易版)、独自ミドルウェアの「条件で拒否 → `return $next($request)`」の形と、`$r = $next($request); $r->header('K','V'); return $r;` の形(ヘッダー追加のみ。`Response` を直接返すルートには付かない)。
- **リレーション**: `hasMany` / `hasOne` / `belongsTo` / `belongsToMany`(中間表を自動生成)/ `morphMany` / `morphOne` / `morphTo`(型の列には持ち主のクラス名を保存。Laravel の完全修飾名とは異なる)。`$u->posts`、`$p->user`、`$u->posts()->create([...])`、`attach / detach / sync`、`with('posts')`(`with('posts.comments')` のネストも可)、`whereHas('posts', fn($q) => $q->where(...))`、`has('posts')`・`whereDoesntHave`・`doesntHave` に対応。外部キー列が無ければ追加する。
- **未対応(TODO として報告)**: クラスベースの Blade コンポーネント(`@component`)・`$attributes`・`@can` 系の認可ディレクティブ。

## 完成予定の機能(未了)
- PHP: `@can` 系の認可とクラスベースの Blade コンポーネント。
- COBOL_to・Java_to は `converters/` に取り込み済み(履歴つきのスナップショット、cobol_to 10件・java_to 15件のテストが通る)。残り: `pl2pl web` を同梱コピーへ切り替える、元リポジトリのその後の変更を取り込む、元リポジトリの削除(所有者の指示。削除は元に戻せないので事前に確認)。
- 共通の中間表現(サーバー側・DB/SQL・画面側)。
- aruaru-llm との連携(`llm setup` の実導入は未確認)。
- Kokkos の GPU 実行・`Kokkos::View`、open-cuda / open-directx 上のカーネル実行。
- `docs/LANGUAGES.md` から選んだ言語の組み合わせの追加(全組み合わせは作らない)。

## 注意
- 判定は保守的です。依存や副作用の恐れがあるものは**付けずに理由を報告**し、性能は保証しません。
- 試験した範囲は小さな自作サンプルです。実際の大きなプログラムでは試していません。
