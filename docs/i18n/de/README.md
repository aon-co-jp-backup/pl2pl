> KI-Übersetzung, noch nicht von Muttersprachlern geprüft. Maßgeblich ist das japanische Original (Repository-Wurzel).

# pl2pl
Ein **Konverter zwischen Programmiersprachen, Frameworks und Betriebssystemen** (v0.1.0 veröffentlicht, in Entwicklung). Installer gibt es für Windows (x64/ARM64), Linux (amd64/arm64) und macOS (arm64/x64). Die Umwandlung läuft in der installierten PC-Anwendung (CLI); die Webversion bietet nur Handbücher und Downloads.
Die Umwandlung ist konservativ: **Was sich nicht umwandeln lässt, wird nie stillschweigend verworfen, sondern bleibt als `TODO` stehen und wird mit Begründung im Bericht aufgeführt.**

## Was heute funktioniert
| Befehl | Beschreibung |
|---|---|
| `pl2pl cobol` | COBOL (IBM) + JCL → GnuCOBOL (Linux) + bash. Mit GnuCOBOL tatsächlich gebaut und ausgeführt. |
| `pl2pl omp` | Fügt unabhängigen C/C++-Schleifen `#pragma omp parallel for` hinzu (inkl. Summenreduktion). Gleiches Ergebnis wie das Original. |
| `pl2pl php` | Laravel / Symfony → FastAPI (siehe unten). Durchgängig mit echtem FastAPI, SQLite und Jinja2 getestet. |
| `pl2pl web` | Erkennt Java / COBOL / Flask·Django und übergibt an java_to, cobol_to, 2fastapi, 2poem. An echten Dateien geprüft. |
| `pl2pl cport` | UNIX-C → Kompatibilitäts-Header + mechanische Ersetzung + Portierungshinweise. Mit gcc geprüft. |
| `pl2pl pyhpc` | Numba `@njit` für numerische Python-Funktionen; `--mpi` erzeugt ein mpi4py-Gerüst. Gleiches Ergebnis unter Numba. |
| `pl2pl kokkos` | Unabhängige Schleifen → Kokkos (`parallel_for` / `parallel_reduce`). Mit Kokkos 4.5.01 (Serial) gebaut und ausgeführt. |
| `pl2pl gpu` | (Prototyp) Schleifen → CUDA-Form `.cu` / HLSL. Nur Ausgabe; Ausführung auf open-cuda / open-directx ungeprüft. |
| `pl2pl llm setup` / `llm start` | Startet das mitgelieferte aruaru-llm und installiert ein zum freien Speicher passendes Qwen2.5-Modell. Nur Auswahllogik und --dry-run geprüft. |
| `pl2pl doctor` | CPU-Informationen über open-cpu (eingebaut). |

## PHP-Umwandlung (`pl2pl php`)
- **Routen**: Laravel `Route::get/post/put/patch/delete/any/resource/apiResource/view`, `prefix`-Gruppen, Symfony `#[Route]`.
- **Rümpfe**: einfaches `return`, mehrere Anweisungen, `response()->json($x, 201)`, `noContent()`.
- **Eloquent → SQLAlchemy** (`db.py`, `models.py`): Spalten aus Migrationen oder `$fillable`, übrige Felder in einer JSON-Spalte `extra`; `DATABASE_URL` wählt die Datenbank (Standard SQLite).
- **Query Builder**: `where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`, `paginate / simplePaginate` (`?page=N`, JSON wie in Laravel, `links()` in Blade), `get / first / firstOrFail / count / exists / delete`, `$request->query('k', Standard)`.
- **Relationen**: `hasMany / hasOne / belongsTo / belongsToMany` (Zwischentabelle wird erzeugt), `with`, `whereHas / has / whereDoesntHave / doesntHave`, `attach / detach / sync`.
- **Blade → Jinja2**: Ausgabe, `@if`, `@foreach`, `@forelse`, `@for`, `@extends/@section/@yield/@include` u. a.
- **Middleware → `Depends`**: `auth` (einfache Bearer-Prüfung, **durch echte Authentifizierung ersetzen**), `guest`, `throttle:N,M`, eigene Middleware „bei Bedingung ablehnen, sonst `return $next($request)`“.

## Geplant (noch offen)
- PHP: verschachteltes `with('a.b')`, `morph`-Relationen, `cursorPaginate`, Blade-Komponenten/Stacks/Autorisierungsdirektiven, Middleware, die die Antwort nach `$next` nachbearbeitet.
- COBOL_to und Java_to nach ausreichender Reife in pl2pl übernehmen und die Originale löschen (Anweisung des Eigentümers; Löschen ist nicht umkehrbar und wird nach der Übernahme noch einmal bestätigt).
- Gemeinsame Zwischendarstellung (Server, DB/SQL, Oberfläche), damit N Quellen und M Ziele nur N+M statt N×M Bausteine brauchen.
- Anbindung an aruaru-llm (Erklärungen und natürlichsprachliche Übersetzung bleiben vom Konverter getrennt; Konverter hängen nie von einem LLM ab).
- Kokkos-GPU-Ausführung und `Kokkos::View`; Kernel-Ausführung auf open-cuda / open-directx.
- CI-Läufe der neuen PHP-Beispiele auf Linux arm64 und macOS; weitere Sprachpaare aus `docs/LANGUAGES.md` (nicht jede Kombination wird gebaut).

## Hinweise
Die Entscheidungen sind konservativ: Bei möglichen Abhängigkeiten oder Nebenwirkungen wird **nicht umgewandelt und der Grund gemeldet**; Leistung wird nicht zugesichert. Getestet wurden nur kleine selbstgeschriebene Beispiele, keine großen realen Programme.
Dokumente: `docs/DESIGN.md`, `docs/LANGUAGES.md`, `docs/COMBINATIONS.md`.
