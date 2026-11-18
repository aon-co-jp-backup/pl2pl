> Traducido por IA, aún sin revisión de hablantes nativos. Prevalece el original en japonés (raíz del repositorio).

# pl2pl
Un **conversor entre lenguajes de programación, frameworks y sistemas operativos** (v0.1.0 publicada, en desarrollo). Hay instaladores para Windows (x64/ARM64), Linux (amd64/arm64) y macOS (arm64/x64). La conversión se hace en la aplicación de PC instalada (CLI); la versión web solo ofrece manuales y descargas.
La conversión es prudente: **lo que no se puede convertir nunca se descarta en silencio; queda como `TODO` y aparece en el informe con su motivo.**

## Qué funciona hoy
| Comando | Descripción |
|---|---|
| `pl2pl cobol` | COBOL (IBM) + JCL → GnuCOBOL (Linux) + bash. Verificado compilando y ejecutando con GnuCOBOL. |
| `pl2pl omp` | Añade `#pragma omp parallel for` a bucles C/C++ independientes (con reducción de sumas). Mismo resultado que el original. |
| `pl2pl php` | Laravel / Symfony → FastAPI (véase abajo). Probado de extremo a extremo con FastAPI, SQLite y Jinja2 reales. |
| `pl2pl web` | Detecta Java / COBOL / Flask·Django y lo pasa a java_to, cobol_to, 2fastapi, 2poem. Verificado con archivos reales. |
| `pl2pl cport` | C de UNIX → cabecera de compatibilidad + sustitución mecánica + guía de portado. Verificado con gcc. |
| `pl2pl pyhpc` | Numba `@njit` para funciones Python numéricas; `--mpi` genera un esqueleto mpi4py. Mismo resultado con Numba. |
| `pl2pl kokkos` | Bucles independientes → Kokkos (`parallel_for` / `parallel_reduce`). Compilado y ejecutado con Kokkos 4.5.01 (Serial). |
| `pl2pl gpu` | (Prototipo) bucles → `.cu` estilo CUDA / HLSL. Solo salida; la ejecución en open-cuda / open-directx no está verificada. |
| `pl2pl llm setup` / `llm start` | Inicia el aruaru-llm incluido e instala un modelo Qwen2.5 acorde con la memoria libre. Solo se verificaron la lógica de selección y --dry-run. |
| `pl2pl doctor` | Información de CPU mediante open-cpu (integrado). |

## Conversión de PHP (`pl2pl php`)
- **Rutas**: Laravel `Route::get/post/put/patch/delete/any/resource/apiResource/view`, grupos `prefix`, Symfony `#[Route]`.
- **Cuerpos**: `return` simple, varias sentencias, `response()->json($x, 201)`, `noContent()`.
- **Eloquent → SQLAlchemy** (`db.py`, `models.py`): las columnas salen de las migraciones o de `$fillable`; el resto va a una columna JSON `extra`; `DATABASE_URL` elige la base de datos (SQLite por defecto).
- **Query builder**: `where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`, `paginate / simplePaginate` (`?page=N`, JSON como en Laravel, `links()` en Blade), `get / first / firstOrFail / count / exists / delete`, `$request->query('k', valor)`.
- **Relaciones**: `hasMany / hasOne / belongsTo / belongsToMany` (se genera la tabla pivote), `with`, `whereHas / has / whereDoesntHave / doesntHave`, `attach / detach / sync`.
- **Blade → Jinja2**: salida, `@if`, `@foreach`, `@forelse`, `@for`, `@extends/@section/@yield/@include`, etc.
- **Middleware → `Depends`**: `auth` (comprobación Bearer simple, **sustituir por autenticación real**), `guest`, `throttle:N,M`, middleware propio «rechazar según una condición; si no, `return $next($request)`».

## Previsto (aún sin hacer)
- PHP: `with('a.b')` anidado, relaciones `morph`, `cursorPaginate`, componentes/pilas/directivas de autorización de Blade, middleware que procesa la respuesta después de `$next`.
- Integrar COBOL_to y Java_to en pl2pl cuando estén maduros y borrar los repositorios originales (indicación del propietario; borrar es irreversible, se confirmará de nuevo tras la integración).
- Una representación intermedia común (servidor, BD/SQL, interfaz) para que N orígenes y M destinos necesiten N+M piezas y no N×M.
- Integración con aruaru-llm (las explicaciones y la traducción de lenguaje natural siguen separadas de los conversores; los conversores nunca dependen de un LLM).
- Ejecución en GPU con Kokkos y `Kokkos::View`; ejecución de kernels en open-cuda / open-directx.
- Ejecutar los nuevos ejemplos PHP en CI en Linux arm64 y macOS; más pares de lenguajes de `docs/LANGUAGES.md` (no se construirán todas las combinaciones).

## Notas
Las decisiones son prudentes: ante posibles dependencias o efectos secundarios, la construcción **se deja sin convertir y se informa del motivo**; no se garantiza el rendimiento. Solo se han probado pequeños ejemplos propios, no programas reales grandes.
Documentos: `docs/DESIGN.md`, `docs/LANGUAGES.md`, `docs/COMBINATIONS.md`.
