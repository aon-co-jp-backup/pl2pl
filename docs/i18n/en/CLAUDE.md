> This is an AI translation that native speakers have not yet reviewed. The Japanese original in the repository root is the authoritative version.

# pl2pl dev notes
- **Role**: a converter across languages, frameworks and operating systems (Rust CLI in `src/`). Source languages are listed in `docs/LANGUAGES.md`, combinations in `docs/COMBINATIONS.md`. The owner writes the design in `docs/DESIGN.md` bit by bit. **Never make up spec for them.**
- **Direction**: many-to-many conversion between well-known languages and frameworks. Not everything at once; build the chosen ones first. The website only offers manuals and downloads; conversion runs in the PC app.
- **Rules**: whatever cannot be converted stays as a `TODO` and is reported (never dropped silently). Stay conservative (if a dependency or side effect is possible, do not convert). Run it for real before saying "done". No new REST. Reply in Japanese only.
- **PHP**: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`. End-to-end test: `tests/run-php.sh` (real FastAPI, SQLite, Jinja2); unit tests: `cargo test --release`. In WSL: `wsl -d Ubuntu -- bash <file>`.
- **Auto-update**: `pl2pl watch` (`src/watch.rs`, `registry/ecosystem.toml`). Entries with an empty verify command are only shown as "unverified" and are never bumped. Notifications go through version-control `--notify-file`.
- **Pitfalls**: write files containing Japanese with Python `-X utf8`; keep shell scripts in LF; Git Bash rewrites `/mnt/...` paths; use the Write tool, not a heredoc, for strings full of quotes.
- **COBOL_to / Java_to**: already merged into `converters/`. Deleting the original repositories cannot be undone, so ask for final confirmation first.
- **Translation system** (natural language) stays separate from the converters. Converters do not depend on an LLM.
- **Documentation languages**: Japanese originals, plus nine languages in `docs/i18n/` (AI translations, not yet reviewed by native speakers). When an original changes, update the translations. Past history: `docs/HISTORY-ja.md`.
