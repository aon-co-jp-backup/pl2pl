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
6. PHP: クラスで作る Blade の部品・`$attributes`・`@php`・`morphToMany`。
7. aruaru-llm との連携(`llm setup` の実導入は未確認)。Kokkos の GPU 実行、open-cuda / open-directx でのカーネル実行。
8. 元の COBOL_to・Java_to の削除(**取り返しがつかないので、最後の確認を取ってから**)。
9. `docs/DESIGN.md` は持ち主が書く。`docs/LANGUAGES.md` は 2026-11-19 に中身を見直し、いまの状態(Java・COBOL は変換器あり)と食い違いはなかった(ただし知識ベースの一覧で、検索による最新性の確認はまだ)。


## テスト
`bash tests/run-all.sh`(WSL)/ `bash tests/run-php.sh`(PHP 通し)/ `cargo test --release`

## 再開用メッセージ
「F:\pl2pl を再開。PORTING.md の『まだのこと』を上から進める。書きかえられない所はこっそり消さず TODO で残す。実際に動かして確かめてから完了と報告する。返事は日本語のみ。区切りごとにコミットして push、VPS にも反映。」
