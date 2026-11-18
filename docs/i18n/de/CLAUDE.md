> KI-Übersetzung, noch nicht von Muttersprachlern geprüft. Maßgeblich ist das japanische Original (Repository-Wurzel).

# pl2pl Entwicklungsnotizen
- Rolle: Konverter zwischen Sprachen/Frameworks/Betriebssystemen (Rust-CLI, `src/`). Quellkandidaten: `docs/LANGUAGES.md`, Kombinationen: `docs/COMBINATIONS.md`. Die Entwurfsspezifikation schreibt der Eigentümer nach und nach in `docs/DESIGN.md`; keine Spezifikation in dessen Namen erfinden.
- Regeln: Nicht umwandelbare Konstrukte bleiben als `TODO` stehen und erscheinen im Bericht; nie stillschweigend verwerfen. Vor der Fertigmeldung tatsächlich ausführen und prüfen. Konservativ entscheiden (bei möglicher Abhängigkeit oder Nebenwirkung nicht umwandeln).
- PHP: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`; Ende-zu-Ende-Test `tests/run-php.sh` (echtes FastAPI, SQLite, Jinja2), Unit-Tests `cargo test --release`. Aus WSL starten: `wsl -d Ubuntu -- bash <Skriptdatei>`.
- Stolperfallen: Unter Windows Dateien mit Japanisch mit Python `-X utf8` schreiben; Shell-Skripte in LF halten; Git Bash schreibt `/mnt/...`-Pfade um.
- COBOL_to und Java_to entwickelt eine andere Sitzung; später übernehmen, Originale erst nach letzter Bestätigung löschen (nicht umkehrbar).
- Das Übersetzungssystem (natürliche Sprache) ist vom Konverter getrennt.
- Dokumentationssprachen: Japanisch (Wurzel) plus 9 Sprachen in `docs/i18n/` (KI-Übersetzungen, noch nicht von Muttersprachlern geprüft).
