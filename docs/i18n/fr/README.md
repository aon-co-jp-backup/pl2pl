> Ce texte est une traduction automatique par IA, pas encore relue par des locuteurs natifs. Le texte de référence est l'original japonais à la racine du dépôt.

# pl2pl

**pl2pl transforme du code écrit dans un langage ou un framework en un autre langage ou framework.**
Par exemple, un site en PHP (Laravel) devient un site en Python (FastAPI). Il fonctionne sous Windows, Linux et Mac (v0.1.0, en cours de développement).

- La conversion se fait dans une application que vous installez sur votre ordinateur (un outil en ligne de commande). Le site web ne propose que des manuels et des téléchargements.
- Ce qui ne peut pas être converti n'est jamais supprimé en silence : pl2pl laisse un `TODO` et explique pourquoi dans un rapport.
- L'objectif : pouvoir convertir les langages et frameworks connus entre eux, dans le plus grand nombre de combinaisons possible.

## Ce qui marche aujourd'hui
| Commande | Ce qu'elle fait |
|---|---|
| `pl2pl php` | PHP (Laravel / Symfony) → FastAPI. Détails : [PHP.md](../../PHP.md) (en japonais) |
| `pl2pl web` | Examine un dossier, reconnaît seul Java, COBOL, PHP et Flask/Django, et passe chaque partie au bon convertisseur |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL (Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | Réécrit les boucles C/C++ pour OpenMP, Kokkos ou le GPU (gpu est un prototype) |
| `pl2pl cport` | Fait fonctionner du C pour UNIX sous Windows comme sous Linux |
| `pl2pl pyhpc` | Accélère les calculs Python avec Numba |
| `pl2pl watch` | Repère les nouvelles versions des langages et frameworks et suit automatiquement (voir plus bas) |
| `pl2pl llm setup` | Installe l'assistant IA (aruaru-llm) adapté à votre ordinateur (l'installation réelle n'est pas encore vérifiée) |

## Suivre les nouvelles versions automatiquement
Lancez `pl2pl watch` chaque soir et chaque matin.
1. Il cherche les nouvelles versions des langages et frameworks (la liste est dans `registry/ecosystem.toml`).
2. Il teste avec la nouvelle version. Si les tests passent, il met lui-même à jour la « version prise en charge ».
3. S'ils échouent, version-control (avec l'aide de l'IA) essaie de corriger.
4. Si cela ne suffit pas, il envoie un message aux adresses e-mail enregistrées et à LINE.

Les scripts de planification sont `scripts/pl2pl-watch.cmd` (Windows) et `scripts/pl2pl-watch.sh` (Linux/Mac).
À noter : pour laravel, symfony, flask, django et d'autres, il se contente pour l'instant de signaler « une nouvelle version est sortie ». Les vrais tests avec les nouvelles versions restent à construire.

## Exemples
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
```

## À savoir
- pl2pl est prudent. Ce qui semble risqué reste inchangé, et la raison est donnée. La vitesse n'est pas garantie.
- Il n'a été testé que sur de petits exemples, pas encore sur de gros programmes réels.

## La suite
- PHP: composants Blade écrits sous forme de classes.
- Une représentation intermédiaire commune (IR) pour ajouter d'un coup beaucoup de paires de langages.
- Vérifier l'envoi réel des e-mails et la correction automatique par l'IA.
- Exécution sur GPU (open-cuda / open-directx).

## Pour en savoir plus
[LANGUAGES.md](../../LANGUAGES.md) (liste des langages) / [COMBINATIONS.md](../../COMBINATIONS.md) (combinaisons) / [DESIGN.md](../../DESIGN.md) (conception, rédigée par le propriétaire) / [components/README.md](../../../components/README.md) (composants inclus). Ces documents sont en japonais.
