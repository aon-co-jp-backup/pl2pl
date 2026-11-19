> This is an AI translation that native speakers have not yet reviewed. The Japanese original in the repository root is the authoritative version.

# pl2pl hand-over
(Detailed past notes are in `docs/HISTORY-ja.md`, in Japanese.)

## Where things stand (2026-11-19)
- v0.1.0 is out, with installers for Windows, Linux and Mac and tests in CI.
- PHP → FastAPI is tested end to end with real FastAPI (49 checks): routes, Eloquent, query builder, Blade, middleware, relations.
- `pl2pl web` now recognizes PHP too and passes it on.
- New: `pl2pl watch` finds new versions, tests, fixes, and notifies if all else fails. version-control got a `--notify-file` option.
- COBOL_to and Java_to are merged into `converters/`.

## Still to do (in priority order)
1. **Verify**: a real e-mail arriving (`version-control --test-notify`), automatic fixing through version-control `--loop`, and a test on the latest fastapi (the nightly CI run will check this).
2. Try `watch` against real new versions (laravel, symfony, spring-boot, flask, django, poem still count as unverified).
3. Register the nightly and morning runs (`schtasks /Create` etc.; not done, because it is the owner's own setup).
4. Prototype the shared intermediate representation (IR), the base for adding many language pairs.
5. PHP: Blade components written as classes, `$attributes`, `@php`, `morphToMany`.
6. aruaru-llm integration (a real `llm setup` install is unverified). Kokkos on GPU; kernels on open-cuda / open-directx.
7. Delete the original COBOL_to and Java_to (**irreversible: ask for final confirmation first**).
8. The owner writes `docs/DESIGN.md`. Check that `docs/LANGUAGES.md` is not out of date.

## Tests
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP end to end) / `cargo test --release`

## Message for resuming
“Resume F:\pl2pl. Work through ‘Still to do’ in PORTING.md from the top. Never drop unconvertible parts silently; leave a TODO. Verify by actually running before reporting done. Reply in Japanese only. Push at each milestone.”
