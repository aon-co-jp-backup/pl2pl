> Ce texte est une traduction automatique par IA, pas encore relue par des locuteurs natifs. Le texte de référence est l'original japonais à la racine du dépôt.

# pl2pl – notes de développement
- **Rôle** : un convertisseur entre langages, frameworks et systèmes d'exploitation (CLI en Rust, `src/`). Les langages sources sont listés dans `docs/LANGUAGES.md`, les combinaisons dans `docs/COMBINATIONS.md`. Le propriétaire écrit la conception petit à petit dans `docs/DESIGN.md`. **Ne jamais inventer de spécification à sa place.**
- **Orientation** : conversion de tout langage ou framework connu vers tout autre. Pas tout d'un coup : on commence par ce qui a été choisi. Le site web ne propose que des manuels et des téléchargements ; la conversion se fait dans l'application PC.
- **Règles** : ce qui ne peut pas être converti reste en `TODO` et est signalé (jamais supprimé en silence). Rester prudent (en cas de dépendance ou d'effet de bord possible, ne pas convertir). Exécuter pour de vrai avant de dire « terminé ». Pas de nouveau REST. Répondre uniquement en japonais.
- **PHP** : `src/php.rs`, `src/blade.rs`, `src/middleware.rs`. Test de bout en bout : `tests/run-php.sh` (vrai FastAPI, SQLite, Jinja2) ; tests unitaires : `cargo test --release`. Sous WSL : `wsl -d Ubuntu -- bash <fichier>`.
- **Suivi automatique** : `pl2pl watch` (`src/watch.rs`, `registry/ecosystem.toml`). Une entrée sans commande de vérification est seulement affichée comme « non vérifiée » et n'est jamais mise à jour toute seule. Les notifications passent par `--notify-file` de version-control.
- **Pièges** : écrire les fichiers contenant du japonais avec Python `-X utf8` ; scripts shell en LF ; Git Bash réécrit les chemins `/mnt/...` ; pour les chaînes pleines de guillemets, utiliser l'outil Write plutôt qu'un heredoc.
- **COBOL_to / Java_to** : déjà intégrés dans `converters/`. La suppression des dépôts d'origine est irréversible : demander une confirmation finale avant.
- **Système de traduction** (langue naturelle) : séparé des convertisseurs. Les convertisseurs ne dépendent pas d'un LLM.
- **Langues de la documentation** : originaux en japonais, plus neuf langues dans `docs/i18n/` (traductions par IA, pas encore relues par des locuteurs natifs). Quand un original change, mettre à jour les traductions. Historique : `docs/HISTORY-ja.md`.
