> 本文由 AI 翻译,尚未经过母语者校对,内容以仓库根目录的日文原文为准。

# pl2pl CLAUDE
- **定位**:语言、框架、操作系统之间的转换工具(`src/` 下的 Rust 命令行程序)。可转换的来源见 `docs/LANGUAGES.md`,组合见 `docs/COMBINATIONS.md`。设计规格由所有者逐步写入 `docs/DESIGN.md`,不要擅自替他们编写规格。
- **规则**:无法转换的部分保留为 `TODO` 并写入报告,绝不悄悄丢弃。说任务完成之前,先实际运行验证。判断要保守:可能有依赖或副作用时就不转换。
- **PHP**:`src/php.rs`、`src/blade.rs`、`src/middleware.rs`。端到端测试是 `tests/run-php.sh`(真实的 FastAPI、SQLite、Jinja2),单元测试用 `cargo test --release`。在 WSL 中用 `wsl -d Ubuntu -- bash <脚本文件>` 运行。
- **常见坑**:在 Windows 上写含日文的文件要用 Python 的 `-X utf8`;shell 脚本保持 LF 换行;Git Bash 会改写 `/mnt/...` 路径。
- **COBOL_to / Java_to**:目前由其他会话开发。之后再并入,只有在最终确认后才能删除原仓库,因为无法撤销。
- **翻译系统**(自然语言)与转换器分开。
- **文档语言**:根目录是日文,`docs/i18n/` 下另有九种语言(AI 翻译,尚未经母语者校对)。日文原文有改动时,请同步更新各语言版本。
