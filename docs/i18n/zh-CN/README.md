> 本文由 AI 翻译,尚未经过母语者校对,内容以仓库根目录的日文原文为准。

# pl2pl
pl2pl 是一款在编程语言、框架和操作系统之间转换代码的工具,目前为 v0.1.0,仍在持续开发中。提供 Windows(x64/ARM64)、Linux(amd64/arm64)和 macOS(arm64/x64)的安装包。转换在安装后的桌面应用(命令行)中完成,网站只提供说明书和下载。
转换策略偏保守:**凡是无法转换的部分,绝不会悄悄丢掉,而是在输出中保留为 `TODO`,并在报告里写明原因。**

## 目前能用的功能
| 命令 | 作用 |
|---|---|
| `pl2pl cobol` | 把 IBM COBOL 加 JCL 转换为 Linux 上的 GnuCOBOL 加 bash。已用 GnuCOBOL 实际构建并运行验证。 |
| `pl2pl omp` | 为相互独立的 C/C++ 循环加上 `#pragma omp parallel for`(包括求和归约),结果与原程序一致。 |
| `pl2pl php` | 把 Laravel / Symfony 转换为 FastAPI(详见下文)。已用真实的 FastAPI、SQLite 和 Jinja2 做过端到端测试。 |
| `pl2pl web` | 自动识别 Java、COBOL 以及 Flask/Django,再交给 java_to、cobol_to、2fastapi 或 2poem 处理。已用真实文件验证。 |
| `pl2pl cport` | 把 UNIX 下的 C 代码转为兼容头文件,加上机械替换和移植说明。已用 gcc 验证。 |
| `pl2pl pyhpc` | 给数值计算的 Python 函数加上 Numba `@njit`;加 `--mpi` 还会生成 mpi4py 骨架。在 Numba 下结果一致。 |
| `pl2pl kokkos` | 把独立循环改写为 Kokkos(`parallel_for` / `parallel_reduce`)。已用 Kokkos 4.5.01(Serial)构建并运行。 |
| `pl2pl gpu` | 试作功能:把循环转为 CUDA 风格的 `.cu` 或 HLSL。目前只检查了生成的输出,尚未在 open-cuda / open-directx 上运行。 |
| `pl2pl llm setup` / `llm start` | 启动内置的 aruaru-llm,并按可用内存自动安装合适的 Qwen2.5 模型。目前只验证了模型选择逻辑和 `--dry-run`,真实安装尚未验证。 |
| `pl2pl doctor` | 通过内置的 open-cpu 显示 CPU 信息。 |

## PHP 转换(`pl2pl php`)
- **路由**:Laravel 的 `Route::get/post/put/patch/delete/any/resource/apiResource/view`、`prefix` 分组,以及 Symfony 的 `#[Route]`。
- **函数体**:简单的 `return`、多条语句、`response()->json($x, 201)` 和 `noContent()`。
- **Eloquent → SQLAlchemy**(`db.py`、`models.py`):列取自迁移文件或 `$fillable`,其余字段存入 JSON 类型的 `extra` 列。用 `DATABASE_URL` 指定数据库(默认 SQLite)。
- **查询构造器**:`where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`、`paginate / simplePaginate / cursorPaginate`(读取 `?page=N`,返回与 Laravel 相同结构的 JSON,Blade 中可用 `links()`)、`get / first / firstOrFail / count / exists / delete`,以及 `$request->query('k', 默认值)`。
- **模型关联**:`hasMany / hasOne / belongsTo / belongsToMany / morphMany / morphOne / morphTo`(自动生成中间表)、`with`(含嵌套的 `with('posts.comments')`)、`whereHas / has / whereDoesntHave / doesntHave`,以及 `attach / detach / sync`。
- **Blade → Jinja2**:输出表达式、`@if`、`@foreach`、`@forelse`、`@for`、`@extends/@section/@yield/@include` 等。
- **中间件 → `Depends`**:`auth`(只是简易的 Bearer 校验,**请换成真正的认证**)、`guest`、`throttle:N,M`,以及“满足条件就拒绝,否则 `return $next($request)`”形式的自定义中间件。

## 后续计划(尚未完成)
- PHP:`@can` 之类的授权指令和基于类的 Blade 组件(匿名的 `<x-...>` 组件已支持),以及在 `$next` 之后修改响应正文的中间件(添加响应头已支持)。
- COBOL_to 和 Java_to 已并入 `converters/`(带历史的快照,测试通过)。剩下的工作:让 `pl2pl web` 改用内置的副本,跟进原仓库之后的改动,然后删除原仓库(所有者的指示;删除无法撤销,所以会先确认)。
- 建立统一的中间表示(服务端、数据库/SQL、界面端),让 N 种来源和 M 种目标只需 N+M 个组件,而不是 N×M 个。
- 接入 aruaru-llm。说明文字和自然语言翻译与转换器分开,转换器本身不依赖 LLM。
- Kokkos 的 GPU 执行和 `Kokkos::View`,以及在 open-cuda / open-directx 上运行内核。
- 从 `docs/LANGUAGES.md` 中再挑选一些语言组合(不会做全部组合)。

## 使用须知
pl2pl 的判断偏谨慎:只要某段代码可能存在依赖或副作用,就**不转换,并在报告中说明原因**,也不保证性能。目前只测试过小型的手写示例,还没有在大型真实项目上试过。
延伸阅读:`docs/DESIGN.md`、`docs/LANGUAGES.md`、`docs/COMBINATIONS.md`。
