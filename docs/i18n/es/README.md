> Nota: el README en japonés de la raíz del repositorio es el original. Esta versión es una traducción hecha con IA, en palabras sencillas, y aún no ha sido revisada por hablantes nativos.

# pl2pl

## ¿Qué es pl2pl?
Los programas están escritos en muchos «idiomas» (lenguajes de programación).
**pl2pl es una herramienta que reescribe un programa de un idioma a otro.**
Por ejemplo, puede convertir un sitio hecho con PHP (Laravel) en un sitio hecho con Python (FastAPI).
Funciona en Windows, Linux y Mac (versión 0.1.0, todavía en construcción).

- Se instala en tu ordenador. El sitio web solo tiene guías y descargas → <https://easy-web.tokyo/pl2pl/>
- Si pl2pl no puede reescribir algo, no lo esconde. Deja un `TODO` (una nota para más tarde) y explica el motivo en un informe.
- Nuestra meta: poder reescribir entre el mayor número posible de lenguajes conocidos.

## Lo que ya puede hacer
| Comando | Qué hace |
|---|---|
| `pl2pl php` | PHP (Laravel / Symfony) → FastAPI. Más en [PHP.md](../../PHP.md) (en japonés) |
| `pl2pl web` | Mira dentro de una carpeta, reconoce Java, COBOL, PHP y Flask/Django, y pasa cada uno a la herramienta adecuada |
| `pl2pl ir` | (versión de prueba) Pone las «rutas» de PHP, Python y Java en una misma forma y genera JSON, OpenAPI o un esqueleto de FastAPI |
| `pl2pl watch` | Encuentra versiones nuevas de lenguajes y herramientas y se pone al día solo |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL (Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | Hace que los bucles de C/C++ vayan más rápido (gpu es una versión de prueba) |
| `pl2pl cport` | Hace que el código C para UNIX funcione tanto en Windows como en Linux |
| `pl2pl pyhpc` | Acelera los cálculos de Python con Numba |
| `pl2pl llm setup` | Instala el ayudante de IA (aruaru-llm). Todavía no hemos comprobado la instalación real |

## Ponerse al día con las versiones nuevas, solo
Ejecutamos `pl2pl watch` cada noche y cada mañana.
1. Mira si han salido versiones nuevas de lenguajes y herramientas (la lista está en `registry/ecosystem.toml`).
2. Prueba con la versión nueva. Si las pruebas pasan, actualiza por sí solo la «versión compatible».
3. Si fallan, version-control (con ayuda de la IA) intenta arreglarlo.
4. Si aun así no se arregla, envía un mensaje a los correos guardados y a LINE.

Ten en cuenta: con laravel, symfony, flask, django y algunos más, por ahora solo avisa de que «ha salido una versión nueva».
En GitHub, también se ejecutan solas cada noche y cada mañana pruebas con las versiones más recientes.

## Cómo se usa
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## Conviene saber
- pl2pl es prudente. Si algo parece arriesgado, lo deja como está y te dice por qué. No podemos prometer velocidad.
- Solo hemos probado ejemplos pequeños. Todavía no hemos probado programas grandes y reales.

## Lo que viene
- Poner también los tipos y el contenido de las funciones en la forma común y hacer pasar por ella la salida de FastAPI.
- PHP: piezas de Blade escritas como clases.
- Comprobar las alertas reales por correo y la corrección automática con IA.
- Comprobar que funciona en la GPU (open-cuda / open-directx).

## Más información
[LANGUAGES.md](../../LANGUAGES.md) (lista de lenguajes) / [COMBINATIONS.md](../../COMBINATIONS.md) (combinaciones) / [DESIGN.md](../../DESIGN.md) (el diseño, escrito por el propietario) / [components/README.md](../../../components/README.md) (piezas que vienen con la herramienta). Estos archivos están en japonés.
