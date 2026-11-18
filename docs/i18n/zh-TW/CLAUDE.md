> 由 AI 翻譯,尚未經母語者校對。以日文原文(儲存庫根目錄)為準。

# pl2pl 開發備忘
- 角色:語言/框架/OS 之間的轉換工具(Rust CLI,`src/`)。轉換來源候選見 `docs/LANGUAGES.md`,組合見 `docs/COMBINATIONS.md`。設計規格由擁有者逐步寫在 `docs/DESIGN.md`,切勿擅自代寫規格。
- 規則:無法轉換的部分保留為 `TODO` 並寫入報告,絕不悄悄丟棄。回報完成前必須實際執行驗證。判斷保守(可能有相依或副作用時不轉換)。
- PHP:`src/php.rs`、`src/blade.rs`、`src/middleware.rs`;端對端測試 `tests/run-php.sh`(真實 FastAPI、SQLite、Jinja2),單元測試 `cargo test --release`。在 WSL 中執行:`wsl -d Ubuntu -- bash <腳本檔>`。
- 注意事項:在 Windows 上寫含日文的檔案須用 Python `-X utf8`;shell 腳本保持 LF;Git Bash 會改寫 `/mnt/...` 路徑。
- COBOL_to 與 Java_to 由其他工作階段開發;之後併入,最終確認後才刪除原儲存庫(無法復原)。
- 翻譯系統(自然語言)與轉換器分離。
- 文件語言:日文(根目錄)加 `docs/i18n/` 中的 9 種語言(AI 翻譯,尚未經母語者校對)。
