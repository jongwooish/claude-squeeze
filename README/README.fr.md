# claude-squeeze

[![CI](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml/badge.svg)](https://github.com/jongwooish/claude-squeeze/actions/workflows/ci.yml) [![Latest release](https://img.shields.io/github/v/release/jongwooish/claude-squeeze)](https://github.com/jongwooish/claude-squeeze/releases/latest) [![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](../v0.2-Auditor/v0.2.0_squeeze-auditor/Cargo.toml) [![Dependencies: 0](https://img.shields.io/badge/dependencies-0-brightgreen)](../v0.2-Auditor/v0.2.0_squeeze-auditor/Cargo.toml) [![Platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](#installation) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | **Français**

> 💬 **Vos retours sont bienvenus.** Améliorations, sortie de la v1.0, mises à jour, erreurs d'installation ou tout autre souci : [ouvrez une issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose) (6 langues).

**Dépensez vos tokens Claude à réfléchir, pas à lire des logs à rallonge.**

squeeze empêche les longues sorties de commandes, les lectures de gros fichiers ou répétées, les sessions devenues trop grandes et les serveurs MCP inutilisés d'entrer dans le contexte de Claude. Claude voit toujours chaque erreur et chaque avertissement. **squeeze-Auditor**, l'application à installer, montre où sont partis vos tokens et vous laisse choisir la force de squeeze.

- [Effet](#effet) · [Versions](#versions) · [Où ça marche](#où-ça-marche) · [v0.2 squeeze-Auditor](#v02-squeeze-auditor) · [v0.1 Sentinel](#v01-sentinel) · [Installation](#installation) · [Utilisation](#utilisation) · [Fonctionnement](#fonctionnement) · [Feuille de route](#feuille-de-route) · [FAQ](#faq)

## Pourquoi

Claude Code renvoie toute la conversation à chaque requête, et la sortie des commandes en fait partie. Un `npm install`, une série de tests ou un `journalctl` peuvent ajouter des milliers de tokens qui voyagent ensuite à chaque tour, alors que Claude n'avait besoin que de « ça passe » ou de trois lignes d'erreur.

squeeze arrête cette sortie **avant** qu'elle n'entre dans le contexte.

## Effet

Sept logs réels passés dans squeeze ([méthode](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/docs/bench.md)) :

| Log | Lignes avant → après | Tokens avant → après (est.) | Économie | Erreurs/avertissements distincts conservés |
|---|---:|---:|---:|---:|
| `pytest`, 2 échecs | 217 → 39 | 4 040 → 516 | **87,2 %** | 11 / 11 |
| `go test -v`, 1 échec | 488 → 16 | 4 247 → 202 | **95,2 %** | 4 / 4 |
| `make` avec avertissements gcc + 1 erreur | 370 → 44 | 7 046 → 850 | **87,9 %** | 7 / 7 |
| `tsc` avec 2 erreurs de type | 124 → 9 | 1 310 → 172 | **86,9 %** | 3 / 3 |
| Liste de paquets `dpkg -l` | 1049 → 63 | 35 237 → 2 106 | **94,0 %** | 8 / 8 |
| Journal serveur, OOM kill (synthétique) | 1400 → 37 | 31 161 → 997 | **96,8 %** | 5 / 5 |
| Sortie `npm` courte | 16 → 16 | 287 → 287 | 0,0 % (inchangée) | 12 / 12 |
| **Total** | | **83 325 → 5 128** | **93,8 %** | **50 / 50** |

**Pourquoi c'est utile**

- **Rien d'important n'a été perdu dans ces tests.** Chaque message d'erreur et d'avertissement distinct a survécu. Les répétitions sont affichées une fois avec un compteur (`… [same message 29 more times] …`).
- **Une sortie courte n'est jamais modifiée.** Sous 80 lignes et 8 Ko, vous obtenez l'original.
- **L'économie se répète à chaque tour.** Un log de 31 000 tokens qui reste dans le contexte pendant les 20 tours suivants représente environ 620 000 tokens d'entrée. Réduit à environ 1 000 tokens, ces mêmes 20 tours en coûtent environ 20 000.
- **Des sessions plus longues avant les limites.** Moins de bruit, c'est un contexte qui se remplit moins vite : la compaction et les limites du forfait arrivent plus tard.
- **Claude se concentre mieux.** La réponse à « pourquoi le build a échoué ? » est sous ses yeux, pas noyée dans 1 000 lignes.

**Limites, honnêtement :** les tokens sont estimés (octets ÷ 4), et ces chiffres viennent du rejeu de logs enregistrés, pas d'un test A/B en direct avec Claude Code (prévu). Lancez `squeeze report` dans votre projet pour voir vos vrais chiffres.

**La v0.2 ajoute quatre blocages** qui empêchent les mêmes tokens d'être renvoyés encore et encore : relire un fichier inchangé, une session qui a dépassé votre budget, une longue session qu'il vaut mieux redémarrer depuis une courte note de passation, et les coûts fixes comme les serveurs MCP inutilisés. Leur effet dépend de votre façon de travailler : le [README de la v0.2](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.fr.md#effet) montre le calcul, et le tableau de bord de squeeze-Auditor vos propres chiffres.

## Versions

Chaque version a son propre dossier et son README. Les versions publiées sont des tags sur l'ensemble du dépôt.

| Version | Nom | Ce qu'elle ajoute | Statut |
|---|---|---|---|
| [v0.1](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README/README.fr.md) | **Sentinel** | Hooks Claude Code qui interceptent les commandes bavardes et les lectures complètes de fichiers, `squeeze run`, `compress`, `clip`, `report` | ✅ v0.1.0 |
| [v0.2](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.fr.md) | **squeeze-Auditor** | Quatre blocages de session (lecture répétée, pause budget, passation, coût fixe), l'application squeeze-Auditor avec tableau de bord d'usage et modes d'économie, prise en charge de l'application de bureau Claude | ✅ v0.2.0 |
| v0.3 | *(nom à venir)* | claude.ai dans le navigateur et les applications mobiles, avec une nouvelle approche des plaintes sur les tokens | 🛠 Prévue |

### Comment les versions s'articulent

| Version | Ce que vous installez | Faut-il la version précédente ? |
|---|---|---|
| v0.1 Sentinel | Le programme `squeeze` | — |
| v0.2 squeeze-Auditor | Le même programme `squeeze`, avec désormais un installateur, une application dans le menu Démarrer (`squeeze-auditor`) et de nouveaux hooks | **Non, elle inclut la v0.1.** Installez-la par-dessus la v0.1 ou seule. |
| v0.3 | Prévue | À annoncer |

- **Le programme est cumulatif.** Un `squeeze` plus récent contient toutes les fonctions précédentes. Vous n'installez jamais la v0.1 et la v0.2 côte à côte.
- **Les dossiers décrivent ce que chaque version a ajouté.** On n'installe pas un dossier.
- **Vos réglages sont conservés.** `.squeeze/config.json` et les hooks de `squeeze init` (v0.1) fonctionnent toujours après mise à jour. La v0.2 ajoute un `~/.squeeze/config.json` global et se connecte une seule fois à Claude Code pour tous les projets.

| Vous utilisez… | Installez |
|---|---|
| Claude Code | squeeze-Auditor (v0.2), puis **Connecter Claude Code** |
| L'application de bureau Claude | squeeze-Auditor (v0.2), puis **Connecter l'application de bureau** |
| claude.ai dans un navigateur, ou les applications mobiles | `squeeze clip` pour les longs collages dès maintenant ; la v0.3 à sa sortie |

## Où ça marche

| Où | Effet | Comment |
|---|---|---|
| **Claude Code** | Automatique, tous les projets | squeeze-Auditor (ou `squeeze setup`) ajoute des hooks à `~/.claude/settings.json`. Les quatre blocages de la v0.2 et les deux gardes de la v0.1 fonctionnent. |
| **Application de bureau Claude** (Windows, macOS) | Quand Claude lit des fichiers ou enregistre une passation | La connexion ajoute les outils `squeeze mcp` : Claude lit vos journaux déjà compressés, uniquement dans les dossiers autorisés, et peut enregistrer et charger des notes de passation. squeeze ne peut ni voir ni modifier les conversations. |
| **claude.ai (web, mobile)** | Manuel, quand vous collez un long texte | Copiez un log, lancez `squeeze clip`, collez. Prévu pour la v0.3. |
| **Tout autre outil en terminal** | Manuel | `<commande> 2>&1 \| squeeze compress` |

Avec le forfait Pro, Claude Code et claude.ai partagent une même limite d'utilisation : les tokens économisés dans Claude Code restent disponibles pour vos conversations.

## v0.2 squeeze-Auditor

La v0.1 regardait une sortie à la fois. **La v0.2 regarde la session entière** : ce que Claude a déjà lu, la taille de la conversation, ce qui accompagne chaque message. Elle bloque quatre types de gaspillage :

| Blocage | Ce qu'il repère | Comment il bloque |
|---|---|---|
| **Lecture répétée** | Relire en entier un fichier qui n'a pas changé | Refuse le `Read` : « vous l'avez déjà ». Autorisé à nouveau si le fichier change ou après compactage. |
| **Pause budget** | Continuer d'envoyer des messages dans une session énorme (150K tokens par défaut) | Retient votre message une fois et propose `/compact` ou `/clear`. Renvoyez-le et il passe. |
| **Passation** | Une longue session qui relit tout son historique (120K par défaut) | Claude écrit une courte note ; après `/clear`, la nouvelle session part de la note au lieu de toute la conversation. |
| **Coût fixe** | Fichiers `CLAUDE.md`, skills, agents et listes d'outils MCP envoyés avec chaque message | Mesure chacun ; coupez d'un clic les serveurs MCP inutilisés (l'interrupteur de Claude Code), ou automatiquement après 14 jours sans usage. |

**squeeze-Auditor** est l'application que vous ouvrez depuis le menu Démarrer ou Applications : un tableau de bord de l'usage réel de Claude Code à côté de ce que squeeze a écarté, des modes d'économie (Fort, Équilibré, Léger), tous les réglages, le diagnostic des coûts fixes, vos notes de passation et les connexions à Claude Code et à l'application de bureau.

![Tableau de bord de squeeze-Auditor](../v0.2-Auditor/v0.2.0_squeeze-auditor/docs/images/auditor-en-dashboard.png)

Documentation complète de la v0.2 : [v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.fr.md](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.fr.md).

## v0.1 Sentinel

Sentinel monte la garde entre Claude Code et votre shell. Un seul binaire, Rust, **zéro dépendance**, macOS / Linux / Windows.

- **Garde Bash** : les commandes bavardes (`npm install`, `go test`, `cargo build`, `pytest`, `make`, `apt-get install`, `journalctl`, `docker build`, `kubectl logs`, `git log`, etc.) sont redirigées vers `squeeze run -- <commande>`. Celles dont la sortie est déjà limitée (`| tail -20`, `-n 50`, `> fichier`, `--quiet`) ne sont pas touchées.
- **Garde Read** : lire un fichier de plus de 400 lignes sans `offset`/`limit` est redirigé. Claude reçoit un plan avec numéros de ligne et choisit une plage.
- **Compression intelligente** : les 8 premières et 20 dernières lignes, chaque erreur et avertissement avec 2 lignes de contexte, les traces de pile ; répétitions regroupées avec un compteur ; les erreurs passent avant les avertissements.
- **Code de sortie conservé** : `squeeze run` se termine avec le code de la commande elle-même.
- **Logs complets sur disque** : `.squeeze/raw/`, les 50 plus récents, lisibles par vous seul. `.squeeze/` est ajouté au `.gitignore`.
- **`squeeze clip`** pour claude.ai et **`squeeze report`** pour vos économies réelles.
- **Sûr par conception** : les hooks n'approuvent jamais rien à votre place, s'effacent à la moindre erreur (fail open), et `squeeze uninstall` retire tout ce qu'il a ajouté. Aucun code réseau.

Ce que Claude voit après `squeeze run -- pytest -v` (sortie réelle, 217 lignes → 39) :

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
… (les deux tracebacks sont gardées en entier) …
=========================== short test summary info ============================
FAILED test_inventory.py::test_bulk_order - AssertionError: bulk order total ...
FAILED test_inventory.py::test_missing_sku - KeyError: 'B7'
======================== 2 failed, 181 passed in 0.14s =========================
[squeeze] 217 lines (15.8 KB) → 39 lines (1.9 KB). Full output: .squeeze/raw/20261103-044044-45b0e7.log (exit 1)
```

Documentation complète de la v0.1 : [v0.1-Sentinel/v0.1.0_squeeze-sentinel/README.fr.md](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README/README.fr.md).

## Installation

**Installateur (recommandé).** Depuis la [dernière version](https://github.com/jongwooish/claude-squeeze/releases/latest) :

| Système | Fichier |
|---|---|
| Windows 10/11 | `squeeze-Auditor-0.2.0-setup-x64.exe` (ARM : `-setup-arm64.exe`) |
| macOS | `squeeze-Auditor-0.2.0-macos-arm64.pkg` (Intel : `-macos-x86_64.pkg`) |
| Linux (Debian, Ubuntu) | `squeeze-auditor_0.2.0_amd64.deb` (ARM : `_arm64.deb`) |

Les installateurs ne sont pas encore signés. Windows SmartScreen : **Informations complémentaires → Exécuter quand même**. macOS : clic droit → **Ouvrir**. Détails : [installation de la v0.2](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.fr.md#installation).

**Terminal :**

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.ps1 | iex

# Depuis les sources (tout système avec Rust 1.80+)
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

Chaque fichier publié figure dans `checksums.txt` (SHA-256), que les scripts d'installation vérifient. `SQUEEZE_VERSION=v0.1.0` fixe une version ; `SQUEEZE_INSTALL_DIR` change le dossier.

## Démarrage rapide

1. Ouvrez **squeeze-Auditor** et cliquez sur **Connecter Claude Code** (tous les projets). Vous utilisez l'application de bureau ? Connectez-la dans **Connexions** et redémarrez-la.
2. Utilisez Claude Code comme d'habitude.
3. Revenez au tableau de bord pour voir ce qui a été écarté. Réglez la force de squeeze dans **Modes d'économie** ou **Réglages**.

Depuis un terminal :

```bash
squeeze setup      # connecter Claude Code (et l'application de bureau si elle est installée)
claude             # utilisez Claude Code comme d'habitude
squeeze auditor    # ouvrir la fenêtre
squeeze setup --remove
```

La connexion ne modifie que `~/.claude/settings.json` (et `claude_desktop_config.json` pour l'application de bureau). Les autres réglages et hooks sont conservés, et l'original est d'abord sauvegardé dans `~/.squeeze/backups/`. Le `squeeze init` par projet de la v0.1 fonctionne toujours.

## Utilisation

| Commande | Ce qu'elle fait |
|---|---|
| `squeeze auditor` | Ouvrir la fenêtre squeeze-Auditor |
| `squeeze setup [--claude-code] [--desktop] [--remove]` | Connecter ou déconnecter Claude Code (tous les projets) et l'application de bureau |
| `squeeze doctor [--dir D] [--desktop] [--json]` | Ce qui part avec chaque message (coût fixe) |
| `squeeze init [--dir D] [--bin PATH] [--dry-run]` | Installer les hooks dans un seul projet (façon v0.1) |
| `squeeze uninstall [--dir D] [--purge] [--dry-run]` | Retirer les hooks et la note d'un projet ; `--purge` supprime aussi `.squeeze/` |
| `squeeze run -- <commande>` | Lancer une commande ; résumé court si la sortie est longue |
| `<commande> 2>&1 \| squeeze compress` | Compresser une sortie redirigée |
| `squeeze clip [--print]` | Compresser le presse-papiers (pour claude.ai) |
| `squeeze report [--since 7d] [--global] [--json]` | Afficher les économies |
| `squeeze bench <dir> [--markdown]` | Rejouer des fichiers `*.log` et mesurer |
| `squeeze mcp` | Le serveur MCP que lance l'application de bureau |
| `squeeze version` | Afficher la version |

Préfixez une commande avec `SQUEEZE_SKIP=1` pour obtenir sa sortie brute une fois, ou mettez `SQUEEZE_SKIP` dans un message pour passer la pause budget. Réglages : [configuration de la v0.2](../v0.2-Auditor/v0.2.0_squeeze-auditor/README/README.fr.md#configuration).

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
        ├─▶ sortie complète ── .squeeze/raw/<heure>.log
        └─▶ sortie courte ──── Claude (erreurs, avertissements, début, fin, chemin du log)
```

Pourquoi « refuser avec un indice » plutôt que réécrire la commande en silence ? Un hook qui réécrit une commande doit aussi l'approuver, ce qui contournerait vos demandes de permission. squeeze n'approuve jamais rien à votre place.

### En quoi squeeze diffère des wrappers

Des outils comme [chaude](https://github.com/int-ttt/chaude) enveloppent la CLI de Claude Code dans leur propre écran et économisent des tokens en faisant tourner les sessions et en confiant le travail à des modèles moins chers. squeeze s'attaque à l'autre côté du problème :

- **Pas de wrapper.** Vous continuez d'utiliser `claude` tel quel, via les hooks propres à Claude Code.
- **Il arrête le gaspillage à l'entrée** au lieu de nettoyer un contexte plein.
- **Les deux approches peuvent être combinées.**

## Feuille de route

- [x] **v0.1 Sentinel** : hooks Claude Code, `run`, `compress`, `clip`, `report`, `bench`
- [x] **v0.2 squeeze-Auditor** : blocages lecture répétée, budget, passation et coût fixe ; application avec tableau de bord et modes d'économie ; outils pour l'application de bureau Claude
- [ ] **v0.3** : claude.ai dans le navigateur et les applications mobiles, avec une nouvelle approche des problèmes de tokens les plus signalés
- [ ] Installateurs signés
- [ ] Benchmark A/B réel avec Claude Code (mêmes tâches, squeeze activé et désactivé)
- [ ] v1.0 : commandes et configuration stables

## FAQ

**Q. Claude peut-il manquer quelque chose d'important ?**
R. Dans tous les journaux de test, chaque message d'erreur et d'avertissement distinct a été conservé. Si le résumé ne suffit pas, la dernière ligne donne le chemin du log complet. Quand une relecture est bloquée, squeeze indique aussi à Claude comment relire le fichier s'il en a vraiment besoin.

**Q. squeeze envoie-t-il mes données quelque part ?**
R. Non. Il n'a pas de code réseau ; la fenêtre squeeze-Auditor est une page sur `127.0.0.1` que seul votre ordinateur peut ouvrir.

**Q. Peut-il casser ma session Claude Code ?**
R. Les hooks échouent en mode ouvert : entrée inattendue, fichier manquant ou plantage interne ne produisent aucune sortie, et Claude Code continue normalement. squeeze n'approuve jamais un outil à votre place.

**Q. Économise-t-il des tokens dans les conversations claude.ai classiques ?**
R. Seulement quand vous collez un long texte après `squeeze clip`. Le navigateur et les applications mobiles sont l'objet de la v0.3.

**Q. Le tableau de bord affiche-t-il la limite de mon offre ?**
R. Non. Il lit les journaux de Claude Code sur cet ordinateur. Il n'existe pas de moyen officiel pour d'autres programmes de lire l'usage de l'offre.

**Q. Pourquoi une version 0.x ?**
R. Tant que la version majeure est 0, les noms de commandes et la configuration peuvent encore changer. La v1.0 signifiera qu'ils sont stables.

## Organisation du dépôt

```
claude-squeeze/
├── README/                      ← présentation en 6 langues (README.md = anglais)
├── CHANGELOG/                   ← CHANGELOG en 6 langues
├── v0.1-Sentinel/               ← v0.1 Sentinel
│   ├── README.md                   présentation de la version
│   └── v0.1.0_squeeze-sentinel/   ← la v0.1.0 telle que publiée (figée)
├── v0.2-Auditor/                ← v0.2 squeeze-Auditor
│   ├── README.md                   présentation de la version
│   └── v0.2.0_squeeze-auditor/    ← v0.2 : le programme squeeze actuel (Rust, sans dépendance)
│       ├── README.md               documentation complète de la v0.2 (anglais)
│       ├── README/                 traductions (5 langues)
│       ├── src/                    moteur, hooks, blocages, serveur de squeeze-Auditor
│       ├── ui/                     la fenêtre squeeze-Auditor (un fichier HTML)
│       ├── packaging/              installateurs Windows, macOS et Linux, icône
│       ├── docs/                   benchmark, guide utilisateur (coréen), captures
│       └── testdata/fixtures/      logs utilisés par les tests et le benchmark
├── INSTALL/                     ← install.sh / install.ps1
├── CARGO/                       ← Cargo.toml / Cargo.lock (workspace Rust)
├── .github/                     ← formulaires d'issues, CI, publication, README de la page d'accueil
├── CLAUDE.md
└── LICENSE
```

## Contribuer

```bash
cargo test --manifest-path CARGO/Cargo.toml
cargo clippy --manifest-path CARGO/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures
```

Les nouveaux logs de test sont bienvenus : ajoutez un vrai `*.log` (sans secrets ni chemins personnels) dans `v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures/`. Les tests échouent si une erreur ou un avertissement distinct est perdu. Bugs et idées : [GitHub Issues](https://github.com/jongwooish/claude-squeeze/issues).

## Communauté

**Un problème ou une idée ? Ouvrez une [issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose).** Améliorations, sortie de la v1.0, mises à jour de version, erreurs d'installation ou tout autre souci logiciel : chaque retour aide à construire la prochaine version.


| Je veux… | Aller à |
|---|---|
| Signaler un bug ou une session cassée | [Issues → Bug report](https://github.com/jongwooish/claude-squeeze/issues/new?template=bug_report.yml) |
| Signaler une ligne perdue, trop peu d'économie, ou une commande (non) interceptée | [Issues → Compression problem](https://github.com/jongwooish/claude-squeeze/issues/new?template=compression.yml) |
| Signaler un problème d'installation, de mise à jour ou de version | [Issues → Install, update or version problem](https://github.com/jongwooish/claude-squeeze/issues/new?template=version_problem.yml) |
| Proposer une fonctionnalité | [Issues → Feature idea](https://github.com/jongwooish/claude-squeeze/issues/new?template=feature_request.yml) |
| Poser une question ou obtenir de l'aide | [Discussions → Q&A](https://github.com/jongwooish/claude-squeeze/discussions/categories/q-a) |
| Parler de la feuille de route et de ce que doit contenir la v1.0 | [Discussions → Ideas](https://github.com/jongwooish/claude-squeeze/discussions/categories/ideas) |
| Être informé des nouvelles versions | [Discussions → Announcements](https://github.com/jongwooish/claude-squeeze/discussions/categories/announcements) et [Releases](https://github.com/jongwooish/claude-squeeze/releases) (cliquez **Watch → Custom → Releases** pour être notifié) |

Écrivez dans n'importe laquelle des six langues. Problèmes de sécurité : [SECURITY.md](../.github/SECURITY.md). Comment contribuer : [CONTRIBUTING.md](../.github/CONTRIBUTING.md).

## Licence

MIT. Voir [LICENSE](../LICENSE). squeeze est un outil communautaire non officiel, sans lien avec Anthropic.
