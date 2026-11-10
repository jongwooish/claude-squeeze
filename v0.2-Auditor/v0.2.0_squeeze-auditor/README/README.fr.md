# v0.2.0 squeeze-Auditor

[English](../README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | **Français**

> 💬 **Vos retours sont les bienvenus.** Améliorations, version 1.0, mises à jour, erreurs d'installation ou tout autre problème : [ouvrez une issue](https://github.com/jongwooish/claude-squeeze/issues/new/choose) (dans l'une des 6 langues).

← [Retour à squeeze](../../../README/README.fr.md)

v0.1 Sentinel arrêtait une longue sortie à la fois. **v0.2 Auditor regarde la session entière.** Il se souvient de ce que Claude a déjà lu, surveille la taille de la conversation et vérifie ce qui part avec chaque message avant même que vous tapiez. Puis il bloque quatre types de gaspillage qu'une simple mesure de taille ne voit pas.

C'est un programme à installer : lancez l'installateur, ouvrez **squeeze-Auditor** depuis le menu Démarrer (ou Applications), et vous voyez où sont partis vos tokens, ce que squeeze a écarté, et vous réglez la force des économies.

**Fonctionne dans :** Claude Code (tous les projets) et l'application de bureau Claude (Windows et macOS). claude.ai dans le navigateur et les applications mobiles arrivent avec la v0.3.
**Prérequis :** aucun. La v0.2 est le même programme `squeeze` que la v0.1, avec tout ce que faisait la v0.1 plus ce qui suit. Voir [comment les versions s'articulent](../../../README/README.fr.md#comment-les-versions-sarticulent).

![Tableau de bord de squeeze-Auditor](../docs/images/auditor-en-dashboard.png)

## Nouveautés : quatre blocages

| | Blocage | Ce qu'il repère | Comment il bloque |
|---|---|---|---|
| 1 | **Lecture répétée** | Claude relit en entier un fichier qui n'a pas changé | Le `Read` est refusé avec « vous l'avez déjà lu, utilisez ce que vous avez ». Autorisé à nouveau dès que le fichier change ou que la conversation est compactée. |
| 2 | **Pause budget** | Vous continuez d'envoyer des messages dans une session devenue énorme | Avant l'envoi, squeeze retient le message une fois et propose `/compact` ou `/clear`. Renvoyez-le (↑, Entrée) et il passe. Il ne redemande qu'une fois la session grossie de 50 % de plus. |
| 3 | **Passation (handoff)** | Une longue session qui relit tout son historique à chaque fois | Quand la session dépasse la taille de passation, Claude écrit une courte note avant de s'arrêter. Vous lancez `/clear` ; la nouvelle session démarre de la note (environ 1 000 tokens) au lieu de toute la conversation. |
| 4 | **Coût fixe** | Ce qui part avec *chaque* message : fichiers `CLAUDE.md` et leurs imports, descriptions des skills et agents, listes d'outils des serveurs MCP | squeeze-Auditor mesure chacun. Coupez d'un clic les serveurs MCP inutilisés, avec l'interrupteur de Claude Code lui-même (le même que `/mcp`). En option : couper automatiquement les serveurs inutilisés depuis 14 jours dans un projet. |

### Ce qui change par rapport à la v0.1

| | v0.1 Sentinel | v0.2 Auditor |
|---|---|---|
| Regarde | la taille d'**une** sortie ou d'un fichier | la **session** : ce qui a été lu, sa taille actuelle, ce qui part à chaque fois |
| Décide selon | « cette sortie est-elle longue ? » | la mémoire (même fichier, inchangé), les totaux (taille de session), l'historique (serveurs jamais utilisés) |
| Hooks | `PreToolUse` (Bash, Read) | aussi `UserPromptSubmit`, `Stop`, `SessionStart`, `PreCompact` |
| Installé avec | `squeeze init` par projet | l'installateur ou `squeeze setup`, une fois pour **tous** les projets |
| Vous voyez | `squeeze report` dans un terminal | la fenêtre squeeze-Auditor |

Tout ce que faisait la v0.1 continue : les commandes bavardes passent par `squeeze run`, les gros fichiers reçoivent un sommaire.

## La fenêtre squeeze-Auditor

| Onglet | Ce que vous pouvez faire |
|---|---|
| **Tableau de bord** | L'usage réel de Claude Code d'après ses propres journaux de session (aujourd'hui, 7 ou 30 jours), à côté de ce que squeeze a écarté. La fenêtre de 5 heures en cours et le rythme de consommation. Ce qui a été bloqué, par type. Les sessions récentes et leur taille par rapport au budget. |
| **Modes d'économie** | Fort, Équilibré (par défaut) ou Léger, avec les valeurs de chacun. Ou Personnalisé. |
| **Réglages** | Toutes les valeurs : quand compresser la sortie, combien de lignes garder, la taille d'un gros fichier, le budget, la taille et le style de passation, les règles de coût fixe, les commandes à ne jamais toucher. |
| **Coût fixe** | Par projet (ou pour l'application de bureau) : chaque fichier mémoire, import, skill, agent et serveur MCP avec ses tokens par message. Mesurer un serveur MCP, l'allumer ou l'éteindre. Conseils pour réduire. |
| **Passations** | Toutes les notes de passation, lisibles sur place. |
| **Connexions** | Connecter ou déconnecter Claude Code et l'application de bureau. Choisir les dossiers que l'application de bureau peut lire. |

La fenêtre est une petite page servie par squeeze uniquement sur `127.0.0.1`, avec une clé différente à chaque lancement, affichée dans une fenêtre d'application Edge ou Chrome (ou votre navigateur par défaut). Elle se ferme d'elle-même peu après la fermeture de la fenêtre. Rien ne quitte votre ordinateur. En anglais et en coréen.

![Onglet coût fixe](../docs/images/auditor-en-fixed.png)

## Effet

**Compression de sortie** (de la v0.1, inchangée) : sept journaux réels, 83 325 → 5 128 tokens estimés (**−93,8 %**), chaque erreur et avertissement distinct conservé. Méthode : [docs/bench.md](../docs/bench.md).

**Les quatre blocages** économisent autrement : ils empêchent les mêmes tokens d'être renvoyés encore et encore. Leur valeur dépend de votre façon de travailler, voici donc le calcul, pas un benchmark :

| Blocage | Exemple | Écarté |
|---|---|---|
| Lecture répétée | Un fichier de 600 lignes (environ 7 000 tokens) relu sans changement | environ 7 000 tokens, plus autant à chaque message suivant de la session |
| Pause budget / passation | Continuer 20 messages dans une session de 150 000 tokens, contre une nouvelle session partant d'une note de 1 000 tokens | environ 3 000 000 de tokens d'entrée relus, contre environ 20 000 plus le nouveau travail |
| Coût fixe | Un serveur MCP inutilisé à 12 outils (1 882 tokens, mesuré sur un serveur de test) sur 100 messages | environ 188 000 tokens |

**Limites, en toute honnêteté.**
- Les tokens sont estimés (octets ÷ 4). L'essentiel d'une longue session est relu depuis le cache de prompts de Claude, moins cher qu'une nouvelle entrée, donc l'économie en coût est plus faible que le nombre de tokens ne le laisse penser. Cela remplit tout de même la fenêtre de contexte et compte dans l'usage.
- L'usage du tableau de bord provient des journaux locaux de Claude Code. Il n'inclut ni claude.ai ni les conversations de l'application de bureau, et ce n'est pas la limite officielle de votre offre. La fenêtre de 5 heures est une estimation de la façon dont Claude Code la compte.
- squeeze ne voit pas les conversations de l'application de bureau. Là, il n'économise que lorsque Claude lit des journaux via les outils squeeze, ou quand vous lui demandez d'enregistrer une passation.

## Installation

Téléchargez depuis la [dernière version](https://github.com/jongwooish/claude-squeeze/releases/latest) :

| Système | Fichier | Ensuite |
|---|---|---|
| Windows 10/11 (la plupart des PC) | `squeeze-Auditor-0.2.0-setup-x64.exe` | Lancez-le. Pas besoin de droits administrateur. Menu Démarrer → **squeeze-Auditor** |
| Windows sur ARM | `squeeze-Auditor-0.2.0-setup-arm64.exe` | Pareil |
| macOS, Apple silicon | `squeeze-Auditor-0.2.0-macos-arm64.pkg` | Ouvrez-le. Applications → **squeeze-Auditor** |
| macOS, Intel | `squeeze-Auditor-0.2.0-macos-x86_64.pkg` | Pareil |
| Linux (Debian, Ubuntu) | `squeeze-auditor_0.2.0_amd64.deb` / `_arm64.deb` | `sudo apt install ./squeeze-auditor_0.2.0_amd64.deb`, puis le menu des applications |

**Les installateurs ne sont pas encore signés**, donc Windows et macOS peuvent les arrêter la première fois. Voir [Si le système le bloque](#si-le-système-le-bloque).

Installation en terminal (les deux programmes, sans icône dans le menu Démarrer) :

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.sh | bash
# Windows (PowerShell)
irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.ps1 | iex
# Depuis les sources (Rust 1.80+)
cargo install --git https://github.com/jongwooish/claude-squeeze squeeze
```

### Si le système le bloque

Les fichiers de la version ne sont pas encore signés, donc Windows et macOS ne savent pas qui les a créés. Ils sont compilés par GitHub Actions à partir de ce dépôt. Avant de passer outre un avertissement, vérifiez que le fichier correspond à `checksums.txt` sur la page de la version :

```bash
# macOS
shasum -a 256 -c checksums.txt --ignore-missing
# Linux
sha256sum -c checksums.txt --ignore-missing
# Windows (PowerShell): comparez avec le hash de la page de la version
Get-FileHash .\squeeze-Auditor-0.2.0-setup-x64.exe -Algorithm SHA256
```

**Windows**
- **SmartScreen** (« Windows a protégé votre ordinateur ») : **Informations complémentaires** → **Exécuter quand même**.
- **Smart App Control** (« Smart App Control a bloqué une application potentiellement dangereuse ») : il n'y a pas de bouton pour l'exécuter quand même, et il vérifie chaque programme, donc `squeeze.exe` et `squeeze-auditor.exe` seraient aussi bloqués après l'installation. Essayez squeeze dans une machine virtuelle ou dans le Bac à sable Windows, ou désactivez-le : Sécurité Windows → Contrôle des applications et du navigateur → Paramètres de Smart App Control → **Désactivé**. Sous Windows 11 25H2 et versions ultérieures, vous pourrez le réactiver ; sur les versions plus anciennes, il reste désactivé jusqu'à la réinstallation de Windows.
- **Zip téléchargé avec le navigateur** : avant l'extraction, clic droit → Propriétés → **Débloquer**, ou `Unblock-File .\squeeze-*.zip`.
- **Antivirus** : signalez le faux positif à l'éditeur ou ajoutez une exception pour `%LOCALAPPDATA%\Programs\squeeze`.

**macOS**
- **« impossible d'ouvrir » / « développeur non identifié »** : Réglages Système → Confidentialité et sécurité → faites défiler → **Ouvrir quand même** à côté du nom du fichier, puis saisissez votre mot de passe. Sous macOS 15 et versions ultérieures, clic droit → Ouvrir ne contourne plus cette vérification.
- **« est endommagé et ne peut pas être ouvert »** : l'app porte encore la marque de téléchargement. Lancez `xattr -dr com.apple.quarantine /Applications/squeeze-Auditor.app`.
- **Programmes en ligne de commande du `.tar.gz`** : `xattr -d com.apple.quarantine squeeze squeeze-auditor`.

**Linux**
- Installez le `.deb` avec `sudo apt install ./squeeze-auditor_0.2.0_amd64.deb`. Certaines logithèques avertissent pour les paquets hors de votre distribution ; apt dans un terminal ne le fait pas.
- **« Permission denied »** avec les programmes du `.tar.gz` : `chmod +x squeeze squeeze-auditor`. Si le dossier est monté en `noexec` (certains `/tmp` et clés USB), déplacez-les dans `~/.local/bin`.
- Autres distributions (Fedora, Arch, etc.) : utilisez le `.tar.gz` ou l'installation en terminal ci-dessus.

## Démarrage rapide

1. Installez, puis ouvrez **squeeze-Auditor**. (Sous Windows, l'installateur peut connecter Claude Code pour vous : laissez la case cochée.)
2. Cliquez sur **Connecter Claude Code**. Cela ajoute les hooks de squeeze à `~/.claude/settings.json` pour tous les projets. Vos autres réglages et hooks sont conservés, et une sauvegarde est écrite d'abord.
3. Vous utilisez l'application de bureau Claude ? Allez dans **Connexions**, cliquez sur **Connecter**, ajoutez les dossiers dont Claude peut lire les journaux, puis redémarrez l'application.
4. Utilisez Claude Code comme d'habitude. Revenez au tableau de bord pour voir ce qui a été écarté.

Depuis un terminal : `squeeze setup` (connecter), `squeeze setup --remove` (déconnecter), `squeeze auditor` (ouvrir la fenêtre).

**Depuis la v0.1 :** installez par-dessus (l'installateur Windows utilise le même dossier que `install.ps1` de la v0.1) et connectez une fois. Les projets où vous avez lancé `squeeze init` continuent de fonctionner ; gardez leurs hooks ou retirez-les avec `squeeze uninstall`. Sous Windows, désinstaller squeeze-Auditor le déconnecte aussi de Claude Code et de l'application de bureau ; sous macOS et Linux, lancez d'abord `squeeze setup --remove`.

## Ce qui se passe pendant une session

```
SessionStart ──▶ une note de passation récente ?         ──oui──▶ la charger dans la nouvelle session
     │
Vous tapez un message
     │
UserPromptSubmit ──▶ session ≥ budget (150K) ?           ──oui──▶ le retenir une fois : « /compact ou /clear, ou renvoyez »
     │
Claude travaille
     ├─ Bash ──▶ commande bavarde ?                       ──oui──▶ « lancez-la avec squeeze run -- … »  (v0.1)
     └─ Read ──▶ lecture complète d'un gros fichier ?     ──oui──▶ sommaire + lecture par plage        (v0.1)
                 même fichier, inchangé, déjà lu en entier ? ──oui──▶ « utilisez ce que vous avez lu » (v0.2)
     │
Stop ──▶ session ≥ taille de passation (120K) ?          ──oui──▶ Claude écrit .squeeze/handoff/<heure>.md
     │                                                            et vous dit de lancer /clear
PreCompact ──▶ oublier quels fichiers ont été lus (Claude ne les a plus)
```

Tous les hooks échouent en mode ouvert : en cas d'erreur, aucune sortie, et Claude Code continue. squeeze n'approuve jamais un outil à votre place.

## Application de bureau Claude

La connexion ajoute une entrée à `claude_desktop_config.json` : un serveur MCP, `squeeze mcp`, avec six outils.

| Outil | Ce qu'il fait |
|---|---|
| `read_log` | Lire un journal ou un fichier texte déjà compressé (ou une plage de lignes exacte) |
| `tail_log` | Les dernières lignes d'un journal, compressées |
| `search_log` | Les lignes correspondantes avec un peu de contexte |
| `list_logs` | Les fichiers d'un dossier autorisé |
| `save_handoff` / `load_handoff` | Enregistrer la conversation en note de passation et en démarrer une nouvelle à partir d'elle |

Claude ne peut lire que les dossiers listés dans **Connexions**. Au lieu de coller un journal de 5 000 lignes, demandez « lis la dernière erreur du journal de mon application » : Claude reçoit la version courte. Quand une conversation devient longue, demandez une passation, ouvrez une nouvelle conversation et demandez de la charger.

## Modes d'économie

| | Fort | Équilibré (par défaut) | Léger |
|---|---:|---:|---:|
| Sortie laissée telle quelle jusqu'à | 40 lignes | 80 lignes | 150 lignes |
| Gros fichier au-delà de | 250 lignes | 400 lignes | 800 lignes |
| Pause budget à | 100K tokens | 150K tokens | 200K tokens |
| Passation à | 80K tokens | 120K tokens | 160K tokens |

## Configuration

Les réglages globaux sont dans `~/.squeeze/config.json` (écrit par l'onglet Réglages). Un projet peut remplacer n'importe quelle clé dans son propre `.squeeze/config.json`. Toutes les clés de la v0.1 restent valables ([liste](../../../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README/README.fr.md#configuration)). Nouveautés de la v0.2 :

| Clé | Défaut | Signification |
|---|---|---|
| `profile` | `"balanced"` | `strong`, `balanced`, `light` ou `custom` |
| `dup_read` | `true` | Bloquer les relectures complètes de fichiers inchangés |
| `budget.enabled` / `budget.tokens` | `true` / `150000` | Pause budget et sa taille |
| `handoff.enabled` / `handoff.tokens` | `true` / `120000` | Passation et sa taille |
| `handoff.mode` | `"note"` | `note` (écrire une note, la charger après `/clear`) ou `remind` (seulement suggérer `/compact`) |
| `handoff.max_age_hours` | `48` | Les notes plus anciennes ne sont pas chargées |
| `mcp_prune.enabled` / `unused_days` | `false` / `14` | Couper les serveurs MCP qu'un projet n'utilise pas |
| `desktop.allowed_dirs` | `[]` | Dossiers que l'application de bureau peut lire |

Mettez `SQUEEZE_SKIP` n'importe où dans un message pour passer la pause budget.

## Commandes

| Commande | Ce qu'elle fait |
|---|---|
| `squeeze auditor` | Ouvrir la fenêtre squeeze-Auditor |
| `squeeze setup [--claude-code] [--desktop] [--remove]` | Connecter ou déconnecter Claude Code (tous les projets) et l'application de bureau |
| `squeeze doctor [--dir D] [--desktop] [--json]` | Coût fixe par message, dans le terminal |
| `squeeze mcp` | Le serveur MCP que lance l'application de bureau |
| `squeeze init`, `run`, `compress`, `clip`, `report`, `bench`, `uninstall` | Comme dans la [v0.1](../../../v0.1-Sentinel/v0.1.0_squeeze-sentinel/README/README.fr.md#utilisation) |

## Confidentialité et sécurité

- **Uniquement en local.** Aucun code réseau, hormis la page `127.0.0.1` de la fenêtre, qui exige une clé changeant à chaque lancement.
- **Sauvegardes.** Avant de modifier `~/.claude/settings.json`, `~/.claude.json` ou `claude_desktop_config.json`, squeeze copie l'original dans `~/.squeeze/backups/`.
- **Lecture seule autant que possible.** squeeze lit les journaux de session de Claude Code pour les chiffres ; il ne les modifie jamais et ne lit jamais vos identifiants.
- **Mesurer un serveur MCP** lance sa propre commande quelques secondes pour demander sa liste d'outils, comme le ferait Claude Code.

## FAQ

**Q. La pause budget ne va-t-elle pas gêner ?**
R. Elle ne retient un message qu'une fois par palier. Renvoyez le même message et il passe. Vous pouvez relever le budget, choisir Léger ou la désactiver dans Réglages.

**Q. La passation fait-elle perdre du contexte ?**
R. Un peu de détail, volontairement : la note garde l'objectif, ce qui est fait, les décisions, les fichiers touchés et les prochaines étapes. Les notes sont de simples fichiers dans `.squeeze/handoff/` ; lisez-les ou modifiez-les avant `/clear`. Vous préférez décider vous-même ? Choisissez le style « Seulement me le rappeler ».

**Q. Pourquoi pas le navigateur et le téléphone ?**
R. Un onglet de navigateur ou une application mobile ne peut pas faire tourner un programme installé à côté de Claude. La v0.3 adoptera une autre approche.

**Q. Le tableau de bord montre-t-il la limite de mon offre ?**
R. Non. Il montre ce que disent les journaux de Claude Code sur cet ordinateur. Anthropic ne propose pas de moyen officiel pour que d'autres programmes lisent l'usage de l'offre.

**Q. Puis-je revenir à la v0.1 ?**
R. Oui : `squeeze setup --remove`, puis installez la v0.1.0 (`SQUEEZE_VERSION=v0.1.0` avec le script d'installation).

## Retours

Laissez améliorations et problèmes dans les [Issues](https://github.com/jongwooish/claude-squeeze/issues/new/choose) : un blocage déclenché à tort, un souci d'installateur, des chiffres qui semblent faux, des idées pour la v0.3 et la v1.0. Les questions vont dans [Discussions](https://github.com/jongwooish/claude-squeeze/discussions). L'une des six langues convient.

## Contribuer

```bash
cargo test --manifest-path CARGO/Cargo.toml
cargo clippy --manifest-path CARGO/Cargo.toml --all-targets -- -D warnings
cargo run --manifest-path CARGO/Cargo.toml --release -- bench v0.2-Auditor/v0.2.0_squeeze-auditor/testdata/fixtures
cargo run --manifest-path CARGO/Cargo.toml -- auditor     # ouvrir la fenêtre depuis les sources
```

Plan du code : `src/guards.rs` (lecture répétée, budget, passation), `src/mcp.rs` et `src/doctor.rs` (coût fixe), `src/transcripts.rs` et `src/usage.rs` (chiffres du tableau de bord), `src/auditor.rs` et `ui/index.html` (la fenêtre), `src/mcp_server.rs` (outils de l'application de bureau), `packaging/` (installateurs). Guide utilisateur en coréen : [docs/guide](../docs/guide/index.md).
