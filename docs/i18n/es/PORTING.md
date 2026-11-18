> Traducido por IA, aún sin revisión de hablantes nativos. Prevalece el original en japonés (raíz del repositorio).

# Traspaso de pl2pl
## Estado (2026-11-19)
La v0.1.0 está publicada (instaladores para Windows/Linux/macOS, pruebas de extremo a extremo en CI). La conversión de PHP cubre rutas, cuerpos de varias sentencias, Eloquent (SQLAlchemy), query builder con paginación, relaciones, Blade (Jinja2) y middleware (`Depends`), todo probado de extremo a extremo. Numba, Kokkos (Serial) y `pl2pl web` están confirmados con ejecuciones reales.
## Pendiente (por prioridad)
1. PHP: `with` anidado, `morph`, `cursorPaginate`, componentes/pilas/autorización de Blade, middleware posterior a `$next`.
2. Integrar COBOL_to / Java_to en pl2pl y luego borrar los originales (confirmar antes; irreversible; otras sesiones trabajan en ellos).
3. Prototipo de la IR común.
4. Integración con aruaru-llm (instalación real de `llm setup` sin verificar).
5. Kokkos GPU / `Kokkos::View`; kernels en open-cuda / open-directx.
6. El propietario escribe `docs/DESIGN.md`; comprobar que `docs/LANGUAGES.md` esté al día.
7. Ejecutar los nuevos ejemplos PHP en CI en Linux arm64 y macOS.
## Pruebas
`bash tests/run-all.sh` (WSL), `bash tests/run-php.sh` (PHP de extremo a extremo), `cargo test --release`.
## Mensaje para retomar
«Retomar F:\pl2pl. Empezar por el punto 1 de PORTING.md. Nunca descartar en silencio lo que no se pueda convertir; dejar TODO. Dar por terminado solo tras ejecutarlo de verdad. Confirmar antes de hacer push.»
