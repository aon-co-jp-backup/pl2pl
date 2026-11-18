> Translated by AI; not yet verified by native speakers. The Japanese originals (repository root) are authoritative.

# pl2pl hand-over
## Status (2026-11-19)
v0.1.0 is released (installers for Windows/Linux/macOS, CI end-to-end tests). PHP conversion covers routes, multi-statement bodies, Eloquent (SQLAlchemy), query builder with pagination, relations, Blade (Jinja2) and middleware (`Depends`), all tested end to end. Numba, Kokkos (Serial) and `pl2pl web` are verified on real runs.
## Open items (priority order)
1. PHP: nested `with`, `morph`, `cursorPaginate`, Blade components/stacks/authorization, post-`$next` middleware.
2. Fold COBOL_to / Java_to into pl2pl, then delete the originals (confirm first; irreversible; other sessions are working on them).
3. Prototype the common IR.
4. aruaru-llm integration (`llm setup` real install unverified).
5. Kokkos GPU / `Kokkos::View`; kernels on open-cuda / open-directx.
6. Owner writes `docs/DESIGN.md`; check that `docs/LANGUAGES.md` is up to date.
7. Run the new PHP samples in CI on Linux arm64 and macOS.
## Tests
`bash tests/run-all.sh` (WSL), `bash tests/run-php.sh` (PHP end to end), `cargo test --release`.
## Resume message
"Resume F:\pl2pl. Start from item 1 of PORTING.md. Never drop unconvertible parts silently; leave TODO. Verify by running before reporting done. Confirm before pushing."
