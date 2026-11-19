> Hinweis: Das japanische README im Wurzelverzeichnis ist das Original. Diese Fassung ist eine KI-Übersetzung in einfachen Worten und wurde noch nicht von Muttersprachlern geprüft.

# pl2pl

## Was ist pl2pl?
Programme sind in vielen „Sprachen“ (Programmiersprachen) geschrieben.
**pl2pl ist ein Werkzeug, das ein Programm aus einer Sprache in eine andere umschreibt.**
Zum Beispiel kann es eine Website aus PHP (Laravel) in eine Website aus Python (FastAPI) verwandeln.
Es läuft unter Windows, Linux und Mac (Version 0.1.0, noch im Bau).

- Man installiert es auf dem eigenen Computer. Auf der Website gibt es nur Anleitungen und Downloads → <https://easy-web.tokyo/pl2pl/>
- Was pl2pl nicht umschreiben kann, versteckt es nicht. Es lässt ein `TODO` (eine Notiz für später) stehen und nennt den Grund im Bericht.
- Unser Ziel: zwischen möglichst vielen bekannten Sprachen umschreiben zu können.

## Was heute schon geht
| Befehl | Was er tut |
|---|---|
| `pl2pl php` | PHP (Laravel / Symfony) → FastAPI. Mehr in [PHP.md](../../PHP.md) (Japanisch) |
| `pl2pl web` | Schaut in einen Ordner, erkennt Java, COBOL, PHP und Flask/Django und gibt jedes an das passende Werkzeug |
| `pl2pl ir` | (Testversion) Bringt die „Routen“ von PHP, Python und Java in eine gemeinsame Form und erzeugt JSON, OpenAPI oder ein FastAPI-Gerüst |
| `pl2pl watch` | Findet neue Versionen von Sprachen und Werkzeugen und zieht von selbst nach |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL (Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | Macht C/C++-Schleifen schneller (gpu ist eine Testversion) |
| `pl2pl cport` | Macht C-Code für UNIX unter Windows und Linux lauffähig |
| `pl2pl pyhpc` | Macht Python-Rechnungen mit Numba schneller |
| `pl2pl llm setup` | Installiert den KI-Helfer (aruaru-llm). Die echte Installation haben wir noch nicht geprüft |

## Neue Versionen von selbst nachziehen
Wir lassen `pl2pl watch` jeden Abend und jeden Morgen laufen.
1. Es prüft, ob es neue Versionen von Sprachen und Werkzeugen gibt (die Liste steht in `registry/ecosystem.toml`).
2. Es testet mit der neuen Version. Bestehen die Tests, trägt es die „unterstützte Version“ selbst nach.
3. Schlagen sie fehl, versucht version-control (mit Hilfe von KI) das Problem zu beheben.
4. Gelingt auch das nicht, schickt es eine Nachricht an die gespeicherten E-Mail-Adressen und an LINE.

Bitte beachten: Bei laravel, symfony, flask, django und einigen anderen meldet es bisher nur „Es gibt eine neue Version“.
Auf GitHub laufen außerdem jeden Abend und jeden Morgen Tests mit den neuesten Versionen von selbst.

## So benutzt man es
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## Gut zu wissen
- pl2pl ist vorsichtig. Was riskant aussieht, lässt es unverändert und nennt den Grund. Geschwindigkeit können wir nicht versprechen.
- Wir haben bisher nur kleine Beispiele getestet. Große, echte Programme haben wir noch nicht ausprobiert.

## Wie es weitergeht
- Auch Typen und Funktionsinhalte in die gemeinsame Form bringen und die FastAPI-Ausgabe darüber laufen lassen.
- PHP: Blade-Bausteine, die als Klassen geschrieben sind.
- Echte E-Mail-Warnungen und die automatische Reparatur durch KI prüfen.
- Prüfen, ob es auf der GPU läuft (open-cuda / open-directx).

## Mehr erfahren
[LANGUAGES.md](../../LANGUAGES.md) (Liste der Sprachen) / [COMBINATIONS.md](../../COMBINATIONS.md) (Kombinationen) / [DESIGN.md](../../DESIGN.md) (der Entwurf, vom Eigentümer geschrieben) / [components/README.md](../../../components/README.md) (mitgelieferte Teile). Diese Dateien sind auf Japanisch.
