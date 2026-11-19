# pl2pl 開発メモ
- **役割**: 言語・フレームワーク・OS をまたぐ変換ツール(Rust 製 CLI、`src/`)。変換元の一覧は `docs/LANGUAGES.md`、組み合わせは `docs/COMBINATIONS.md`。設計は所有者が `docs/DESIGN.md` に少しずつ書く。**勝手に仕様を決めて書かない。**
- **方針**: 有名な言語・フレームワークどうしの多対多の変換を想定する。全部は作らず、選んだものから実装する。WEB版は説明とダウンロードだけで、変換は PC のアプリで行う。
- **決まり**: 変換できない所は `TODO` で残して報告する(黙って消さない)。判定は保守的(依存や副作用が心配なら変えない)。実際に動かしてから「完了」と言う。新しい REST は作らない。返答は日本語のみ。
- **PHP**: `src/php.rs`・`src/blade.rs`・`src/middleware.rs`。通し試験は `tests/run-php.sh`(実 FastAPI・SQLite・Jinja2)、単体は `cargo test --release`。WSL では `wsl -d Ubuntu -- bash <ファイル>`。
- **自動追従**: `pl2pl watch`(`src/watch.rs`、`registry/ecosystem.toml`)。確認コマンドが空の項目は「未検証」と表示するだけで、勝手に更新しない。通知は version-control の `--notify-file`。
- **落とし穴**: 日本語を含むファイルは Python に `-X utf8` で書く。シェルスクリプトは LF。Git Bash は `/mnt/...` を書き換える。引用符の多い文字列は heredoc でなく Write ツールで。
- **COBOL_to / Java_to**: `converters/` に取り込み済み。元リポジトリの削除は元に戻せないので、最終確認を取ってから。
- **翻訳システム**(自然言語)は変換器と分ける。変換器は LLM に依存しない。
- **文書の言語**: 原本は日本語、`docs/i18n/` に9言語(AI翻訳、ネイティブ確認前)。原本を変えたら翻訳も直す。過去の経緯は `docs/HISTORY-ja.md`。
