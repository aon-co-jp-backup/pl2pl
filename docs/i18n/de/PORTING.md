> Diese Fassung ist eine KI-Übersetzung und wurde noch nicht von Muttersprachlern geprüft. Maßgeblich ist das japanische Original im Wurzelverzeichnis des Repositorys.

# pl2pl – Übergabe
(Ausführliche frühere Notizen stehen in `docs/HISTORY-ja.md`, auf Japanisch.)

## Stand (2026-11-19)
- Version 0.1.0 ist veröffentlicht, mit Installern für Windows, Linux und Mac sowie Tests in der CI.
- PHP → FastAPI ist mit echtem FastAPI durchgängig getestet (49 Prüfungen): Routen, Eloquent, Query Builder, Blade, Middleware, Beziehungen.
- `pl2pl web` erkennt jetzt auch PHP und gibt es weiter.
- Neu: `pl2pl watch` findet neue Versionen, testet, repariert und benachrichtigt, wenn alles andere scheitert. version-control hat die Option `--notify-file` bekommen.
- COBOL_to und Java_to sind in `converters/` übernommen.

## Noch offen (nach Priorität)
1. **Prüfen**: Kommt eine echte E-Mail an (`version-control --test-notify`)? Funktioniert die automatische Reparatur über `version-control --loop`? Test mit dem neuesten fastapi (der nächtliche CI-Lauf prüft das).
2. `watch` mit echten neuen Versionen ausprobieren (laravel, symfony, spring-boot, flask, django und poem gelten noch als ungeprüft).
3. Den nächtlichen und morgendlichen Lauf einrichten (`schtasks /Create` usw.; noch nicht getan, weil es die eigene Umgebung des Eigentümers betrifft).
4. Prototyp der gemeinsamen Zwischendarstellung (IR) – die Grundlage für viele weitere Sprachpaare.
5. PHP: `@can`, Blade-Komponenten als Klassen, `$attributes`, `@php`, `morphToMany`.
6. Anbindung an aruaru-llm (die echte Installation per `llm setup` ist ungeprüft). Kokkos auf der GPU; Kernel auf open-cuda / open-directx.
7. Die ursprünglichen COBOL_to und Java_to löschen (**nicht rückgängig zu machen – vorher endgültige Bestätigung einholen**).
8. `docs/DESIGN.md` schreibt der Eigentümer. Prüfen, ob `docs/LANGUAGES.md` noch aktuell ist.

## Tests
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP durchgängig) / `cargo test --release`

## Nachricht zum Weitermachen
„F:\pl2pl fortsetzen. Die Liste ‚Noch offen‘ in PORTING.md von oben abarbeiten. Nicht Umwandelbares nie stillschweigend weglassen, sondern als TODO stehen lassen. Erst durch echtes Ausführen prüfen, dann Erledigung melden. Antworten nur auf Japanisch. Bei jedem Etappenziel pushen.“
