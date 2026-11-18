> Traduit par IA, pas encore relu par des locuteurs natifs. L'original japonais (racine du dépôt) fait foi.

# Notes de développement pl2pl
- Rôle : convertisseur entre langages/frameworks/OS (CLI Rust, `src/`). Sources candidates : `docs/LANGUAGES.md` ; combinaisons : `docs/COMBINATIONS.md`. Le propriétaire rédige peu à peu la spécification dans `docs/DESIGN.md` ; ne jamais inventer de spécification à sa place.
- Règles : les constructions non convertibles restent en `TODO` et figurent dans le rapport ; ne jamais les abandonner en silence. Vérifier en exécutant réellement avant de déclarer terminé. Décisions prudentes (pas de conversion en cas de dépendance ou d'effet de bord possible).
- PHP : `src/php.rs`, `src/blade.rs`, `src/middleware.rs` ; test de bout en bout `tests/run-php.sh` (vrai FastAPI, SQLite, Jinja2), tests unitaires `cargo test --release`. Depuis WSL : `wsl -d Ubuntu -- bash <fichier de script>`.
- Pièges : sous Windows, écrire les fichiers contenant du japonais avec Python `-X utf8` ; garder les scripts shell en LF ; Git Bash réécrit les chemins `/mnt/...`.
- COBOL_to et Java_to sont développés par d'autres sessions ; les intégrer plus tard, ne supprimer les originaux qu'après confirmation finale (irréversible).
- Le système de traduction (langue naturelle) est séparé des convertisseurs.
- Langues de la documentation : japonais (racine) et 9 langues dans `docs/i18n/` (traductions IA, pas encore relues par des locuteurs natifs).
