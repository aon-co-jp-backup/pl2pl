> Diese Fassung ist eine KI-Übersetzung und wurde noch nicht von Muttersprachlern geprüft. Maßgeblich ist das japanische Original im Wurzelverzeichnis des Repositorys.

# pl2pl – Übergabe
## Stand der Dinge (2026-11-19)
Version 0.1.0 ist veröffentlicht, mit Installern für Windows, Linux und macOS sowie Ende-zu-Ende-Tests in der CI. Die PHP-Umwandlung deckt Routen, Rümpfe mit mehreren Anweisungen, Eloquent (SQLAlchemy), den Query Builder mit Paginierung, Relationen (auch verschachteltes `with`), Blade (Jinja2) und Middleware (`Depends`) ab – alles Ende-zu-Ende getestet. Numba, Kokkos (Serial) und `pl2pl web` sind durch echte Läufe bestätigt.
## Offene Punkte nach Priorität
1. PHP: `morph`, Blade-Komponenten/-Stacks/-Autorisierung, Middleware, die nach `$next` wirkt.
2. COBOL_to / Java_to in pl2pl aufnehmen und danach die Originale löschen (vorher bestätigen; nicht rückgängig zu machen; andere Sitzungen arbeiten daran).
3. Prototyp der gemeinsamen Zwischendarstellung.
4. Anbindung an aruaru-llm (eine echte Installation mit `llm setup` ist noch ungeprüft).
5. Kokkos auf der GPU / `Kokkos::View`; Kernels auf open-cuda / open-directx.
6. Der Eigentümer schreibt `docs/DESIGN.md`; prüfen, ob `docs/LANGUAGES.md` aktuell ist.
7. Die neuen PHP-Beispiele in der CI unter Linux arm64 und macOS ausführen.
## Tests
`bash tests/run-all.sh` (WSL), `bash tests/run-php.sh` (PHP Ende-zu-Ende), `cargo test --release`.
## Nachricht zum Weiterarbeiten
„Mach mit F:\pl2pl weiter. Beginne bei Punkt 1 in PORTING.md. Nicht umwandelbare Teile nie stillschweigend weglassen, sondern als TODO stehen lassen. Erst nach einem echten Lauf als erledigt melden. Vor dem Push nachfragen.“
