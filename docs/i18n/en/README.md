> Note: the Japanese README in the repository root is the original. This English version was rewritten in plain words so that anyone, even a child, can follow it.

# pl2pl

## What is pl2pl?
Programs are written in many "languages" (programming languages).
**pl2pl is a tool that rewrites a program from one language into another.**
For example, it can change a website made with PHP (Laravel) into a website made with Python (FastAPI).
It works on Windows, Linux and Mac (version 0.1.0, still being built).

- You install it on your computer. The website only has guides and downloads → <https://easy-web.tokyo/pl2pl/>
- If pl2pl cannot rewrite something, it does not hide it. It leaves a `TODO` (a note for later) and tells you why in a report.
- Our goal: to rewrite between as many well-known languages as we can.

## What it can do now
| Command | What it does |
|---|---|
| `pl2pl php` | PHP (Laravel / Symfony) → FastAPI. More in [PHP.md](../../PHP.md) (Japanese) |
| `pl2pl web` | Looks inside a folder, spots Java, COBOL, PHP and Flask/Django, and hands each one to the right tool |
| `pl2pl ir` | (a test version) Puts the "routes" of PHP, Python and Java into one shape, then makes JSON, OpenAPI or a FastAPI starter |
| `pl2pl watch` | Finds new versions of languages and tools, and catches up by itself |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL (Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | Makes C/C++ loops run faster (gpu is a test version) |
| `pl2pl cport` | Makes C code for UNIX run on both Windows and Linux |
| `pl2pl pyhpc` | Makes Python math faster with Numba |
| `pl2pl llm setup` | Installs the AI helper (aruaru-llm). We have not checked the real install yet |

## Catching up with new versions by itself
We run `pl2pl watch` every night and every morning.
1. It checks if new versions of languages and tools are out (the list is in `registry/ecosystem.toml`).
2. It tests with the new version. If the tests pass, it updates the "supported version" by itself.
3. If they fail, version-control (with help from AI) tries to fix it.
4. If it still cannot be fixed, it sends a message to the saved e-mail addresses and LINE.

Please note: for laravel, symfony, flask, django and some others, it only says "a new version is out" for now.
On GitHub, tests with the newest versions also run by themselves every night and every morning.

## How to use it
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## Good to know
- pl2pl is careful. If something looks risky, it leaves it as it is and tells you why. We cannot promise speed.
- We have only tested small samples. We have not tried big, real programs yet.

## What comes next
- Put types and function bodies into the shared shape too, and send the FastAPI output through it.
- PHP: Blade parts that are written as classes.
- Check real e-mail alerts and automatic fixing by AI.
- Check running on the GPU (open-cuda / open-directx).

## Learn more
[LANGUAGES.md](../../LANGUAGES.md) (list of languages) / [COMBINATIONS.md](../../COMBINATIONS.md) (combinations) / [DESIGN.md](../../DESIGN.md) (the design, written by the owner) / [components/README.md](../../../components/README.md) (parts that come with it). These files are in Japanese.
