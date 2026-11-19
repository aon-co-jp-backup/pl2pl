> 说明:仓库根目录的日文 README 是原文。本文是 AI 翻译,用简单易懂的话写成,尚未经母语者审校。

# pl2pl

## pl2pl 是什么?
程序是用各种“语言”(编程语言)写成的。
**pl2pl 是把用一种语言写的程序,改写成另一种语言的工具。**
比如,可以把用 PHP(Laravel)做的网站,变成 Python(FastAPI)的网站。
支持 Windows、Linux 和 Mac(v0.1.0,还在开发中)。

- 使用时要安装到自己的电脑上。网站上只有说明和下载 → <https://easy-web.tokyo/pl2pl/>
- 改写不了的地方,pl2pl 不会偷偷删掉。它会留下 `TODO`(以后要做的事),并在报告里写明原因。
- 我们的目标是:让知名的语言之间,尽可能多地互相改写。

## 现在能做什么
| 命令 | 作用 |
|---|---|
| `pl2pl php` | PHP(Laravel / Symfony)→ FastAPI。详情见 [PHP.md](../../PHP.md)(日文) |
| `pl2pl web` | 查看文件夹,认出 Java、COBOL、PHP、Flask/Django,交给合适的工具 |
| `pl2pl ir` | (试作)把 PHP、Python、Java 的“路由”变成同一种形式,再生成 JSON、OpenAPI 或 FastAPI 的雏形 |
| `pl2pl watch` | 发现语言和工具的新版本,并自动跟上 |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL(Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | 让 C/C++ 的循环跑得更快(gpu 为试作) |
| `pl2pl cport` | 让 UNIX 用的 C 代码在 Windows 和 Linux 上都能运行 |
| `pl2pl pyhpc` | 用 Numba 让 Python 的计算更快 |
| `pl2pl llm setup` | 安装 AI 助手(aruaru-llm)。实际安装还没有验证 |

## 自动跟上新版本
每天晚上和早上运行 `pl2pl watch`。
1. 检查语言和工具有没有新版本(清单在 `registry/ecosystem.toml`)。
2. 用新版本测试。通过的话,就自动更新“已支持的版本”。
3. 没通过,就由 version-control(借助 AI)尝试修复。
4. 还是修不好,就给已登记的邮箱和 LINE 发通知。

请注意:laravel、symfony、flask、django 等目前只会提醒“有新版本了”。
在 GitHub 上,每天晚上和早上也会自动用最新版本做测试。

## 使用方法
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## 请知道
- pl2pl 很谨慎。看起来有风险的地方,会原样保留并说明原因。我们不保证速度。
- 目前只在小型示例上测试过,还没有在大型真实程序上试过。

## 接下来
- 把类型和函数的内容也放进共同的形式,让 FastAPI 的转换也经过它。
- PHP:用类编写的 Blade 组件。
- 确认真实的邮件通知和 AI 自动修复。
- 确认在 GPU 上运行(open-cuda / open-directx)。

## 了解更多
[LANGUAGES.md](../../LANGUAGES.md)(语言清单) / [COMBINATIONS.md](../../COMBINATIONS.md)(组合) / [DESIGN.md](../../DESIGN.md)(设计,由所有者撰写) / [components/README.md](../../../components/README.md)(附带的组件)。这些文件为日文。
