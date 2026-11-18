> Cette version est une traduction réalisée par IA, qui n'a pas encore été relue par des locuteurs natifs. Seul l'original japonais, à la racine du dépôt, fait foi.

# pl2pl
pl2pl convertit du code d'un langage de programmation, d'un framework ou d'un système d'exploitation à un autre. Le projet en est à la version 0.1.0 et reste en plein développement. Des installateurs sont disponibles pour Windows (x64/ARM64), Linux (amd64/arm64) et macOS (arm64/x64). La conversion se fait dans l'application de bureau installée (en ligne de commande) ; le site web ne propose que les manuels et les téléchargements.
Le convertisseur est volontairement prudent. **Ce qu'il ne sait pas convertir n'est jamais supprimé en silence : cela reste dans le résultat sous forme de `TODO` et figure dans le rapport, avec la raison.**

## Ce qui fonctionne aujourd'hui
| Commande | Ce qu'elle fait |
|---|---|
| `pl2pl cobol` | Convertit du COBOL IBM avec JCL en GnuCOBOL sous Linux avec bash. Vérifié en compilant et en exécutant le résultat avec GnuCOBOL. |
| `pl2pl omp` | Ajoute `#pragma omp parallel for` aux boucles C/C++ indépendantes, y compris avec réduction de somme. Le résultat est identique à celui de l'original. |
| `pl2pl php` | Convertit Laravel / Symfony en FastAPI (détails plus bas). Testé de bout en bout avec un vrai FastAPI, SQLite et Jinja2. |
| `pl2pl web` | Reconnaît Java, COBOL et Flask/Django, puis confie le travail à java_to, cobol_to, 2fastapi ou 2poem. Vérifié sur de vrais fichiers. |
| `pl2pl cport` | Transforme du C pour UNIX en un en-tête de compatibilité, avec des remplacements mécaniques et des conseils de portage. Vérifié avec gcc. |
| `pl2pl pyhpc` | Ajoute Numba `@njit` aux fonctions Python numériques ; avec `--mpi`, génère aussi un squelette mpi4py. Résultat identique sous Numba. |
| `pl2pl kokkos` | Réécrit les boucles indépendantes en Kokkos (`parallel_for` / `parallel_reduce`). Compilé et exécuté avec Kokkos 4.5.01 (Serial). |
| `pl2pl gpu` | Prototype : transforme les boucles en `.cu` de style CUDA ou en HLSL. Seule la sortie générée est vérifiée ; l'exécution sur open-cuda / open-directx n'a pas encore été testée. |
| `pl2pl llm setup` / `llm start` | Lance l'aruaru-llm fourni et installe un modèle Qwen2.5 adapté à la mémoire disponible. Seuls le choix du modèle et `--dry-run` sont vérifiés, pas une installation réelle. |
| `pl2pl doctor` | Affiche les informations du processeur grâce à open-cpu, intégré. |

## Conversion de PHP (`pl2pl php`)
- **Routes** : Laravel `Route::get/post/put/patch/delete/any/resource/apiResource/view`, groupes `prefix`, et Symfony `#[Route]`.
- **Corps de fonction** : `return` simple, corps à plusieurs instructions, `response()->json($x, 201)` et `noContent()`.
- **Eloquent → SQLAlchemy** (`db.py`, `models.py`) : les colonnes viennent des migrations ou de `$fillable` ; tout le reste va dans une colonne JSON `extra`. `DATABASE_URL` choisit la base de données (SQLite par défaut).
- **Query builder** : `where / orWhere / whereIn / orderBy / latest / oldest / limit / take / offset / skip`, `paginate / simplePaginate / cursorPaginate` (lit `?page=N`, renvoie du JSON au format Laravel, `links()` fonctionne dans Blade), `get / first / firstOrFail / count / exists / delete`, ainsi que `$request->query('k', valeur par défaut)`.
- **Relations** : `hasMany / hasOne / belongsTo / belongsToMany` (table pivot générée), `with` y compris imbriqué (`with('posts.comments')`), `whereHas / has / whereDoesntHave / doesntHave`, et `attach / detach / sync`.
- **Blade → Jinja2** : affichages, `@if`, `@foreach`, `@forelse`, `@for`, `@extends/@section/@yield/@include`, etc.
- **Middleware → `Depends`** : `auth` (une simple vérification Bearer — **à remplacer par une vraie authentification**), `guest`, `throttle:N,M`, et les middlewares personnalisés du type « refuser selon une condition, sinon `return $next($request)` ».

## À venir (pas encore fait)
- PHP : relations `morph`, composants, piles et directives d'autorisation de Blade, et middleware qui modifie le corps de la réponse après `$next` (l'ajout d'en-têtes fonctionne déjà).
- Intégrer COBOL_to et Java_to à pl2pl une fois qu'ils seront matures, puis supprimer les dépôts d'origine. C'est la consigne du propriétaire ; comme une suppression est irréversible, elle sera reconfirmée après l'intégration.
- Une représentation intermédiaire commune (côté serveur, base de données/SQL, interface), pour que N sources et M cibles demandent N+M composants au lieu de N×M.
- Intégration d'aruaru-llm. Les explications et la traduction en langue naturelle restent séparées des convertisseurs, qui ne dépendent jamais d'un LLM.
- Kokkos sur GPU et `Kokkos::View`, ainsi que l'exécution de noyaux sur open-cuda / open-directx.
- Exécuter les nouveaux exemples PHP en CI sous Linux arm64 et macOS, et ajouter d'autres paires de langages tirées de `docs/LANGUAGES.md` (toutes les combinaisons ne seront pas construites).

## Bon à savoir
En cas de doute, pl2pl choisit la prudence. Lorsqu'une construction pourrait avoir une dépendance ou un effet de bord, elle est **laissée telle quelle et signalée avec la raison** ; aucune performance n'est promise. Jusqu'ici, seuls de petits exemples écrits à la main ont été testés, pas de gros programmes réels.
Pour aller plus loin : `docs/DESIGN.md`, `docs/LANGUAGES.md`, `docs/COMBINATIONS.md`.
