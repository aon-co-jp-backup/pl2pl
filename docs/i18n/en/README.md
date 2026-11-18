> Translated by AI; not yet verified by native speakers. The Japanese originals (repository root) are authoritative.

# pl2pl
A **converter between programming languages, frameworks and operating systems** (v0.1.0 released, under development). Installers exist for Windows (x64/ARM64), Linux (amd64/arm64) and macOS (arm64/x64). Conversion runs in the installed PC app (CLI); the web version only offers manuals and downloads.
Conversion is conservative: **anything that cannot be converted is never dropped silently; it stays as `TODO` and is listed in the report with a reason.**

## What works today
| Command | Description |
|---|---|
| `pl2pl cobol` | COBOL (IBM) + JCL → GnuCOBOL (Linux) + bash. Verified by building and running with GnuCOBOL. |
| `pl2pl omp` | Adds `#pragma omp parallel for` to independent C/C++ loops (with sum reductions). Same result as the original. |
| `pl2pl php` | Laravel / Symfony → FastAPI (see below). Verified end to end with real FastAPI, SQLite and Jinja2. |
| `pl2pl web` | Detects Java / COBOL / Flask·Django and hands over to java_to, cobol_to, 2fastapi, 2poem. Verified on real files. |
| `pl2pl cport` | UNIX C → compatibility header + mechanical replacement + porting notes. Verified with gcc. |
| `pl2pl pyhpc` | Numba `@njit` for numeric Python functions; `--mpi` adds an mpi4py skeleton. Same result under Numba. |
| `pl2pl kokkos` | Independent loops → Kokkos (`parallel_for` / `parallel_reduce`). Built and run with Kokkos 4.5.01 (Serial). |
| `pl2pl gpu` | (Prototype) loops → CUDA-style `.cu` / HLSL. Output only; execution on open-cuda / open-directx is unverified. |
| `pl2pl llm setup` / `llm start` | Starts the bundled aruaru-llm and installs a Qwen2.5 model that fits free memory. Only the selection logic and --dry-run are verified. |
| `pl2pl doctor` | CPU information via open-cpu (built in). |

## PHP conversion (`pl2pl php`)
- **Routes**: Laravel `Route::get/post/put/patch/delete/any/resource/apiResource/view`, `prefix` groups, Symfony `#[Route]`.
- **Bodies**: simple `return`, multi-statement bodies, `response()->json($x, 201)`, `noContent()`.
- **Eloquent → SQLAlchemy** (`db.py`, `models.py`): columns come from migrations or `$fillable`; other fields go to an `extra` JSON column; `DATABASE_URL` selects the database (SQLite by default).
- **Query builder**: `where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`, `paginate / simplePaginate` (`?page=N`, Laravel-style JSON, `links()` in Blade), `get / first / firstOrFail / count / exists / delete`, `$request->query('k', default)`.
- **Relations**: `hasMany / hasOne / belongsTo / belongsToMany` (pivot table generated), `with`, `whereHas / has / whereDoesntHave / doesntHave`, `attach / detach / sync`.
- **Blade → Jinja2**: output, `@if`, `@foreach`, `@forelse`, `@for`, `@extends/@section/@yield/@include`, and more.
- **Middleware → `Depends`**: `auth` (simple Bearer check; **replace with real authentication**), `guest`, `throttle:N,M`, custom "reject on condition, then `return $next($request)`".

## Planned (not yet done)
- PHP: nested `with('a.b')`, `morph` relations, `cursorPaginate`, Blade components / stacks / authorization directives, middleware that post-processes the response after `$next`.
- Fold COBOL_to and Java_to into pl2pl once they are mature, then delete the originals (the owner's instruction; deletion is irreversible, so it will be confirmed after the import).
- A common intermediate representation (server side, DB/SQL, UI side) so that N sources and M targets need N+M parts, not N×M.
- aruaru-llm integration (explanations and natural-language translation stay separate from the converters; converters never depend on an LLM).
- Kokkos GPU execution and `Kokkos::View`; kernel execution on open-cuda / open-directx.
- CI runs of the new PHP samples on Linux arm64 and macOS; more language pairs chosen from `docs/LANGUAGES.md` (not every combination will be built).

## Notes
Decisions are conservative: when a dependency or side effect is possible, the construct is **left unconverted and reported with the reason**; performance is not guaranteed. Only small hand-made samples have been tested, not large real-world programs.
Documents: `docs/DESIGN.md`, `docs/LANGUAGES.md`, `docs/COMBINATIONS.md`.
