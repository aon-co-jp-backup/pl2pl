# Java_to 開発メモ
- Rust 製 CLI。`lexer.rs`(字句)→`parse.rs`(宣言レベルの構文)→`frameworks.rs`(台帳と判定)→`endpoints.rs`(ルート抽出)→`body.rs`(式・文の Python 翻訳)→`pygen.rs`(Python 出力)/`rsgen.rs`(Poem・RPoem 出力)。`java_rt.py` は生成物に同梱する実行時部品。
- 方針: 訳せないものは黙って落とさず TODO。REST は作らない(API 方針は runo/CLAUDE.md)。
- RPoem の出力形は 2Poem の emit.py に合わせた(`#[open_runo_poem_compat_macro::handler] async fn x(_req: Request, params: Params) -> Response`)。
- 試験は WSL: `wsl -d Ubuntu -- bash /mnt/f/Java_to/tests/run-all.sh`。`bash -c "…$PATH…"` はホスト側で展開されるのでスクリプトファイル経由で。
- 台帳(58件)は調査(日英の検索+GitHub)に基づく目安。公開年は概数で、未検証の項目がある。
