> Diese Fassung ist eine KI-Übersetzung und wurde noch nicht von Muttersprachlern geprüft. Maßgeblich ist das japanische Original im Wurzelverzeichnis des Repositorys.

# pl2pl – Entwicklungsnotizen
- **Aufgabe**: ein Umwandler zwischen Sprachen, Frameworks und Betriebssystemen (Rust-Kommandozeile, `src/`). Quellsprachen stehen in `docs/LANGUAGES.md`, Kombinationen in `docs/COMBINATIONS.md`. Den Entwurf schreibt der Eigentümer nach und nach in `docs/DESIGN.md`. **Keine Spezifikation auf eigene Faust erfinden.**
- **Richtung**: Umwandlung von jeder bekannten Sprache bzw. jedem bekannten Framework in jede andere. Nicht alles auf einmal, sondern zuerst das Ausgewählte. Die Website bietet nur Anleitungen und Downloads; umgewandelt wird in der PC-App.
- **Regeln**: Nicht Umwandelbares bleibt als `TODO` stehen und wird gemeldet (nie stillschweigend weglassen). Vorsichtig urteilen (bei möglichen Abhängigkeiten oder Nebenwirkungen nicht umwandeln). Erst wirklich ausführen, dann „fertig“ sagen. Keine neuen REST-Schnittstellen. Antworten nur auf Japanisch.
- **PHP**: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`. Durchgängiger Test: `tests/run-php.sh` (echtes FastAPI, SQLite, Jinja2); Unit-Tests: `cargo test --release`. In WSL: `wsl -d Ubuntu -- bash <Datei>`.
- **Automatisches Nachziehen**: `pl2pl watch` (`src/watch.rs`, `registry/ecosystem.toml`). Einträge ohne Prüfbefehl werden nur als „ungeprüft“ angezeigt und nie von selbst angehoben. Benachrichtigungen laufen über `--notify-file` von version-control.
- **Stolperfallen**: Dateien mit Japanisch mit Python `-X utf8` schreiben; Shell-Skripte mit LF; Git Bash ändert `/mnt/...`-Pfade; für Zeichenketten mit vielen Anführungszeichen das Write-Werkzeug statt Heredoc.
- **COBOL_to / Java_to**: bereits in `converters/` übernommen. Das Löschen der ursprünglichen Repositorys lässt sich nicht rückgängig machen – vorher die endgültige Bestätigung einholen.
- **Übersetzungssystem** (natürliche Sprache) bleibt von den Konvertern getrennt. Die Konverter hängen nicht von einem LLM ab.
- **Sprachen der Dokumente**: Originale auf Japanisch, dazu neun Sprachen in `docs/i18n/` (KI-Übersetzungen, noch nicht von Muttersprachlern geprüft). Ändert sich ein Original, die Übersetzungen mitziehen. Frühere Entwicklung: `docs/HISTORY-ja.md`.
