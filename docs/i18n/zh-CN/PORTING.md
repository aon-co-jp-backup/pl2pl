> 本文由 AI 翻译,尚未经过母语者校对,内容以仓库根目录的日文原文为准。

# pl2pl 交接说明
## 目前进展(2026-11-19)
v0.1.0 已发布,提供 Windows、Linux、macOS 安装包,CI 中有端到端测试。PHP 转换已覆盖路由、多语句函数体、Eloquent(SQLAlchemy)、带分页的查询构造器、模型关联(含嵌套 `with`)、Blade(Jinja2)和中间件(`Depends`),均已做端到端测试。Numba、Kokkos(Serial)和 `pl2pl web` 都已实际运行确认。
## 待办事项(按优先级)
1. PHP:`@can` 授权和基于类的 Blade 组件、在 `$next` 之后修改正文的中间件。
2. 把 COBOL_to / Java_to 并入 pl2pl,然后删除原仓库(须先确认;无法撤销;其他会话正在开发)。
3. 制作统一中间表示的原型。
4. 接入 aruaru-llm(`llm setup` 的真实安装尚未验证)。
5. Kokkos 的 GPU / `Kokkos::View`;open-cuda / open-directx 上的内核。
6. 由所有者撰写 `docs/DESIGN.md`;核对 `docs/LANGUAGES.md` 是否为最新。
7. 在 Linux arm64 和 macOS 的 CI 中运行新增的 PHP 示例。
## 测试
`bash tests/run-all.sh`(WSL)、`bash tests/run-php.sh`(PHP 端到端)、`cargo test --release`。
## 恢复工作用的消息
“继续 F:\pl2pl。从 PORTING.md 的第 1 项开始。无法转换的部分不要悄悄丢掉,留下 TODO。先实际运行验证,再汇报完成。push 之前先问我。”
