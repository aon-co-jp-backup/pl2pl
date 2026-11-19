> 本文为 AI 翻译,尚未经母语者审校。请以仓库根目录的日文原文为准。

# pl2pl 开发笔记
- **定位**:跨语言、框架、操作系统的转换工具(Rust 命令行,`src/`)。源语言清单见 `docs/LANGUAGES.md`,组合见 `docs/COMBINATIONS.md`。设计由所有者在 `docs/DESIGN.md` 中逐步撰写。**不要擅自替他们定规格。**
- **方向**:面向知名语言和框架之间的多对多转换。不一次做完,先做选定的。网站只提供说明书和下载,转换在 PC 应用中进行。
- **规则**:无法转换的地方留 `TODO` 并报告(不要悄悄删掉)。判断要保守(可能有依赖或副作用就不转换)。真正运行过再说“完成”。不新增 REST。回复只用日语。
- **PHP**:`src/php.rs`、`src/blade.rs`、`src/middleware.rs`。端到端测试 `tests/run-php.sh`(真实 FastAPI、SQLite、Jinja2),单元测试 `cargo test --release`。WSL 中用 `wsl -d Ubuntu -- bash <文件>`。
- **自动跟进**:`pl2pl watch`(`src/watch.rs`、`registry/ecosystem.toml`)。验证命令为空的条目只显示“未验证”,绝不自动更新。通知通过 version-control 的 `--notify-file`。
- **常见坑**:含日文的文件用 Python `-X utf8` 写入;shell 脚本保持 LF;Git Bash 会改写 `/mnt/...` 路径;引号很多的字符串用 Write 工具,不要用 heredoc。
- **COBOL_to / Java_to**:已并入 `converters/`。删除原仓库无法撤销,须先得到最终确认。
- **翻译系统**(自然语言)与转换器分开。转换器不依赖 LLM。
- **文档语言**:原文为日语,`docs/i18n/` 中有 9 种语言(AI 翻译,未经母语者审校)。原文修改后,翻译也要同步。过往经过见 `docs/HISTORY-ja.md`。
