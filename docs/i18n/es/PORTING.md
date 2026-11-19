> Este texto es una traducción hecha con IA y aún no ha sido revisada por hablantes nativos. La versión de referencia es el original en japonés, en la raíz del repositorio.

# pl2pl – traspaso
(Las notas detalladas anteriores están en `docs/HISTORY-ja.md`, en japonés.)

## Situación actual (2026-11-19)
- La versión 0.1.0 está publicada, con instaladores para Windows, Linux y Mac y pruebas en CI.
- PHP → FastAPI está probado de extremo a extremo con FastAPI real (49 comprobaciones): rutas, Eloquent, constructor de consultas, Blade, middleware, relaciones.
- `pl2pl web` ahora también reconoce PHP y lo pasa al conversor.
- Novedad: `pl2pl watch` detecta versiones nuevas, prueba, arregla y, si nada funciona, avisa. version-control ha recibido la opción `--notify-file`.
- COBOL_to y Java_to están integrados en `converters/`.

## Pendiente (por prioridad)
1. **Comprobar**: que llegue un correo real (`version-control --test-notify`), la corrección automática con `version-control --loop` y una prueba con el último fastapi (la ejecución nocturna de CI lo comprobará).
2. Probar `watch` con versiones nuevas reales (laravel, symfony, spring-boot, flask, django y poem siguen como «sin verificar»).
3. Registrar las ejecuciones de la noche y de la mañana (`schtasks /Create`, etc.; aún no hecho, porque es la configuración personal del propietario).
4. Prototipo de la representación intermedia común (IR), base para añadir muchos pares de lenguajes.
5. PHP: componentes Blade escritos como clases, `$attributes`, `@php`, `morphToMany`.
6. Integración con aruaru-llm (la instalación real con `llm setup` no está comprobada). Kokkos en GPU; núcleos en open-cuda / open-directx.
7. Borrar los COBOL_to y Java_to originales (**irreversible: pedir antes confirmación final**).
8. El propietario escribe `docs/DESIGN.md`. Comprobar que `docs/LANGUAGES.md` no esté desactualizado.

## Pruebas
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP de extremo a extremo) / `cargo test --release`

## Mensaje para retomar
«Retomar F:\pl2pl. Seguir «Pendiente» de PORTING.md desde arriba. Nunca borrar en silencio lo que no se pueda convertir: dejar un TODO. Comprobar ejecutándolo de verdad antes de dar algo por terminado. Responder solo en japonés. Hacer push en cada hito.»
