# v0.1.0 Sentinel

[English](../README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | **Français**

> 💬 **Vos retours sont bienvenus.** Améliorations, sortie de la v1.0, mises à jour, erreurs d'installation ou tout autre souci : [ouvrez une issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose) (6 langues).

← [Retour à squeeze](../../../README/README.fr.md)

Sentinel monte la garde entre Claude Code et votre shell. Quand Claude s'apprête à lancer une commande qui inonde le contexte (installations, builds, tests, logs) ou à lire d'un coup un énorme fichier, Sentinel le redirige vers une version courte. Claude reçoit toujours chaque erreur et chaque avertissement.

Un seul binaire, écrit en Rust, **zéro dépendance**. Fonctionne sous macOS, Linux et Windows.

**Prérequis :** aucun. C'est le programme `squeeze` de base. Les versions suivantes s'appuient dessus : v0.3 Conduit ajoute une commande à ce même programme (mettez-le à jour pour l'obtenir), tandis que v0.2 Interceptor est une extension de navigateur séparée qui fonctionne avec ou sans lui. Voir [comment les versions s'articulent](../../../README/README.fr.md#comment-les-versions-sarticulent).

## Effet

Rejeu de 7 logs de test : de 83 325 à 5 128 tokens estimés (**−93,8 %**), chaque erreur et avertissement distinct conservé. Détails et méthode : [docs/bench.md](../docs/bench.md) (anglais).

Ce que Claude voit après `squeeze run -- pytest -v` (217 lignes → 39). C'est la sortie réelle du log `pytest-fail.log` :

```
============================= test session starts ==============================
platform linux -- Python 3.13.18, pytest-9.1.1, pluggy-1.6.0 -- /usr/local/bin/python3
cachedir: .pytest_cache
rootdir: ~/project/
plugins: platformdirs-4.13.3, anyio-4.15.1
collecting ... collected 183 items

test_inventory.py::test_restock[0] PASSED                                [  0%]
… [178 lines omitted] …
test_inventory.py::test_restock[179] PASSED                              [ 98%]
test_inventory.py::test_negative PASSED                                  [ 98%]
test_inventory.py::test_bulk_order FAILED                                [ 99%]
test_inventory.py::test_missing_sku FAILED                               [100%]

=================================== FAILURES ===================================
_______________________________ test_bulk_order ________________________________

    def test_bulk_order():
        orders = {"apple": 3, "pear": 5}
>       assert sum(orders.values()) == 9, "bulk order total mismatch"
E       AssertionError: bulk order total mismatch
E       assert 8 == 9
E        +  where 8 = sum(dict_values([3, 5]))
E        +    where dict_values([3, 5]) = <built-in method values of dict object at 0x7f4c5d1f9900>()
E        +      where <built-in method values of dict object at 0x7f4c5d1f9900> = {'apple': 3, 'pear': 5}.values

test_inventory.py:12: AssertionError
_______________________________ test_missing_sku _______________________________

    def test_missing_sku():
        stock = {"A1": 3}
>       assert stock["B7"] == 0
               ^^^^^^^^^^^
E       KeyError: 'B7'

test_inventory.py:15: KeyError
=========================== short test summary info ============================
FAILED test_inventory.py::test_bulk_order - AssertionError: bulk order total ...
FAILED test_inventory.py::test_missing_sku - KeyError: 'B7'
======================== 2 failed, 181 passed in 0.14s =========================
[squeeze] 217 lines (15.8 KB) → 39 lines (1.9 KB). Full output: .squeeze/raw/20261103-044044-45b0e7.log (exit 1)
```

## Fonctionnalités

- **Garde Bash** : les commandes bavardes (`npm install`, `go test`, `cargo build`, `pytest`, `make`, `apt-get install`, `journalctl`, `docker build`, `kubectl logs`, `git log`, etc.) sont redirigées vers `squeeze run -- <commande>`.
- **Garde Read** : la lecture d'un fichier de plus de 400 lignes sans `offset`/`limit` est redirigée. Claude reçoit un plan avec numéros de ligne et choisit une plage.
- **Compression intelligente** : garde les 8 premières et les 20 dernières lignes, chaque erreur et avertissement avec son contexte, et les traces de pile. Les messages répétés sont regroupés avec un compteur. Les erreurs passent avant les avertissements.
- **Code de sortie conservé** : `squeeze run` se termine avec le code de la commande elle-même.
- **Logs complets sur disque** : `.squeeze/raw/`, les 50 plus récents sont gardés, lisibles par vous seul (0600). `.squeeze/` est ajouté au `.gitignore`.
- **`squeeze clip`** pour claude.ai : compresse le presse-papiers avant de coller un log dans une conversation.
- **`squeeze report`** : ce qui serait entré dans le contexte, ce qui y est entré, et les plus grosses économies.
- **Sûr par conception** : les hooks n'approuvent jamais un appel d'outil ; à la moindre erreur ils restent silencieux (fail open) ; `squeeze uninstall` retire tout ce qu'il a ajouté.

## Installation

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.sh | bash
# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.ps1 | iex
# Depuis les sources
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

Les installateurs vérifient le `checksums.txt` de la version avant d'installer. `SQUEEZE_VERSION=v0.1.0` fixe une version ; `SQUEEZE_INSTALL_DIR` change le dossier.

## Démarrage rapide

```bash
cd ~/my-project
squeeze init
claude
```

`squeeze init` modifie quatre choses, et `squeeze uninstall` les annule :

| Fichier | Modification |
|---|---|
| `.claude/settings.json` | Ajoute deux hooks `PreToolUse` (`Bash`, `Read`). Vos autres réglages et hooks sont conservés ; une copie `.squeeze.bak` est écrite d'abord. |
| `.squeeze/config.json` | Configuration par défaut (voir plus bas). |
| `.gitignore` | Ajoute `.squeeze/`, car les logs bruts peuvent contenir des secrets. |
| `CLAUDE.md` | Ajoute une note de 4 lignes entre les marqueurs `<!-- squeeze:start -->`. |

## Utilisation

| Commande | Rôle |
|---|---|
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | Installer dans un projet. Utilisez `--bin` avec un chemin absolu si `squeeze` n'est pas dans le `PATH`. |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | Retirer les hooks et la note de CLAUDE.md. `--purge` supprime aussi `.squeeze/`. |
| `squeeze run -- <commande>` | Lancer une commande ; afficher un résumé court si la sortie est longue. |
| `<commande> 2>&1 \| squeeze compress` | Compresser une sortie reçue par pipe. |
| `squeeze clip [--print]` | Remplacer le presse-papiers par la version compressée (`--print` l'affiche seulement). |
| `squeeze report [--since 7d] [--global] [--json] [--top N]` | Économies de ce projet. `--global` affiche celles de `squeeze clip`. |
| `squeeze bench <dir> [--markdown]` | Rejouer des fichiers `*.log` et mesurer les économies. |
| `squeeze hook pre-bash` / `pre-read` | Points d'entrée des hooks appelés par Claude Code. |
| `squeeze version` | Afficher la version. |

Si la sortie brute d'une commande est vraiment nécessaire, Claude (ou vous) peut la préfixer par `SQUEEZE_SKIP=1`.

## Configuration

`.squeeze/config.json` (toutes les clés sont facultatives) :

| Clé | Défaut | Signification |
|---|---|---|
| `compress.max_lines` / `max_bytes` | `80` / `8000` | Une sortie sous ces deux limites est laissée telle quelle. |
| `compress.head_lines` / `tail_lines` | `8` / `20` | Lignes toujours gardées au début / à la fin. |
| `compress.context` | `2` | Lignes gardées autour de chaque erreur ou avertissement. |
| `compress.budget` | `120` | Plafond approximatif de lignes gardées. |
| `guard_bash` / `guard_read` | `true` / `true` | Activer ou désactiver chaque garde. |
| `read_max_lines` | `400` | Les fichiers plus longs sont redirigés vers une lecture par plage. |
| `extra_noisy` | `[]` | Préfixes de commandes à traiter comme bavardes, ex. `"./deploy.sh"`. |
| `quiet` | `[]` | Préfixes de commandes que la garde ne touche jamais, ex. `"make docs"`. |
| `keep_raw` | `50` | Nombre de logs bruts gardés dans `.squeeze/raw/`. |

## Fonctionnement

```
Claude veut lancer :  npm test
        │
        ▼
Hook PreToolUse ── squeeze hook pre-bash
        │  bavarde ? ── non ──▶ silence ; flux de permissions normal
        │  oui
        ▼
refus + indice : « lance-la avec  squeeze run -- 'npm test' »
        │
        ▼
Claude lance : squeeze run -- 'npm test'   (vos règles de permissions s'appliquent comme d'habitude)
        │
        ▼
sortie complète ──▶ .squeeze/raw/<heure>.log
sortie courte ────▶ Claude (erreurs, avertissements, début, fin et ligne avec le chemin du log)
```

Pourquoi « refuser avec un indice » plutôt que réécrire la commande en silence ? Un hook qui réécrit une commande doit aussi l'approuver, ce qui contournerait vos demandes de permission. squeeze ne le fait jamais.

Plus de détails : [docs/guide/how-it-works.md](../docs/guide/how-it-works.md) (coréen).

## Où ça s'applique

- **Claude Code** : automatiquement, via les hooks.
- **claude.ai** : seulement quand vous collez un long texte en passant par `squeeze clip`. Les conversations normales ne sont pas affectées ; squeeze ne peut ni voir ni modifier vos conversations.
- **Autres outils en terminal** : passez la sortie à `squeeze compress`.

## FAQ

**Q. Claude peut-il rater quelque chose d'important ?**
R. Dans tous les logs de test, tous les messages d'erreur et d'avertissement distincts ont été conservés. Si le résumé ne suffit pas, la dernière ligne donne le chemin du log complet et Claude peut le lire par plage.

**Q. squeeze envoie-t-il mes données quelque part ?**
R. Non. Il ne contient aucun code réseau. Les logs restent dans `.squeeze/` sur votre machine.

**Q. Peut-il casser ma session ?**
R. Les hooks s'effacent en cas d'échec : entrée inattendue, fichier manquant ou panic ne produisent aucune sortie, et Claude Code continue normalement.

**Q. Claude bute sans cesse sur un gros fichier et j'en ai besoin en entier.**
R. L'indice dit à Claude de rappeler Read avec `limit=<lignes>`, ce qui passe. Ou désactivez la garde avec `"guard_read": false`.

**Q. Est-ce que ça économise des tokens dans les conversations claude.ai normales ?**
R. Seulement quand vous collez un long texte après avoir lancé `squeeze clip`.

## Retours

Laissez améliorations et problèmes dans les [Issues](https://github.com/jongwooish/claude-squeeze/issues/new/choose) : idées pour la v1.0, erreurs de mise à jour ou d'installation, compression qui a perdu quelque chose, ou tout autre souci. Choisissez un formulaire (bug, problème de compression, problème d'installation/mise à jour/version, idée). Questions et feuille de route dans les [Discussions](https://github.com/jongwooish/claude-squeeze/discussions). N'importe laquelle des six langues convient.

## Contribuer

```bash
cargo test --manifest-path CARGO/Cargo.toml                       # tests unitaires, y compris les vérifications des logs
cargo clippy --manifest-path CARGO/Cargo.toml -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml -- bench v0.1-Sentinel/v0.1.0_squeeze-sentinel/testdata/fixtures
```

Les nouveaux logs de test sont bienvenus : ajoutez un vrai `*.log` (secrets et chemins personnels retirés au préalable) dans `testdata/fixtures/`. Les tests échouent si une erreur ou un avertissement distinct est perdu.

Problèmes et idées : [GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues).
