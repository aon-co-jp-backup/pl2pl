> 本文由 AI 翻譯,尚未經母語人士校對,內容以儲存庫根目錄的日文原文為準。

# pl2pl CLAUDE
- **定位**:語言、框架與作業系統之間的轉換工具(`src/` 下的 Rust 命令列程式)。可轉換的來源見 `docs/LANGUAGES.md`,組合見 `docs/COMBINATIONS.md`。設計規格由擁有者逐步寫入 `docs/DESIGN.md`,請勿擅自替他們撰寫規格。
- **規則**:無法轉換的部分保留為 `TODO` 並寫進報告,絕不默默丟棄。說任務完成之前,先實際執行驗證。判斷要保守:可能有相依性或副作用時就不轉換。
- **PHP**:`src/php.rs`、`src/blade.rs`、`src/middleware.rs`。端對端測試為 `tests/run-php.sh`(真實的 FastAPI、SQLite、Jinja2),單元測試用 `cargo test --release`。在 WSL 中以 `wsl -d Ubuntu -- bash <腳本檔>` 執行。
- **常見陷阱**:在 Windows 上寫入含日文的檔案要用 Python 的 `-X utf8`;shell 腳本維持 LF 換行;Git Bash 會改寫 `/mnt/...` 路徑。
- **COBOL_to / Java_to**:目前由其他工作階段開發。之後再併入,必須在最終確認後才能刪除原儲存庫,因為無法復原。
- **翻譯系統**(自然語言)與轉換器分開。
- **文件語言**:根目錄為日文,`docs/i18n/` 下另有九種語言(AI 翻譯,尚未經母語人士校對)。日文原文有變動時,請同步更新各語言版本。
