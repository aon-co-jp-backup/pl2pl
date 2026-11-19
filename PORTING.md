# pl2pl 引き継ぎ
(むかしの細かい記録は `docs/HISTORY-ja.md`)

## いまの状態(2026-11-19)
- v0.1.0 を公開済み。Windows・Linux・Mac のインストーラーと、CI のテストがあります。
- PHP → FastAPI は、ルート・Eloquent・クエリビルダ・Blade・ミドルウェア・リレーション・`@can` 系まで、本物の FastAPI で通し試験済みです。
- `pl2pl web` が PHP も見分けます。
- `pl2pl watch`: 新しい版を見つけ、テストし、直し、だめなら通知します。GitHub の毎晩・毎朝の実行でも新しい版でテストしています。
- `pl2pl ir`(試作): PHP・Python・Java(Spring・JAX-RS・Servlet・Spark)のルートを共通の形にして、JSON・OpenAPI・FastAPI のひな形を出します。出力は実際に Python で読めることを確認しました。
- 紹介・ダウンロードのページを公開: <https://easy-web.tokyo/pl2pl/>(VPS、静的)。
- COBOL_to・Java_to は `converters/` に取り込み済みです。

## まだのこと(優先順)
1. **確認**: 実際のメールの着信(`version-control --test-notify`。この PC に SMTP の設定が無い)、version-control の `--loop` による自動修正、fastapi 最新版でのテスト(GitHub の毎晩の実行が確かめる)。
2. IR に型と関数の本体をのせる。いまの FastAPI 変換(`pl2pl php`)を IR 経由にする。ルート抽出が取りこぼすようなら tree-sitter を検討。
3. `watch` を「本物の新しい版」で試す(laravel・symfony・spring-boot・flask・django・poem は未検証あつかい)。
4. 毎晩・毎朝の自動実行の登録(この PC では確認コマンドが動かず誤報になるので未登録。登録の例は `scripts/pl2pl-watch.cmd` の先頭)。
5. easy-web.tokyo のトップページに、pl2pl の紹介カードを足す。
6. PHP: クラスで作る Blade の部品・`$attributes`・`@php`・`morphToMany`。
7. aruaru-llm との連携(`llm setup` の実導入は未確認)。Kokkos の GPU 実行、open-cuda / open-directx でのカーネル実行。
8. 元の COBOL_to・Java_to の削除(**取り返しがつかないので、最後の確認を取ってから**)。
9. `docs/DESIGN.md` は持ち主が書く。`docs/LANGUAGES.md` が古くないか確認。
10. 英語以外の8言語の文書を、日本語・英語の新しい内容に合わせる。

## テスト
`bash tests/run-all.sh`(WSL)/ `bash tests/run-php.sh`(PHP 通し)/ `cargo test --release`

## 再開用メッセージ
「F:\pl2pl を再開。PORTING.md の『まだのこと』を上から進める。書きかえられない所はこっそり消さず TODO で残す。実際に動かして確かめてから完了と報告する。返事は日本語のみ。区切りごとにコミットして push、VPS にも反映。」
