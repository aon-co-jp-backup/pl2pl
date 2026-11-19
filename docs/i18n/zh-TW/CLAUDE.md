> 說明:儲存庫根目錄的日文 CLAUDE.md 是原文。本文是 AI 翻譯,用簡單易懂的話寫成,尚未經母語人士審閱。

# pl2pl 開發筆記
- **任務**:在語言、框架、作業系統之間改寫程式碼的工具(用 Rust 撰寫,位於 `src/`)。來源語言清單見 `docs/LANGUAGES.md`,組合見 `docs/COMBINATIONS.md`。設計由擁有者在 `docs/DESIGN.md` 中一點一點地寫。**不要擅自替他們訂規格。**
- **方向**:在知名語言之間做「多對多」的改寫。不一次做完,先做選定的。網站只有說明和下載,改寫在 PC 應用程式裡進行。
- **規則**:改寫不了的地方留 `TODO` 並回報(不要偷偷刪掉)。要謹慎(可能有相依或副作用就不改)。真正執行過再說「完成」。不新增 REST。回覆只用日語。
- **共同的形式(IR)**:`src/ir.rs`。輸入端(PHP、Python、Java)和輸出端(JSON、OpenAPI、FastAPI)分開,方便增加新組合。沒找到路由的檔案要回報。輸出要真正檢查,例如用 `py_compile`。
- **PHP**:`src/php.rs`、`src/blade.rs`、`src/middleware.rs`。端對端測試:`tests/run-php.sh`(真實的 FastAPI、SQLite、Jinja2)。單元測試:`cargo test --release`。WSL 中:`wsl -d Ubuntu -- bash <檔案>`。
- **自動跟上**:`pl2pl watch`(`src/watch.rs`、`registry/ecosystem.toml`)。檢查指令為空的項目只顯示「未檢查」,絕不自動更新。通知透過 version-control 的 `--notify-file`。
- **部署**:介紹頁是 `webpage/index.html`。VPS 上是 `/root/easy-web.tokyo/pl2pl`(用 `git pull` 更新,是靜態頁面,無需重啟)、`pl2pl-web.service` 連接埠 8142,已寫入 `domains.toml`。
- **陷阱**:含日文的檔案用 Python `-X utf8` 寫入。shell 腳本保持 LF。Git Bash 會改寫 `/mnt/...` 路徑。引號或 `\` 很多的文字,用 Write / Edit 工具,不要用 heredoc。
- **COBOL_to / Java_to**:已併入 `converters/`。刪除原儲存庫無法復原,須先取得最終確認。
- **翻譯系統**(人類語言)與轉換器分開。轉換器不依賴 LLM。
- **文件語言**:日文檔案是原文。英文檔案用簡單的話重寫過。`docs/i18n/` 裡的其他語言是 AI 翻譯(尚未經母語人士審閱)。更早的經過見 `docs/HISTORY-ja.md`。
