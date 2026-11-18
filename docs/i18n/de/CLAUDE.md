> Diese Fassung ist eine KI-Übersetzung und wurde noch nicht von Muttersprachlern geprüft. Maßgeblich ist das japanische Original im Wurzelverzeichnis des Repositorys.

# pl2pl CLAUDE
- **Rolle**: ein Konverter zwischen Sprachen, Frameworks und Betriebssystemen (Rust-Kommandozeilenprogramm in `src/`). Mögliche Quellen stehen in `docs/LANGUAGES.md`, Kombinationen in `docs/COMBINATIONS.md`. Die Entwurfsspezifikation schreibt der Eigentümer nach und nach in `docs/DESIGN.md`; erfinde keine Spezifikation in seinem Namen.
- **Regeln**: Was sich nicht umwandeln lässt, bleibt als `TODO` stehen und kommt in den Bericht – niemals stillschweigend weglassen. Vor der Meldung „fertig“ alles tatsächlich ausführen. Im Zweifel vorsichtig bleiben: Bei möglichen Abhängigkeiten oder Nebenwirkungen nicht umwandeln.
- **PHP**: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`. Ende-zu-Ende-Test: `tests/run-php.sh` (echtes FastAPI, SQLite, Jinja2). Unit-Tests: `cargo test --release`. Aus WSL starten mit `wsl -d Ubuntu -- bash <Skriptdatei>`.
- **Stolperfallen**: Unter Windows Dateien mit japanischem Text mit Python `-X utf8` schreiben; Shell-Skripte mit LF-Zeilenenden belassen; Git Bash schreibt `/mnt/...`-Pfade um.
- **COBOL_to / Java_to**: Daran arbeiten andere Sitzungen. Später übernehmen und die Originale erst nach einer letzten Bestätigung löschen – das lässt sich nicht rückgängig machen.
- **Übersetzungssystem** (natürliche Sprache): bleibt vom Konverter getrennt.
- **Dokumentationssprachen**: Japanisch im Wurzelverzeichnis sowie neun weitere Sprachen unter `docs/i18n/` (KI-Übersetzungen, noch nicht von Muttersprachlern geprüft). Ändert sich das japanische Original, die Übersetzungen bitte mitziehen.
