> Este texto es una traducción hecha con IA y aún no ha sido revisada por hablantes nativos. La versión de referencia es el original en japonés, en la raíz del repositorio.

# pl2pl

**pl2pl convierte código escrito en un lenguaje o framework a otro lenguaje o framework.**
Por ejemplo, un sitio hecho con PHP (Laravel) puede pasar a ser un sitio en Python (FastAPI). Funciona en Windows, Linux y Mac (v0.1.0, en desarrollo).

- La conversión se hace en una aplicación que instalas en tu ordenador (una herramienta de línea de comandos). El sitio web solo ofrece manuales y descargas.
- Lo que no se puede convertir no se borra en silencio: pl2pl deja un `TODO` y explica el motivo en un informe.
- El objetivo es poder convertir entre los lenguajes y frameworks más conocidos, en el mayor número de combinaciones posible.

## Lo que ya funciona
| Comando | Qué hace |
|---|---|
| `pl2pl php` | PHP (Laravel / Symfony) → FastAPI. Detalles: [PHP.md](../../PHP.md) (en japonés) |
| `pl2pl web` | Examina una carpeta, reconoce por sí solo Java, COBOL, PHP y Flask/Django, y pasa cada parte al conversor adecuado |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL (Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | Reescribe bucles de C/C++ para OpenMP, Kokkos o la GPU (gpu es un prototipo) |
| `pl2pl cport` | Hace que el C de UNIX funcione tanto en Windows como en Linux |
| `pl2pl pyhpc` | Acelera los cálculos de Python con Numba |
| `pl2pl watch` | Detecta versiones nuevas de lenguajes y frameworks y se actualiza solo (ver abajo) |
| `pl2pl llm setup` | Instala el asistente de IA (aruaru-llm) según tu ordenador (la instalación real aún no está comprobada) |

## Ponerse al día con las versiones nuevas, de forma automática
Ejecuta `pl2pl watch` cada noche y cada mañana.
1. Busca versiones nuevas de lenguajes y frameworks (la lista está en `registry/ecosystem.toml`).
2. Prueba con la versión nueva. Si las pruebas pasan, actualiza por sí solo la «versión compatible».
3. Si fallan, version-control (con ayuda de IA) intenta arreglarlo.
4. Si aun así no se arregla, avisa a los correos y a LINE registrados.

Los scripts para programarlo son `scripts/pl2pl-watch.cmd` (Windows) y `scripts/pl2pl-watch.sh` (Linux/Mac).
Nota: con laravel, symfony, flask, django y otros, por ahora solo avisa de que «ha salido una versión nueva». Las pruebas reales con las versiones nuevas están por construir.

## Ejemplos
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
```

## Conviene saber
- pl2pl es prudente. Lo que parece arriesgado lo deja sin cambiar y explica por qué. No garantiza la velocidad.
- Solo se ha probado con ejemplos pequeños, todavía no con programas grandes y reales.

## Lo que viene
- PHP: `@can` (permisos) y componentes Blade escritos como clases.
- Una representación intermedia común (IR) para añadir de golpe muchos pares de lenguajes.
- Comprobar el envío real de correos y la corrección automática con IA.
- Ejecución en GPU (open-cuda / open-directx).

## Más información
[LANGUAGES.md](../../LANGUAGES.md) (lista de lenguajes) / [COMBINATIONS.md](../../COMBINATIONS.md) (combinaciones) / [DESIGN.md](../../DESIGN.md) (diseño, escrito por el propietario) / [components/README.md](../../../components/README.md) (piezas incluidas). Estos documentos están en japonés.
