# Journal des modifications

[English](CHANGELOG.md) | [한국어](CHANGELOG.ko.md) | [日本語](CHANGELOG.ja.md) | [简体中文](CHANGELOG.zh-CN.md) | [Español](CHANGELOG.es.md) | **Français**

Les changements notables sont listés ici. Les versions suivent le [versionnage sémantique](https://semver.org/lang/fr/). Tant que la version majeure vaut 0, les commandes et la configuration peuvent encore changer.

## [0.1.0] - 2026-11-03 — Sentinel

Première version. Code dans [`v0.1_squeeze-sentinel/`](../v0.1_squeeze-sentinel/).

### Ajouté

- Hooks `PreToolUse` de Claude Code :
  - **Garde Bash** : redirige les commandes bavardes (installations, builds, tests, logs) vers `squeeze run`. Ne touche pas aux commandes déjà bornées (`| tail`, `-n 50`, redirections, `--quiet`).
  - **Garde Read** : redirige la lecture complète des fichiers de plus de 400 lignes vers une lecture par plage, avec un plan.
  - Les hooks n'approuvent jamais un appel d'outil et s'effacent à la moindre erreur (fail open).
- `squeeze run -- <commande>` : lancer, compresser une longue sortie, enregistrer le log complet, conserver le code de sortie.
- `squeeze compress` : compresser une sortie reçue par pipe.
- `squeeze clip` : compresser le presse-papiers pour le coller dans claude.ai (macOS, Windows, Linux).
- `squeeze report` : économies par projet (`--global` pour clip, `--json`, `--since`).
- `squeeze bench` : rejouer des logs et mesurer les économies et les messages conservés.
- `squeeze init` / `squeeze uninstall` : installer et retirer proprement hooks, configuration, entrée `.gitignore` et note `CLAUDE.md`.
- Compresseur : garde le début et la fin, chaque erreur et avertissement distinct avec son contexte, les traces de pile ; regroupe les répétitions ; privilégie les erreurs.
- Sept logs de benchmark : 93,8 % de tokens estimés en moins, 50/50 messages distincts conservés.
- Installateurs pour macOS/Linux (`install.sh`) et Windows (`install.ps1`) avec vérification SHA-256.
- README en anglais, coréen, japonais, chinois simplifié, espagnol et français.

### Prévu

- v0.2 Interceptor : extension de navigateur pour les collages dans claude.ai.
- v0.3 Conduit : serveur MCP pour l'application de bureau Claude.
- Benchmark A/B en direct avec Claude Code.
