> Remarque : le README japonais à la racine du dépôt est l'original. Cette version est une traduction par IA, en mots simples, et n'a pas encore été relue par des locuteurs natifs.

# pl2pl

## Qu'est-ce que pl2pl ?
Les programmes sont écrits dans beaucoup de « langues » (des langages de programmation).
**pl2pl est un outil qui récrit un programme d'une langue dans une autre.**
Par exemple, il peut transformer un site fait avec PHP (Laravel) en un site fait avec Python (FastAPI).
Il fonctionne sous Windows, Linux et Mac (version 0.1.0, encore en construction).

- On l'installe sur son ordinateur. Le site web ne propose que des guides et des téléchargements → <https://easy-web.tokyo/pl2pl/>
- Quand pl2pl ne peut pas récrire quelque chose, il ne le cache pas. Il laisse un `TODO` (une note pour plus tard) et explique pourquoi dans un rapport.
- Notre but : pouvoir récrire entre le plus grand nombre possible de langages connus.

## Ce qu'il sait faire aujourd'hui
| Commande | Ce qu'elle fait |
|---|---|
| `pl2pl php` | PHP (Laravel / Symfony) → FastAPI. Plus de détails dans [PHP.md](../../PHP.md) (en japonais) |
| `pl2pl web` | Regarde dans un dossier, repère Java, COBOL, PHP et Flask/Django, et confie chacun au bon outil |
| `pl2pl ir` | (version d'essai) Met les « routes » de PHP, Python et Java sous une même forme, puis produit du JSON, de l'OpenAPI ou un squelette FastAPI |
| `pl2pl watch` | Trouve les nouvelles versions des langages et des outils, et se met à jour tout seul |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL (Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | Rend les boucles C/C++ plus rapides (gpu est une version d'essai) |
| `pl2pl cport` | Fait fonctionner du code C pour UNIX sous Windows comme sous Linux |
| `pl2pl pyhpc` | Accélère les calculs Python avec Numba |
| `pl2pl llm setup` | Installe l'assistant IA (aruaru-llm). Nous n'avons pas encore vérifié l'installation réelle |

## Suivre les nouvelles versions tout seul
Nous lançons `pl2pl watch` chaque soir et chaque matin.
1. Il regarde si de nouvelles versions des langages et des outils sont sorties (la liste est dans `registry/ecosystem.toml`).
2. Il teste avec la nouvelle version. Si les tests passent, il met lui-même à jour la « version prise en charge ».
3. S'ils échouent, version-control (avec l'aide de l'IA) essaie de réparer.
4. Si ça ne suffit pas, il envoie un message aux adresses e-mail enregistrées et à LINE.

À noter : pour laravel, symfony, flask, django et quelques autres, il dit seulement « une nouvelle version est sortie » pour le moment.
Sur GitHub, des tests avec les versions les plus récentes tournent aussi tout seuls chaque soir et chaque matin.

## Comment l'utiliser
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## Bon à savoir
- pl2pl est prudent. Si quelque chose paraît risqué, il le laisse tel quel et dit pourquoi. Nous ne pouvons pas promettre la vitesse.
- Nous n'avons testé que de petits exemples. Nous n'avons pas encore essayé de gros programmes réels.

## La suite
- Mettre aussi les types et le contenu des fonctions dans la forme commune, et faire passer la sortie FastAPI par là.
- PHP : les composants Blade écrits sous forme de classes.
- Vérifier les vraies alertes par e-mail et la réparation automatique par l'IA.
- Vérifier le fonctionnement sur GPU (open-cuda / open-directx).

## En savoir plus
[LANGUAGES.md](../../LANGUAGES.md) (liste des langages) / [COMBINATIONS.md](../../COMBINATIONS.md) (combinaisons) / [DESIGN.md](../../DESIGN.md) (la conception, écrite par le propriétaire) / [components/README.md](../../../components/README.md) (éléments fournis avec l'outil). Ces fichiers sont en japonais.
