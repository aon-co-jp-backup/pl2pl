> 由 AI 翻譯,尚未經母語者校對。以日文原文(儲存庫根目錄)為準。

# pl2pl 交接
## 目前進度(2026-11-19)
v0.1.0 已發布(Windows/Linux/macOS 安裝程式、CI 端對端測試)。PHP 轉換已涵蓋路由、多敘述函式本體、Eloquent(SQLAlchemy)、含分頁的查詢建構器、關聯、Blade(Jinja2)與中介層(`Depends`),均已端對端測試。Numba、Kokkos(Serial)與 `pl2pl web` 已實際執行驗證。
## 未完成事項(依優先順序)
1. PHP:巢狀 `with`、`morph`、`cursorPaginate`、Blade 元件/堆疊/授權、`$next` 之後處理回應的中介層。
2. 將 COBOL_to / Java_to 併入 pl2pl,然後刪除原儲存庫(須先確認;無法復原;其他工作階段正在開發)。
3. 通用 IR 的原型。
4. 與 aruaru-llm 整合(`llm setup` 的實際安裝未驗證)。
5. Kokkos GPU / `Kokkos::View`;open-cuda / open-directx 上的核心。
6. 擁有者撰寫 `docs/DESIGN.md`;確認 `docs/LANGUAGES.md` 是否為最新。
7. 在 Linux arm64 與 macOS 的 CI 中執行新的 PHP 範例。
## 測試
`bash tests/run-all.sh`(WSL)、`bash tests/run-php.sh`(PHP 端對端)、`cargo test --release`。
## 恢復用訊息
「繼續 F:\pl2pl。從 PORTING.md 的第 1 項開始。無法轉換的部分不要悄悄丟棄,保留 TODO。實際執行驗證後再回報完成。push 前先確認。」
