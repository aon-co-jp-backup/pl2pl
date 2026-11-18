> Esta versión es una traducción hecha con IA que todavía no ha revisado ningún hablante nativo. Lo que vale es el original en japonés, en la raíz del repositorio.

# pl2pl CLAUDE
- **Función**: un conversor entre lenguajes, frameworks y sistemas operativos (programa de línea de comandos en Rust, en `src/`). Los orígenes posibles están en `docs/LANGUAGES.md` y las combinaciones en `docs/COMBINATIONS.md`. El propietario va escribiendo poco a poco la especificación en `docs/DESIGN.md`; no inventes especificación en su nombre.
- **Reglas**: lo que no se pueda convertir queda como `TODO` y va al informe; nunca lo omitas en silencio. Ejecuta de verdad antes de decir que una tarea está terminada. Mantente prudente: si hay posibles dependencias o efectos secundarios, no conviertas.
- **PHP**: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`. Prueba de punta a punta: `tests/run-php.sh` (FastAPI, SQLite y Jinja2 reales). Pruebas unitarias: `cargo test --release`. Desde WSL: `wsl -d Ubuntu -- bash <archivo de script>`.
- **Trampas**: en Windows, escribe los archivos con texto en japonés usando Python con `-X utf8`; mantén los scripts de shell con saltos de línea LF; Git Bash reescribe las rutas `/mnt/...`.
- **COBOL_to / Java_to**: otras sesiones los están desarrollando. Se incorporarán más adelante, y los originales solo se borran tras una confirmación final, porque no se puede deshacer.
- **Sistema de traducción** (lenguaje natural): se mantiene separado de los conversores.
- **Idiomas de la documentación**: japonés en la raíz y otros nueve idiomas en `docs/i18n/` (traducciones con IA, aún sin revisión de hablantes nativos). Cuando cambie el original en japonés, actualiza también las traducciones.
