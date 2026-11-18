> 本文由 AI 翻譯,尚未經母語人士校對,內容以儲存庫根目錄的日文原文為準。

# pl2pl 交接說明
## 目前進度(2026-11-19)
v0.1.0 已發布,提供 Windows、Linux、macOS 安裝程式,CI 中有端對端測試。PHP 轉換已涵蓋路由、多行函式本體、Eloquent(SQLAlchemy)、含分頁的查詢建構器、模型關聯(含巢狀 `with`)、Blade(Jinja2)與中介層(`Depends`),皆已做端對端測試。Numba、Kokkos(Serial)與 `pl2pl web` 都已實際執行確認。
## 待辦事項(依優先順序)
1. PHP:`morph`、Blade 的元件/堆疊/授權、在 `$next` 之後修改本文的中介層。
2. 將 COBOL_to / Java_to 併入 pl2pl,再刪除原儲存庫(須先確認;無法復原;其他工作階段正在開發)。
3. 製作統一中介表示的原型。
4. 串接 aruaru-llm(`llm setup` 的實際安裝尚未驗證)。
5. Kokkos 的 GPU / `Kokkos::View`;open-cuda / open-directx 上的 kernel。
6. 由擁有者撰寫 `docs/DESIGN.md`;確認 `docs/LANGUAGES.md` 是否為最新。
7. 在 Linux arm64 與 macOS 的 CI 中執行新增的 PHP 範例。
## 測試
`bash tests/run-all.sh`(WSL)、`bash tests/run-php.sh`(PHP 端對端)、`cargo test --release`。
## 恢復工作用的訊息
「繼續 F:\pl2pl。從 PORTING.md 的第 1 項開始。無法轉換的部分不要默默丟掉,留下 TODO。先實際執行驗證,再回報完成。push 之前先問我。」
