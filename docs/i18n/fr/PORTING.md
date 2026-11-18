> Cette version est une traduction réalisée par IA, qui n'a pas encore été relue par des locuteurs natifs. Seul l'original japonais, à la racine du dépôt, fait foi.

# pl2pl — passation
## Où en est-on (2026-11-19)
La version 0.1.0 est publiée, avec des installateurs pour Windows, Linux et macOS et des tests de bout en bout en CI. La conversion PHP couvre les routes, les corps à plusieurs instructions, Eloquent (SQLAlchemy), le query builder avec pagination, les relations (y compris `with` imbriqué), Blade (Jinja2) et les middlewares (`Depends`), le tout testé de bout en bout. Numba, Kokkos (Serial) et `pl2pl web` ont été confirmés par de vraies exécutions.
## Points ouverts, par ordre de priorité
1. PHP : `morph`, autorisation `@can` et composants Blade à base de classes, middleware modifiant le corps après `$next`.
2. Intégrer COBOL_to / Java_to à pl2pl, puis supprimer les originaux (confirmer d'abord ; irréversible ; d'autres sessions y travaillent).
3. Prototype de la représentation intermédiaire commune.
4. Intégration d'aruaru-llm (une vraie installation par `llm setup` reste à vérifier).
5. Kokkos sur GPU / `Kokkos::View` ; noyaux sur open-cuda / open-directx.
6. Le propriétaire rédige `docs/DESIGN.md` ; vérifier que `docs/LANGUAGES.md` est à jour.
7. Exécuter les nouveaux exemples PHP en CI sous Linux arm64 et macOS.
## Tests
`bash tests/run-all.sh` (WSL), `bash tests/run-php.sh` (PHP de bout en bout), `cargo test --release`.
## Message pour reprendre le travail
« Reprends F:\pl2pl. Commence par le point 1 de PORTING.md. Ne supprime jamais en silence ce qui n'est pas convertible : laisse un TODO. Vérifie par une vraie exécution avant d'annoncer que c'est terminé. Demande-moi avant de pousser. »
