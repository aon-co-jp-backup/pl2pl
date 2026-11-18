> 由 AI 翻译,尚未经母语者校对。以日文原文(仓库根目录)为准。

# pl2pl
**编程语言、框架与操作系统之间的转换工具**(v0.1.0 已发布,开发中)。提供 Windows(x64/ARM64)、Linux(amd64/arm64)、macOS(arm64/x64)安装包。转换在已安装的 PC 应用(CLI)中进行,网页版仅提供说明书和下载。
转换采取保守策略:**无法转换的部分绝不会被悄悄丢弃,而是保留为 `TODO`,并在报告中说明原因。**

## 目前可用的功能
| 命令 | 说明 |
|---|---|
| `pl2pl cobol` | COBOL(IBM)+JCL → GnuCOBOL(Linux)+bash。已用 GnuCOBOL 实际构建并运行验证。 |
| `pl2pl omp` | 为独立的 C/C++ 循环添加 `#pragma omp parallel for`(含求和归约)。结果与原程序一致。 |
| `pl2pl php` | Laravel / Symfony → FastAPI(详见下文)。已用真实的 FastAPI、SQLite、Jinja2 完整测试。 |
| `pl2pl web` | 识别 Java / COBOL / Flask·Django,并交给 java_to、cobol_to、2fastapi、2poem。已用真实文件验证。 |
| `pl2pl cport` | UNIX 的 C → 兼容头文件+机械替换+移植指南。已用 gcc 验证。 |
| `pl2pl pyhpc` | 为数值型 Python 函数添加 Numba `@njit`;`--mpi` 生成 mpi4py 骨架。Numba 下结果一致。 |
| `pl2pl kokkos` | 独立循环 → Kokkos(`parallel_for` / `parallel_reduce`)。已用 Kokkos 4.5.01(Serial)构建并运行。 |
| `pl2pl gpu` | (试作)循环 → CUDA 风格 `.cu` / HLSL。仅输出;在 open-cuda / open-directx 上的执行尚未验证。 |
| `pl2pl llm setup` / `llm start` | 启动内置的 aruaru-llm,并按可用内存自动安装合适的 Qwen2.5 模型。仅验证了选择逻辑和 --dry-run。 |
| `pl2pl doctor` | 通过 open-cpu(内置)显示 CPU 信息。 |

## PHP 转换(`pl2pl php`)
- **路由**:Laravel 的 `Route::get/post/put/patch/delete/any/resource/apiResource/view`、`prefix` 分组、Symfony 的 `#[Route]`。
- **函数体**:简单的 `return`、多条语句、`response()->json($x, 201)`、`noContent()`。
- **Eloquent → SQLAlchemy**(`db.py`、`models.py`):列取自迁移文件或 `$fillable`,其余字段放入 `extra`(JSON)列;用 `DATABASE_URL` 指定数据库(默认 SQLite)。
- **查询构造器**:`where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`、`paginate / simplePaginate`(`?page=N`、与 Laravel 相同的 JSON、Blade 的 `links()`)、`get / first / firstOrFail / count / exists / delete`、`$request->query('k', 默认值)`。
- **关联**:`hasMany / hasOne / belongsTo / belongsToMany`(自动生成中间表)、`with`、`whereHas / has / whereDoesntHave / doesntHave`、`attach / detach / sync`。
- **Blade → Jinja2**:输出、`@if`、`@foreach`、`@forelse`、`@for`、`@extends/@section/@yield/@include` 等。
- **中间件 → `Depends`**:`auth`(简易 Bearer 判断,**须替换为真正的认证**)、`guest`、`throttle:N,M`、自定义的"满足条件则拒绝,否则 `return $next($request)`"。

## 计划中(尚未完成)
- PHP:嵌套 `with('a.b')`、`morph` 关联、`cursorPaginate`、Blade 的组件/栈/授权指令、在 `$next` 之后处理响应的中间件。
- 待 COBOL_to 和 Java_to 开发成熟后并入 pl2pl,并删除原仓库(所有者的指示;删除不可撤销,并入后会再次确认)。
- 通用中间表示(服务端、DB/SQL、界面端),使 N 个源与 M 个目标只需 N+M 个部件而非 N×M。
- 与 aruaru-llm 联动(说明与自然语言翻译与转换器分离,转换器不依赖 LLM)。
- Kokkos 的 GPU 执行与 `Kokkos::View`;在 open-cuda / open-directx 上执行内核。
- 在 Linux arm64 与 macOS 的 CI 中运行新的 PHP 示例;从 `docs/LANGUAGES.md` 中挑选更多语言组合(不会实现全部组合)。

## 注意
判断保守:可能存在依赖或副作用时,**不做转换并说明原因**,不保证性能。仅测试过小型自制示例,尚未在大型真实程序上试验。
文档:`docs/DESIGN.md`、`docs/LANGUAGES.md`、`docs/COMBINATIONS.md`。
