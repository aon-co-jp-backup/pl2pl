> Nota: el PORTING.md en japonés de la raíz del repositorio es el original. Esta versión es una traducción hecha con IA, en palabras sencillas, y aún no ha sido revisada por hablantes nativos.

# pl2pl – traspaso
(Las notas detalladas anteriores están en `docs/HISTORY-ja.md`, en japonés.)

## Dónde estamos (2026-11-19)
- La versión 0.1.0 está publicada, con instaladores para Windows, Linux y Mac y pruebas en CI.
- PHP → FastAPI está probado de extremo a extremo con FastAPI real: rutas, Eloquent, constructor de consultas, Blade, middleware, relaciones y la familia `@can`.
- `pl2pl web` ahora también reconoce PHP.
- `pl2pl watch` encuentra versiones nuevas, las prueba, arregla problemas y envía una alerta si nada funciona. GitHub también prueba con las versiones más recientes cada noche y cada mañana.
- `pl2pl ir` (versión de prueba) pone las rutas de PHP, Python y Java (Spring, JAX-RS, Servlet, Spark) en una forma común y genera JSON, OpenAPI o un esqueleto de FastAPI. Hemos comprobado que Python puede leer de verdad la salida.
- La página de presentación y descarga está en línea: <https://easy-web.tokyo/pl2pl/> (en el VPS, estática).
- COBOL_to y Java_to están integrados en `converters/`.

## Pendiente (lo más importante primero)
1. **Comprobar**: que llegue un correo real (`version-control --test-notify`; este PC no tiene ajustes SMTP), la corrección automática con version-control `--loop` y una prueba con el último fastapi (la ejecución nocturna en GitHub lo comprobará).
2. Poner los tipos y el contenido de las funciones en la IR. Hacer pasar por la IR la salida actual de FastAPI (`pl2pl php`). Si la búsqueda de rutas se deja cosas, pensar en tree-sitter.
3. Probar `watch` con versiones nuevas reales (laravel, symfony, spring-boot, flask, django y poem siguen contando como sin comprobar).
4. Programar en un PC las ejecuciones de la noche y de la mañana (aún no hecho: el comando de comprobación no funciona en este PC y daría falsas alarmas; hay un ejemplo al principio de `scripts/pl2pl-watch.cmd`).
5. Añadir una tarjeta de pl2pl en la página principal de easy-web.tokyo.
6. PHP: piezas de Blade escritas como clases, `$attributes`, `@php`, `morphToMany`.
7. Integración con aruaru-llm (la instalación real con `llm setup` no está comprobada). Kokkos en GPU; núcleos en open-cuda / open-directx.
8. Borrar los COBOL_to y Java_to originales (**no se puede deshacer: pedir antes la aprobación final**).
9. El propietario escribe `docs/DESIGN.md`. Comprobar que `docs/LANGUAGES.md` no esté desactualizado.

## Pruebas
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP de extremo a extremo) / `cargo test --release`

## Mensaje para retomar
«Retomar F:\pl2pl. Seguir «Pendiente» de PORTING.md desde arriba. Nunca esconder lo que no se pueda reescribir: dejar un TODO. Comprobar ejecutándolo de verdad antes de decir que está terminado. Responder solo en japonés. Hacer commit y push en cada paso y actualizar también el VPS.»
