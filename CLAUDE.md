# pl2pl 開発メモ
- **やくわり**: 言葉(言語)・フレームワーク・OS をまたいで書きかえる道具(Rust 製、`src/`)。元になる言葉の一覧は `docs/LANGUAGES.md`、組み合わせは `docs/COMBINATIONS.md`。設計は持ち主が `docs/DESIGN.md` に少しずつ書く。**勝手に仕様を決めて書かない。**
- **ほうこう**: 有名な言葉どうしの「多対多」の書きかえを目ざす。全部を一度には作らず、選んだものから。WEB は説明とダウンロードだけで、書きかえは PC のアプリで行う。
- **きまり**: 書きかえられない所は `TODO` で残して報告する(こっそり消さない)。判定は慎重に(依存や副作用が心配なら変えない)。実際に動かしてから「できた」と言う。新しい REST は作らない。返事は日本語のみ。
- **共通の形(IR)**: `src/ir.rs`(型・本体・モデル・ミドルウェア・テンプレート)と、書き出し側の `src/ir_emit.rs`。入力側(PHP・Python・Java)と出力側(JSON・OpenAPI・FastAPI)を分けて、組み合わせを増やしやすくする。`pl2pl php` もこの共通の書き出しを通る(出力を変えない変更は、前後の出力の diff が空であることで確かめる)。ルートを見つけられなかったファイルは報告する。出力は `py_compile` などで実際に確かめる。
- **PHP**: `src/php.rs`・`src/blade.rs`・`src/middleware.rs`。通し試験は `tests/run-php.sh`(本物の FastAPI・SQLite・Jinja2)、単体は `cargo test --release`。WSL では `wsl -d Ubuntu -- bash <ファイル>`。
- **自動追従**: `pl2pl watch`(`src/watch.rs`、`registry/ecosystem.toml`)。確認コマンドが空の項目は「未検証」と出すだけで、勝手に更新しない。通知は version-control の `--notify-file`。
- **配備**: 紹介ページは `webpage/index.html`。VPS は `/root/easy-web.tokyo/pl2pl`(`git pull` で更新、静的なので再起動は不要)、`pl2pl-web.service` ポート 8142、`domains.toml` に追記済み。
- **落とし穴**: 日本語を含むファイルは Python に `-X utf8` で書く。シェルスクリプトは LF。Git Bash は `/mnt/...` を書きかえる。引用符や `\` の多い文字列は、heredoc でなく Write / Edit ツールで。
- **COBOL_to / Java_to**: `converters/` に取り込み済み。元のリポジトリを消すのは取り返しがつかないので、最後の確認を取ってから。
- **翻訳システム**(自然言語)は変換器と分ける。変換器は LLM に頼らない。
- **文書の言葉**: 原本は日本語。英語はやさしい言葉で書き直した。ほかの8言語(`docs/i18n/`)も、同じ内容に合わせて AI 翻訳した(ネイティブ確認前)。過去の経緯は `docs/HISTORY-ja.md`。

## 2026-11-19 いまの状況と、これから作るもの / Where we are now, and what comes next

### 日本語(やさしく)
**いまできること**: pl2pl は、PHP(Laravel など)や Python や Java で書かれたウェブアプリを、Python の FastAPI に作り直すお助けロボットです。道順(ルート)、データの入れ物(モデル)、見た目の部品(Blade の画面)、見張り番(ミドルウェア)まで変えられます。通し試験は 55 項目すべて合格です。
**新しく直したところ**: 「新しい版が出たら自動で確かめる」機能(`pl2pl watch`)が、古い道具のまま試していたので、合格しても本当の確認になっていませんでした。いまは、毎回**まっさらな部屋(新しい venv)に最新版を入れてから**試験します。fastapi 0.165 で本当に通ることを確かめました。Windows では WSL の中で動かすので、まちがった警報も出ません。
**これからやること(確認はいりません。自動で順に進めます)**:
1. 毎晩・毎朝の自動実行の登録(まちがい警報が出ないことを確かめてから)
2. メールが本当に届くかの確認(このPCにはメール送信の設定がありません)
3. aruaru-llm・Kokkos・open-cuda・open-directx での GPU 実行の確認
4. PHP の細かい残り(クラス部品のメソッドなど)
5. 8 か国語の文書を最新に合わせる
**人に聞くこと**: 元の COBOL_to / Java_to を消す作業だけは、元に戻せないので、最後に必ずあなたに聞きます。
**ルール**: 「未了(まだ終わっていないこと)は確認せず、自動で進める」と決まっています。

### English (simple words, easy to read)
**What it can do now**: pl2pl is a helper robot. It rebuilds web apps written in PHP (like Laravel), Python or Java into Python FastAPI. It can change routes, data models, Blade screens and middleware. All 55 end-to-end checks pass.
**What we fixed today**: The "check new versions by itself" feature (`pl2pl watch`) used old tools for its tests. So a pass did not really prove anything. Now every check starts in a **clean new room (a fresh venv), installs the newest version, and then runs the tests**. We confirmed it really passes on fastapi 0.165. On Windows it runs inside WSL, so false alarms are gone.
**What comes next (no need to ask; we go on by ourselves, in order)**:
1. Register the nightly and morning runs (after we are sure there are no false alarms)
2. Check that e-mail really arrives (this PC has no mail settings)
3. Check GPU runs with aruaru-llm, Kokkos, open-cuda and open-directx
4. Small PHP leftovers (class-component methods, and so on)
5. Bring the 8 other language docs up to date
**What we still ask a person**: deleting the original COBOL_to / Java_to cannot be undone, so we always ask you first.
**Rule**: "Unfinished work goes on automatically, without asking." This is the standing rule.

- 2026-11-19 追記 / Update: 夜 03:00・朝 07:00 の自動実行を登録し、手動実行で exit=0(誤警報なし)を確認。/ Nightly (03:00) and morning (07:00) runs are registered; a manual run ended with exit=0 and no false alarm. 次: メール到達・GPU(Kokkos等)・PHP残り。Next: mail arrival, GPU (Kokkos etc.), PHP leftovers.
