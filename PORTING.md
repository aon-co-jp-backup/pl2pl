# pl2pl 引き継ぎ
(むかしの細かい記録は `docs/HISTORY-ja.md`)

## いまの状態(2026-11-19)
- v0.1.0 を公開済み。Windows・Linux・Mac のインストーラーと、CI のテストがあります。
- PHP → FastAPI は、ルート・Eloquent・クエリビルダ・Blade・ミドルウェア・リレーション・`@can` 系まで、本物の FastAPI で通し試験済みです。
- `pl2pl web` が PHP も見分けます。
- `pl2pl watch`: 新しい版を見つけ、テストし、直し、だめなら通知します。GitHub の毎晩・毎朝の実行でも新しい版でテストしています。
- `pl2pl ir`(試作): PHP・Python・Java(Spring・JAX-RS・Servlet・Spark)のルートを共通の形にして、JSON・OpenAPI・FastAPI のひな形を出します。IR は型つき引数・関数の本体・モデル・ミドルウェア・テンプレートも持ち、`pl2pl php` の出力は `src/ir_emit.rs` の共通の書き出しを通ります(出力は変更前と同一、通し試験 49 件)。
- 紹介・ダウンロードのページを公開: <https://easy-web.tokyo/pl2pl/>(VPS、静的)。easy-web.tokyo のトップにも紹介カードがあります。
- COBOL_to・Java_to は `converters/` に取り込み済みです。

## まだのこと(優先順)
1. **確認**: 実際のメールの着信(`version-control --test-notify`。この PC に SMTP の設定が無い)、version-control の `--loop` による自動修正、fastapi 最新版でのテスト(GitHub の毎晩の実行が確かめる)。
2. (済み・ただし薄い)IR に型つき引数・依存・本体(`Body`)・モデル(`Model`/`Relation`)・ミドルウェア・テンプレート(`Template`)をのせ、`pl2pl php` の出力は `ir_emit.rs` の共通出力を通す(出力は変更前と同一、単体40件・e2e 49件成功)。ただしテンプレートは変換済みの Jinja2 文字列を持つだけで、Blade のノード木ではない(ほかの言語・出力先へ回すなら木にする必要あり)。ルート抽出が取りこぼすなら tree-sitter を検討。
3. `watch` を本物の新しい版で試した: fastapi 0.165.0(starlette 1.10)で `tests/run-php.sh` が 49 件成功したので、registry の tested を 0.165 へ進めた。残り: laravel 13 / django 6.1 / spring-boot 4.1 の新版は、通し試験が実物のフレームワークを使わないため「未検証」のまま(実物を使う試験を作るまで報告のみ)。
4. 毎晩・毎朝の自動実行の登録(この PC では確認コマンドが動かず誤報になるので未登録。登録の例は `scripts/pl2pl-watch.cmd` の先頭)。
5. (済み)easy-web.tokyo のトップページに pl2pl の紹介カードを足し、本番(VPS の open-easy-web)へ反映しました。open-easy-web `95cb1e7`、旧バイナリは `server/target/release/open-easy-web-server.before-pl2pl-card-20261119`。
6. PHP(済み・限りあり): Blade の `@php` は代入と `++` `--` `+=` `-=` `*=` `/=` `.=` まで `{% set %}` に変換、`$attributes` は出力・`->merge`(値は文字列か変換できる式)・`->class([...])`(文字列の並び)・`->only`・`->except`、`morphToMany`(中間表に型列)、クラスで作る Blade の部品(`app/View/Components/*.php` のコンストラクタ引数を既定値と `_props` にし、`render()` の `view('x.y')` も追う)を実装した(通し試験 55 件、単体 41 件)。残り: クラス部品の公開メソッド(テンプレートの `{{ $メソッド() }}`)は TODO のまま(レポートに件数を出す)、`->class()` のキー付き(条件つき)、`@php` の if/foreach や関数呼び出し(TODO のまま)。
7. aruaru-llm との連携(済み): 2026-11-19 に aruaru-llm v0.2.5 を取り込み、`pl2pl llm setup` で実際に導入した。自動選択の 3B は重みが分割されていて 404 で失敗したため、失敗したら一段小さいモデルへ自動で下げる(固定指定のときは下げない)。qwen2.5-1.5b-instruct(約 2.9GB)で `GenerateQwen` が返ることを確認(CPU)。**`pl2pl php --explain`**(任意、`--explain-max N` 既定 3)は、レポートの TODO 項目ごとに AI の説明を「参考」として足す。実測: 1 件に約 2 分(CPU)、**説明の質は低く間違いも出る**(2 件中 1 件は的外れ)ので既定はオフ。起動し直した aruaru-llm はモデルが未選択(400)なので、説明の前に導入済みモデルを選び直す。**もっと大きいモデル(約 12GB)**: aruaru-llm のカタログの最大は 3B(約 6.2GB)で、3B・7B は重みが複数ファイルに分割されていて今の導入処理では取れない。分割重みへの対応(aruaru-llm 側)と、CPU の速度(1.5B でもこの遅さ)の見直しが先。Kokkos の GPU 実行、open-cuda / open-directx でのカーネル実行は未確認。 **GPU(2026-11-19 確認)**: GeForce GT 730(2GB、sm_35、ドライバ 475 = CUDA 11.4 相当)に、管理者権限なしで CUDA 11.8 の nvcc を micromamba(NVIDIA の conda チャンネル)で入れ、素の CUDA カーネルと `pl2pl gpu --target open-cuda` の出力(`vec_loop12.cu`)を実際の GPU で動かし、CPU と不一致 0 を確認した(`tests/gpu-check.ps1`)。落とし穴: MSVC 14.44 には `-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH` と `-allow-unsupported-compiler` が要る、PyPI には Windows 用の nvcc が無い、CUDA 12 系は sm_35 不可。**未確認のまま**: Kokkos の CUDA バックエンド、open-cuda(Rust 実装)・open-directx 上での実行。
8. 元の COBOL_to・Java_to の削除(**取り返しがつかないので、最後の確認を取ってから**)。
9. `docs/DESIGN.md` は持ち主が書く。`docs/LANGUAGES.md` は 2026-11-19 に中身を見直し、いまの状態(Java・COBOL は変換器あり)と食い違いはなかった(ただし知識ベースの一覧で、検索による最新性の確認はまだ)。


## テスト
`bash tests/run-all.sh`(WSL)/ `bash tests/run-php.sh`(PHP 通し)/ `cargo test --release`

## 再開用メッセージ
「F:\pl2pl を再開。PORTING.md の『まだのこと』を上から進める。書きかえられない所はこっそり消さず TODO で残す。実際に動かして確かめてから完了と報告する。返事は日本語のみ。区切りごとにコミットして push、VPS にも反映。」

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

- 2026-11-19 追記 / Update: Kokkos の CUDA 実行はこのPCでは不可(GT 730=Kepler は WSL の GPU 対象外・Kokkos 新版は sm_35 非対応)。CPU 版(OpenMP/Serial)と Windows の CUDA 11.8 直接実行は確認済み。別の新しい GPU の機械で試す。/ Kokkos CUDA cannot run on this PC (GT 730 Kepler is not supported by WSL GPU, and recent Kokkos drops sm_35). CPU backends and direct CUDA 11.8 on Windows are verified; try on a newer GPU machine. 大規模モデル側: 実3B(2分割)がCPUで動作(約0.6語/秒)。/ Large-model side: the real 3B (2 shards) runs on CPU (about 0.6 tokens/s).
