> KI-Übersetzung, noch nicht von Muttersprachlern geprüft. Maßgeblich ist das japanische Original (Repository-Wurzel).

# pl2pl Übergabe
## Stand (2026-11-19)
v0.1.0 ist veröffentlicht (Installer für Windows/Linux/macOS, CI-Ende-zu-Ende-Tests). Die PHP-Umwandlung deckt Routen, mehrteilige Rümpfe, Eloquent (SQLAlchemy), Query Builder mit Paginierung, Relationen, Blade (Jinja2) und Middleware (`Depends`) ab, alles Ende-zu-Ende getestet. Numba, Kokkos (Serial) und `pl2pl web` sind in echten Läufen bestätigt.
## Offene Punkte (nach Priorität)
1. PHP: verschachteltes `with`, `morph`, `cursorPaginate`, Blade-Komponenten/Stacks/Autorisierung, Middleware nach `$next`.
2. COBOL_to / Java_to in pl2pl übernehmen, danach Originale löschen (vorher bestätigen; nicht umkehrbar; andere Sitzungen arbeiten daran).
3. Prototyp der gemeinsamen IR.
4. Anbindung an aruaru-llm (echte Installation von `llm setup` ungeprüft).
5. Kokkos-GPU / `Kokkos::View`; Kernel auf open-cuda / open-directx.
6. Eigentümer schreibt `docs/DESIGN.md`; Aktualität von `docs/LANGUAGES.md` prüfen.
7. Neue PHP-Beispiele in der CI auf Linux arm64 und macOS ausführen.
## Tests
`bash tests/run-all.sh` (WSL), `bash tests/run-php.sh` (PHP Ende-zu-Ende), `cargo test --release`.
## Nachricht zum Fortsetzen
„F:\pl2pl fortsetzen. Mit Punkt 1 aus PORTING.md beginnen. Nicht umwandelbare Teile nie stillschweigend verwerfen, TODO stehen lassen. Erst nach tatsächlichem Lauf als fertig melden. Vor dem Push bestätigen.“
