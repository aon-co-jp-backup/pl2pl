> 由 AI 翻译,尚未经母语者校对。以日文原文(仓库根目录)为准。

# pl2pl 交接
## 当前进度(2026-11-19)
v0.1.0 已发布(Windows/Linux/macOS 安装包、CI 端到端测试)。PHP 转换已覆盖路由、多语句函数体、Eloquent(SQLAlchemy)、含分页的查询构造器、关联、Blade(Jinja2)和中间件(`Depends`),均已端到端测试。Numba、Kokkos(Serial)和 `pl2pl web` 已实际运行验证。
## 未完成事项(按优先级)
1. PHP:嵌套 `with`、`morph`、`cursorPaginate`、Blade 组件/栈/授权、`$next` 之后处理响应的中间件。
2. 将 COBOL_to / Java_to 并入 pl2pl,然后删除原仓库(须先确认;不可撤销;其他会话正在开发)。
3. 通用 IR 的原型。
4. 与 aruaru-llm 联动(`llm setup` 的实际安装未验证)。
5. Kokkos GPU / `Kokkos::View`;open-cuda / open-directx 上的内核。
6. 所有者撰写 `docs/DESIGN.md`;核实 `docs/LANGUAGES.md` 是否最新。
7. 在 Linux arm64 与 macOS 的 CI 中运行新的 PHP 示例。
## 测试
`bash tests/run-all.sh`(WSL)、`bash tests/run-php.sh`(PHP 端到端)、`cargo test --release`。
## 恢复用消息
"继续 F:\pl2pl。从 PORTING.md 的第 1 项开始。无法转换的部分不要悄悄丢弃,保留 TODO。实际运行验证后再报告完成。push 前先确认。"
