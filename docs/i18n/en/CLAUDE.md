> Translated by AI; not yet verified by native speakers. The Japanese originals (repository root) are authoritative.

# pl2pl development notes
- Role: converter between languages/frameworks/OSes (Rust CLI, `src/`). Source candidates: `docs/LANGUAGES.md`; combinations: `docs/COMBINATIONS.md`. The owner writes the design spec gradually in `docs/DESIGN.md`; never invent spec on their behalf.
- Rules: unconvertible constructs stay as `TODO` and appear in the report; never drop them silently. Verify by actually running things before reporting completion. Decisions are conservative (no conversion if a dependency or side effect is possible).
- PHP: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`; end-to-end test `tests/run-php.sh` (real FastAPI, SQLite, Jinja2), unit tests `cargo test --release`. Run from WSL: `wsl -d Ubuntu -- bash <script file>`.
- Pitfalls: on Windows write files containing Japanese with Python `-X utf8`; keep shell scripts in LF; Git Bash rewrites `/mnt/...` paths.
- COBOL_to and Java_to are developed by other sessions; fold them in later, then delete the originals only after final confirmation (irreversible).
- The translation system (natural language) is separate from the converters.
- Documentation languages: Japanese (root) plus 9 languages in `docs/i18n/` (AI translations, not yet verified by native speakers).
