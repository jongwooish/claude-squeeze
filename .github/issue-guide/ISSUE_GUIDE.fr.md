# 📌 Guide des issues

[English](ISSUE_GUIDE.md) | [한국어](ISSUE_GUIDE.ko.md) | [日本語](ISSUE_GUIDE.ja.md) | [简体中文](ISSUE_GUIDE.zh-CN.md) | [Español](ISSUE_GUIDE.es.md) | **Français**

Bonjour, et merci de votre visite ! 👋
claude-squeeze s'améliore grâce aux personnes qui nous disent ce qui n'a pas marché et ce qu'elles aimeraient. Chaque retour compte, même d'une seule ligne. Cette page explique comment écrire une issue qui sera corrigée rapidement.

### Ce que vous pouvez publier ici

- 🐞 **Quelque chose ne marche pas** : squeeze a planté, affiché quelque chose d'étrange, ou une session Claude Code s'est comportée bizarrement
- ✂️ **La compression semblait fausse** : une ligne importante a disparu, trop de lignes ont été gardées, ou une commande a (ou n'a pas) été redirigée
- 📦 **Problème d'installation, de mise à jour ou de version** : l'installateur a échoué, un fichier de release manquait, ou quelque chose a changé après une mise à jour
- 💡 **Idées et améliorations** : nouvelles fonctionnalités, meilleurs réglages par défaut, et ce que vous aimeriez voir dans la **v1.0**
- ❓ **Questions** : pas sûr que ce soit un bug ? Demandez quand même, personne ne s'en offusquera.

### Avant de publier (2 minutes)

1. **Vérifiez votre version** : lancez `squeeze version`. Si ce n'est pas la [dernière release](https://github.com/jongwooish/claude-squeeze/releases/latest), mettez à jour d'abord. Le problème est peut-être déjà corrigé.
2. **Cherchez dans les issues existantes** : quelqu'un l'a peut-être déjà signalé. Dans ce cas, ajoutez un 👍 ou un commentaire avec vos détails plutôt qu'une nouvelle issue.
3. **Parcourez la [FAQ](https://github.com/jongwooish/claude-squeeze/blob/main/README/README.fr.md#faq)** : les questions courantes y sont traitées.

### Choisissez le bon formulaire

Cliquez sur **New issue** et choisissez :

| Si… | Choisissez |
|---|---|
| squeeze a planté, donné une sortie fausse ou cassé une session | **Bug report / 버그 신고** |
| une ligne importante a disparu, ou une commande a (ou n'a pas) été interceptée | **Compression problem / 압축 문제** |
| le problème vient de l'installation, d'une mise à jour ou d'une version précise | **Install / update / version problem / 설치·업데이트·버전 문제** |
| vous avez une idée ou un souhait pour la v1.0 | **Feature idea / 기능 제안** |

Un doute ? Choisissez le plus proche, nous le déplacerons si besoin.

### Ce qui fait un excellent rapport

- **Votre environnement** : version de squeeze, système d'exploitation et shell (sous Windows : Git Bash, WSL ou PowerShell)
- **La commande exacte** lancée par vous ou par Claude
- **Ce que vous attendiez** et **ce qui s'est réellement passé**
- **La sortie en texte**, pas en capture d'écran (les captures sont bienvenues en plus)
- **Les étapes pour reproduire**, aussi courtes que possible

Commandes pratiques pour rassembler les informations :

```bash
squeeze version            # votre version
squeeze report --json      # ce que squeeze a récemment gardé hors du contexte
SQUEEZE_SKIP=1 <commande>  # la sortie brute, pour la comparer au résumé
```

Quand squeeze compresse une sortie, la dernière ligne indique où le log complet a été enregistré (`.squeeze/raw/...`). Joindre ce fichier (après l'avoir nettoyé, voir ci-dessous) aide énormément pour les problèmes de compression.

### 🔒 Protégez vos secrets

Les logs peuvent contenir des choses à ne pas publier. Avant de coller, retirez ou remplacez :

- Clés d'API, jetons, mots de passe, cookies
- Noms d'hôtes internes, adresses IP, noms d'entreprise
- Chemins personnels comme `C:\Users\votre-nom\...` ou `/home/votre-nom/...` (remplacez par `~/`)

Vous avez trouvé un **problème de sécurité** ? Ne le publiez pas. Suivez [SECURITY.md](../SECURITY.md) pour le signaler en privé.

### Et ensuite ?

- Nous lisons chaque issue et lui ajoutons une étiquette (`bug`, `enhancement`, `question`, …).
- C'est un projet open source personnel : une réponse peut prendre quelques jours. Merci de votre patience.
- Nous pourrons vous poser une question complémentaire. Y répondre est souvent le chemin le plus court vers une correction.
- Les corrections et nouveautés sont listées dans le [CHANGELOG](../../CHANGELOG/CHANGELOG.fr.md) et annoncées dans les [Releases](https://github.com/jongwooish/claude-squeeze/releases). Cliquez sur **Watch → Custom → Releases** pour être notifié.

### Quelques règles bienveillantes

- Un problème par issue, pour pouvoir suivre et fermer chacune.
- Restez aimable : tout le monde ici essaie d'aider.
- N'importe laquelle des six langues convient. L'anglais ou le coréen obtiennent la réponse la plus rapide.

Merci d'aider à améliorer claude-squeeze ! 🙏
