> Este texto es una traducción hecha con IA y aún no ha sido revisada por hablantes nativos. La versión de referencia es el original en japonés, en la raíz del repositorio.

# pl2pl – notas de desarrollo
- **Función**: un conversor entre lenguajes, frameworks y sistemas operativos (CLI en Rust, `src/`). Los lenguajes de origen están en `docs/LANGUAGES.md` y las combinaciones en `docs/COMBINATIONS.md`. El propietario escribe el diseño poco a poco en `docs/DESIGN.md`. **No inventes especificaciones en su nombre.**
- **Rumbo**: conversión de cualquier lenguaje o framework conocido a cualquier otro. No todo de una vez: primero lo elegido. El sitio web solo ofrece manuales y descargas; la conversión se hace en la aplicación de PC.
- **Reglas**: lo que no se puede convertir queda como `TODO` y se informa (nunca se borra en silencio). Juzgar con prudencia (si puede haber dependencias o efectos secundarios, no convertir). Ejecutarlo de verdad antes de decir «terminado». Sin nuevos REST. Responder solo en japonés.
- **PHP**: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`. Prueba de extremo a extremo: `tests/run-php.sh` (FastAPI real, SQLite, Jinja2); pruebas unitarias: `cargo test --release`. En WSL: `wsl -d Ubuntu -- bash <archivo>`.
- **Actualización automática**: `pl2pl watch` (`src/watch.rs`, `registry/ecosystem.toml`). Las entradas sin comando de verificación solo se muestran como «sin verificar» y nunca se suben solas. Los avisos pasan por `--notify-file` de version-control.
- **Trampas**: escribir los archivos con japonés usando Python `-X utf8`; scripts de shell en LF; Git Bash reescribe las rutas `/mnt/...`; para cadenas con muchas comillas, usar la herramienta Write en lugar de un heredoc.
- **COBOL_to / Java_to**: ya integrados en `converters/`. Borrar los repositorios originales no se puede deshacer: pedir confirmación final antes.
- **Sistema de traducción** (lenguaje natural): separado de los conversores. Los conversores no dependen de un LLM.
- **Idiomas de la documentación**: originales en japonés, más nueve idiomas en `docs/i18n/` (traducciones con IA, sin revisión de nativos). Cuando cambie un original, actualizar las traducciones. Historia anterior: `docs/HISTORY-ja.md`.
