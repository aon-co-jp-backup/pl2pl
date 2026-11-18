> Traducido por IA, aún sin revisión de hablantes nativos. Prevalece el original en japonés (raíz del repositorio).

# Notas de desarrollo de pl2pl
- Función: conversor entre lenguajes/frameworks/SO (CLI en Rust, `src/`). Orígenes candidatos: `docs/LANGUAGES.md`; combinaciones: `docs/COMBINATIONS.md`. El propietario escribe poco a poco la especificación en `docs/DESIGN.md`; no inventar especificación en su nombre.
- Reglas: lo que no se pueda convertir queda como `TODO` y aparece en el informe; nunca se descarta en silencio. Verificar ejecutando de verdad antes de dar algo por terminado. Decisiones prudentes (no convertir ante posible dependencia o efecto secundario).
- PHP: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`; prueba de extremo a extremo `tests/run-php.sh` (FastAPI, SQLite y Jinja2 reales), pruebas unitarias `cargo test --release`. Desde WSL: `wsl -d Ubuntu -- bash <archivo de script>`.
- Trampas: en Windows, escribir los archivos con japonés usando Python `-X utf8`; mantener los scripts de shell en LF; Git Bash reescribe las rutas `/mnt/...`.
- COBOL_to y Java_to los desarrolla otra sesión; integrarlos más tarde y borrar los originales solo tras la confirmación final (irreversible).
- El sistema de traducción (lenguaje natural) está separado de los conversores.
- Idiomas de la documentación: japonés (raíz) y 9 idiomas en `docs/i18n/` (traducciones por IA, sin revisión de hablantes nativos).
