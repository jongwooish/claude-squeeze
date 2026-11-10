# Journal des modifications

[English](CHANGELOG.md) | [한국어](CHANGELOG.ko.md) | [日本語](CHANGELOG.ja.md) | [简体中文](CHANGELOG.zh-CN.md) | [Español](CHANGELOG.es.md) | **Français**

Les changements notables sont listés ici. Les versions suivent le [versionnage sémantique](https://semver.org/lang/fr/). Tant que la version majeure vaut 0, les commandes et la configuration peuvent encore changer.

## [0.2.0] - 2026-11-04 — squeeze-Auditor

La v0.2 regarde la session entière, pas une sortie à la fois. Code dans [`v0.2-Auditor/v0.2.0_squeeze-auditor/`](../v0.2-Auditor/v0.2.0_squeeze-auditor/). Inclut tout ce qu'apportait la 0.1.0.

### Ajouté

- Quatre blocages de session (Claude Code) :
  - **Lecture répétée** : un `Read` complet d'un fichier déjà lu dans cette session et inchangé est refusé. Autorisé à nouveau si le fichier change ou si le contexte est compacté.
  - **Pause budget** (`UserPromptSubmit`) : quand la session dépasse le budget (150K tokens par défaut), le message est retenu une fois avec la suggestion de `/compact` ou `/clear`. Le renvoyer le laisse passer.
  - **Passation** (`Stop` + `SessionStart`) : quand la session dépasse la taille de passation (120K par défaut), Claude écrit une note dans `.squeeze/handoff/` ; après `/clear`, la nouvelle session part de la note. Le mode `remind` se contente de suggérer `/compact`.
  - **Coût fixe** : mesure les fichiers `CLAUDE.md` et leurs imports, les skills, les agents et les listes d'outils MCP envoyés avec chaque message. Coupez les serveurs MCP par projet avec l'interrupteur de Claude Code, ou automatiquement après N jours sans usage (désactivé par défaut).
- Application **squeeze-Auditor** (`squeeze-auditor`, `squeeze auditor`) : tableau de bord de l'usage réel de Claude Code d'après ses journaux et de ce que squeeze a écarté, fenêtre de 5 heures en cours, modes d'économie (Fort, Équilibré, Léger, Personnalisé), tous les réglages, diagnostic des coûts fixes avec Mesurer et Allumer/Éteindre, notes de passation, connexions. Uniquement en local (`127.0.0.1`, clé à usage unique). Anglais et coréen.
- Prise en charge de l'**application de bureau Claude** : `squeeze mcp`, un serveur MCP avec `read_log`, `tail_log`, `search_log`, `list_logs`, `save_handoff`, `load_handoff`, limité aux dossiers autorisés.
- `squeeze setup [--claude-code] [--desktop] [--remove]` : connexion unique pour tous les projets (`~/.claude/settings.json`) et l'application de bureau, avec sauvegardes dans `~/.squeeze/backups/`.
- `squeeze doctor` : coût fixe par message dans le terminal.
- Installateurs : Windows Setup (x64, ARM64, par utilisateur, icône dans le menu Démarrer, désinstallation qui déconnecte), `.pkg` macOS avec `squeeze-Auditor.app`, `.deb` Linux avec une entrée de menu. Pas encore signés.
- Configuration globale `~/.squeeze/config.json` (le `.squeeze/config.json` d'un projet reste prioritaire) et journal global `~/.squeeze/ledger.jsonl`.

### Modifié

- Les scripts d'installation installent aussi `squeeze-auditor`.
- Le titre de chaque version publiée vient désormais du titre du CHANGELOG. La CI construit aussi les installateurs et teste l'instantané figé de la v0.1.0.
- La crate figée de la v0.1.0 s'appelle `squeeze-sentinel` (le programme reste `squeeze`), donc `cargo install --git … squeeze` construit la v0.2.

### Prévu

- v0.3 : claude.ai dans le navigateur et les applications mobiles, avec une nouvelle approche.

## [0.1.0] - 2026-11-03 — Sentinel

Première version. Code dans [`v0.1-Sentinel/v0.1.0_squeeze-sentinel/`](../v0.1-Sentinel/v0.1.0_squeeze-sentinel/).

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
