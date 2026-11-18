> Cette version est une traduction réalisée par IA, qui n'a pas encore été relue par des locuteurs natifs. Seul l'original japonais, à la racine du dépôt, fait foi.

# pl2pl CLAUDE
- **Rôle** : un convertisseur entre langages, frameworks et systèmes d'exploitation (programme Rust en ligne de commande dans `src/`). Les sources possibles sont listées dans `docs/LANGUAGES.md`, les combinaisons dans `docs/COMBINATIONS.md`. Le propriétaire rédige peu à peu la spécification dans `docs/DESIGN.md` ; n'invente jamais de spécification à sa place.
- **Règles** : ce qui ne peut pas être converti reste en `TODO` et va dans le rapport — ne jamais le supprimer en silence. Exécuter réellement avant d'annoncer qu'une tâche est terminée. Rester prudent : en cas de dépendance ou d'effet de bord possible, ne pas convertir.
- **PHP** : `src/php.rs`, `src/blade.rs`, `src/middleware.rs`. Test de bout en bout : `tests/run-php.sh` (vrai FastAPI, SQLite, Jinja2). Tests unitaires : `cargo test --release`. Depuis WSL : `wsl -d Ubuntu -- bash <fichier de script>`.
- **Pièges** : sous Windows, écrire les fichiers contenant du japonais avec Python `-X utf8` ; garder les scripts shell en fins de ligne LF ; Git Bash réécrit les chemins `/mnt/...`.
- **COBOL_to / Java_to** : d'autres sessions les développent. Les intégrer plus tard, et ne supprimer les originaux qu'après une confirmation finale — c'est irréversible.
- **Système de traduction** (langue naturelle) : il reste séparé des convertisseurs.
- **Langues de la documentation** : le japonais à la racine, plus neuf langues dans `docs/i18n/` (traductions par IA, pas encore relues par des locuteurs natifs). Quand l'original japonais change, mettre aussi à jour les traductions.
