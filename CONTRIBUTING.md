# Contribuer à Rustlings FR

Merci de ton intérêt pour contribuer à la version française de Rustlings ! Ce document explique comment tu peux nous aider à améliorer la traduction.

## À propos de ce projet

Rustlings FR est une traduction française du projet [Rustlings](https://github.com/rust-lang/rustlings). L'objectif est de rendre l'apprentissage de Rust plus accessible aux francophones.

## Comment contribuer

### Contribuer à la traduction

#### Option 1 : Via une Pull Request (pour les modifications importantes)

Si tu souhaites améliorer la traduction existante ou traduire de nouvelles parties :

1. **Fork** le dépôt
2. Crée une **branche** pour tes modifications (`git checkout -b amelioration-traduction`)
3. **Commit** tes changements (`git commit -m 'Améliore la traduction de X'`)
4. **Push** ta branche (`git push origin amelioration-traduction`)
5. Ouvre une **Pull Request**

#### Option 2 : Via une Issue (plus simple)

Si tu préfères ne pas passer par le processus de fork et de Pull Request :

1. Ouvre une **Issue** avec un titre clair décrivant ta proposition
   - Exemple : "Correction : erreur dans la section variables" ou "Suggestion : amélioration explication des ownership"
   - Utilise les tags appropriés : `correction` (pour les erreurs), `suggestion` (pour les améliorations), `bug` (pour les problèmes techniques liés à la traduction)
2. Décris clairement :
   - Le fichier/la section à modifier
   - Le texte original
   - Ta proposition de traduction
   - (Optionnel) Une explication de ton choix de traduction

Les mainteneurs pourront ensuite implémenter ta suggestion s'ils la trouvent pertinente.

### Principes de traduction

Pour maintenir la cohérence :

- Utilise le **tutoiement** avec un ton détendu mais professionnel
- Conserve les **termes techniques** en anglais avec une traduction entre parenthèses : `ownership (possession)`
- Essaie de rester proche de l'original en longueur
- Ne modifie **jamais** le code (même s'il contient des erreurs, c'est intentionnel !)
- Tu peux traduire les textes dans les fonctions `print` et commentaires

### Signaler un problème

Si tu trouves un problème dans la traduction :

1. Vérifie d'abord que le problème n'a pas déjà été signalé
2. Ouvre une **Issue** en décrivant précisément le problème et où il se trouve
3. Utilise les tags appropriés :
   - `correction` - Pour les erreurs de traduction
   - `suggestion` - Pour les propositions d'amélioration
   - `bug` - Pour les problèmes techniques liés à la traduction

## Distinction importante

### Pour les problèmes de traduction

Utilise ce dépôt pour tout ce qui concerne la traduction française.

### Pour les bugs ou améliorations du code original

Si tu trouves un bug ou souhaites proposer une amélioration au code original de Rustlings :

1. Vérifie d'abord si le problème n'est pas lié à la traduction
2. Si le problème vient du projet original, reporte-le sur le [dépôt officiel de Rustlings](https://github.com/rust-lang/rustlings/issues)

## Processus de révision

Toutes les Pull Requests seront examinées par les mainteneurs. Nous pourrons te demander d'apporter des modifications pour maintenir la cohérence du style.

Merci de contribuer à rendre Rust plus accessible à la communauté francophone !
