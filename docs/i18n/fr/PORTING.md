> Traduit par IA, pas encore relu par des locuteurs natifs. L'original japonais (racine du dépôt) fait foi.

# Passation pl2pl
## État (2026-11-19)
v0.1.0 est publiée (installateurs Windows/Linux/macOS, tests CI de bout en bout). La conversion PHP couvre routes, corps à plusieurs instructions, Eloquent (SQLAlchemy), query builder avec pagination, relations, Blade (Jinja2) et middleware (`Depends`), le tout testé de bout en bout. Numba, Kokkos (Serial) et `pl2pl web` sont confirmés par de vraies exécutions.
## Points ouverts (par priorité)
1. PHP : `with` imbriqué, `morph`, `cursorPaginate`, composants/piles/autorisation Blade, middleware après `$next`.
2. Intégrer COBOL_to / Java_to à pl2pl puis supprimer les originaux (confirmer d'abord ; irréversible ; d'autres sessions y travaillent).
3. Prototype de l'IR commune.
4. Intégration d'aruaru-llm (installation réelle de `llm setup` non vérifiée).
5. Kokkos GPU / `Kokkos::View` ; noyaux sur open-cuda / open-directx.
6. Le propriétaire rédige `docs/DESIGN.md` ; vérifier que `docs/LANGUAGES.md` est à jour.
7. Exécuter les nouveaux exemples PHP en CI sur Linux arm64 et macOS.
## Tests
`bash tests/run-all.sh` (WSL), `bash tests/run-php.sh` (PHP de bout en bout), `cargo test --release`.
## Message de reprise
« Reprendre F:\pl2pl. Commencer par le point 1 de PORTING.md. Ne jamais abandonner en silence ce qui n'est pas convertible, laisser un TODO. Ne déclarer terminé qu'après exécution réelle. Confirmer avant de pousser. »
