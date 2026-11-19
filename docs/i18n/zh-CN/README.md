> 本文为 AI 翻译,尚未经母语者审校。请以仓库根目录的日文原文为准。

# pl2pl

**pl2pl 可以把用一种编程语言或框架写的代码,转换成另一种。**
比如,把用 PHP(Laravel)做的网站,变成 Python(FastAPI)的网站。支持 Windows、Linux 和 Mac(v0.1.0,开发中)。

- 转换在你电脑上安装的应用(命令行工具)里进行。网站只提供说明书和下载。
- 遇到无法转换的地方,pl2pl 不会悄悄删掉,而是留下 `TODO`,并在报告里写明原因。
- 目标是让知名的语言和框架之间,尽可能多地互相转换。

## 现在能做什么
| 命令 | 作用 |
|---|---|
| `pl2pl php` | PHP(Laravel / Symfony)→ FastAPI。详情见 [PHP.md](../../PHP.md)(日文) |
| `pl2pl web` | 检查文件夹,自动识别 Java、COBOL、PHP、Flask/Django,并交给合适的转换器 |
| `pl2pl ir` | (试作)把 PHP、Python、Java 的路由变成共同的形式,再输出 JSON、OpenAPI 或 FastAPI 骨架 |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL(Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | 把 C/C++ 的循环改写成 OpenMP、Kokkos 或 GPU 版本(gpu 为试作) |
| `pl2pl cport` | 让 UNIX 的 C 代码在 Windows 和 Linux 上都能运行 |
| `pl2pl pyhpc` | 用 Numba 加快 Python 的计算 |
| `pl2pl watch` | 发现语言和框架的新版本,并自动跟上(见下文) |
| `pl2pl llm setup` | 按你的电脑配置安装 AI 助手(aruaru-llm)(实际安装尚未验证) |

## 自动跟上新版本
每天晚上和早上运行 `pl2pl watch`。
1. 检查语言和框架有没有新版本(清单在 `registry/ecosystem.toml`)。
2. 用新版本测试,通过就自动更新“已支持的版本”。
3. 没通过,就由 version-control(借助 AI)尝试修复。
4. 还是不行,就给已登记的邮箱和 LINE 发通知。

定时运行的脚本是 `scripts/pl2pl-watch.cmd`(Windows)和 `scripts/pl2pl-watch.sh`(Linux/Mac)。
注意:laravel、symfony、flask、django 等目前只会提醒“有新版本了”。用真正的新版本做测试,还有待完成。

## 使用示例
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
```

## 请注意
- pl2pl 很谨慎。看起来有风险的地方,会原样保留并说明原因。不保证速度。
- 目前只在小型示例上测试过,还没有在大型真实程序上试过。

## 接下来
- PHP:用类编写的 Blade 组件。
- 共同的中间表示(IR),一次性增加更多语言组合。
- 确认真实的邮件通知和 AI 自动修复。
- 在 GPU 上运行(open-cuda / open-directx)。

## 了解更多
[LANGUAGES.md](../../LANGUAGES.md)(语言清单) / [COMBINATIONS.md](../../COMBINATIONS.md)(组合) / [DESIGN.md](../../DESIGN.md)(设计,由所有者撰写) / [components/README.md](../../../components/README.md)(内置组件)。这些文档为日文。
