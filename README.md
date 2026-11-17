# pl2pl

**日本語**: プログラミング言語(programming language)から プログラミング言語へ、言語・フレームワークをまたいで変換する**統合ツール**の構想リポジトリです。
設計・仕様は少しずつ書きます(`docs/DESIGN.md`)。現時点でコードはありません。

**English**: A planning repository for one unified "programming language to programming language" converter across languages and frameworks.
Design and specification will be written gradually (`docs/DESIGN.md`). There is no code yet.

## 統合・融合の候補 / Candidates to merge or fuse
| リポジトリ / Repo | 入力 → 出力 / Input → Output | 状態 / Status |
|---|---|---|
| [COBOL_to](https://github.com/aon-co-jp/COBOL_to) | COBOL → FastAPI / MFastAPI / Poem / RPoem | 実装あり / implemented |
| Java_to | Java (Spring, JAX-RS, Servlet, Spark…) → FastAPI / MFastAPI / Poem / RPoem | 開発中 / in progress |
| [2FastAPI](https://github.com/aon-co-jp/2FastAPI) / 2FastAPI-VSC | Flask / Django → FastAPI | 実装あり |
| [2MFastAPI](https://github.com/aon-co-jp/2MFastAPI) | Flask / Django / FastAPI → MFastAPI | 実装あり |
| [2Poem](https://github.com/aon-co-jp/2Poem) | Flask / Django / FastAPI → Poem / RPoem | 実装あり |
| [MFastAPI](https://github.com/aon-co-jp/MFastAPI) / [RMojo](https://github.com/aon-co-jp/RMojo) / [RPoem](https://github.com/aon-co-jp/RPoem) | 変換先の実行基盤 / target runtimes | 実装あり |

共通点: 入力の自動判定 → ルート/型の抽出 → 4種の出力先(fastapi / mfastapi / rust-poem / rpoem) → 変換レポート(`*-REPORT.md`) → 手作業箇所は黙って落とさず `TODO` で残す。
Common shape: detect input → extract routes/types → four targets → conversion report → never drop untranslatable code silently.

## 方針案 / Draft direction (未確定 / undecided)
- 共通の中間表現(IR: ルート・型・関数本体)を作り、各 `X_to` を「入力側フロントエンド」、4種の出力を「バックエンド」として分ける(N×M を N+M に)。
- 既存リポジトリは当面そのまま残し、pl2pl が成熟してから取り込む(破壊的な統合はしない)。

## 変換元の候補一覧 / Source candidates
言語・フレームワーク・DB・画面側を網羅した一覧は[`docs/LANGUAGES.md`](docs/LANGUAGES.md)。最初に変換する言語は選定中(案: PHP、次点 Go / Kotlin、DBは別線で SQL 方言 → aruaru-db)。
The full list (languages, frameworks, databases, UI) is in [`docs/LANGUAGES.md`](docs/LANGUAGES.md). The first language is still being chosen (suggestion: PHP; runners-up Go / Kotlin; databases as a separate line, SQL dialects → aruaru-db).

組み合わせ表と優先フレームワークは[`docs/COMBINATIONS.md`](docs/COMBINATIONS.md)。 / OS combinations and priority frameworks: [`docs/COMBINATIONS.md`](docs/COMBINATIONS.md).

## 実装済みのコマンド(2026-11-18、`src/`) / Implemented commands
| コマンド | 内容 | 検証 |
|---|---|---|
| `pl2pl cobol` | COBOL(IBM)+JCL → GnuCOBOL(Linux)+bash。DD名→`DD_xxx`、非互換の検出 | GnuCOBOL で実際にビルド・実行(JCL 経由) |
| `pl2pl omp` | C/C++ の独立ループへ `#pragma omp parallel for`(和の縮約つき) | 元のコードと同じ結果 |
| `pl2pl php` | Laravel / Symfony のルート → FastAPI の骨組み(本体は TODO) | TestClient で実行 |
| `pl2pl web` | Java / COBOL / Flask・Django を判定し java_to・cobol_to・2fastapi・2poem へ振り分け | 単体試験(振り分け計画) |
| `pl2pl cport` | UNIX の C → 互換ヘッダー+機械置換+移植の指針(fork・socket 等は報告) | gcc でビルド・実行 |
| `pl2pl pyhpc` | Python の数値関数へ Numba `@njit`、`--mpi` で mpi4py 雛形 | 単体試験(Numba 実行は確認中) |
| `pl2pl kokkos` | 独立ループ → Kokkos(`parallel_for` / 和の `parallel_reduce`) | 出力のみ(Kokkos 未導入) |
| `pl2pl gpu` | (試作)独立ループ → CUDA 形式 `.cu` / HLSL `.hlsl` | 出力のみ(open-cuda/open-directx 未実行) |
| `pl2pl doctor` | open-cpu(内蔵)の CPU 情報 | 実行確認 |
同梱コンポーネントは [`components/README.md`](components/README.md)。試験は `bash tests/run-all.sh`(WSL)。
Implemented commands above; bundled components in `components/README.md`; tests via `bash tests/run-all.sh` (WSL).
