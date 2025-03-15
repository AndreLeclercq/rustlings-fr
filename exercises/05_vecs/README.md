# Vecteurs (Vectors)

Les vecteurs sont l'une des structures de données Rust les plus utilisées. Dans d'autres langages de programmation, on les appellerait simplement des Tableaux (Arrays), mais comme Rust opère à un niveau un peu plus bas, un tableau (array) en Rust est stocké sur la pile (stack) (ce qui signifie qu'il ne peut pas grandir ou rétrécir, et sa taille doit être connue à la compilation), tandis qu'un Vecteur (Vector) est stocké dans le tas (heap) (où ces restrictions ne s'appliquent pas).

Les vecteurs sont un chapitre un peu plus avancé dans le rust book, mais nous pensons qu'ils sont suffisamment utiles pour en parler un peu plus tôt. Nous parlerons de l'autre structure de données utile, les tables de hachage (hash maps), plus tard.

## Informations complémentaires
- [Stocker des Listes de Valeurs avec des Vecteurs](https://jimskapt.github.io/rust-book-fr/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
