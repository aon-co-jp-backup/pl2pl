> Note: the Japanese PORTING.md in the repository root is the original. This English version uses plain words.

# pl2pl hand-over
(Older, detailed notes are in `docs/HISTORY-ja.md`, in Japanese.)

## Where we are (2026-11-19)
- Version 0.1.0 is out, with installers for Windows, Linux and Mac, and tests in CI.
- PHP → FastAPI is tested end to end with real FastAPI: routes, Eloquent, query builder, Blade, middleware, relations and the `@can` family.
- `pl2pl web` also spots PHP now.
- `pl2pl watch` finds new versions, tests them, fixes problems, and sends an alert if nothing works. GitHub also tests with the newest versions every night and every morning.
- `pl2pl ir` (a test version) puts the routes of PHP, Python and Java (Spring, JAX-RS, Servlet, Spark) into one shared shape, and writes JSON, OpenAPI or a FastAPI starter. The IR also carries typed parameters, function bodies, models, middleware and templates, and `pl2pl php` now writes its output through the shared emitter in `src/ir_emit.rs` (output unchanged from before; 49 end-to-end checks pass).
- The guide and download page is live: <https://easy-web.tokyo/pl2pl/> (on the VPS, static). There is also a card on the easy-web.tokyo top page.
- COBOL_to and Java_to are merged into `converters/`.

## Still to do (most important first)
1. **Check**: a real e-mail arriving (`version-control --test-notify`; this PC has no SMTP settings), automatic fixing through version-control `--loop`, and a test on the newest fastapi (the nightly run on GitHub will check it).
2. Put types and function bodies into the IR. Send the current FastAPI output (`pl2pl php`) through the IR. If route finding misses things, think about tree-sitter.
3. Try `watch` with real new versions (laravel, symfony, spring-boot, flask, django and poem still count as unchecked).
4. Set up the nightly and morning runs on a PC (not done: the check command does not work on this PC and would give false alarms; see the top of `scripts/pl2pl-watch.cmd` for an example).
5. (Done) A pl2pl card is on the easy-web.tokyo top page and live in production (open-easy-web `95cb1e7`; the old binary is `server/target/release/open-easy-web-server.before-pl2pl-card-20261119`).
6. PHP (mostly done): Blade `@php` (assignments only) becomes `{% set %}`; `$attributes` (`{{ $attributes }}` and `->merge([...])` with plain strings; class is joined with the caller's value, and names declared in `@props` are left out) and `morphToMany` (the pivot table gets a type column and rows are split by type) are implemented (54 end-to-end checks, 41 unit tests). Still open: Blade parts written as classes (`app/View/Components/*.php`), `$attributes->class()` / `->only()` and merge with non-string values, and `@php` with more than assignments (stays a TODO).
7. aruaru-llm link-up (done): on 2026-11-19 we pulled in aruaru-llm v0.2.5 and ran a real `pl2pl llm setup`. The automatic pick, 3B, failed because its weights are split into shards, so setup now falls back to the next smaller model (not when you pin one). `GenerateQwen` returns text with qwen2.5-1.5b-instruct (about 2.9 GB, on CPU). **`pl2pl php --explain`** (optional; `--explain-max N`, default 3) adds an AI note under each TODO item of the report. Measured: about 2 minutes per item on CPU, and **the quality is poor and sometimes wrong** (1 of 2 was off the mark), so it is off by default. A restarted aruaru-llm has no model selected (400), so it selects an installed one first. **A bigger model (about 12 GB)**: the largest in aruaru-llm's catalog is 3B (about 6.2 GB), and 3B and 7B ship as several shard files that the current installer cannot fetch. Shard support (in aruaru-llm) and CPU speed (it is already this slow at 1.5B) come first. Kokkos on the GPU and kernels on open-cuda / open-directx are still unchecked. **GPU (checked 2026-11-19)**: on a GeForce GT 730 (2 GB, sm_35, driver 475 = CUDA 11.4 class) we installed the CUDA 11.8 nvcc without admin rights through micromamba (NVIDIA's conda channel), ran a plain CUDA kernel and the output of `pl2pl gpu --target open-cuda` (`vec_loop12.cu`) on the real GPU, and saw 0 mismatches against the CPU (`tests/gpu-check.ps1`). Pitfalls: MSVC 14.44 needs `-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH` and `-allow-unsupported-compiler`; PyPI has no Windows nvcc; CUDA 12 cannot target sm_35. **Still unchecked**: the Kokkos CUDA backend, and running on open-cuda (the Rust implementation) and open-directx.
8. Delete the original COBOL_to and Java_to (**this cannot be undone, so ask for a final OK first**).
9. The owner writes `docs/DESIGN.md`. We re-read `docs/LANGUAGES.md` on 2026-11-19 and found nothing out of line with the current state (Java and COBOL have converters). It is still a knowledge-based list, not checked against a fresh web search.

## Tests
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP end to end) / `cargo test --release`

## Message for starting again
“Resume F:\pl2pl. Work through ‘Still to do’ in PORTING.md from the top. Never hide what cannot be rewritten; leave a TODO. Check by really running before saying it is done. Reply in Japanese only. Commit and push at each step, and update the VPS too.”
