# pl2pl 引き継ぎ
(過去の細かい記録は `docs/HISTORY-ja.md`)

## 今の状態(2026-11-19)
- v0.1.0 公開済み。Windows・Linux・Mac のインストーラーと、CI のテストがあります。
- PHP → FastAPI は、ルート・Eloquent・クエリビルダ・Blade・ミドルウェア・リレーションまで、本物の FastAPI で通し試験済み(49項目)。
- `pl2pl web` が PHP も自動で見分けて回すようになりました。
- 新機能 `pl2pl watch`: 新しい版を見つけ、テストし、直し、だめなら通知します。version-control に `--notify-file` を足しました。
- COBOL_to・Java_to は `converters/` に取り込み済みです。

## まだのこと(優先順)
1. **確認**: 実際のメールの着信(`version-control --test-notify`)、version-control の `--loop` による自動修正、fastapi 最新版でのテスト(CI の毎晩実行で確認される)。
2. `watch` の対象を「本物の新しい版」で試す(laravel・symfony・spring-boot・flask・django・poem はまだ未検証扱い)。
3. 毎晩・毎朝の登録(`schtasks /Create` など。本人の環境の設定なので未実行)。
4. 共通の中間表現(IR)の試作。言語×言語の組み合わせを増やす土台です。
5. PHP: `@can`・クラスで作る Blade 部品・`$attributes`・`@php`・`morphToMany`。
6. aruaru-llm との連携(`llm setup` の実導入は未確認)。Kokkos の GPU 実行、open-cuda / open-directx でのカーネル実行。
7. 元の COBOL_to・Java_to の削除(**元に戻せないので、最終確認を取ってから**)。
8. `docs/DESIGN.md` は所有者が書く。`docs/LANGUAGES.md` が古くないか確認。

## テスト
`bash tests/run-all.sh`(WSL)/ `bash tests/run-php.sh`(PHP 通し)/ `cargo test --release`

## 再開用メッセージ
「F:\pl2pl を再開。PORTING.md の『まだのこと』を上から進める。変換できない所は黙って消さず TODO で残す。実際に動かして確認してから完了と報告する。返答は日本語のみ。push は区切りごとに。」
