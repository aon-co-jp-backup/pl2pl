> Ce texte est une traduction automatique par IA, pas encore relue par des locuteurs natifs. Le texte de référence est l'original japonais à la racine du dépôt.

# pl2pl – passation
(Les anciennes notes détaillées sont dans `docs/HISTORY-ja.md`, en japonais.)

## Où en est-on (2026-11-19)
- La version 0.1.0 est publiée, avec des installateurs pour Windows, Linux et Mac et des tests en CI.
- PHP → FastAPI est testé de bout en bout avec un vrai FastAPI (49 vérifications) : routes, Eloquent, constructeur de requêtes, Blade, middleware, relations.
- `pl2pl web` reconnaît désormais aussi PHP et le transmet.
- Nouveau : `pl2pl watch` repère les nouvelles versions, teste, corrige, et prévient en dernier recours. version-control a reçu l'option `--notify-file`.
- COBOL_to et Java_to sont intégrés dans `converters/`.

## Reste à faire (par priorité)
1. **Vérifier** : l'arrivée d'un vrai e-mail (`version-control --test-notify`), la correction automatique via `version-control --loop`, un test avec le dernier fastapi (l'exécution CI de chaque nuit le vérifiera).
2. Essayer `watch` avec de vraies nouvelles versions (laravel, symfony, spring-boot, flask, django et poem restent « non vérifiés »).
3. Enregistrer les exécutions du soir et du matin (`schtasks /Create`, etc. ; pas encore fait, car c'est la configuration personnelle du propriétaire).
4. Prototype de la représentation intermédiaire commune (IR), base pour ajouter beaucoup de paires de langages.
5. PHP : `@can`, composants Blade en classes, `$attributes`, `@php`, `morphToMany`.
6. Liaison avec aruaru-llm (l'installation réelle par `llm setup` n'est pas vérifiée). Kokkos sur GPU ; noyaux sur open-cuda / open-directx.
7. Supprimer les COBOL_to et Java_to d'origine (**irréversible : demander d'abord une confirmation finale**).
8. Le propriétaire écrit `docs/DESIGN.md`. Vérifier que `docs/LANGUAGES.md` n'est pas périmé.

## Tests
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP de bout en bout) / `cargo test --release`

## Message pour reprendre
« Reprendre F:\pl2pl. Parcourir « Reste à faire » dans PORTING.md depuis le haut. Ne jamais supprimer en silence ce qui ne se convertit pas : laisser un TODO. Vérifier en exécutant pour de vrai avant d'annoncer que c'est fini. Répondre uniquement en japonais. Pousser à chaque étape. »
