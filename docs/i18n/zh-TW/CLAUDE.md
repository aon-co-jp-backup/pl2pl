> 本文為 AI 翻譯,尚未經母語人士審閱。請以儲存庫根目錄的日文原文為準。

# pl2pl 開發筆記
- **定位**:跨語言、框架、作業系統的轉換工具(Rust 命令列,`src/`)。來源語言清單見 `docs/LANGUAGES.md`,組合見 `docs/COMBINATIONS.md`。設計由擁有者在 `docs/DESIGN.md` 中逐步撰寫。**不要擅自替他們訂規格。**
- **方向**:面向知名語言和框架之間的多對多轉換。不一次做完,先做選定的。網站只提供說明書和下載,轉換在 PC 應用程式中進行。
- **規則**:無法轉換的地方留 `TODO` 並回報(不要默默刪掉)。判斷要保守(可能有相依或副作用就不轉換)。真正執行過再說「完成」。不新增 REST。回覆只用日語。
- **PHP**:`src/php.rs`、`src/blade.rs`、`src/middleware.rs`。端對端測試 `tests/run-php.sh`(真實 FastAPI、SQLite、Jinja2),單元測試 `cargo test --release`。WSL 中用 `wsl -d Ubuntu -- bash <檔案>`。
- **自動跟進**:`pl2pl watch`(`src/watch.rs`、`registry/ecosystem.toml`)。驗證指令為空的項目只顯示「未驗證」,絕不自動更新。通知透過 version-control 的 `--notify-file`。
- **常見陷阱**:含日文的檔案用 Python `-X utf8` 寫入;shell 腳本保持 LF;Git Bash 會改寫 `/mnt/...` 路徑;引號很多的字串用 Write 工具,不要用 heredoc。
- **COBOL_to / Java_to**:已併入 `converters/`。刪除原儲存庫無法復原,須先取得最終確認。
- **翻譯系統**(自然語言)與轉換器分開。轉換器不依賴 LLM。
- **文件語言**:原文為日語,`docs/i18n/` 中有 9 種語言(AI 翻譯,未經母語人士審閱)。原文修改後,翻譯也要同步。過往經過見 `docs/HISTORY-ja.md`。
