> This is an AI translation that native speakers have not yet reviewed. The Japanese original in the repository root is the authoritative version.

# pl2pl

**pl2pl turns code written in one programming language or framework into another.**
For example, it can turn a PHP (Laravel) site into a Python (FastAPI) site. It works on Windows, Linux and Mac (v0.1.0, still in development).

- You convert with an app you install on your computer (a command-line tool). The website only has manuals and downloads.
- If something cannot be converted, pl2pl does not quietly delete it. It leaves a `TODO` and explains why in a report.
- The goal is to convert between as many well-known languages and frameworks as possible.

## What it can do today
| Command | What it does |
|---|---|
| `pl2pl php` | PHP (Laravel / Symfony) → FastAPI. Details: [PHP.md](../../PHP.md) (Japanese) |
| `pl2pl web` | Looks at a folder, recognizes Java, COBOL, PHP and Flask/Django by itself, and passes each to the right converter |
| `pl2pl ir` | (prototype) Turns PHP, Python and Java routes into a common form, then writes JSON, OpenAPI or a FastAPI skeleton |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL (Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | Rewrites C/C++ loops for OpenMP, Kokkos or the GPU (gpu is a prototype) |
| `pl2pl cport` | Makes UNIX C code run on both Windows and Linux |
| `pl2pl pyhpc` | Speeds up Python math with Numba |
| `pl2pl watch` | Finds new versions of languages and frameworks and keeps up automatically (see below) |
| `pl2pl llm setup` | Installs the AI helper (aruaru-llm) to match your computer (a real install is not verified yet) |

## Keeping up with new versions, automatically
Run `pl2pl watch` every night and every morning.
1. It checks for new versions of languages and frameworks (the list is in `registry/ecosystem.toml`).
2. It tests with the new version. If the tests pass, it updates the "supported version" by itself.
3. If they fail, version-control (with AI help) tries to fix it.
4. If that still fails, it sends a message to the registered e-mail addresses and LINE.

Scripts for scheduling are `scripts/pl2pl-watch.cmd` (Windows) and `scripts/pl2pl-watch.sh` (Linux/Mac).
Note: for laravel, symfony, flask, django and others, it only tells you "a new version is out" for now. Real tests against the new versions are still to be built.

## Examples
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
```

## Please note
- pl2pl is careful. If something looks risky, it leaves it unchanged and tells you why. Speed is not guaranteed.
- It has only been tested on small samples, not yet on large real programs.

## Coming next
- PHP: Blade components written as classes.
- A shared intermediate representation (IR), so many more language pairs can be added at once.
- Checking real e-mail delivery and automatic fixing by AI.
- Running on the GPU (open-cuda / open-directx).

## Learn more
[LANGUAGES.md](../../LANGUAGES.md) (list of languages) / [COMBINATIONS.md](../../COMBINATIONS.md) (combinations) / [DESIGN.md](../../DESIGN.md) (design, written by the owner) / [components/README.md](../../../components/README.md) (bundled parts). These documents are in Japanese.
