> This is an AI translation that native speakers have not yet reviewed. The Japanese original in the repository root is the authoritative version.

# pl2pl hand-over
## Where things stand (2026-11-19)
v0.1.0 is out, with installers for Windows, Linux and macOS and end-to-end tests in CI. PHP conversion covers routes, multi-statement bodies, Eloquent (SQLAlchemy), the query builder with pagination, relations (including nested `with`), Blade (Jinja2) and middleware (`Depends`), all tested end to end. Numba, Kokkos (Serial) and `pl2pl web` have been confirmed by real runs.
## Open items, in priority order
1. PHP: `@can` authorization and class-based Blade components, middleware that rewrites the body after `$next`.
2. Merge COBOL_to / Java_to into pl2pl, then delete the originals (confirm first; irreversible; other sessions are working on them).
3. Prototype the shared intermediate representation.
4. aruaru-llm integration (a real `llm setup` install is still unverified).
5. Kokkos on GPU / `Kokkos::View`; kernels on open-cuda / open-directx.
6. The owner writes `docs/DESIGN.md`; check that `docs/LANGUAGES.md` is up to date.
7. Run the new PHP samples in CI on Linux arm64 and macOS.
## Tests
`bash tests/run-all.sh` (WSL), `bash tests/run-php.sh` (PHP end to end), `cargo test --release`.
## Message for resuming
“Resume F:\pl2pl. Start with item 1 in PORTING.md. Never drop unconvertible parts silently — leave a TODO. Verify by actually running before reporting done. Ask before pushing.”
