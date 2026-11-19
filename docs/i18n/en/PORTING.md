> Note: the Japanese PORTING.md in the repository root is the original. This English version uses plain words.

# pl2pl hand-over
(Older, detailed notes are in `docs/HISTORY-ja.md`, in Japanese.)

## Where we are (2026-11-19)
- Version 0.1.0 is out, with installers for Windows, Linux and Mac, and tests in CI.
- PHP → FastAPI is tested end to end with real FastAPI: routes, Eloquent, query builder, Blade, middleware, relations and the `@can` family.
- `pl2pl web` also spots PHP now.
- `pl2pl watch` finds new versions, tests them, fixes problems, and sends an alert if nothing works. GitHub also tests with the newest versions every night and every morning.
- `pl2pl ir` (a test version) puts the routes of PHP, Python and Java (Spring, JAX-RS, Servlet, Spark) into one shared shape, and writes JSON, OpenAPI or a FastAPI starter. We checked that the output can really be read by Python.
- The guide and download page is live: <https://easy-web.tokyo/pl2pl/> (on the VPS, static).
- COBOL_to and Java_to are merged into `converters/`.

## Still to do (most important first)
1. **Check**: a real e-mail arriving (`version-control --test-notify`; this PC has no SMTP settings), automatic fixing through version-control `--loop`, and a test on the newest fastapi (the nightly run on GitHub will check it).
2. Put types and function bodies into the IR. Send the current FastAPI output (`pl2pl php`) through the IR. If route finding misses things, think about tree-sitter.
3. Try `watch` with real new versions (laravel, symfony, spring-boot, flask, django and poem still count as unchecked).
4. Set up the nightly and morning runs on a PC (not done: the check command does not work on this PC and would give false alarms; see the top of `scripts/pl2pl-watch.cmd` for an example).
5. Add a pl2pl card to the top page of easy-web.tokyo.
6. PHP: Blade parts written as classes, `$attributes`, `@php`, `morphToMany`.
7. aruaru-llm link-up (a real `llm setup` install is unchecked). Kokkos on the GPU; kernels on open-cuda / open-directx.
8. Delete the original COBOL_to and Java_to (**this cannot be undone, so ask for a final OK first**).
9. The owner writes `docs/DESIGN.md`. Check that `docs/LANGUAGES.md` is not out of date.

## Tests
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP end to end) / `cargo test --release`

## Message for starting again
“Resume F:\pl2pl. Work through ‘Still to do’ in PORTING.md from the top. Never hide what cannot be rewritten; leave a TODO. Check by really running before saying it is done. Reply in Japanese only. Commit and push at each step, and update the VPS too.”
