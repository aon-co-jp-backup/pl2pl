> 本文为 AI 翻译,尚未经母语者审校。请以仓库根目录的日文原文为准。

# pl2pl 交接
(过去的详细记录在 `docs/HISTORY-ja.md`,日文。)

## 当前状态(2026-11-19)
- 已发布 v0.1.0,有 Windows、Linux、Mac 安装程序和 CI 测试。
- PHP → FastAPI 已用真实 FastAPI 做过端到端测试(49 项):路由、Eloquent、查询构建器、Blade、中间件、关联关系。
- `pl2pl web` 现在也能自动识别 PHP 并转交。
- 新功能:`pl2pl watch` 发现新版本、测试、修复,实在不行就发通知。version-control 新增了 `--notify-file` 选项。
- COBOL_to 和 Java_to 已并入 `converters/`。

## 待办(按优先级)
1. **确认**:真实邮件是否送达(`version-control --test-notify`)、通过 version-control `--loop` 自动修复、用最新的 fastapi 测试(CI 每晚运行会验证)。
2. 用真正的新版本试 `watch`(laravel、symfony、spring-boot、flask、django、poem 目前仍算“未验证”)。
3. 登记每晚和每早的运行(`schtasks /Create` 等;这是所有者自己的环境设置,尚未执行)。
4. 试作共同的中间表示(IR),作为增加更多语言组合的基础。
5. PHP:用类编写的 Blade 组件、`$attributes`、`@php`、`morphToMany`。
6. 与 aruaru-llm 联动(`llm setup` 的实际安装未验证)。Kokkos 的 GPU 运行;在 open-cuda / open-directx 上运行内核。
7. 删除原来的 COBOL_to 和 Java_to(**无法撤销,须先得到最终确认**)。
8. `docs/DESIGN.md` 由所有者撰写。确认 `docs/LANGUAGES.md` 没有过时。

## 测试
`bash tests/run-all.sh`(WSL)/ `bash tests/run-php.sh`(PHP 端到端)/ `cargo test --release`

## 恢复工作用的消息
“继续 F:\pl2pl。从 PORTING.md 的‘待办’由上往下做。无法转换的地方不要悄悄删掉,留 TODO。真正运行确认后再报告完成。回复只用日语。每到一个段落就 push。”
