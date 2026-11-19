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
6. PHP (partly done): Blade `@php` that only assigns variables (also `@php($x = 1)`) now becomes `{% set %}` (50 end-to-end checks pass). Still open: Blade parts written as classes, `$attributes`, `morphToMany`; an `@php` with anything but assignments stays a TODO as before.
7. aruaru-llm link-up (done): on 2026-11-19 we pulled in aruaru-llm v0.2.5 and ran a real `pl2pl llm setup`. The automatic pick, 3B, failed with a 404 because its weights are split into shards, so setup now falls back to the next smaller model on failure (not when you pin a model). qwen2.5-1.5b-instruct (about 2.9 GB) installed, and `GenerateQwen` returned text (on CPU). Still open: calling it to explain the parts we could not convert (the TODOs); Kokkos on the GPU; kernels on open-cuda / open-directx.
8. Delete the original COBOL_to and Java_to (**this cannot be undone, so ask for a final OK first**).
9. The owner writes `docs/DESIGN.md`. We re-read `docs/LANGUAGES.md` on 2026-11-19 and found nothing out of line with the current state (Java and COBOL have converters). It is still a knowledge-based list, not checked against a fresh web search.

## Tests
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP end to end) / `cargo test --release`

## Message for starting again
“Resume F:\pl2pl. Work through ‘Still to do’ in PORTING.md from the top. Never hide what cannot be rewritten; leave a TODO. Check by really running before saying it is done. Reply in Japanese only. Commit and push at each step, and update the VPS too.”
