> This is an AI translation that native speakers have not yet reviewed. The Japanese original in the repository root is the authoritative version.

# pl2pl
pl2pl converts code between programming languages, frameworks and operating systems. It is at v0.1.0 and still under active development. Installers are available for Windows (x64/ARM64), Linux (amd64/arm64) and macOS (arm64/x64). Conversion happens in the installed desktop app (a CLI); the website only hosts the manuals and downloads.
The converter is deliberately cautious. **Whatever it cannot convert is never dropped silently. It stays in the output as a `TODO` and is listed in the report together with the reason.**

## What works today
| Command | What it does |
|---|---|
| `pl2pl cobol` | Converts IBM COBOL plus JCL to GnuCOBOL on Linux plus bash. Verified by building and running the result with GnuCOBOL. |
| `pl2pl omp` | Adds `#pragma omp parallel for` to independent C/C++ loops, including sum reductions. Produces the same result as the original. |
| `pl2pl php` | Converts Laravel / Symfony to FastAPI (details below). Tested end to end against real FastAPI, SQLite and Jinja2. |
| `pl2pl web` | Detects Java, COBOL and Flask/Django, then hands the work to java_to, cobol_to, 2fastapi or 2poem. Verified on real files. |
| `pl2pl cport` | Turns UNIX C into a compatibility header plus mechanical replacements and porting notes. Verified with gcc. |
| `pl2pl pyhpc` | Adds Numba `@njit` to numeric Python functions; `--mpi` also writes an mpi4py skeleton. Gives the same result under Numba. |
| `pl2pl kokkos` | Rewrites independent loops as Kokkos (`parallel_for` / `parallel_reduce`). Built and run with Kokkos 4.5.01 (Serial). |
| `pl2pl gpu` | Prototype: turns loops into CUDA-style `.cu` or HLSL. Only the generated output is checked; running it on open-cuda / open-directx is untested. |
| `pl2pl llm setup` / `llm start` | Starts the bundled aruaru-llm and installs a Qwen2.5 model that fits the free memory. Only the model selection and `--dry-run` are verified; a real install is not. |
| `pl2pl doctor` | Shows CPU information through the built-in open-cpu. |

## PHP conversion (`pl2pl php`)
- **Routes**: Laravel `Route::get/post/put/patch/delete/any/resource/apiResource/view`, `prefix` groups, and Symfony `#[Route]`.
- **Bodies**: simple `return`, multi-statement bodies, `response()->json($x, 201)` and `noContent()`.
- **Eloquent → SQLAlchemy** (`db.py`, `models.py`): columns come from migrations or `$fillable`; anything else goes into a JSON `extra` column. `DATABASE_URL` selects the database (SQLite by default).
- **Query builder**: `where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`, `paginate / simplePaginate / cursorPaginate` (reads `?page=N`, returns Laravel-style JSON, `links()` works in Blade), `get / first / firstOrFail / count / exists / delete`, and `$request->query('k', default)`.
- **Relations**: `hasMany / hasOne / belongsTo / belongsToMany` (pivot table generated), `with` including nested `with('posts.comments')`, `whereHas / has / whereDoesntHave / doesntHave`, and `attach / detach / sync`.
- **Blade → Jinja2**: echoes, `@if`, `@foreach`, `@forelse`, `@for`, `@extends/@section/@yield/@include` and more.
- **Middleware → `Depends`**: `auth` (a simple Bearer check — **replace it with real authentication**), `guest`, `throttle:N,M`, and custom middleware of the form “reject on a condition, otherwise `return $next($request)`”.

## Planned (not done yet)
- PHP: `morph` relations, Blade components, stacks and authorization directives, and middleware that post-processes the response after `$next`.
- Merge COBOL_to and Java_to into pl2pl once they are mature, then delete the original repositories. This is the owner's instruction; since deletion cannot be undone, it will be confirmed again after the merge.
- A shared intermediate representation (server side, DB/SQL, UI side), so N sources and M targets need N+M parts instead of N×M.
- aruaru-llm integration. Explanations and natural-language translation stay separate from the converters, which never depend on an LLM.
- Kokkos on GPU and `Kokkos::View`, plus running kernels on open-cuda / open-directx.
- Running the new PHP samples in CI on Linux arm64 and macOS, and adding more language pairs from `docs/LANGUAGES.md` (not every combination will be built).

## Good to know
pl2pl errs on the side of caution. If a construct might have a dependency or side effect, it is **left unconverted and reported with the reason**, and no performance is promised. So far only small hand-written samples have been tested, not large real-world programs.
Further reading: `docs/DESIGN.md`, `docs/LANGUAGES.md`, `docs/COMBINATIONS.md`.
