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
