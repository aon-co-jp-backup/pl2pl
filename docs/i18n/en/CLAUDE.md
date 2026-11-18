> This is an AI translation that native speakers have not yet reviewed. The Japanese original in the repository root is the authoritative version.

# pl2pl CLAUDE
- **Role**: a converter between languages, frameworks and operating systems (Rust CLI in `src/`). Candidate sources are listed in `docs/LANGUAGES.md`, combinations in `docs/COMBINATIONS.md`. The owner writes the design spec bit by bit in `docs/DESIGN.md`; never make up spec on their behalf.
- **Rules**: anything that cannot be converted stays as a `TODO` and goes into the report — never drop it silently. Run things for real before saying a task is done. Stay conservative: if a dependency or side effect is possible, do not convert.
- **PHP**: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`. End-to-end test: `tests/run-php.sh` (real FastAPI, SQLite, Jinja2). Unit tests: `cargo test --release`. Run from WSL with `wsl -d Ubuntu -- bash <script file>`.
- **Pitfalls**: on Windows, write files containing Japanese with Python `-X utf8`; keep shell scripts in LF; Git Bash rewrites `/mnt/...` paths.
- **COBOL_to / Java_to**: other sessions are developing them. Merge them later, and delete the originals only after a final confirmation — it cannot be undone.
- **Translation system** (natural language) is kept separate from the converters.
- **Documentation languages**: Japanese in the root, plus nine languages under `docs/i18n/` (AI translations, not yet reviewed by native speakers). When the Japanese original changes, update the translations too.
