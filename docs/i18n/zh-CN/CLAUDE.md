> 由 AI 翻译,尚未经母语者校对。以日文原文(仓库根目录)为准。

# pl2pl 开发备忘
- 角色:语言/框架/OS 之间的转换工具(Rust CLI,`src/`)。转换源候选见 `docs/LANGUAGES.md`,组合见 `docs/COMBINATIONS.md`。设计规格由所有者逐步写在 `docs/DESIGN.md`,切勿擅自代写规格。
- 规则:无法转换的部分保留为 `TODO` 并写入报告,绝不悄悄丢弃。报告完成前必须实际运行验证。判断保守(可能有依赖或副作用时不转换)。
- PHP:`src/php.rs`、`src/blade.rs`、`src/middleware.rs`;端到端测试 `tests/run-php.sh`(真实 FastAPI、SQLite、Jinja2),单元测试 `cargo test --release`。在 WSL 中运行:`wsl -d Ubuntu -- bash <脚本文件>`。
- 注意事项:在 Windows 上写含日文的文件须用 Python `-X utf8`;shell 脚本保持 LF;Git Bash 会改写 `/mnt/...` 路径。
- COBOL_to 与 Java_to 由其他会话开发;之后并入,最终确认后才删除原仓库(不可撤销)。
- 翻译系统(自然语言)与转换器分离。
- 文档语言:日文(根目录)加 `docs/i18n/` 中的 9 种语言(AI 翻译,尚未经母语者校对)。
