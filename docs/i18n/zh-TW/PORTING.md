> 本文為 AI 翻譯,尚未經母語人士審閱。請以儲存庫根目錄的日文原文為準。

# pl2pl 交接
(過去的詳細紀錄在 `docs/HISTORY-ja.md`,日文。)

## 目前狀態(2026-11-19)
- 已發布 v0.1.0,有 Windows、Linux、Mac 安裝程式和 CI 測試。
- PHP → FastAPI 已用真實 FastAPI 做過端對端測試(49 項):路由、Eloquent、查詢建構器、Blade、中介軟體、關聯。
- `pl2pl web` 現在也能自動辨識 PHP 並轉交。
- 新功能:`pl2pl watch` 發現新版本、測試、修復,實在不行就發通知。version-control 新增了 `--notify-file` 選項。
- COBOL_to 和 Java_to 已併入 `converters/`。

## 待辦(依優先順序)
1. **確認**:真實電子郵件是否送達(`version-control --test-notify`)、透過 version-control `--loop` 自動修復、用最新的 fastapi 測試(CI 每晚執行會驗證)。
2. 用真正的新版本試 `watch`(laravel、symfony、spring-boot、flask、django、poem 目前仍算「未驗證」)。
3. 登記每晚和每早的執行(`schtasks /Create` 等;這是擁有者自己的環境設定,尚未執行)。
4. 試作共同的中介表示(IR),作為增加更多語言組合的基礎。
5. PHP:用類別撰寫的 Blade 元件、`$attributes`、`@php`、`morphToMany`。
6. 與 aruaru-llm 整合(`llm setup` 的實際安裝未驗證)。Kokkos 的 GPU 執行;在 open-cuda / open-directx 上執行核心。
7. 刪除原本的 COBOL_to 和 Java_to(**無法復原,須先取得最終確認**)。
8. `docs/DESIGN.md` 由擁有者撰寫。確認 `docs/LANGUAGES.md` 沒有過時。

## 測試
`bash tests/run-all.sh`(WSL)/ `bash tests/run-php.sh`(PHP 端對端)/ `cargo test --release`

## 恢復工作用的訊息
「繼續 F:\pl2pl。從 PORTING.md 的『待辦』由上往下做。無法轉換的地方不要默默刪掉,留 TODO。真正執行確認後再回報完成。回覆只用日語。每到一個段落就 push。」
