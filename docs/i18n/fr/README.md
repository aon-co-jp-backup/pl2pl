> Traduit par IA, pas encore relu par des locuteurs natifs. L'original japonais (racine du dépôt) fait foi.

# pl2pl
Un **convertisseur entre langages de programmation, frameworks et systèmes d'exploitation** (v0.1.0 publiée, en développement). Des installateurs existent pour Windows (x64/ARM64), Linux (amd64/arm64) et macOS (arm64/x64). La conversion se fait dans l'application PC installée (CLI) ; la version web ne propose que des manuels et des téléchargements.
La conversion est prudente : **ce qui ne peut pas être converti n'est jamais abandonné en silence ; cela reste en `TODO` et figure dans le rapport avec sa raison.**

## Ce qui fonctionne aujourd'hui
| Commande | Description |
|---|---|
| `pl2pl cobol` | COBOL (IBM) + JCL → GnuCOBOL (Linux) + bash. Vérifié par compilation et exécution avec GnuCOBOL. |
| `pl2pl omp` | Ajoute `#pragma omp parallel for` aux boucles C/C++ indépendantes (avec réduction de somme). Même résultat que l'original. |
| `pl2pl php` | Laravel / Symfony → FastAPI (voir ci-dessous). Testé de bout en bout avec un vrai FastAPI, SQLite et Jinja2. |
| `pl2pl web` | Détecte Java / COBOL / Flask·Django et transmet à java_to, cobol_to, 2fastapi, 2poem. Vérifié sur de vrais fichiers. |
| `pl2pl cport` | C UNIX → en-tête de compatibilité + remplacement mécanique + conseils de portage. Vérifié avec gcc. |
| `pl2pl pyhpc` | Numba `@njit` pour les fonctions Python numériques ; `--mpi` génère un squelette mpi4py. Même résultat sous Numba. |
| `pl2pl kokkos` | Boucles indépendantes → Kokkos (`parallel_for` / `parallel_reduce`). Compilé et exécuté avec Kokkos 4.5.01 (Serial). |
| `pl2pl gpu` | (Prototype) boucles → `.cu` de style CUDA / HLSL. Sortie seulement ; exécution sur open-cuda / open-directx non vérifiée. |
| `pl2pl llm setup` / `llm start` | Démarre aruaru-llm fourni et installe un modèle Qwen2.5 adapté à la mémoire libre. Seules la logique de choix et --dry-run sont vérifiés. |
| `pl2pl doctor` | Informations CPU via open-cpu (intégré). |

## Conversion PHP (`pl2pl php`)
- **Routes** : Laravel `Route::get/post/put/patch/delete/any/resource/apiResource/view`, groupes `prefix`, Symfony `#[Route]`.
- **Corps** : `return` simple, plusieurs instructions, `response()->json($x, 201)`, `noContent()`.
- **Eloquent → SQLAlchemy** (`db.py`, `models.py`) : colonnes issues des migrations ou de `$fillable`, autres champs dans une colonne JSON `extra` ; `DATABASE_URL` choisit la base (SQLite par défaut).
- **Query builder** : `where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`, `paginate / simplePaginate` (`?page=N`, JSON comme Laravel, `links()` dans Blade), `get / first / firstOrFail / count / exists / delete`, `$request->query('k', défaut)`.
- **Relations** : `hasMany / hasOne / belongsTo / belongsToMany` (table pivot générée), `with`, `whereHas / has / whereDoesntHave / doesntHave`, `attach / detach / sync`.
- **Blade → Jinja2** : sortie, `@if`, `@foreach`, `@forelse`, `@for`, `@extends/@section/@yield/@include`, etc.
- **Middleware → `Depends`** : `auth` (vérification Bearer simple, **à remplacer par une vraie authentification**), `guest`, `throttle:N,M`, middleware personnalisé « refuser selon une condition, sinon `return $next($request)` ».

## Prévu (pas encore fait)
- PHP : `with('a.b')` imbriqué, relations `morph`, `cursorPaginate`, composants/piles/directives d'autorisation Blade, middleware qui retraite la réponse après `$next`.
- Intégrer COBOL_to et Java_to à pl2pl lorsqu'ils seront mûrs, puis supprimer les dépôts d'origine (consigne du propriétaire ; la suppression est irréversible, elle sera reconfirmée après l'intégration).
- Une représentation intermédiaire commune (serveur, BD/SQL, interface) pour que N sources et M cibles demandent N+M composants et non N×M.
- Intégration d'aruaru-llm (explications et traduction en langue naturelle restent séparées des convertisseurs ; les convertisseurs ne dépendent jamais d'un LLM).
- Exécution GPU Kokkos et `Kokkos::View` ; exécution de noyaux sur open-cuda / open-directx.
- Exécution des nouveaux exemples PHP en CI sur Linux arm64 et macOS ; d'autres paires de langages tirées de `docs/LANGUAGES.md` (toutes les combinaisons ne seront pas construites).

## Remarques
Les décisions sont prudentes : en cas de dépendance ou d'effet de bord possible, la construction est **laissée telle quelle et signalée avec la raison** ; les performances ne sont pas garanties. Seuls de petits exemples faits main ont été testés, pas de gros programmes réels.
Documents : `docs/DESIGN.md`, `docs/LANGUAGES.md`, `docs/COMBINATIONS.md`.
