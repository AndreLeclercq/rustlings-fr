<div class="oranda-hide">

# Rustlings-FR 🦀 ❤️ 🇫🇷

</div>

> Ceci est la version française du projet [Rustlings 🇬🇧](https://github.com/rust-lang/rustlings). Les exercices ont été traduits pour faciliter l'apprentissage de Rust pour les francophones.

> ⚠️ Attention : Ce projet est actuellement en cours de traduction, la première version brut est disponible pour review. **Nous avons aussi créé des [issues](https://github.com/AndreLeclercq/rustlings-fr/issues) dédiées pour chaque chapitre afin de recueillir vos retours d'expérience, essentiels pour valider la qualité du projet avant sa sortie officielle.** ⚠️

Salut et bienvenue dans Rustlings.
Ce projet contient de petits exercices pour t'habituer à lire et écrire du code Rust.
Cela inclut la lecture et la compréhension des messages du compilateur !

Il est recommandé de faire les exercices Rustlings en parallèle de la lecture du [livre officiel Rust 🇬🇧](https://doc.rust-lang.org/book/)([Traduction FR 🇫🇷](https://jimskapt.github.io/rust-book-fr/)), la ressource la plus complète pour apprendre Rust 📚️

[Rust By Example 🇬🇧](https://doc.rust-lang.org/rust-by-example/) est une autre ressource recommandée qui pourrait t'être utile.
Il contient des exemples de code et des exercices similaires à Rustlings, mais en ligne.

## Démarrage

### Installation de Rust

Avant d'installer Rustlings, tu dois avoir la **dernière version de Rust** installée.
Visite [www.rust-lang.org/fr/tools/install 🇫🇷](https://www.rust-lang.org/fr/tools/install) pour plus d'instructions sur l'installation de Rust.

Cela installera également _Cargo_, le gestionnaire de paquets/projets de Rust.

> 🐧 Si tu es sur Linux, assure-toi d'avoir installé `gcc` (pour un linker).
>
> Deb: `sudo apt install gcc`
>
> Dnf: `sudo dnf install gcc`

> 🍎 Si tu es sur MacOS, assure-toi d'avoir installé Xcode et ses outils de développement en exécutant `xcode-select --install`.

### Installation de Rustlings

La commande suivante téléchargera et compilera Rustlings :

```bash
cargo install rustlings
```

<details>
<summary><strong>Si l'installation échoue…</strong> (<em>cliquer pour développer</em>)</summary>

- Assure-toi d'avoir la dernière version de Rust en exécutant `rustup update`
- Essaie d'ajouter le flag `--locked` : `cargo install rustlings --locked`
- Sinon, merci de [signaler le problème 🇬🇧](https://github.com/rust-lang/rustlings/issues/new)

</details>

### Initialisation

Après avoir installé Rustlings, clone ce dépôt pour obtenir la version française des exercices :

```bash
git clone https://github.com/andreleclercq/rustlings-fr.git
```

Maintenant, va dans le répertoire nouvellement initialisé et lance Rustlings pour obtenir plus d'instructions sur comment commencer les exercices :

```bash
cd rustlings-fr/
rustlings
```

<details>
<summary><strong>Si la commande <code>rustlings</code> n'est pas trouvée…</strong> (<em>cliquer pour développer</em>)</summary>

Tu utilises probablement Linux et as installé Rust avec ton gestionnaire de paquets.

Cargo installe les binaires dans le répertoire `~/.cargo/bin`.
Malheureusement, les gestionnaires de paquets n'ajoutent souvent pas `~/.cargo/bin` à ta variable d'environnement `PATH`.

La solution est de...

- soit ajouter `~/.cargo/bin` manuellement au `PATH` `export PATH="$PATH:$HOME/.cargo/bin"`
- soit désinstaller Rust du gestionnaire de paquets et l'installer en utilisant la méthode officielle avec `rustup` : [www.rust-lang.org/fr/tools/install 🇫🇷](https://www.rust-lang.org/fr/tools/install)

</details>

## Environnement de travail

### Éditeur

Notre recommandation générale est [VS Code](https://code.visualstudio.com/) avec le [plugin rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
Mais n'importe quel éditeur qui prend en charge [rust-analyzer](https://rust-analyzer.github.io/) devrait être suffisant pour travailler sur les exercices.

### Terminal

Lorsque tu travailles avec Rustlings, utilise un terminal moderne pour la meilleure expérience utilisateur.
Le terminal par défaut sur Linux et Mac devrait être suffisant.
Sur Windows, nous recommandons le [Windows Terminal](https://aka.ms/terminal).

## Faire les exercices

Les exercices sont triés par sujet et peuvent être trouvés dans le sous-répertoire `exercises/<topic>`.
Pour chaque sujet, il y a un fichier `README.md` supplémentaire avec des ressources pour t'aider à démarrer sur le sujet.
Nous te recommandons vivement d'y jeter un œil avant de commencer 📚️

La plupart des exercices contiennent une erreur qui les empêche de compiler, et c'est à toi de la corriger !
Certains exercices contiennent des tests qui doivent passer pour que l'exercice soit considéré comme terminé ✅

Cherche les `TODO` et `todo!()` pour découvrir ce que tu dois changer.
Demande des indices en tapant `h` dans le _mode watch_ 💡

### Mode Watch

Après l'[initialisation](#initialisation), Rustlings peut être lancé simplement en exécutant la commande `rustlings`.

Cela démarrera le _mode watch_ qui te guide à travers les exercices dans un ordre prédéfini (ce que nous pensons être le meilleur pour les débutants).
Il réexécutera automatiquement l'exercice actuel chaque fois que tu modifies le fichier de l'exercice dans le répertoire `exercises/`.

<details>
<summary><strong>Si la détection des modifications de fichiers dans le répertoire <code>exercises/</code> échoue…</strong> (<em>cliquer pour développer</em>)</summary>

> Tu peux ajouter le flag **`--manual-run`** (`rustlings --manual-run`) pour réexécuter manuellement l'exercice actuel en tapant `r` dans le mode watch.
>
> Merci de [signaler le problème 🇬🇧](https://github.com/rust-lang/rustlings/issues/new) avec quelques informations sur ton système d'exploitation et si tu exécutes Rustlings dans un conteneur ou une machine virtuelle (par exemple WSL).

</details>

### Liste des exercices

Dans le [mode watch](#mode-watch) (après avoir lancé `rustlings`), tu peux entrer `l` pour ouvrir la liste interactive des exercices.

La liste te permet de...

- Voir le statut de tous les exercices (terminé ou en attente)
- `c` : Continuer à un autre exercice (sauter temporairement certains exercices ou revenir à un précédent)
- `r` : Réinitialiser le statut et le fichier de l'exercice sélectionné (tu devras _recharger/rouvrir_ son fichier dans ton éditeur par la suite)

Regarde le pied de page de la liste pour toutes les touches possibles.

## Questions ?

Si tu as besoin d'aide pendant les exercices et que les indices intégrés ne sont pas utiles, n'hésite pas à poser ta question dans la [catégorie _Q&A_ des discussions 🇬🇧](https://github.com/rust-lang/rustlings/discussions/categories/q-a?discussions_q=) si ta question n'a pas encore été posée 💡

## Exercices tiers

Les exercices tiers sont un ensemble d'exercices maintenus par la communauté.
Tu peux utiliser le même programme `rustlings` que tu as installé avec `cargo install rustlings` pour les exécuter :

- 🇯🇵 [Japanese Rustlings](https://github.com/sotanengel/rustlings-jp) : Une traduction japonaise des exercices Rustlings.
- 🇨🇳 [Simplified Chinese Rustlings](https://github.com/SandmeyerX/rustlings-zh-cn) : Une traduction chinoise simplifiée des exercices Rustlings.
- 🇫🇷 [French Rustlings](https://github.com/andreleclercq/rustlings-fr) : La traduction française des exercices Rustlings.

Tu veux créer ton propre ensemble d'exercices Rustlings pour te concentrer sur un sujet spécifique ?
Ou tu veux traduire les exercices originaux de Rustlings ?
Alors suis le guide sur les [exercices tiers 🇬🇧](https://github.com/rust-lang/rustlings/blob/main/THIRD_PARTY_EXERCISES.md) !

## Continuer

Une fois que tu as terminé Rustlings, mets tes nouvelles connaissances à profit !
Continue à pratiquer tes compétences en Rust en construisant tes propres projets, en contribuant à Rustlings, ou en trouvant d'autres projets open-source auxquels contribuer.

## Désinstallation de Rustlings

Si tu veux supprimer Rustlings de ton système, exécute la commande suivante :

```bash
cargo uninstall rustlings
```

## Contribuer

Pour contribuer à la version originale en anglais, consulte [CONTRIBUTING.md](https://github.com/rust-lang/rustlings/blob/main/CONTRIBUTING.md) 🔗

Les contributions à la version française sont désormais acceptées ! Si tu souhaites améliorer la traduction, ajouter des explications ou signaler des problèmes, consulte notre [guide de contribution](./CONTRIBUTING.md) qui détaille les différentes façons de participer au projet.
