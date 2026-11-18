> Esta versión es una traducción hecha con IA que todavía no ha revisado ningún hablante nativo. Lo que vale es el original en japonés, en la raíz del repositorio.

# pl2pl: traspaso
## Cómo están las cosas (2026-11-19)
La versión 0.1.0 está publicada, con instaladores para Windows, Linux y macOS y pruebas de punta a punta en la CI. La conversión de PHP cubre rutas, cuerpos con varias sentencias, Eloquent (SQLAlchemy), el query builder con paginación, relaciones (incluido `with` anidado), Blade (Jinja2) y middleware (`Depends`), todo probado de punta a punta. Numba, Kokkos (Serial) y `pl2pl web` se han confirmado con ejecuciones reales.
## Pendiente, por orden de prioridad
1. PHP: `morph`, componentes/pilas/autorización de Blade, middleware que modifique el cuerpo tras `$next`.
2. Incorporar COBOL_to / Java_to a pl2pl y luego borrar los originales (confirmar antes; no se puede deshacer; otras sesiones trabajan en ellos).
3. Prototipo de la representación intermedia común.
4. Integración con aruaru-llm (una instalación real con `llm setup` sigue sin verificarse).
5. Kokkos en GPU / `Kokkos::View`; kernels en open-cuda / open-directx.
6. El propietario escribe `docs/DESIGN.md`; comprobar que `docs/LANGUAGES.md` esté al día.
7. Ejecutar los nuevos ejemplos de PHP en la CI en Linux arm64 y macOS.
## Pruebas
`bash tests/run-all.sh` (WSL), `bash tests/run-php.sh` (PHP de punta a punta), `cargo test --release`.
## Mensaje para retomar el trabajo
«Retoma F:\pl2pl. Empieza por el punto 1 de PORTING.md. Nunca omitas en silencio lo que no se pueda convertir: deja un TODO. Verifica ejecutando de verdad antes de dar algo por terminado. Pregúntame antes de hacer push.»
