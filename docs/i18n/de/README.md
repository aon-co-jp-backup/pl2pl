> Diese Fassung ist eine KI-Übersetzung und wurde noch nicht von Muttersprachlern geprüft. Maßgeblich ist das japanische Original im Wurzelverzeichnis des Repositorys.

# pl2pl
pl2pl wandelt Code zwischen Programmiersprachen, Frameworks und Betriebssystemen um. Das Projekt steht bei Version 0.1.0 und wird aktiv weiterentwickelt. Installer gibt es für Windows (x64/ARM64), Linux (amd64/arm64) und macOS (arm64/x64). Die Umwandlung läuft in der installierten Desktop-Anwendung (einem Kommandozeilenprogramm); die Website bietet nur Handbücher und Downloads.
Der Konverter geht bewusst vorsichtig vor. **Was sich nicht umwandeln lässt, wird nie stillschweigend weggelassen, sondern bleibt als `TODO` im Ergebnis stehen und wird samt Begründung im Bericht aufgeführt.**

## Was heute funktioniert
| Befehl | Was er tut |
|---|---|
| `pl2pl cobol` | Wandelt IBM-COBOL samt JCL in GnuCOBOL unter Linux samt bash um. Geprüft, indem das Ergebnis mit GnuCOBOL tatsächlich gebaut und ausgeführt wurde. |
| `pl2pl omp` | Setzt `#pragma omp parallel for` vor voneinander unabhängige C/C++-Schleifen, auch mit Summenreduktion. Das Ergebnis stimmt mit dem Original überein. |
| `pl2pl php` | Wandelt Laravel / Symfony in FastAPI um (Details unten). Durchgängig gegen echtes FastAPI, SQLite und Jinja2 getestet. |
| `pl2pl web` | Erkennt Java, COBOL sowie Flask/Django und übergibt an java_to, cobol_to, 2fastapi oder 2poem. An echten Dateien geprüft. |
| `pl2pl cport` | Macht aus UNIX-C-Code einen Kompatibilitäts-Header mit mechanischen Ersetzungen und Portierungshinweisen. Mit gcc geprüft. |
| `pl2pl pyhpc` | Versieht numerische Python-Funktionen mit Numba `@njit`; mit `--mpi` entsteht zusätzlich ein mpi4py-Gerüst. Unter Numba identisches Ergebnis. |
| `pl2pl kokkos` | Schreibt unabhängige Schleifen als Kokkos um (`parallel_for` / `parallel_reduce`). Mit Kokkos 4.5.01 (Serial) gebaut und ausgeführt. |
| `pl2pl gpu` | Prototyp: macht aus Schleifen CUDA-artige `.cu`-Dateien oder HLSL. Geprüft wurde nur die erzeugte Ausgabe; die Ausführung auf open-cuda / open-directx ist noch nicht getestet. |
| `pl2pl llm setup` / `llm start` | Startet das mitgelieferte aruaru-llm und installiert ein Qwen2.5-Modell, das zum freien Arbeitsspeicher passt. Geprüft sind nur die Modellauswahl und `--dry-run`, nicht eine echte Installation. |
| `pl2pl doctor` | Zeigt CPU-Informationen über das eingebaute open-cpu an. |

## PHP-Umwandlung (`pl2pl php`)
- **Routen**: Laravel `Route::get/post/put/patch/delete/any/resource/apiResource/view`, `prefix`-Gruppen und Symfony `#[Route]`.
- **Funktionsrümpfe**: einfaches `return`, Rümpfe mit mehreren Anweisungen, `response()->json($x, 201)` und `noContent()`.
- **Eloquent → SQLAlchemy** (`db.py`, `models.py`): Die Spalten stammen aus den Migrationen oder aus `$fillable`, alles Übrige landet in einer JSON-Spalte `extra`. Über `DATABASE_URL` wird die Datenbank gewählt (standardmäßig SQLite).
- **Query Builder**: `where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`, `paginate / simplePaginate` (liest `?page=N`, liefert JSON im Laravel-Format, in Blade funktioniert `links()`), `get / first / firstOrFail / count / exists / delete` sowie `$request->query('k', Standardwert)`.
- **Relationen**: `hasMany / hasOne / belongsTo / belongsToMany` (Zwischentabelle wird erzeugt), `with` auch verschachtelt als `with('posts.comments')`, `whereHas / has / whereDoesntHave / doesntHave` sowie `attach / detach / sync`.
- **Blade → Jinja2**: Ausgaben, `@if`, `@foreach`, `@forelse`, `@for`, `@extends/@section/@yield/@include` und mehr.
- **Middleware → `Depends`**: `auth` (nur eine einfache Bearer-Prüfung – **bitte durch eine echte Authentifizierung ersetzen**), `guest`, `throttle:N,M` und eigene Middleware nach dem Muster „bei Bedingung ablehnen, sonst `return $next($request)`“.

## Geplant (noch nicht umgesetzt)
- PHP: `morph`-Relationen, `cursorPaginate`, Blade-Komponenten, -Stacks und -Autorisierungsdirektiven sowie Middleware, die die Antwort nach `$next` nachbearbeitet.
- COBOL_to und Java_to in pl2pl aufnehmen, sobald sie ausgereift sind, und die ursprünglichen Repositorys löschen. Das ist die Vorgabe des Eigentümers; da sich das Löschen nicht rückgängig machen lässt, wird es nach der Übernahme noch einmal bestätigt.
- Eine gemeinsame Zwischendarstellung (Serverseite, Datenbank/SQL, Oberfläche), damit N Quellen und M Ziele nur N+M statt N×M Bausteine brauchen.
- Anbindung an aruaru-llm. Erklärungen und die Übersetzung natürlicher Sprache bleiben vom Konverter getrennt; der Konverter selbst hängt nie von einem LLM ab.
- Kokkos auf der GPU und `Kokkos::View` sowie die Ausführung von Kernels auf open-cuda / open-directx.
- Die neuen PHP-Beispiele in der CI unter Linux arm64 und macOS ausführen und weitere Sprachpaare aus `docs/LANGUAGES.md` ergänzen (nicht jede Kombination wird gebaut).

## Gut zu wissen
pl2pl entscheidet im Zweifel vorsichtig. Wo ein Konstrukt Abhängigkeiten oder Nebenwirkungen haben könnte, wird es **nicht umgewandelt, sondern mit Begründung gemeldet**; eine bestimmte Laufzeit-Leistung wird nicht zugesichert. Getestet wurden bisher nur kleine, selbst geschriebene Beispiele, keine großen realen Programme.
Weiterführend: `docs/DESIGN.md`, `docs/LANGUAGES.md`, `docs/COMBINATIONS.md`.
