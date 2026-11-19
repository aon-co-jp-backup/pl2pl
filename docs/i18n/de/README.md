> Diese Fassung ist eine KI-Übersetzung und wurde noch nicht von Muttersprachlern geprüft. Maßgeblich ist das japanische Original im Wurzelverzeichnis des Repositorys.

# pl2pl

**pl2pl verwandelt Code aus einer Programmiersprache oder einem Framework in eine andere Sprache oder ein anderes Framework.**
Zum Beispiel wird aus einer PHP-Website (Laravel) eine Python-Website (FastAPI). pl2pl läuft unter Windows, Linux und Mac (v0.1.0, noch in Entwicklung).

- Die Umwandlung passiert in einer App, die Sie auf Ihrem Computer installieren (ein Kommandozeilenprogramm). Die Website bietet nur Anleitungen und Downloads.
- Was sich nicht umwandeln lässt, wird nicht stillschweigend gelöscht. pl2pl lässt ein `TODO` stehen und nennt den Grund im Bericht.
- Das Ziel: bekannte Sprachen und Frameworks in möglichst vielen Kombinationen ineinander umzuwandeln.

## Was heute schon geht
| Befehl | Was er tut |
|---|---|
| `pl2pl php` | PHP (Laravel / Symfony) → FastAPI. Einzelheiten: [PHP.md](../../PHP.md) (Japanisch) |
| `pl2pl web` | Schaut in einen Ordner, erkennt Java, COBOL, PHP und Flask/Django selbst und gibt alles an den passenden Konverter weiter |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL (Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | Schreibt C/C++-Schleifen für OpenMP, Kokkos oder die GPU um (gpu ist ein Prototyp) |
| `pl2pl cport` | Macht UNIX-C-Code unter Windows und Linux lauffähig |
| `pl2pl pyhpc` | Beschleunigt Python-Rechnungen mit Numba |
| `pl2pl watch` | Findet neue Versionen von Sprachen und Frameworks und zieht automatisch nach (siehe unten) |
| `pl2pl llm setup` | Installiert den KI-Helfer (aruaru-llm) passend zum Computer (die echte Installation ist noch nicht geprüft) |

## Neue Versionen automatisch nachziehen
Lassen Sie `pl2pl watch` jeden Abend und jeden Morgen laufen.
1. Es sucht nach neuen Versionen von Sprachen und Frameworks (die Liste steht in `registry/ecosystem.toml`).
2. Es testet mit der neuen Version. Bestehen die Tests, trägt es die „unterstützte Version“ selbst nach.
3. Schlagen sie fehl, versucht version-control (mit KI-Hilfe) das Problem zu beheben.
4. Gelingt auch das nicht, schickt es eine Nachricht an die hinterlegten E-Mail-Adressen und an LINE.

Für die zeitgesteuerte Ausführung gibt es `scripts/pl2pl-watch.cmd` (Windows) und `scripts/pl2pl-watch.sh` (Linux/Mac).
Hinweis: Bei laravel, symfony, flask, django und anderen meldet es bisher nur „Es gibt eine neue Version“. Echte Tests mit den neuen Versionen müssen noch gebaut werden.

## Beispiele
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
```

## Bitte beachten
- pl2pl geht vorsichtig vor. Was riskant aussieht, bleibt unverändert, und der Grund wird genannt. Eine bestimmte Geschwindigkeit wird nicht zugesichert.
- Getestet wurde bisher nur mit kleinen Beispielen, noch nicht mit großen echten Programmen.

## Als Nächstes
- PHP: `@can` (Berechtigungen) und Blade-Komponenten als Klassen.
- Eine gemeinsame Zwischendarstellung (IR), damit viele weitere Sprachpaare auf einmal möglich werden.
- Prüfen, ob E-Mail-Benachrichtigung und automatische Reparatur durch KI wirklich funktionieren.
- Ausführung auf der GPU (open-cuda / open-directx).

## Mehr erfahren
[LANGUAGES.md](../../LANGUAGES.md) (Liste der Sprachen) / [COMBINATIONS.md](../../COMBINATIONS.md) (Kombinationen) / [DESIGN.md](../../DESIGN.md) (Entwurf, vom Eigentümer verfasst) / [components/README.md](../../../components/README.md) (mitgelieferte Teile). Diese Dokumente sind auf Japanisch.
