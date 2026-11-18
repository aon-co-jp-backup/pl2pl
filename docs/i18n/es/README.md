> Esta versión es una traducción hecha con IA que todavía no ha revisado ningún hablante nativo. Lo que vale es el original en japonés, en la raíz del repositorio.

# pl2pl
pl2pl convierte código entre lenguajes de programación, frameworks y sistemas operativos. Está en la versión 0.1.0 y sigue en pleno desarrollo. Hay instaladores para Windows (x64/ARM64), Linux (amd64/arm64) y macOS (arm64/x64). La conversión se hace en la aplicación de escritorio instalada (una herramienta de línea de comandos); el sitio web solo ofrece los manuales y las descargas.
El conversor es prudente a propósito. **Lo que no puede convertir nunca se omite en silencio: queda en el resultado como un `TODO` y aparece en el informe junto con el motivo.**

## Lo que funciona hoy
| Comando | Qué hace |
|---|---|
| `pl2pl cobol` | Convierte COBOL de IBM con JCL a GnuCOBOL en Linux con bash. Verificado compilando y ejecutando el resultado con GnuCOBOL. |
| `pl2pl omp` | Añade `#pragma omp parallel for` a los bucles de C/C++ que son independientes entre sí, incluso con reducción de sumas. El resultado coincide con el original. |
| `pl2pl php` | Convierte Laravel / Symfony a FastAPI (detalles más abajo). Probado de punta a punta con FastAPI, SQLite y Jinja2 reales. |
| `pl2pl web` | Detecta Java, COBOL y Flask/Django, y pasa el trabajo a java_to, cobol_to, 2fastapi o 2poem. Verificado con archivos reales. |
| `pl2pl cport` | Convierte C de UNIX en una cabecera de compatibilidad con sustituciones mecánicas y notas de portado. Verificado con gcc. |
| `pl2pl pyhpc` | Añade Numba `@njit` a las funciones numéricas de Python; con `--mpi` también genera un esqueleto de mpi4py. Mismo resultado con Numba. |
| `pl2pl kokkos` | Reescribe los bucles independientes como Kokkos (`parallel_for` / `parallel_reduce`). Compilado y ejecutado con Kokkos 4.5.01 (Serial). |
| `pl2pl gpu` | Prototipo: convierte bucles en `.cu` al estilo CUDA o en HLSL. Solo se ha comprobado la salida generada; ejecutarla en open-cuda / open-directx aún no se ha probado. |
| `pl2pl llm setup` / `llm start` | Arranca el aruaru-llm incluido e instala un modelo Qwen2.5 acorde con la memoria libre. Solo se han verificado la elección del modelo y `--dry-run`, no una instalación real. |
| `pl2pl doctor` | Muestra información de la CPU mediante open-cpu, que va integrado. |

## Conversión de PHP (`pl2pl php`)
- **Rutas**: Laravel `Route::get/post/put/patch/delete/any/resource/apiResource/view`, grupos `prefix` y Symfony `#[Route]`.
- **Cuerpos de función**: `return` simple, cuerpos con varias sentencias, `response()->json($x, 201)` y `noContent()`.
- **Eloquent → SQLAlchemy** (`db.py`, `models.py`): las columnas salen de las migraciones o de `$fillable`; lo demás va a una columna JSON `extra`. `DATABASE_URL` elige la base de datos (SQLite por defecto).
- **Query builder**: `where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`, `paginate / simplePaginate / cursorPaginate` (lee `?page=N`, devuelve JSON con el formato de Laravel y en Blade funciona `links()`), `get / first / firstOrFail / count / exists / delete`, y `$request->query('k', valor por defecto)`.
- **Relaciones**: `hasMany / hasOne / belongsTo / belongsToMany` (se genera la tabla pivote), `with` incluso anidado (`with('posts.comments')`), `whereHas / has / whereDoesntHave / doesntHave`, y `attach / detach / sync`.
- **Blade → Jinja2**: salidas, `@if`, `@foreach`, `@forelse`, `@for`, `@extends/@section/@yield/@include`, etc.
- **Middleware → `Depends`**: `auth` (una simple comprobación Bearer; **hay que sustituirla por una autenticación real**), `guest`, `throttle:N,M` y middleware propio del tipo «rechazar si se cumple una condición; si no, `return $next($request)`».

## Previsto (todavía sin hacer)
- PHP: relaciones `morph`, componentes, pilas y directivas de autorización de Blade, y middleware que modifique el cuerpo de la respuesta después de `$next` (añadir cabeceras ya funciona).
- Incorporar COBOL_to y Java_to a pl2pl cuando estén maduros y borrar los repositorios originales. Es una indicación del propietario; como borrar no se puede deshacer, se confirmará de nuevo tras la incorporación.
- Una representación intermedia común (lado servidor, base de datos/SQL, interfaz) para que N orígenes y M destinos necesiten N+M piezas en lugar de N×M.
- Integración con aruaru-llm. Las explicaciones y la traducción de lenguaje natural siguen separadas de los conversores, que nunca dependen de un LLM.
- Kokkos en GPU y `Kokkos::View`, además de ejecutar kernels en open-cuda / open-directx.
- Ejecutar los nuevos ejemplos de PHP en la CI en Linux arm64 y macOS, y añadir más pares de lenguajes de `docs/LANGUAGES.md` (no se construirán todas las combinaciones).

## Conviene saber
Ante la duda, pl2pl opta por la prudencia. Si una construcción podría tener dependencias o efectos secundarios, **se deja sin convertir y se informa del motivo**, y no se promete ningún rendimiento. Hasta ahora solo se han probado ejemplos pequeños escritos a mano, no programas reales grandes.
Para saber más: `docs/DESIGN.md`, `docs/LANGUAGES.md`, `docs/COMBINATIONS.md`.
